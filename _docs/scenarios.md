# Example Scenarios

## 1. Submitting and Approving a Trade

**Scenario**: A user submits a trade for approval, and it is approved.

| Step | Action  | User ID | State Before     | State After      | Notes                      |
|------|---------|---------|------------------|------------------|----------------------------|
| 1    | Submit  | User1   | Draft            | PendingApproval  | Trade details provided.   |
| 2    | Approve | User2   | PendingApproval  | Approved         | Approver confirms trade.  |

---

## 2. Updating a Trade Detail

**Scenario**: An approver updates the trade details, requiring reapproval.

| Step | Action | User ID | State Before     | State After      | Notes                                  |
|------|--------|---------|------------------|------------------|----------------------------------------|
| 1    | Submit | User1   | Draft            | PendingApproval  | Trade details provided.                |
| 2    | Update | User2   | PendingApproval  | NeedsReapproval  | Approver updates the notional amount. |
| 3    | Approve| User1   | NeedsReapproval  | Approved         | Requester reapproves updated trade.   |

---

## 3. Execution

**Scenario**: An approved trade is sent to the counterparty and marked as executed.

| Step | Action         | User ID | State Before       | State After         | Notes                           |
|------|----------------|---------|--------------------|---------------------|---------------------------------|
| 1    | Submit         | User1   | Draft              | PendingApproval     | Trade details provided.         |
| 2    | Approve        | User2   | PendingApproval    | Approved            | Approver confirms trade.        |
| 3    | SendToExecute  | User2   | Approved           | SentToCounterparty  | Trade sent to counterparty.     |
| 4    | Book           | User1   | SentToCounterparty | Executed            | Trade executed and booked.      |

---

## 4. Viewing History and Differences

- **History API**: Return a table of all actions with details (e.g., user, timestamp, state transition).
- **Differences API**: Show changes between trade versions.

**Example output**:
```json
{
  "notionalAmount": ["1,000,000", "1,200,000"]
}
