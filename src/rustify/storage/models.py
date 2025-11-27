"""
Database Models - SQLModel definitions for storage.
"""

from datetime import datetime
from enum import Enum
from typing import Optional, List
from sqlmodel import Field, SQLModel, Relationship
import json


class TranslationStatus(str, Enum):
    """Status of a translation."""
    
    PENDING = "pending"
    ANALYZING = "analyzing"
    TRANSLATING = "translating"
    VALIDATING = "validating"
    FIXING = "fixing"
    COMPLETED = "completed"
    FAILED = "failed"


class Project(SQLModel, table=True):
    """Project model - represents a translation project."""
    
    __tablename__ = "projects"
    
    id: Optional[int] = Field(default=None, primary_key=True)
    name: str = Field(index=True)
    source_path: str
    target_path: str
    description: Optional[str] = None
    status: TranslationStatus = TranslationStatus.PENDING
    created_at: datetime = Field(default_factory=datetime.now)
    updated_at: datetime = Field(default_factory=datetime.now)
    metadata_json: Optional[str] = None
    
    # Relationships
    modules: List["Module"] = Relationship(back_populates="project")
    
    @property
    def metadata(self) -> dict:
        if self.metadata_json:
            return json.loads(self.metadata_json)
        return {}
    
    @metadata.setter
    def metadata(self, value: dict):
        self.metadata_json = json.dumps(value)


class Module(SQLModel, table=True):
    """Module model - represents a translation module."""
    
    __tablename__ = "modules"
    
    id: Optional[int] = Field(default=None, primary_key=True)
    project_id: int = Field(foreign_key="projects.id", index=True)
    name: str = Field(index=True)
    description: Optional[str] = None
    source_files_json: str = "[]"  # JSON list of file paths
    status: TranslationStatus = TranslationStatus.PENDING
    created_at: datetime = Field(default_factory=datetime.now)
    updated_at: datetime = Field(default_factory=datetime.now)
    
    # Relationships
    project: Optional[Project] = Relationship(back_populates="modules")
    records: List["TranslationRecord"] = Relationship(back_populates="module")
    
    @property
    def source_files(self) -> list[str]:
        return json.loads(self.source_files_json)
    
    @source_files.setter
    def source_files(self, value: list[str]):
        self.source_files_json = json.dumps(value)


class TranslationRecord(SQLModel, table=True):
    """Translation record - individual translation unit result."""
    
    __tablename__ = "translation_records"
    
    id: Optional[int] = Field(default=None, primary_key=True)
    module_id: int = Field(foreign_key="modules.id", index=True)
    unit_name: str = Field(index=True)
    unit_type: str  # function, struct, enum, etc.
    source_code: str
    rust_code: Optional[str] = None
    status: TranslationStatus = TranslationStatus.PENDING
    error_message: Optional[str] = None
    attempts: int = 0
    created_at: datetime = Field(default_factory=datetime.now)
    updated_at: datetime = Field(default_factory=datetime.now)
    metadata_json: Optional[str] = None
    
    # Relationships
    module: Optional[Module] = Relationship(back_populates="records")
    
    @property
    def metadata(self) -> dict:
        if self.metadata_json:
            return json.loads(self.metadata_json)
        return {}
    
    @metadata.setter
    def metadata(self, value: dict):
        self.metadata_json = json.dumps(value)


class TranslationExperience(SQLModel, table=True):
    """Translation experience - learned patterns for RAG."""
    
    __tablename__ = "translation_experiences"
    
    id: Optional[int] = Field(default=None, primary_key=True)
    pattern_type: str = Field(index=True)  # error_fix, idiom, optimization
    c_pattern: str  # Original C code pattern
    rust_pattern: str  # Corresponding Rust solution
    description: Optional[str] = None
    success_count: int = 0
    created_at: datetime = Field(default_factory=datetime.now)
    embedding_json: Optional[str] = None  # Vector embedding for similarity search
    
    @property
    def embedding(self) -> list[float]:
        if self.embedding_json:
            return json.loads(self.embedding_json)
        return []
    
    @embedding.setter
    def embedding(self, value: list[float]):
        self.embedding_json = json.dumps(value)

