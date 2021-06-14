# fcm_async

Async Fcm (Firebase Cloud Messages) library.

Thin wrapper over rust [fcm](https://docs.rs/fcm/0.9.1/fcm/) library. 

## Install

todo

## Usage

```python
import fcm_async
import asyncio

loop = asyncio.get_event_loop()
cls = fcm_async.FcmClient(YOUR_FCM_KEY)

# as payload message
some_dict = {"key1": "value1", "key2": "value2"}
loop.run_until_complete(cls.send_message(some_dict, DEVICE_TOKEN))


# as notification
loop.run_until_complete(cls.send_notification("this is the title", "this is the body", DEVICE_TOKEN))

```



## build instructions

**Requirements:**

-   rust-toolchain (i.e cargo, rustc)
-   python3-dev or python3-devel

**building and installing**
```
pip install setuptools-rust setuptools wheel
python3 setup.py install --user
```

