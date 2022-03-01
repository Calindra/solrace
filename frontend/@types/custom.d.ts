type ENDPOINT_NAME =
  | 'mainnet-beta'
  | 'testnet'
  | 'devnet'
  | 'localnet'
  | 'lending'

interface ENDPOINT {
  name: ENDPOINT_NAME
  label: string
  url: string
  chainId: number
}

interface User {
  id: number
  email: string
  createdAt: string
  updatedAt: string
}

interface CandyMachineState {
  itemsAvailable: number
  itemsRedeemed: number
  itemsRemaining: number
  treasury: anchor.web3.PublicKey
  tokenMint: anchor.web3.PublicKey
  isSoldOut: boolean
  isActive: boolean
  isPresale: boolean
  isWhitelistOnly: boolean
  goLiveDate: anchor.BN
  price: anchor.BN
  gatekeeper: null | {
    expireOnUse: boolean
    gatekeeperNetwork: anchor.web3.PublicKey
  }
  endSettings: null | {
    number: anchor.BN
    endSettingType: any
  }
  whitelistMintSettings: null | {
    mode: any
    mint: anchor.web3.PublicKey
    presale: boolean
    discountPrice: null | anchor.BN
  }
  hiddenSettings: null | {
    name: string
    uri: string
    hash: Uint8Array
  }
}
