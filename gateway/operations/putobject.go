package operations

import (
	"io/ioutil"
	"net/http"
	"time"
	"versio-index/gateway/errors"
	"versio-index/gateway/permissions"
	"versio-index/ident"
	"versio-index/index/model"
)

type PutObject struct{}

func (controller *PutObject) GetArn() string {
	return "arn:versio:repos:::{bucket}"
}

func (controller *PutObject) GetPermission() string {
	return permissions.PermissionWriteRepo
}

func (controller *PutObject) Handle(o *PathOperation) {
	// handle the upload itself
	data, err := ioutil.ReadAll(o.Request.Body)
	if err != nil {
		o.EncodeError(errors.Codes.ToAPIErr(errors.ErrInternalError))
		return
	}

	// write to adapter
	blocks := make([]string, 0)
	blockAddr := ident.Bytes(data)
	err = o.BlockStore.Put(data, blockAddr)
	if err != nil {
		o.EncodeError(errors.Codes.ToAPIErr(errors.ErrInternalError))
		return
	}
	blocks = append(blocks, blockAddr)

	// write metadata
	err = o.Index.WriteObject(o.ClientId, o.Repo, o.Branch, o.Path, &model.Object{
		Blob: &model.Blob{
			Blocks: blocks,
		},
		Metadata:  nil,
		Timestamp: time.Now().Unix(),
		Size:      int64(len(data)),
	})

	if err != nil {
		o.EncodeError(errors.Codes.ToAPIErr(errors.ErrInternalError))
		return
	}
	o.ResponseWriter.WriteHeader(http.StatusCreated)
}
