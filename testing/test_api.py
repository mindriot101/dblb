import pytest
import sys

sys.path.append(".")
from app import app, DBS
from starlette.testclient import TestClient


@pytest.fixture
def client():
    with TestClient(app) as client:
        yield client


def test_switching(client):
    assert DBS.idx == 0

    response = client.post("/switch")
    assert response.status_code == 200
    assert DBS.idx == 1
    assert response.json() == {"status": "ok"}

    response = client.post("/switch")
    assert response.status_code == 200
    assert DBS.idx == 0
    assert response.json() == {"status": "ok"}
