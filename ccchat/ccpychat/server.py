import asyncio
from websockets.server import serve

async def echo(websocket):
    i = 0
    name = await websocket.recv()
    print("Received name: " + name)
    async for message in websocket:
        print("Sending message: " + message)
        await websocket.send(name + ": " + message)

async def main():
    print('starting server')
    async with serve(echo, 'localhost', 7007):
        await asyncio.Future()  # run forever

if __name__ == '__main__':
    asyncio.run(main())