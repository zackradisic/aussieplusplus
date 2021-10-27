import React from 'react'

const features = [
  {
    name: 'Aussie Syntax',
    description: 'Oi mate! All your strayan lingo is valid syntax!',
    icon: '🇦🇺'
  },
  {
    name: 'Boomerangs > Curly braces',
    description:
      'Curly braces are for daft buggers, wield dangerous boomerangs as your block delimiters.',
    icon: '🪃'
  },
  {
    name: 'True aussie mode',
    description: 'Where uʍop ǝpᴉsdn characters become valid code',
    icon: '🙃'
  }
]

const Features = () => {
  return (
    <div className="py-12 bg-bg">
      <div className="max-w-xl mx-auto px-4 sm:px-6 lg:max-w-7xl lg:px-8">
        <dl className="space-y-10 lg:space-y-0 lg:grid lg:grid-cols-3 lg:gap-8">
          {features.map((feature, i) => (
            <div key={feature.name}>
              <dt>
                <div className="flex items-center justify-center h-12 w-12 rounded-md bg-[#2E334E] text-white">
                  {feature.icon}
                </div>
                <p className="mt-5 text-lg leading-6 font-medium text-gray-50">
                  <div className="flex flex-row has-tooltip">
                    {feature.name}{' '}
                    {i === 2 && (
                      <div>
                        <div className="tooltip relative mx-2">
                          <div className="bg-black text-white text-xs rounded py-1 px-4 right-0 bottom-full">
                            Some characters don&apos;t have upside-down
                            counterparts so this may break identifiers.
                            <svg
                              className="absolute text-black h-[5px] left-0 ml-3 top-full"
                              x="0px"
                              y="0px"
                              viewBox="0 0 255 255"
                              xmlSpace="preserve">
                              <polygon
                                className="fill-current"
                                points="0,0 127.5,127.5 255,0"
                              />
                            </svg>
                          </div>
                        </div>
                        <svg
                          className="ml-2 w-6 h-6 opacity-60"
                          fill="currentColor"
                          viewBox="0 0 20 20"
                          xmlns="http://www.w3.org/2000/svg">
                          <path
                            fillRule="evenodd"
                            d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z"
                            clipRule="evenodd"
                          />
                        </svg>
                      </div>
                    )}
                  </div>
                </p>
              </dt>
              <dd className="mt-2 text-base text-gray-300">
                {feature.description}
              </dd>
            </div>
          ))}
        </dl>
      </div>
    </div>
  )
}

export default Features
