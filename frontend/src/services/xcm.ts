"use client";

import { ApiPromise } from "@polkadot/api";
import { blockchainService } from "./blockchain";

export interface XcmTransferParams {
  destinationParaId: number;
  beneficiary: string; // SS58 on destination
  amount: string; // planck string
  assetMultiLocation?: any; // optional, defaults to Here (native)
  feeAssetItem?: number; // index of fee asset, default 0
  fromAddress?: string; // optional explicit signer address
}

export async function limitedReserveTransferAssets(
  params: XcmTransferParams
): Promise<{ success: boolean; hash?: string; error?: string }> {
  try {
    if (!blockchainService)
      return { success: false, error: "Blockchain service not available" };

    let api: ApiPromise | null = blockchainService.getApi();
    const account = blockchainService.getSelectedAccount();
    if (!api) {
      // Attempt to restore prior connection if available
      await (blockchainService as any).restoreConnection?.();
      api = blockchainService.getApi();
    }
    if (!api) return { success: false, error: "Not connected to network" };

    // Prefer explicit fromAddress if provided, otherwise use service account
    const fromAddress = params.fromAddress || account?.address;
    console.log("🔍 XCM: fromAddress resolved to:", fromAddress);
    if (!fromAddress) return { success: false, error: "Wallet not connected" };

    // Get signer from Polkadot extension
    let signer;
    try {
      const { web3Enable, web3FromAddress } = await import(
        "@polkadot/extension-dapp"
      );
      // Ensure extension is enabled
      await web3Enable("InkTix Sports Platform");
      const injector = await web3FromAddress(fromAddress);
      signer = injector?.signer;
    } catch (e) {
      console.error("Failed to resolve signer for fromAddress", e);
      return { success: false, error: "No signer available" };
    }
    if (!signer) return { success: false, error: "No signer available" };

    // LimitedReserveTransferAssets call
    // This uses a basic V3 MultiLocation with parents=0 and interior=Parachain(paraId)
    const dest = {
      V3: {
        parents: 0,
        interior: { X1: { Parachain: params.destinationParaId } },
      },
    } as any;

    // Beneficiary MultiLocation: AccountId32 on destination
    const beneficiary = {
      V3: {
        parents: 0,
        interior: {
          X1: {
            AccountId32: {
              id: api?.createType("AccountId32", params.beneficiary).toHex(),
              network: null,
            },
          },
        },
      },
    } as any;

    // Assets: one fungible, Here (native) by default
    const assets = {
      V3: [
        {
          id: params.assetMultiLocation || {
            Concrete: { parents: 0, interior: "Here" },
          },
          fun: { Fungible: params.amount },
        },
      ],
    } as any;

    const feeAssetItem = params.feeAssetItem ?? 0;
    // WeightV2 shape required: { Limited: { refTime, proofSize } }
    const weightLimit = {
      Limited: {
        refTime: "6000000000",
        proofSize: "0",
      },
    } as any;

    const tx = (api.tx as any).polkadotXcm.limitedReserveTransferAssets(
      dest,
      beneficiary,
      assets,
      feeAssetItem,
      weightLimit
    );

    const unsub = await tx.signAndSend(
      fromAddress,
      { signer },
      (result: any) => {
        if (result.status?.isInBlock) {
          console.log("XCM in block:", result.status.asInBlock.toHex());
        }
        if (result.status?.isFinalized) {
          console.log("XCM finalized:", result.status.asFinalized.toHex());
        }
      }
    );

    // We cannot easily return hash synchronously; provide best-effort hash where available
    const hash =
      (tx as any).hash && (tx as any).hash.toHex
        ? (tx as any).hash.toHex()
        : undefined;
    // Caller may choose to rely on UI events instead
    // Unsubscribe after short delay to avoid memory leaks in demo
    setTimeout(() => {
      try {
        if (unsub) unsub();
      } catch {}
    }, 15000);

    return { success: true, hash };
  } catch (e: any) {
    console.error("limitedReserveTransferAssets error", e);
    return { success: false, error: e?.message || String(e) };
  }
}
