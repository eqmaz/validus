# Trade Approval Process

The trade approval process supports the following actions:

| Action            | Explanation                                      | Inputs                                                                 |
|-------------------|--------------------------------------------------|------------------------------------------------------------------------|
| **Submit**        | Submit a new trade request.                      | User ID (Requester), trade details (described below).                 |
| **Accept/Approve**| Approve a submitted trade request.               | User ID (Approver).                                                   |
| **Cancel**        | Cancel the trade request.                        | User ID (Requester or Approver).                                      |
| **Update**        | Update trade details (requires reapproval).      | User ID (Approver), updated trade details.                            |
| **SendToExecute** | Send the approved trade to the counterparty for execution. | User ID (Approver).                                          |
| **Book**          | Book the trade into the system once executed by the counterparty. | User ID (Requester or Approver), confirmation of execution. |