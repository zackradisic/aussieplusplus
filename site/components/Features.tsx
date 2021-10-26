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
          {features.map(feature => (
            <div key={feature.name}>
              <dt>
                <div className="flex items-center justify-center h-12 w-12 rounded-md bg-[#2E334E] text-white">
                  {feature.icon}
                </div>
                <p className="mt-5 text-lg leading-6 font-medium text-gray-50">
                  {feature.name}
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
