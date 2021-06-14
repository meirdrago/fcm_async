import fcm_async
import asyncio
import os
import logging


FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO) # change this to debug

loop = asyncio.get_event_loop()

fcm_key = os.environ.get("FCM_KEY")
token = os.environ.get("TEST_TOKEN")

d = {"key1": "value1", "key2": "value2"}


async def f1():
    print("start")
    res = await cls.send_message(d, token)
    print("done")
    return res

# messages
cls = fcm_async.FcmClient(fcm_key)
res = loop.run_until_complete(f1())
assert res == True

cls = fcm_async.FcmClient("wrong key")
res = loop.run_until_complete(f1())
assert res == False

cls = fcm_async.FcmClient(fcm_key)
res = loop.run_until_complete(asyncio.gather(f1(), f1(), f1(), f1()))
assert res == [True, True, True, True]


#notifications
res = loop.run_until_complete(
                     cls.send_notification("this is the title", "this is the body", token))
assert res



