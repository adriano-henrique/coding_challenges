import asyncio
from websockets.server import serve

CONNECTIONS = set()

async def register(websocket):
    global CONNECTIONS
    CONNECTIONS.add(websocket)
    try:
        await websocket.wait_closed()
    except:
        CONNECTIONS.remove(websocket)

async def chat_server(websocket):
    print("New connection from " + websocket.remote_address[0] + ":" + str(websocket.remote_address[1]))
    name = await websocket.recv()
    await register(websocket)
    print("Received name: " + name)
    async for message in websocket:
        print("Sending message: " + message)
        await websocket.send(name + ": " + message)

async def main():
    print('starting server')
    async with serve(chat_server, 'localhost', 7007):
        await asyncio.Future()  # run forever

if __name__ == '__main__':
    asyncio.run(main())