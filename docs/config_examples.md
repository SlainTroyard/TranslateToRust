# LLM配置示例

本文档提供了不同LLM提供商的配置示例，并说明哪些参数是可选的。

## 参数说明

所有`model_params`中的参数都是可选的。不同的模型支持不同的参数集，如果某个模型不支持特定参数，可以安全地省略。

## OpenAI配置示例

```json
{
  "provider": "OpenAI",
  "api_key": "your-api-key-here",
  "api_url": "https://api.openai.com/v1/chat/completions",
  "default_model": "gpt-4o",
  
  "model_params": {
    // 所有参数都是可选的
    "temperature": 0.2,         // 可选：控制随机性
    "max_tokens": 4000,         // 可选：最大输出标记数
    "top_p": 1.0,               // 可选：nucleus sampling概率阈值
    "frequency_penalty": 0.0,   // 可选：减少重复
    "presence_penalty": 0.0     // 可选：增加话题多样性
  },
  
  "headers": {
    "content-type": "application/json"
  },
  
  "system_message": "You are a C/C++ to Rust code translator expert."
}
```

## Anthropic配置示例

```json
{
  "provider": "Anthropic",
  "api_key": "your-anthropic-api-key-here",
  "api_url": "https://api.anthropic.com/v1/messages",
  "default_model": "claude-3-opus-20240229",
  
  "model_params": {
    // 所有参数都是可选的
    "temperature": 0.2,         // 可选：控制随机性
    "max_tokens": 4000,         // 可选：最大输出标记数
    "top_p": 0.9,               // 可选：控制采样多样性
    "top_k": null,              // 可选但不是所有模型都支持
    "stop_sequences": ["\n\nHuman:"]  // Anthropic特有参数
  },
  
  "headers": {
    "anthropic-version": "2023-06-01",
    "content-type": "application/json"
  },
  
  "system_message": "You are a C/C++ to Rust code translator expert."
}
```

## Google AI配置示例

```json
{
  "provider": "GoogleAI",
  "api_key": "your-google-ai-api-key-here",
  "api_url": "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro:generateContent",
  "default_model": "gemini-1.5-pro",
  
  "model_params": {
    // 所有参数都是可选的
    "temperature": 0.2,        // 可选：控制随机性
    "maxOutputTokens": 4000,   // Google特有参数，代替max_tokens
    "topP": 0.95,              // Google特有格式，代替top_p
    "topK": 40                 // Google特有参数
    // Google不支持frequency_penalty和presence_penalty
  },
  
  "headers": {
    "content-type": "application/json"
  },
  
  "system_message": "You are a C/C++ to Rust code translator expert."
}
```

## 参数兼容性表

下表显示了不同LLM提供商支持的参数：

| 参数 | OpenAI | Anthropic | Google AI |
|------|--------|-----------|-----------|
| temperature | ✅ | ✅ | ✅ |
| max_tokens | ✅ | ✅ | ❌ (使用maxOutputTokens) |
| maxOutputTokens | ❌ | ❌ | ✅ |
| top_p | ✅ | ✅ | ❌ (使用topP) |
| topP | ❌ | ❌ | ✅ |
| top_k | ❌ | ❌ (某些版本支持) | ❌ (使用topK) |
| topK | ❌ | ❌ | ✅ |
| frequency_penalty | ✅ | ❌ | ❌ |
| presence_penalty | ✅ | ❌ | ❌ |
| stop_sequences | ❌ | ✅ | ❌ |

## 特别注意

1. 在代码中，我们使用统一的结构体`ModelParams`来表示所有参数，所有字段都是`Option`类型，因此可以优雅地处理缺失的参数。

2. 此外，使用`extra_params`字段(一个HashMap)来存储结构体中未明确定义的参数，以支持不同模型的特殊参数。

3. 对于不同命名约定的参数(如`max_tokens`和`maxOutputTokens`)，代码会自动处理这些差异。 