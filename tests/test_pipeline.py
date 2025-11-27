"""
Tests for the Pipeline module.
"""

import pytest
from pathlib import Path

from rustify.engine.pipeline import Pipeline, PipelineContext
from rustify.engine.stage import Stage, StageResult, StageStatus


class MockStage(Stage[PipelineContext]):
    """Mock stage for testing."""
    
    name = "mock"
    description = "Mock stage for testing"
    
    def __init__(self, should_fail: bool = False):
        super().__init__()
        self.should_fail = should_fail
        self.executed = False
    
    async def execute(self, context: PipelineContext) -> PipelineContext:
        self.executed = True
        if self.should_fail:
            raise RuntimeError("Mock failure")
        context.metadata["mock_executed"] = True
        return context


@pytest.fixture
def context():
    """Create a test context."""
    return PipelineContext(
        source_path=Path("/tmp/source"),
        target_path=Path("/tmp/target"),
    )


@pytest.mark.asyncio
async def test_pipeline_runs_all_stages(context):
    """Test that pipeline runs all stages in order."""
    stage1 = MockStage()
    stage2 = MockStage()
    
    pipeline = Pipeline([stage1, stage2])
    result = await pipeline.run(context)
    
    assert stage1.executed
    assert stage2.executed
    assert result.is_success


@pytest.mark.asyncio
async def test_pipeline_stops_on_failure(context):
    """Test that pipeline stops when a stage fails."""
    stage1 = MockStage(should_fail=True)
    stage2 = MockStage()
    
    pipeline = Pipeline([stage1, stage2])
    result = await pipeline.run(context)
    
    assert stage1.executed
    assert not stage2.executed
    assert not result.is_success


@pytest.mark.asyncio
async def test_pipeline_records_stage_results(context):
    """Test that pipeline records results for each stage."""
    stage1 = MockStage()
    stage1.name = "stage1"
    stage2 = MockStage()
    stage2.name = "stage2"
    
    pipeline = Pipeline([stage1, stage2])
    result = await pipeline.run(context)
    
    assert "stage1" in result.stage_results
    assert "stage2" in result.stage_results
    assert result.stage_results["stage1"].status == StageStatus.SUCCESS
    assert result.stage_results["stage2"].status == StageStatus.SUCCESS


@pytest.mark.asyncio
async def test_context_metadata_preserved(context):
    """Test that context metadata is preserved through pipeline."""
    context.metadata["initial"] = "value"
    
    stage = MockStage()
    pipeline = Pipeline([stage])
    result = await pipeline.run(context)
    
    assert result.metadata["initial"] == "value"
    assert result.metadata["mock_executed"] is True

