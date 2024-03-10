"use client"
import {RadioGroup, Radio} from "@nextui-org/react";
import {Input} from "@nextui-org/react";
import { Button } from "@nextui-org/react";
import { useState } from "react";

import { useWeb3ModalProvider, useWeb3ModalAccount } from '@web3modal/ethers5/react'
import { ethers } from 'ethers'

import {clientAbi, providerAbi, clientContractAddress, providerContractAddress} from "./constants"

const clientAPI = () => {

}

export default function Home() {  
  const[showDiv, setshowDiv]= useState(true);

  const { address, chainId, isConnected } = useWeb3ModalAccount()
  const { walletProvider } = useWeb3ModalProvider()

  // client state
  const [clientVCPU,setClientVCPU] = useState("")
  const [clientRAM,setClientRAM] = useState("")
  const [clientStorage,setClientStorage] = useState("")
  const [clientPublicAddress,setClientPublicAddress] = useState("")

  // provider state
  const [providerVCPU,setProviderVCPU] = useState("")
  const [providerRAM,setProviderRAM] = useState("")
  const [providerStorage,setProviderStorage] = useState("")
  const [providerIPAddress,setProviderIPAddress] = useState("")

  const handleSubmit = () =>{
    if(showDiv){
      clientAPI()
    }else {
      providerAPI()
    }
  }

  async function clientAPI () {
    if (!isConnected) throw Error('User disconnected')

    const ethersProvider = new ethers.providers.Web3Provider(walletProvider)
    const signer = await ethersProvider.getSigner()
    // The Contract object
    const clientContract = new ethers.Contract(clientContractAddress, clientAbi, signer)

    // parsing the states to suitable type
    const _vcpu = parseInt(clientVCPU)
    const _ram = parseInt(clientRAM)
    const _storage = parseInt(clientStorage)
    const _pubkey = String(clientPublicAddress)
    // console.log(clientVCPU,clientRAM)
    // console.log(_vcpu,_ram,_storage,_pubkey)
    clientContract.registerClient(_vcpu,_ram,_storage,_pubkey)
    
  }

  async function providerAPI () {
    if (!isConnected) throw Error('User disconnected')

    const ethersProvider = new ethers.providers.Web3Provider(walletProvider)
    const signer = await ethersProvider.getSigner()
    // The Contract object
    const providerContract = new ethers.Contract(providerContractAddress, providerAbi, signer)

    // Parsing the state to suitalbe type
    const _vcpu = parseInt(providerVCPU)
    const _ram = parseInt(providerRAM)
    const _storage = parseInt(providerStorage)
    const _ipaddr = String(providerIPAddress)
    // console.log(_vcpu,_ram,_storage,_ipaddr)

    providerContract.registerProvider(_vcpu,_ram,_storage,_ipaddr)
    
  }

  return (
    <div className="bg-[url('../../public/bg.svg')] h-screen bg-no-repeat bg-cover bg-center "  >
      <div className=" mx-32  h-full  rounded-xl flex justify-center " >
        
        <div className="m-20 rounded-xl w-1/2 flex flex-col justify-evenly border-[#55b424] items-center bg-opacity-20 rounded-lg shadow-md backdrop-blur-md border border-opacity-30 text-[#000000] ">
          <p className="text-center text-white ">vmkube</p>
          <w3m-button />
          <RadioGroup
      label="What do you want to be"
      defaultValue="client"
        >
      <Radio value="client" id="client" checked="checked" onChange={()=>setshowDiv(!showDiv)}>Client</Radio>
      <Radio value="provider" id="provider" onChange={()=>setshowDiv(!showDiv)} >Provider  </Radio>
    </RadioGroup>
          <form action="#" className="w-full">
            {
              showDiv ? 
              <div className="flex flex-col items-center w-full flex-wrap md:flex-nowrap gap-4" id="clientcontainer">      
      <Input type="text" label="VCPU" placeholder="Enter your VCPU"  className="w-5/6" value={clientVCPU} onChange={(e)=>{setClientVCPU(e.target.value)}}/>
      <Input type="text" label="RAM" placeholder="Enter RAM" className="w-5/6" value={clientRAM} onChange={(e)=>{setClientRAM(e.target.value)}}/>
      <Input type="text" label="Storage" placeholder="Enter your Storage" className="w-5/6" value={clientStorage}  onChange={(e)=>{setClientStorage(e.target.value)}}/>
      <Input type="text" label="RSA Link" placeholder="Enter RSA Public Link" className="w-5/6" value={clientPublicAddress}  onChange={(e)=>{setClientPublicAddress(e.target.value)}}/>
    </div>
              :
              <div className="flex flex-col items-center w-full flex-wrap md:flex-nowrap gap-4" id="providercontainer" >      
      <Input type="text" label="VCPU" placeholder="Enter your VCPU"  className="w-5/6" onChange={(e)=>{setProviderVCPU(e.target.value)}}/>
      <Input type="text" label="RAM" placeholder="Enter RAM" className="w-5/6" onChange={(e)=>{setProviderRAM(e.target.value)}}/>
      <Input type="text" label="Storage" placeholder="Enter your Storage" className="w-5/6" onChange={(e)=>{setProviderStorage(e.target.value)}}/>
      <Input type="text" label="IP Address" placeholder="Enter your IP Address" className="w-5/6" onChange={(e)=>{setProviderIPAddress(e.target.value)}}/>
    </div>
            }
          <div className="flex flex-col items-center w-full flex-wrap md:flex-nowrap gap-4 m-4" >
            <Button onClick={handleSubmit}>Submit</Button>
          </div>
          </form>
        </div>
      </div>
    </div>
  );
}
