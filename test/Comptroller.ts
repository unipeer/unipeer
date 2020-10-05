import { ethers, run } from "@nomiclabs/buidler";
import { Signer } from "ethers";

import { expect } from 'chai';

import { ComptrollerFactory } from "../types"
import { Comptroller as ComptrollerContract } from "../types/Comptroller"

let comptroller: ComptrollerContract;
let accounts: Signer[];

describe('Comptroller', function () {
  before(async function () {
    accounts = await ethers.getSigners();
    const Comptroller = await new ComptrollerFactory(accounts[0]);

    comptroller = await Comptroller.deploy("", "");
  });
});
