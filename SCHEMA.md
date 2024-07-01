# Database Schema

- currency(__code__, name, symbol, rate):

- account(__id__, name, type, currency):
  - currency: FK(currency)

- transaction(__id__, timestamp, sender, amount_sent, receiver, amount_received):
  - sender: FK(account)
  - receiver: FK(account)
  - category: FK(category)

- category(__name__, color, super):
  - super: FK(super_category)

- super_category(__name__, color):

- categorization(__category__, __transaction__):
  - category: FK(category)
  - transaction: FK(transaction)
