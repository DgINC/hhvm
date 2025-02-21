// @generated by Thrift for [[[ program path ]]]
// This file is probably not the place you want to edit!

package module // [[[ program thrift source path ]]]

import (
    "fmt"
    "strings"

    thrift "github.com/facebook/fbthrift/thrift/lib/go/thrift"
)


// (needed to ensure safety because of naive import list construction)
var _ = fmt.Printf
var _ = thrift.ZERO
var _ = strings.Split


type CustomException struct {
    Message string `thrift:"message,1" json:"message" db:"message"`
}
// Compile time interface enforcer
var _ thrift.Struct = &CustomException{}

func NewCustomException() *CustomException {
    return (&CustomException{}).
        SetMessageNonCompat("")
}

func (x *CustomException) GetMessageNonCompat() string {
    return x.Message
}

func (x *CustomException) GetMessage() string {
    return x.Message
}

func (x *CustomException) SetMessageNonCompat(value string) *CustomException {
    x.Message = value
    return x
}

func (x *CustomException) SetMessage(value string) *CustomException {
    x.Message = value
    return x
}

func (x *CustomException) writeField1(p thrift.Protocol) error {  // Message
    if err := p.WriteFieldBegin("message", thrift.STRING, 1); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field begin error: ", x), err)
    }

    item := x.GetMessageNonCompat()
    if err := p.WriteString(item); err != nil {
    return err
}

    if err := p.WriteFieldEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field end error: ", x), err)
    }
    return nil
}

func (x *CustomException) readField1(p thrift.Protocol) error {  // Message
    result, err := p.ReadString()
if err != nil {
    return err
}

    x.SetMessageNonCompat(result)
    return nil
}

func (x *CustomException) toString1() string {  // Message
    return fmt.Sprintf("%v", x.GetMessageNonCompat())
}


// Deprecated: Use CustomException.Set* methods instead or set the fields directly.
type CustomExceptionBuilder struct {
    obj *CustomException
}

func NewCustomExceptionBuilder() *CustomExceptionBuilder {
    return &CustomExceptionBuilder{
        obj: NewCustomException(),
    }
}

func (x *CustomExceptionBuilder) Message(value string) *CustomExceptionBuilder {
    x.obj.Message = value
    return x
}

func (x *CustomExceptionBuilder) Emit() *CustomException {
    var objCopy CustomException = *x.obj
    return &objCopy
}

func (x *CustomException) Write(p thrift.Protocol) error {
    if err := p.WriteStructBegin("CustomException"); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct begin error: ", x), err)
    }

    if err := x.writeField1(p); err != nil {
        return err
    }

    if err := p.WriteFieldStop(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write field stop error: ", x), err)
    }

    if err := p.WriteStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T write struct end error: ", x), err)
    }
    return nil
}

func (x *CustomException) Read(p thrift.Protocol) error {
    if _, err := p.ReadStructBegin(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read error: ", x), err)
    }

    for {
        _, wireType, id, err := p.ReadFieldBegin()
        if err != nil {
            return thrift.PrependError(fmt.Sprintf("%T field %d read error: ", x, id), err)
        }

        if wireType == thrift.STOP {
            break;
        }

        switch id {
        case 1:  // message
            expectedType := thrift.Type(thrift.STRING)
            if wireType == expectedType {
                if err := x.readField1(p); err != nil {
                   return err
                }
            } else {
                if err := p.Skip(wireType); err != nil {
                    return err
                }
            }
        default:
            if err := p.Skip(wireType); err != nil {
                return err
            }
        }
    }

    if err := p.ReadFieldEnd(); err != nil {
        return err
    }

    if err := p.ReadStructEnd(); err != nil {
        return thrift.PrependError(fmt.Sprintf("%T read struct end error: ", x), err)
    }

    return nil
}

func (x *CustomException) String() string {
    if x == nil {
        return "<nil>"
    }

    var sb strings.Builder

    sb.WriteString("CustomException({")
    sb.WriteString(fmt.Sprintf("Message:%s", x.toString1()))
    sb.WriteString("})")

    return sb.String()
}
func (x *CustomException) Error() string {
    return x.String()
}

// RegisterTypes registers types found in this file that have a thrift_uri with the passed in registry.
func RegisterTypes(registry interface {
	  RegisterType(name string, initializer func() any)
}) {

}
