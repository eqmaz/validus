# Trade states
| State               | Description                                                                 | Allowed Next Actions        |
|--------------------|-----------------------------------------------------------------------------|-----------------------------|
| Draft              | The trade has been created but not submitted.                               | Submit                      |
| PendingApproval    | The trade has been submitted and is awaiting approval.                      | Accept, Cancel              |
| NeedsReapproval    | Trade details were updated by the approver, requiring reapproval.           | Approve, Cancel             |
| Approved           | The trade has been approved and is ready to send to the counterparty.       | SendToExecute, Cancel       |
| SentToCounterparty | The trade has been sent to the counterparty for execution.                  | Book, Cancel                |
| Executed           | The trade has been executed and booked.                                     | None (end state)            |
| Cancelled          | The trade has been cancelled.                                               | None (end state)            |
