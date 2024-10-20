import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type DisputeDecision = { 'RefundBuyer' : null } |
  { 'ReleaseFunds' : null };
export interface EscrowAgreement {
  'status' : EscrowStatus,
  'terms' : string,
  'agreed_at' : [] | [bigint],
  'created_at' : bigint,
  'seller' : Principal,
  'release_method' : [] | [ReleaseMethod],
  'funded_at' : [] | [bigint],
  'buyer' : Principal,
  'disputed_at' : [] | [bigint],
  'escrow_id' : bigint,
  'amount' : bigint,
  'shipped_at' : [] | [bigint],
  'resolved_at' : [] | [bigint],
  'released_at' : [] | [bigint],
}
export type EscrowStatus = { 'Disputed' : null } |
  { 'Refunded' : null } |
  { 'GoodsShipped' : null } |
  { 'Agreed' : null } |
  { 'Funded' : null } |
  { 'FundsReleased' : null } |
  { 'Created' : null } |
  { 'Resolved' : null };
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
}
export interface HttpResponse {
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
  'status_code' : number,
}
export type ReleaseMethod = { 'Icp' : null };
export type Result = { 'Ok' : null } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : EscrowAgreement } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : [Principal, Principal] } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : bigint } |
  { 'Err' : string };
export interface _SERVICE {
  'agree_escrow' : ActorMethod<[bigint], Result>,
  'cancel_escrow' : ActorMethod<[bigint], Result>,
  'confirm_goods_received' : ActorMethod<[bigint, ReleaseMethod], Result>,
  'confirm_goods_shipped' : ActorMethod<[bigint], Result>,
  'confirm_icp_transfer' : ActorMethod<[bigint], Result>,
  'fund_escrow' : ActorMethod<[bigint], Result>,
  'get_escrow' : ActorMethod<[bigint], Result_1>,
  'get_owner' : ActorMethod<[], [] | [Principal]>,
  'get_participants' : ActorMethod<[bigint], Result_2>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'initiate_escrow' : ActorMethod<[string, string, bigint, string], Result_3>,
  'list_my_escrows' : ActorMethod<[], Array<EscrowAgreement>>,
  'open_dispute' : ActorMethod<[bigint], Result>,
  'resolve_dispute' : ActorMethod<[bigint, DisputeDecision], Result>,
  'set_admin' : ActorMethod<[Principal], Result>,
  'set_off_chain_server' : ActorMethod<[Principal], Result>,
  'set_owner' : ActorMethod<[Principal], undefined>,
  'whoami' : ActorMethod<[], Principal>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
