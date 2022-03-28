/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-return */

import os from 'os';
import fs from 'mz/fs';
import path from 'path';
import yaml from 'yaml';
import {Keypair} from '@solana/web3.js';
import axios from 'axios';

const BASE58 = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz'
const baseX = require('base-x');
const HEX = '0123456789abcdef';
export const hex = baseX(HEX);
export const bs58 = baseX(BASE58);

/**
 * @private
 */
async function getConfig(): Promise<any> {
  // Path to Solana CLI config file
  const CONFIG_FILE_PATH = path.resolve(
    os.homedir(),
    '.config',
    'solana',
    'cli',
    'config.yml',
  );
  const configYml = await fs.readFile(CONFIG_FILE_PATH, {encoding: 'utf8'});
  return yaml.parse(configYml);
}

/**
 * Load and parse the Solana CLI config file to determine which RPC url to use
 */
export async function getRpcUrl(): Promise<string> {
  try {
    const config = await getConfig();
    if (!config.json_rpc_url) throw new Error('Missing RPC URL');
    return config.json_rpc_url;
  } catch (err) {
    console.warn(
      'Failed to read RPC url from CLI config file, falling back to localhost',
    );
    return 'http://localhost:8899';
  }
}

/**
 * Load and parse the Solana CLI config file to determine which payer to use
 */
export async function getPayer(): Promise<Keypair> {
  try {
    const config = await getConfig();
    if (!config.keypair_path) throw new Error('Missing keypair path');
    return await createKeypairFromFile(config.keypair_path);
  } catch (err) {
    console.warn(
      'Failed to create keypair from CLI config file, falling back to new random keypair',
    );
    return Keypair.generate();
  }
}

/**
 * Create a Keypair from a secret key stored in file as bytes' array
 */
export async function createKeypairFromFile(
  filePath: string,
): Promise<Keypair> {
  const secretKeyString = await fs.readFile(filePath, {encoding: 'utf8'});
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return Keypair.fromSecretKey(secretKey);
}

type MuonRequest = {
  app?: string,
  method: string,
  params?: any
}

type MuonResponse = {
  success: boolean,
  result: any
}

const MUON_TESTNET_URL = 'http://104.131.177.195:8080/v1/'

export async function callMuon(
    req: MuonRequest,
    baseUrl?: string
): Promise<MuonResponse> {
  return axios.post(baseUrl || MUON_TESTNET_URL, req).then(({data}) => data)
}
