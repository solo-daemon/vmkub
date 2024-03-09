"use client"
import {RadioGroup, Radio} from "@nextui-org/react";

export default function Home() {
  return (
    <div className="bg-[url('resources/bg1.webp')] h-screen bg-no-repeat bg-cover bg-center "  >
      <div className=" mx-32  h-full border-8 border-[#A7A1BB]  rounded-xl flex " >
        <div className="w-1/2 "></div>
        <div className="w-1/2 flex flex-col justify-evenly items-center bg-[#A7A1BB] text-[#000000] ">
          <p className="text-center">Logo</p>
          <w3m-button />
          <RadioGroup
      label="What do you want to be"
        >
      <Radio value="client">Client</Radio>
      <Radio value="provider">Provider  </Radio>
    </RadioGroup>
          <form action="#" className="w-full">
            
          </form>
        </div>
      </div>
    </div>
  );
}
