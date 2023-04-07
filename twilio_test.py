"""
This is a test to see if I can send a text message using twilio.
Credentials etc.
"""
import os
from dotenv import load_dotenv
from twilio.rest import Client

if __name__ == "__main__":
    load_dotenv()
    account_sid = os.getenv("ACCOUNT_SID")
    auth_token = os.getenv("ACCOUNT_TOKEN")
    from_number = os.getenv("PHONE_FROM")

    client = Client(account_sid, auth_token)

    message = client.messages.create(
        body="Hi Sean another test from python with these keys.",
        from_=from_number,
        to="+447704179714",
    )
    print(message.sid)
