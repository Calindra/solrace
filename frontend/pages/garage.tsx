import styled from 'styled-components'
import { toast } from 'react-toastify'
import AppLayout from '~/app/AppLayout'
import { useWorkspace } from '~/workspace/hooks'
import { useAllNFT } from '~/nft/hooks'
import { POOL_NAME } from '~/api/solana/constants'
import { usePoolAccount } from '~/hooks/useAccount'
import { mint } from '~/mint/services'
import GarageCard from '~/garage/GarageCard'
import Button from '~/ui/Button'
import Title from '~/ui/Title'
import { Row } from '~/ui'
import { useWallet } from '@solana/wallet-adapter-react'
import ConnectWalletButton from '~/wallet/ConnectWalletButton'
import { useMemo } from 'react'
import { BN } from '@project-serum/anchor'
import { usePool } from '~/pool/hooks'

const Main = styled(Row)`
  justify-content: space-around;
  flex-wrap: wrap;
`

const CTAButton = styled(Button)`
  margin-bottom: 1rem;
`

const GaragePage = () => {
  const { provider, wallet } = useWorkspace()
  const { poolInfo, apr } = usePool()
  const { connected } = useWallet()
  const { nfts, revalidate: revalidateNFTs } = useAllNFT(wallet?.publicKey)

  const handleMint = async () => {
    if (!provider || !wallet) {
      toast('Please connect wallet', { type: 'warning' })
      return
    }

    try {
      const tx = await mint(wallet.publicKey, provider)
      const resp = await provider.connection.confirmTransaction(tx)
      if (resp.value.err) {
        toast('Stake Failed', { type: 'error' })
      } else {
        toast('Stake Succeed', { type: 'success' })
      }
      await revalidateNFTs()
    } catch (e) {
      console.log(e)
      toast('Stake Failed', { type: 'error' })
    }
  }

  return (
    <AppLayout>
      <Title>GARAGE</Title>
      {!connected ? (
        <ConnectWalletButton />
      ) : (
        <>
          <h1>APR: {apr} % </h1>
          <CTAButton onClick={handleMint}>MOCK MINT</CTAButton>
          {poolInfo && (
            <Main>
              {nfts.map((nft) => (
                <GarageCard
                  key={nft.tokenAccountAddress.toBase58()}
                  nft={nft}
                />
              ))}
            </Main>
          )}
        </>
      )}
    </AppLayout>
  )
}

export default GaragePage
