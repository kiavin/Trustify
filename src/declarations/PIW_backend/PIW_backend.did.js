export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const ReleaseMethod = IDL.Variant({ 'Icp' : IDL.Null });
  const EscrowStatus = IDL.Variant({
    'Disputed' : IDL.Null,
    'Refunded' : IDL.Null,
    'GoodsShipped' : IDL.Null,
    'Agreed' : IDL.Null,
    'Funded' : IDL.Null,
    'FundsReleased' : IDL.Null,
    'Created' : IDL.Null,
    'Resolved' : IDL.Null,
  });
  const EscrowAgreement = IDL.Record({
    'status' : EscrowStatus,
    'terms' : IDL.Text,
    'agreed_at' : IDL.Opt(IDL.Nat64),
    'created_at' : IDL.Nat64,
    'seller' : IDL.Principal,
    'release_method' : IDL.Opt(ReleaseMethod),
    'funded_at' : IDL.Opt(IDL.Nat64),
    'buyer' : IDL.Principal,
    'disputed_at' : IDL.Opt(IDL.Nat64),
    'escrow_id' : IDL.Nat64,
    'amount' : IDL.Nat64,
    'shipped_at' : IDL.Opt(IDL.Nat64),
    'resolved_at' : IDL.Opt(IDL.Nat64),
    'released_at' : IDL.Opt(IDL.Nat64),
  });
  const Result_1 = IDL.Variant({ 'Ok' : EscrowAgreement, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Tuple(IDL.Principal, IDL.Principal),
    'Err' : IDL.Text,
  });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'status_code' : IDL.Nat16,
  });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : IDL.Text });
  const DisputeDecision = IDL.Variant({
    'RefundBuyer' : IDL.Null,
    'ReleaseFunds' : IDL.Null,
  });
  return IDL.Service({
    'agree_escrow' : IDL.Func([IDL.Nat64], [Result], []),
    'cancel_escrow' : IDL.Func([IDL.Nat64], [Result], []),
    'confirm_goods_received' : IDL.Func(
        [IDL.Nat64, ReleaseMethod],
        [Result],
        [],
      ),
    'confirm_goods_shipped' : IDL.Func([IDL.Nat64], [Result], []),
    'confirm_icp_transfer' : IDL.Func([IDL.Nat64], [Result], []),
    'fund_escrow' : IDL.Func([IDL.Nat64], [Result], []),
    'get_escrow' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'get_owner' : IDL.Func([], [IDL.Opt(IDL.Principal)], ['query']),
    'get_participants' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'initiate_escrow' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Nat64, IDL.Text],
        [Result_3],
        [],
      ),
    'list_my_escrows' : IDL.Func([], [IDL.Vec(EscrowAgreement)], ['query']),
    'open_dispute' : IDL.Func([IDL.Nat64], [Result], []),
    'resolve_dispute' : IDL.Func([IDL.Nat64, DisputeDecision], [Result], []),
    'set_admin' : IDL.Func([IDL.Principal], [Result], []),
    'set_off_chain_server' : IDL.Func([IDL.Principal], [Result], []),
    'set_owner' : IDL.Func([IDL.Principal], [], []),
    'whoami' : IDL.Func([], [IDL.Principal], []),
  });
};
export const init = ({ IDL }) => { return []; };
