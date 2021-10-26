import { Menu, Transition } from '@headlessui/react'
import { ChevronDownIcon } from '@heroicons/react/solid'
import classNames from 'classnames'
import React, { Fragment, useState } from 'react'

import { examples } from '../lib/example'

const exampleList = [
  'dreamtime.aussie',
  'fibonacci.aussie',
  'time.aussie',
  'random_beer.aussie'
]

const getExample = (name: string) => {
  return examples[name.split('.')[0]]
}

const Item = ({
  active,
  name,
  onClick
}: {
  active: boolean
  name: string
  onClick: () => void
}) => {
  return (
    <button
      onClick={onClick}
      className={classNames(
        active ? 'bg-gray-100 text-gray-900' : 'text-gray-700',
        'block px-4 py-2 text-sm w-full'
      )}>
      {name}
    </button>
  )
}

type Props = {
  setCode: (s: string) => void
}

const CodeDropdown = ({ setCode }: Props) => {
  const [activeExample, setActiveExample] = useState<string>(exampleList[0])
  return (
    <Menu as="div" className="relative inline-block text-left">
      <div>
        <Menu.Button className="inline-flex justify-center w-full rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-100 focus:ring-indigo-500">
          {activeExample}
          <ChevronDownIcon className="-mr-1 ml-2 h-5 w-5" aria-hidden="true" />
        </Menu.Button>
      </div>

      <Transition
        as={Fragment}
        enter="transition ease-out duration-100"
        enterFrom="transform opacity-0 scale-95"
        enterTo="transform opacity-100 scale-100"
        leave="transition ease-in duration-75"
        leaveFrom="transform opacity-100 scale-100"
        leaveTo="transform opacity-0 scale-95">
        <Menu.Items className="z-50 origin-top-right absolute right-0 mt-2 w-56 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 focus:outline-none">
          <div className="py-1">
            {exampleList.map(name => {
              return (
                <Menu.Item key={name}>
                  {() => (
                    <Item
                      active={activeExample === name}
                      name={name}
                      onClick={() => {
                        setCode(getExample(name))
                        setActiveExample(name)
                      }}
                    />
                  )}
                </Menu.Item>
              )
            })}
          </div>
        </Menu.Items>
      </Transition>
    </Menu>
  )
}

export default CodeDropdown
