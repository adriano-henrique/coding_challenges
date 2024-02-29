import asyncio
from websockets.sync.client import connect

async def hello():
    with connect('ws://localhost:7007') as websocket:
        print("Connected to server")
        while True:
            value = input()
            if value == "exit":
                print("Finishing connection")
                return
            websocket.send(value)
            print(websocket.recv())

if __name__ == '__main__':
    asyncio.run(hello())