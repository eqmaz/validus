## Trade Details

Each trade includes the following fields:

| Field             | Description                                                                                      |
|------------------|--------------------------------------------------------------------------------------------------|
| **Trading Entity** | The legal entity conducting the trade.                                                         |
| **Counterparty**   | The entity on the other side of the trade.                                                     |
| **Direction**      | Specifies whether the trade is a `"Buy"` or `"Sell"`.                                          |
| **Style**          | Assumes the trade is a forward contract.                                                       |
| **Notional Currency** | Currency of the notional amount (e.g., EUR, GBP, USD). The full list is available at [IBAN Currency Codes](https://www.iban.com/currency-codes). |
| **Notional Amount** | The size of the trade in the selected notional currency.                                      |
| **Underlying**     | A combination of eligible notional currencies. The selected notional currency must be part of the underlying. |
| **Trade Date**     | The date when the trade is initiated.                                                          |
| **Value Date**     | The date when the trade value is realized.                                                     |
| **Delivery Date**  | The date when the trade assets are delivered.                                                  |
| **Strike**         | The agreed rate. This is only available after the trade is executed.                           |

### Validation Rule
```Trade Date ≤ Value Date ≤ Delivery Date```