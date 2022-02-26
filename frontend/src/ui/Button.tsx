import React from 'react'
import styled from 'styled-components'

interface Props
  extends React.DetailedHTMLProps<
    React.ButtonHTMLAttributes<HTMLButtonElement>,
    HTMLButtonElement
  > {
  width: string
}
const Button = styled.button`
  padding: 1rem;
  width: ${(props: Props) => props.width || 'auto'};
`

export default Button