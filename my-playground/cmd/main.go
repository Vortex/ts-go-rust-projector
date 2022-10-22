package main

import "fmt"

func returnsError(value int) error {
	return fmt.Errorf("This is an error with value %v", value)
}

type Foo struct {
}

func (f *Foo) thisIsOnFoo() error {
	err := returnsError(6)
	foo := Foo{}

	foo.thisIsOnFoo()
}

func CreateFoo(fail bool) (*Foo, error) {
	if fail {
		return nil, fmt.Errorf("This is an error with value")
	}

	return &Foo{}, nil
}

func main() {
	foo, err := CreateFoo(false)
	if err != nil {
		return nil, err
	}
}
