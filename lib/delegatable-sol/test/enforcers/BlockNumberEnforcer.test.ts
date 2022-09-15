import { ethers, network } from "hardhat";
import { expect } from "chai";
import { Provider } from "@ethersproject/providers";
import { BigNumber, Contract, ContractFactory, Wallet } from "ethers";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
// @ts-ignore
import { generateUtil } from "eth-delegatable-utils";
import { getPrivateKeys } from "../../utils/getPrivateKeys";
import { generateDelegation } from "../utils";

const { getSigners } = ethers;

describe("BlockNumberEnforcer", () => {
  const CONTACT_NAME = "ERC20Delegatable";
  let CONTRACT_INFO: any;
  let delegatableUtils: any;
  let signer0: SignerWithAddress;
  let wallet0: Wallet;
  let wallet1: Wallet;
  let pk0: string;
  let pk1: string;

  // Smart Contracts
  let BlockNumberEnforcer: Contract;
  let BlockNumberEnforcerFactory: ContractFactory;
  let ERC20Delegatable: Contract;
  let ERC20DelegatableFactory: ContractFactory;

  before(async () => {
    [signer0] = await getSigners();
    [wallet0, wallet1] = getPrivateKeys(
      signer0.provider as unknown as Provider
    );
    ERC20DelegatableFactory = await ethers.getContractFactory(
      "ERC20Delegatable"
    );
    BlockNumberEnforcerFactory = await ethers.getContractFactory(
      "BlockNumberEnforcer"
    );
    pk0 = wallet0._signingKey().privateKey;
    pk1 = wallet1._signingKey().privateKey;
  });

  beforeEach(async () => {
    await network.provider.request({
      method: "hardhat_reset",
      params: [],
    });
    ERC20Delegatable = await ERC20DelegatableFactory.connect(wallet0).deploy(
      CONTACT_NAME,
      "TRUST",
      ethers.utils.parseEther("1")
    );
    BlockNumberEnforcer = await BlockNumberEnforcerFactory.connect(
      wallet0
    ).deploy();

    CONTRACT_INFO = {
      chainId: ERC20Delegatable.deployTransaction.chainId,
      verifyingContract: ERC20Delegatable.address,
      name: CONTACT_NAME,
    };
    delegatableUtils = generateUtil(CONTRACT_INFO);
  });

  it("should SUCCEED to INVOKE method AFTER blockNumber reached", async () => {
    const PK = wallet0._signingKey().privateKey.substring(2);
    expect(await ERC20Delegatable.balanceOf(wallet0.address)).to.eq(
      ethers.utils.parseEther("1")
    );
    const _delegation = generateDelegation(
      CONTACT_NAME,
      ERC20Delegatable,
      PK,
      wallet1.address,
      [
        {
          enforcer: BlockNumberEnforcer.address,
          terms:
            "0x0000000000000000000000000000000000000000000000000000000000000000",
        },
      ]
    );
    const INVOCATION_MESSAGE = {
      replayProtection: {
        nonce: "0x01",
        queue: "0x00",
      },
      batch: [
        {
          authority: [_delegation],
          transaction: {
            to: ERC20Delegatable.address,
            gasLimit: "210000000000000000",
            data: (
              await ERC20Delegatable.populateTransaction.transfer(
                wallet1.address,
                ethers.utils.parseEther("0.5")
              )
            ).data,
          },
        },
      ],
    };
    const invocation = delegatableUtils.signInvocation(INVOCATION_MESSAGE, pk1);
    await ERC20Delegatable.invoke([
      {
        signature: invocation.signature,
        invocations: invocation.invocations,
      },
    ]);
    expect(await ERC20Delegatable.balanceOf(wallet0.address)).to.eq(
      ethers.utils.parseEther("0.5")
    );
  });

  it("should FAIL to INVOKE method BEFORE blockNumber reached", async () => {
    expect(await ERC20Delegatable.balanceOf(wallet0.address)).to.eq(
      ethers.utils.parseEther("1")
    );
    const _delegation = generateDelegation(
      CONTACT_NAME,
      ERC20Delegatable,
      pk0,
      wallet1.address,
      [
        {
          enforcer: BlockNumberEnforcer.address,
          terms:
            "0x0000000000000000000000000000000000000000000000000000000000000032",
        },
      ]
    );
    const INVOCATION_MESSAGE = {
      replayProtection: {
        nonce: "0x01",
        queue: "0x00",
      },
      batch: [
        {
          authority: [_delegation],
          transaction: {
            to: ERC20Delegatable.address,
            gasLimit: "210000000000000000",
            data: (
              await ERC20Delegatable.populateTransaction.transfer(
                wallet1.address,
                ethers.utils.parseEther("0.5")
              )
            ).data,
          },
        },
      ],
    };
    const invocation = delegatableUtils.signInvocation(INVOCATION_MESSAGE, pk1);

    await expect(
      ERC20Delegatable.invoke([
        {
          signature: invocation.signature,
          invocations: invocation.invocations,
        },
      ])
    ).to.be.revertedWith("BlockNumberEnforcer:expired-delegation");
  });

  it("should SUCCEED to INVOKE method BEFORE blockNumber reached", async () => {
    const PK = wallet0._signingKey().privateKey.substring(2);
    expect(await ERC20Delegatable.balanceOf(wallet0.address)).to.eq(
      ethers.utils.parseEther("1")
    );
    const _delegation = generateDelegation(
      CONTACT_NAME,
      ERC20Delegatable,
      PK,
      wallet1.address,
      [
        {
          enforcer: BlockNumberEnforcer.address,
          terms:
            "0x0000000000000000000000000000000100000000000000000000000000000032",
        },
      ]
    );
    const INVOCATION_MESSAGE = {
      replayProtection: {
        nonce: "0x01",
        queue: "0x00",
      },
      batch: [
        {
          authority: [_delegation],
          transaction: {
            to: ERC20Delegatable.address,
            gasLimit: "210000000000000000",
            data: (
              await ERC20Delegatable.populateTransaction.transfer(
                wallet1.address,
                ethers.utils.parseEther("0.5")
              )
            ).data,
          },
        },
      ],
    };
    const invocation = delegatableUtils.signInvocation(INVOCATION_MESSAGE, pk1);
    await ERC20Delegatable.invoke([
      {
        signature: invocation.signature,
        invocations: invocation.invocations,
      },
    ]);
    expect(await ERC20Delegatable.balanceOf(wallet0.address)).to.eq(
      ethers.utils.parseEther("0.5")
    );
  });

  it("should FAIL to INVOKE method AFTER blockNumber reached", async () => {
    const PK = wallet0._signingKey().privateKey.substring(2);
    await signer0.sendTransaction({
      to: wallet0.address,
      data: "0x",
      value: ethers.utils.parseEther("0.1"),
    });
    await signer0.sendTransaction({
      to: wallet0.address,
      data: "0x",
      value: ethers.utils.parseEther("0.1"),
    });
    expect(await ERC20Delegatable.balanceOf(wallet0.address)).to.eq(
      ethers.utils.parseEther("1")
    );
    const _delegation = generateDelegation(
      CONTACT_NAME,
      ERC20Delegatable,
      PK,
      wallet1.address,
      [
        {
          enforcer: BlockNumberEnforcer.address,
          terms:
            "0x0000000000000000000000000000000100000000000000000000000000000000",
        },
      ]
    );
    const INVOCATION_MESSAGE = {
      replayProtection: {
        nonce: "0x01",
        queue: "0x00",
      },
      batch: [
        {
          authority: [_delegation],
          transaction: {
            to: ERC20Delegatable.address,
            gasLimit: "210000000000000000",
            data: (
              await ERC20Delegatable.populateTransaction.transfer(
                wallet1.address,
                ethers.utils.parseEther("0.5")
              )
            ).data,
          },
        },
      ],
    };
    const invocation = delegatableUtils.signInvocation(INVOCATION_MESSAGE, pk1);
    await expect(
      ERC20Delegatable.invoke([
        {
          signature: invocation.signature,
          invocations: invocation.invocations,
        },
      ])
    ).to.be.revertedWith("BlockNumberEnforcer:early-delegation");
  });
});
