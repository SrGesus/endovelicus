# Database Schema

- currency(__name__, code, symbol):

- account(__id__, name, type, currency):
  - currency: FK(currency)

- transaction(__id__, timestamp, sender, receiver, currency, amount):
  - sender: FK(account)
  - receiver: FK(account)

