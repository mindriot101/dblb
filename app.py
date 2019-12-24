from fastapi import FastAPI
from pydantic import BaseModel
from typing import List, Any, Dict
import databases
import logging

logging.basicConfig()
logger = logging.getLogger("dblb")
logger.setLevel(level=logging.DEBUG)


class DatabaseCollection(object):
    """A collection of databases which are switchable and async

    Responds to HTTP methods to manage database backends, and maps directly to
    `encoding/databases` API
    """
    def __init__(self, *urls):
        self.urls = urls
        self.idx = 0
        self.database = None

    async def create(self):
        await self.connect()

    async def next(self):
        self.idx = (self.idx + 1) % len(self.urls)
        await self.connect()

    async def connect(self):
        if self.database is not None:
            await self.database.disconnect()
        self.database = databases.Database(self.urls[self.idx])
        await self.database.connect()

    async def disconnect(self):
        if self.database is not None:
            await self.database.disconnect()

    async def fetch_all(self, *args, **kwargs):
        return await self.database.fetch_all(*args, **kwargs)


DBS = DatabaseCollection(
    "postgresql://crates.io:Password123@localhost/cargo_registry",
    "postgresql://crates.io:Password123@localhost/cargo_registry_test",
)


class QueryRequest(BaseModel):
    query: str
    params: Dict[str, Any]


class QueryResponse(BaseModel):
    result: List[Any]


class SwitchResponse(BaseModel):
    status: str


app = FastAPI()


@app.on_event("startup")
async def startup():
    await DBS.create()


@app.on_event("shutdown")
async def shutdown():
    await DBS.disconnect()


@app.post("/query", response_model=List[Any])
async def run_query(query: QueryRequest):
    return await DBS.fetch_all(query=query.query, values=query.params)


@app.post("/switch", response_model=SwitchResponse)
async def switch_backends():
    await DBS.next()
    return {"status": "ok"}
