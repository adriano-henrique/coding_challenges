import asyncio
from websockets.server import serve

async def echo(websocket):
    async for message in websocket:
        await websocket.send(message)

async def main():
    print('starting server')
    async with serve(echo, 'localhost', 7007):
        await asyncio.Future()  # run forever

if __name__ == '__main__':
    asyncio.run(main())