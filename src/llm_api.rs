use anyhow::{bail, Context, Error, Result};
use log::{debug, info, warn};
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};
use crate::translation::{LlmConfig, LlmProvider};

// This module contains implementations for LLM API calls

/// Make an API call to the configured LLM provider to translate code
pub fn translate_code_with_llm(prompt: &str, config: &LlmConfig) -> Result<String> {
    match config.provider {
        LlmProvider::OpenAI => call_openai_api(prompt, config),
        LlmProvider::Anthropic => call_anthropic_api(prompt, config),
        LlmProvider::GoogleAI => call_google_api(prompt, config),
        LlmProvider::Local => {
            warn!("Using local placeholder as no actual LLM API is configured");
            // Return the prompt as is to indicate this is a placeholder
            Ok(format!(
                "// This is a placeholder translation (Local mode)\n\n// Prompt that would be sent to LLM:\n/*\n{}\n*/\n\n// Add actual Rust code implementation here",
                prompt
            ))
        }
    }
}

fn call_openai_api(prompt: &str, config: &LlmConfig) -> Result<String> {
    info!("Calling OpenAI API with model: {}", config.default_model);
    info!("Using timeout of {} seconds", 
           config.model_params.extra_params.get("timeout")
              .and_then(|v| v.as_u64())
              .unwrap_or(300));
    
    // 创建一个带有更长超时时间的客户端
    let timeout = config.model_params.extra_params.get("timeout")
        .and_then(|v| v.as_u64())
        .unwrap_or(300);
    
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout))
        .build()?;
    
    info!("Sending request to URL: {}", config.api_url);
    
    // Prepare headers from config
    let mut request = client.post(&config.api_url)
        .header("Authorization", format!("Bearer {}", config.api_key));
    
    // Add any custom headers from config
    for (key, value) in &config.headers {
        request = request.header(key, value);
    }
    
    // Build request body with optional parameters
    let mut messages = vec![
        json!({
            "role": "user",
            "content": prompt
        })
    ];
    
    // Add system message if present
    if let Some(system_msg) = &config.system_message {
        messages.insert(0, json!({
            "role": "system",
            "content": system_msg
        }));
    }
    
    // Create base request payload
    let mut payload = json!({
        "model": config.default_model,
        "messages": messages
    });
    
    // Add optional parameters if they exist
    if let Some(temp) = config.model_params.temperature {
        payload["temperature"] = json!(temp);
    }
    
    if let Some(max_tokens) = config.model_params.max_tokens {
        payload["max_tokens"] = json!(max_tokens);
        debug!("Setting max_tokens: {}", max_tokens);
    }
    
    if let Some(top_p) = config.model_params.top_p {
        payload["top_p"] = json!(top_p);
    }
    
    if let Some(freq_penalty) = config.model_params.frequency_penalty {
        payload["frequency_penalty"] = json!(freq_penalty);
    }
    
    if let Some(pres_penalty) = config.model_params.presence_penalty {
        payload["presence_penalty"] = json!(pres_penalty);
    }
    
    // Add any extra parameters from the config
    for (key, value) in &config.model_params.extra_params {
        if key != "timeout" {  // 跳过timeout，它只用于客户端设置
            payload[key] = value.clone();
        }
    }
    
    // 确保启用流式模式
    payload["stream"] = json!(true);
    
    debug!("OpenAI request payload: {}", payload);
    
    info!("Sending request to LLM API...");
    
    // 使用流式模式发送请求
    let response = request.json(&payload).send()
        .with_context(|| format!("Failed to send request to OpenAI API at {}", config.api_url))?;
    
    // Check response status
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text()
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        return Err(anyhow::anyhow!("OpenAI API error ({}): {}", 
            status, error_text));
    }
    
    // 处理流式响应
    let text = response.text()?;
    let mut full_content = String::new();
    
    // 记录原始响应文本
    debug!("Raw response text (truncated): {}", text.chars().take(500).collect::<String>());
    
    // 处理每一行作为单独的SSE事件
    for line in text.lines() {
        if line.starts_with("data:") && !line.contains("data: [DONE]") {
            // 跳过"data: "前缀
            let json_content = if line.len() > 6 { &line[6..] } else { continue };
            
            debug!("Processing stream line: {}", json_content);
            
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_content) {
                debug!("Parsed JSON: {}", json);
                
                // 正常情况下从delta中提取内容
                if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                    debug!("Found content in delta: {}", content);
                    full_content.push_str(content);
                }
                // 对于某些API，可能是在message中而不是delta
                else if let Some(content) = json["choices"][0]["message"]["content"].as_str() {
                    debug!("Found content in message: {}", content);
                    full_content.push_str(content);
                }
                else {
                    debug!("Could not find content in JSON structure: {:?}", json);
                }
            } else {
                debug!("Failed to parse JSON: {}", json_content);
            }
        } else if line.contains("data: [DONE]") {
            debug!("End of stream marker found");
        }
    }
    
    if full_content.is_empty() {
        warn!("No content extracted from stream response");
    }
    
    debug!("Response combined. Total length: {}", full_content.len());
    Ok(full_content)
}

fn call_anthropic_api(prompt: &str, config: &LlmConfig) -> Result<String> {
    info!("Calling Anthropic API with model: {}", config.default_model);
    info!("Using timeout of {} seconds", 
           config.model_params.extra_params.get("timeout")
              .and_then(|v| v.as_u64())
              .unwrap_or(300));
    
    // 创建一个带有更长超时时间的客户端
    let timeout = config.model_params.extra_params.get("timeout")
        .and_then(|v| v.as_u64())
        .unwrap_or(300);
    
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout))
        .build()?;
    
    info!("Sending request to URL: {}", config.api_url);
    
    // Prepare headers for Anthropic API
    let mut request = client.post(&config.api_url)
        .header("x-api-key", &config.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json");
    
    // Add any custom headers from config
    for (key, value) in &config.headers {
        request = request.header(key, value);
    }
    
    // 检查是否有流式处理设置
    let use_stream = config.model_params.extra_params.get("stream")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);  // 默认使用流式处理
    
    // 使用最新的Claude Messages API格式构建请求体
    let mut messages = vec![
        json!({
            "role": "user",
            "content": prompt
        })
    ];
    
    // Add system message if present
    if let Some(system_msg) = &config.system_message {
        messages.insert(0, json!({
            "role": "system",
            "content": system_msg
        }));
    }
    
    // 为Claude API构建请求体
    let mut payload = json!({
        "model": config.default_model,
        "messages": messages,
        "stream": use_stream
    });
    
    // Add optional parameters if they exist
    if let Some(temp) = config.model_params.temperature {
        payload["temperature"] = json!(temp);
    }
    
    if let Some(max_tokens) = config.model_params.max_tokens {
        payload["max_tokens"] = json!(max_tokens);
        debug!("Setting max_tokens: {}", max_tokens);
    }
    
    if let Some(top_p) = config.model_params.top_p {
        payload["top_p"] = json!(top_p);
    }
    
    if let Some(top_k) = config.model_params.top_k {
        payload["top_k"] = json!(top_k);
    }
    
    // Add any extra parameters from the config
    for (key, value) in &config.model_params.extra_params {
        if key != "timeout" && key != "stream" {  // 跳过timeout和stream，它们只用于客户端设置
            payload[key] = value.clone();
        }
    }
    
    debug!("Anthropic request payload: {}", payload);
    
    info!("Sending request to Anthropic API...");
    
    // 发送请求
    let response = request.json(&payload).send()
        .with_context(|| format!("Failed to send request to Anthropic API at {}", config.api_url))?;
    
    // Check response status
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text()
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        return Err(anyhow::anyhow!("Anthropic API error ({}): {}", 
            status, error_text));
    }
    
    // 处理响应
    let text = response.text()?;
    let mut full_content = String::new();
    
    // 记录原始响应文本
    debug!("Raw response text (truncated): {}", text.chars().take(500).collect::<String>());
    
    if use_stream {
        // 处理流式响应
        for line in text.lines() {
            if line.starts_with("data:") && line != "data: [DONE]" {
                // 跳过"data: "前缀
                let json_content = if line.len() > 6 { &line[6..] } else { continue };
                
                debug!("Processing stream line: {}", json_content);
                
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_content) {
                    debug!("Parsed JSON: {}", json);
                    
                    // 从新的Claude Messages API响应格式中提取内容
                    if let Some(delta) = json["delta"]["text"].as_str() {
                        debug!("Found content in delta: {}", delta);
                        full_content.push_str(delta);
                    } else {
                        debug!("Could not find delta.text in JSON: {:?}", json);
                    }
                } else {
                    debug!("Failed to parse JSON: {}", json_content);
                }
            } else if line == "data: [DONE]" {
                debug!("End of stream marker found");
            }
        }
    } else {
        // 处理非流式响应
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            // 从新的Claude Messages API响应中获取内容
            if let Some(content) = json["content"][0]["text"].as_str() {
                full_content = content.to_string();
            } else {
                warn!("Could not find content[0].text in Anthropic response");
            }
        } else {
            warn!("Failed to parse Anthropic response as JSON");
        }
    }
    
    if full_content.is_empty() {
        warn!("No content extracted from Anthropic response");
    }
    
    debug!("Response combined. Total length: {}", full_content.len());
    Ok(full_content)
}

fn call_google_api(prompt: &str, config: &LlmConfig) -> Result<String> {
    info!("Calling Google AI API with model: {}", config.default_model);
    info!("Using timeout of {} seconds", 
           config.model_params.extra_params.get("timeout")
              .and_then(|v| v.as_u64())
              .unwrap_or(300));
    
    // 创建一个带有更长超时时间的客户端
    let timeout = config.model_params.extra_params.get("timeout")
        .and_then(|v| v.as_u64())
        .unwrap_or(300);
    
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout))
        .build()?;
    
    info!("Sending request to URL: {}", config.api_url);
    
    // Prepare headers for Google AI API
    // Google API需要在URL中添加API Key
    let url_with_key = format!("{}?key={}", config.api_url, config.api_key);
    let mut request = client.post(&url_with_key);
    
    // Add any custom headers from config
    for (key, value) in &config.headers {
        request = request.header(key, value);
    }
    
    // 检查是否有流式处理设置
    let use_stream = config.model_params.extra_params.get("stream")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);  // 默认使用流式处理
    
    // 构建Google AI Gemini API请求结构
    let mut content = vec![
        json!({
            "role": "user",
            "parts": [
                {
                    "text": prompt
                }
            ]
        })
    ];
    
    // 添加系统消息（如果有）
    if let Some(system_msg) = &config.system_message {
        content.insert(0, json!({
            "role": "system",
            "parts": [
                {
                    "text": system_msg
                }
            ]
        }));
    }
    
    // 构建完整的请求体
    let mut generation_config = json!({});
    
    // Add optional parameters to generationConfig
    if let Some(temp) = config.model_params.temperature {
        generation_config["temperature"] = json!(temp);
    }
    
    // Google Gemini使用maxOutputTokens而不是max_tokens
    if let Some(max_tokens) = config.model_params.max_tokens {
        generation_config["maxOutputTokens"] = json!(max_tokens);
        debug!("Setting maxOutputTokens: {}", max_tokens);
    } else if let Some(max_output_tokens) = config.model_params.extra_params.get("maxOutputTokens") {
        generation_config["maxOutputTokens"] = max_output_tokens.clone();
        debug!("Setting maxOutputTokens from extra_params: {}", max_output_tokens);
    }
    
    if let Some(top_p) = config.model_params.top_p {
        generation_config["topP"] = json!(top_p);
    }
    
    if let Some(top_k) = config.model_params.top_k {
        generation_config["topK"] = json!(top_k);
    }
    
    // Add any extra parameters from the config to generationConfig
    for (key, value) in &config.model_params.extra_params {
        if key != "timeout" && key != "stream" && key != "maxOutputTokens" {
            generation_config[key] = value.clone();
        }
    }
    
    // 构建最终的请求体
    let mut payload = json!({
        "contents": content
    });
    
    // 只有当有参数时才添加generationConfig
    if !generation_config.as_object().unwrap().is_empty() {
        payload["generationConfig"] = generation_config;
    }
    
    // 添加流式参数
    if use_stream {
        payload["stream"] = json!(true);
    }
    
    debug!("Google AI request payload: {}", payload);
    
    info!("Sending request to Google AI API...");
    
    // 发送请求
    let response = request.json(&payload).send()
        .with_context(|| format!("Failed to send request to Google AI API at {}", config.api_url))?;
    
    // Check response status
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text()
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        return Err(anyhow::anyhow!("Google AI API error ({}): {}", 
            status, error_text));
    }
    
    // 处理响应
    let text = response.text()?;
    let mut full_content = String::new();
    
    // 记录原始响应文本
    debug!("Raw response text (truncated): {}", text.chars().take(500).collect::<String>());
    
    if use_stream {
        // 处理流式响应（Google API的流式格式）
        for line in text.lines() {
            // 移除开头的 "data: "
            if !line.starts_with("data: ") && line != "[DONE]" {
                debug!("Unexpected line format: {}", line);
                continue;
            }
            
            let json_content = if line.starts_with("data: ") { &line[6..] } else { line };
            
            if json_content == "[DONE]" {
                debug!("End of stream marker found");
                continue;
            }
            
            // 尝试解析JSON
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_content) {
                debug!("Parsed JSON: {}", json);
                
                // 在Google AI响应中提取内容
                if let Some(candidates) = json["candidates"].as_array() {
                    for candidate in candidates {
                        if let Some(content) = candidate["content"]["parts"][0]["text"].as_str() {
                            debug!("Found content in part: {}", content);
                            full_content.push_str(content);
                        } else if let Some(delta) = candidate["delta"]["textDelta"].as_str() {
                            // 新的格式可能使用delta.textDelta
                            debug!("Found content in delta: {}", delta);
                            full_content.push_str(delta);
                        } else {
                            debug!("Could not find text in candidate part or delta");
                        }
                    }
                } else {
                    debug!("Could not find candidates in JSON: {:?}", json);
                }
            } else if !json_content.is_empty() {
                debug!("Failed to parse JSON: {}", json_content);
            }
        }
    } else {
        // 处理非流式响应
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(candidates) = json["candidates"].as_array() {
                if !candidates.is_empty() {
                    if let Some(parts) = candidates[0]["content"]["parts"].as_array() {
                        // 合并所有部分的文本内容
                        for part in parts {
                            if let Some(text) = part["text"].as_str() {
                                full_content.push_str(text);
                            }
                        }
                    } else {
                        warn!("Could not find parts array in Google AI response");
                    }
                } else {
                    warn!("Empty candidates array in Google AI response");
                }
            } else {
                warn!("Could not find candidates in Google AI response");
            }
        } else {
            warn!("Failed to parse Google AI response as JSON");
        }
    }
    
    if full_content.is_empty() {
        warn!("No content extracted from Google AI response");
    }
    
    debug!("Response combined. Total length: {}", full_content.len());
    Ok(full_content)
} 