"""
Database - SQLModel database operations.
"""

from pathlib import Path
from typing import Optional, List
from datetime import datetime
from loguru import logger

from sqlmodel import SQLModel, Session, create_engine, select
from sqlalchemy.ext.asyncio import create_async_engine, AsyncSession
from sqlalchemy.orm import sessionmaker

from rustify.config import StorageConfig, get_config
from rustify.storage.models import (
    Project,
    Module,
    TranslationRecord,
    TranslationStatus,
)


class Database:
    """
    Database manager for Rustify.
    
    Handles all database operations using SQLModel.
    """
    
    def __init__(self, url: str = None):
        self.url = url or get_config().storage.database_url
        self._engine = None
        self._async_engine = None
        self._session_factory = None
    
    def _get_engine(self):
        """Get or create sync engine."""
        if self._engine is None:
            # Convert async URL to sync if needed
            sync_url = self.url.replace("+aiosqlite", "")
            self._engine = create_engine(sync_url, echo=False)
        return self._engine
    
    def _get_async_engine(self):
        """Get or create async engine."""
        if self._async_engine is None:
            self._async_engine = create_async_engine(self.url, echo=False)
        return self._async_engine
    
    async def init(self) -> None:
        """Initialize database and create tables."""
        engine = self._get_async_engine()
        
        async with engine.begin() as conn:
            await conn.run_sync(SQLModel.metadata.create_all)
        
        logger.debug("Database initialized")
    
    def init_sync(self) -> None:
        """Initialize database synchronously."""
        engine = self._get_engine()
        SQLModel.metadata.create_all(engine)
        logger.debug("Database initialized (sync)")
    
    # Project operations
    
    def create_project(
        self,
        name: str,
        source_path: str,
        target_path: str,
        description: str = None
    ) -> Project:
        """Create a new project."""
        engine = self._get_engine()
        
        with Session(engine) as session:
            project = Project(
                name=name,
                source_path=source_path,
                target_path=target_path,
                description=description,
            )
            session.add(project)
            session.commit()
            session.refresh(project)
            return project
    
    def get_project(self, project_id: int) -> Optional[Project]:
        """Get project by ID."""
        engine = self._get_engine()
        
        with Session(engine) as session:
            return session.get(Project, project_id)
    
    def get_project_by_path(self, source_path: str) -> Optional[Project]:
        """Get project by source path."""
        engine = self._get_engine()
        
        with Session(engine) as session:
            statement = select(Project).where(Project.source_path == source_path)
            return session.exec(statement).first()
    
    def update_project_status(
        self,
        project_id: int,
        status: TranslationStatus
    ) -> None:
        """Update project status."""
        engine = self._get_engine()
        
        with Session(engine) as session:
            project = session.get(Project, project_id)
            if project:
                project.status = status
                project.updated_at = datetime.now()
                session.add(project)
                session.commit()
    
    # Module operations
    
    def create_module(
        self,
        project_id: int,
        name: str,
        source_files: list[str],
        description: str = None
    ) -> Module:
        """Create a new module."""
        engine = self._get_engine()
        
        with Session(engine) as session:
            module = Module(
                project_id=project_id,
                name=name,
                description=description,
            )
            module.source_files = source_files
            session.add(module)
            session.commit()
            session.refresh(module)
            return module
    
    def get_module(self, module_id: int) -> Optional[Module]:
        """Get module by ID."""
        engine = self._get_engine()
        
        with Session(engine) as session:
            return session.get(Module, module_id)
    
    def get_modules_by_project(self, project_id: int) -> List[Module]:
        """Get all modules for a project."""
        engine = self._get_engine()
        
        with Session(engine) as session:
            statement = select(Module).where(Module.project_id == project_id)
            return list(session.exec(statement).all())
    
    def update_module_status(
        self,
        module_id: int,
        status: TranslationStatus
    ) -> None:
        """Update module status."""
        engine = self._get_engine()
        
        with Session(engine) as session:
            module = session.get(Module, module_id)
            if module:
                module.status = status
                module.updated_at = datetime.now()
                session.add(module)
                session.commit()
    
    # Translation record operations
    
    def create_translation_record(
        self,
        module_id: int,
        unit_name: str,
        unit_type: str,
        source_code: str
    ) -> TranslationRecord:
        """Create a new translation record."""
        engine = self._get_engine()
        
        with Session(engine) as session:
            record = TranslationRecord(
                module_id=module_id,
                unit_name=unit_name,
                unit_type=unit_type,
                source_code=source_code,
            )
            session.add(record)
            session.commit()
            session.refresh(record)
            return record
    
    def update_translation_record(
        self,
        record_id: int,
        rust_code: str = None,
        status: TranslationStatus = None,
        error_message: str = None
    ) -> None:
        """Update a translation record."""
        engine = self._get_engine()
        
        with Session(engine) as session:
            record = session.get(TranslationRecord, record_id)
            if record:
                if rust_code is not None:
                    record.rust_code = rust_code
                if status is not None:
                    record.status = status
                if error_message is not None:
                    record.error_message = error_message
                record.attempts += 1
                record.updated_at = datetime.now()
                session.add(record)
                session.commit()
    
    def get_translation_records(
        self,
        module_id: int,
        status: TranslationStatus = None
    ) -> List[TranslationRecord]:
        """Get translation records for a module."""
        engine = self._get_engine()
        
        with Session(engine) as session:
            statement = select(TranslationRecord).where(
                TranslationRecord.module_id == module_id
            )
            if status:
                statement = statement.where(TranslationRecord.status == status)
            return list(session.exec(statement).all())


# Global database instance
_database: Optional[Database] = None


def get_database() -> Database:
    """Get or create global database instance."""
    global _database
    if _database is None:
        _database = Database()
        _database.init_sync()
    return _database


def set_database(database: Database) -> None:
    """Set global database instance."""
    global _database
    _database = database

