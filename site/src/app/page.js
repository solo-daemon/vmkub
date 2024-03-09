"use client"
import {RadioGroup, Radio} from "@nextui-org/react";
import {Input} from "@nextui-org/react";
import { useState } from "react";
export default function Home() {  
  const[showDiv, setshowDiv]= useState(true);
  return (
    <div className="bg-[url('../../public/bg.svg')] h-screen bg-no-repeat bg-cover bg-center "  >
      <div className=" mx-32  h-full  rounded-xl flex justify-center " >
        
        <div className="m-20 rounded-xl w-1/2 flex flex-col justify-evenly border-[#55b424] items-center bg-opacity-20 rounded-lg shadow-md backdrop-blur-md border border-opacity-30 text-[#000000] ">
          <p className="text-center text-white ">Logo</p>
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
      <Input type="text" label="VCPU" placeholder="Enter your VCPU"  className="w-5/6"/>
      <Input type="text" label="RAM" placeholder="Enter RAM" className="w-5/6" />
      <Input type="text" label="Storage" placeholder="Enter your Storage" className="w-5/6" />
      <Input type="text" label="RSA Link" placeholder="Enter RSA Public Link" className="w-5/6" />
    </div>
              :
              <div className="flex flex-col items-center w-full flex-wrap md:flex-nowrap gap-4" id="providercontainer">      
      <Input type="text" label="VCPU" placeholder="Enter your VCPU"  className="w-5/6"/>
      <Input type="text" label="RAM" placeholder="Enter RAM" className="w-5/6" />
      <Input type="text" label="Storage" placeholder="Enter your Storage" className="w-5/6" />
      <Input type="text" label="IP Address" placeholder="Enter your IP Address" className="w-5/6" />
    </div>
            }
          
    
          </form>
        </div>
      </div>
    </div>
  );
}
