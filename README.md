# TranslateToRust

A tool for translating C/C++ solutions to Rust and testing them against original test cases.

## Features

- Translate C/C++ code to Rust using LLM (Large Language Models)
- Test translated Rust code against original test cases
- Generate comprehensive test reports
- Support batch processing for translation and testing
- Resource-aware execution with system load monitoring
- Robust timeout handling and process management
- Parallel processing with configurable workers
- Detailed test case reporting and analysis
- Support for multiple LLM providers

## Prerequisites

- Rust (latest stable version)
- Python 3.8+
- Access to LLM API (OpenAI, Anthropic, or Google AI)
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
pip install -r scripts/requirements.txt
```

4. Configure LLM API:
   - Copy `llm_config.json.example` to `llm_config.json`
   - Edit `llm_config.json` with your API credentials

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

1. Batch translation:
```bash
python scripts/batch_translate.py [--language <C|CPP>] [--contest <number>] [--problem <number>] [--method llm] [--timeout <seconds>] [--max-workers <number>]
```

2. Batch testing:
```bash
python scripts/batch_test.py [--directory <path>] [--timeout <seconds>] [--max-workers <number>] [--retry-count <number>]
```

3. Generate test reports:
```bash
python scripts/generate_test_reports.py [--days <number>] [--format <markdown|json>] [--include-details]
```

## Project Structure

```
TranslateToRust/
├── src/                    # Rust source code
│   ├── main.rs            # Main entry point
│   ├── translation.rs     # Translation logic
│   ├── testing.rs         # Testing logic
│   ├── llm_api.rs         # LLM API integration
│   └── utils.rs           # Utility functions
├── scripts/               # Python scripts
│   ├── batch_translate.py # Batch translation script
│   ├── batch_test.py     # Batch testing script
│   ├── generate_test_reports.py # Report generation
│   └── timeout_handler.py # Process timeout management
├── translated/           # Output directory for translated files
├── test_results/        # Test results directory
├── test_reports/        # Generated test reports
├── llm_translated/      # LLM-specific translated files
├── llm_test_results/    # LLM-specific test results
├── llm_test_reports/    # LLM-specific test reports
└── llm_configs/         # LLM configuration files
```

## Configuration

### LLM Configuration

Create `llm_config.json` with the following structure:

```json
{
    "provider": "OpenAI",
    "api_key": "your-api-key",
    "api_url": "https://api.openai.com/v1/chat/completions",
    "default_model": "gpt-4",
    "model_params": {
        "temperature": 0.7,
        "max_tokens": 2000
    },
    "headers": {
        "content-type": "application/json"
    }
}
```

Supported providers:
- OpenAI
- Anthropic
- GoogleAI
- Local (for testing)

### Environment Variables

You can also configure the LLM using environment variables:
- `LLM_PROVIDER`: Provider name
- `LLM_API_KEY`: API key
- `LLM_API_URL`: API endpoint URL
- `LLM_TEMPERATURE`: Temperature parameter
- `LLM_MAX_TOKENS`: Maximum tokens parameter
- `LLM_SYSTEM_MESSAGE`: System message for the LLM

## Test Reports

Test reports are generated in both Markdown and JSON formats, containing:
- Overall success rate
- Compilation status
- Test case results
- Timeout information
- Performance metrics
- Problem difficulty and tags (if available)
- Resource usage statistics
- Error analysis and categorization

Reports are organized by:
- Contest and problem number
- Language (C/C++)
- Test date and time
- LLM provider (if applicable)

## Advanced Features

### Resource Management

- System load monitoring
- Automatic delay when system is overloaded
- Configurable process timeouts
- Graceful process termination
- Memory usage tracking
- CPU utilization monitoring

### Concurrency Control

- File-based locking for cargo operations
- Configurable number of worker threads
- Resource-aware task scheduling
- Process tree cleanup
- Automatic retry mechanisms

### Error Handling

- Robust timeout handling
- Detailed error reporting
- Automatic retry mechanisms
- Process tree cleanup
- Error categorization and analysis
- Resource exhaustion protection

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details. 