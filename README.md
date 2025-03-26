# TranslateToRust

A tool for automated translation of C/C++ solutions to Rust and testing them against original test cases, with extensive LLM integration for high-quality code conversion.

## Overview

TranslateToRust automates the code translation process from C/C++ to Rust using various Large Language Models (LLMs). The tool provides an end-to-end workflow from code translation to comprehensive testing and detailed test reports. It integrates with the FuzzForLeetcode project for test case support.

## Features

- Translate C/C++ code to Rust using various LLM providers (OpenAI, Anthropic, Google AI, etc.)
- Test translated Rust code against original test cases
- Generate comprehensive test reports in multiple formats (Markdown, JSON, CSV)
- Support for batch processing with resource-aware scheduling
- Multiple LLM support with configurable parameters (temperature, tokens, etc.)
- Robust timeout handling and process management
- Detailed test case reporting with performance metrics
- Model comparison capabilities across different LLM providers and configurations

## Prerequisites

- Rust (latest stable version)
- Python 3.8+
- Access to LLM APIs (OpenAI, Anthropic, Google AI, etc.)
- FuzzForLeetcode project (for test cases)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/TranslateToRust.git
cd TranslateToRust
```

2. Install Rust dependencies:
```bash
cargo build
```

3. Install Python dependencies:
```bash
pip install psutil
```

4. Configure LLM API:
   - Copy `llm_config.json.example` to `llm_config.json`
   - Edit `llm_config.json` with your API credentials
   - Alternatively, set environment variables (LLM_PROVIDER, LLM_API_KEY, etc.)

## Usage

### Single File Translation and Testing

1. Translate a solution:
```bash
cargo run -- translate --contest <contest_number> --problem <problem_number> --language <C|CPP> [--method llm] [--output-dir <path>]
```

2. Test a translated solution:
```bash
cargo run -- test --contest <contest_number> --problem <problem_number> --language <C|CPP> --rust-solution <path>
```

3. Run both translation and testing:
```bash
cargo run -- run --contest <contest_number> --problem <problem_number> --language <C|CPP> [--method llm]
```

### Batch Processing

See [README_BATCH.md](README_BATCH.md) for detailed batch processing instructions.

1. Batch translation:
```bash
python scripts/batch_translate.py [--language <C|CPP>] [--contest <number>] [--problem <number>]
```

2. Batch testing:
```bash
python scripts/batch_test.py [--directory <path>] [--timeout <seconds>]
```

3. LLM-specific testing:
```bash
python scripts/batch_test_llm.py [--skip-tests] [--skip-report]
```

4. Generate test reports:
```bash
python scripts/generate_test_reports.py [--days <number>] [--format <markdown|json|csv|all>] [--llm]
```

## Project Structure

```
TranslateToRust/
├── src/                    # Rust source code
│   ├── main.rs             # Main entry point
│   ├── translation.rs      # Translation logic
│   ├── testing.rs          # Testing logic
│   ├── llm_api.rs          # LLM API integration
│   └── utils.rs            # Utility functions
├── scripts/                # Python scripts
│   ├── batch_translate.py  # Batch translation script
│   ├── batch_test.py       # Batch testing script
│   ├── batch_test_llm.py   # LLM-specific testing script
│   ├── generate_test_reports.py # Report generation
│   └── timeout_handler.py  # Process timeout management
├── translated/             # Output directory for translated files
├── test_results/           # Test results directory
├── test_reports/           # Generated test reports
├── llm_translated/         # LLM-specific translated files
│   ├── claude-3-7-sonnet-20250219-default/  # Translated with Claude 3.7 Sonnet (default settings)
│   ├── gpt-4o-2024-11-20-default/          # Translated with GPT-4o (default settings)
│   └── ...                 # Other LLM model directories
├── llm_test_results/       # LLM-specific test results
│   ├── claude-3-7-sonnet-20250219-default/  # Test results for Claude 3.7 Sonnet translations
│   └── ...                 # Other LLM model directories
├── llm_test_reports/       # LLM-specific test reports
│   ├── claude-3-7-sonnet-20250219-default/  # Test reports for Claude 3.7 Sonnet translations
│   └── ...                 # Other LLM model directories
└── llm_configs/            # LLM configuration files
```

## Configuration

### LLM Configuration

Create `llm_config.json` with the following structure:

```json
{
    "provider": "OpenAI",
    "api_key": "your-api-key",
    "api_url": "https://api.openai.com/v1/chat/completions",
    "default_model": "gpt-4o",
    "model_params": {
        "temperature": 0.7,
        "max_tokens": 4000,
        "stream": true,
        "timeout": 300
    },
    "headers": {
        "content-type": "application/json"
    },
    "system_message": "Translate this C/C++ code to idiomatic Rust."
}
```

Supported providers:
- OpenAI (gpt-4o, gpt-4, etc.)
- Anthropic (claude-3-7-sonnet, etc.)
- GoogleAI (gemini-2.0-flash, etc.)
- Local (for testing)

## Test Reports

The test reports contain:
- Overall success rate and statistics
- Compilation status and errors
- Test case results and performance metrics 
- Language-specific statistics
- Detailed failure analysis
- LLM-specific performance comparisons

Reports are organized by:
- Contest and problem number
- Language (C/CPP)
- Test date and time
- LLM provider and specific model

## LLM Comparison

The project supports running the same translation tasks across multiple LLM providers and configurations, enabling direct comparison of translation quality and performance. Key comparison metrics include:
- Compilation success rate
- Test case pass rate
- Runtime performance
- Timeout frequency
- Error patterns

## License

This project is licensed under the MIT License - see the LICENSE file for details. 