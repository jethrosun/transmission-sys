#include "transmission/libtransmission/transmission.h"

// Use this to pretend to be libtransmission
#ifndef __TRANSMISSION__
#define __TRANSMISSION__
#endif

#include "transmission/libtransmission/torrent.h"
#include "transmission/libtransmission/error-types.h"
#include "transmission/libtransmission/error.h"
#include "transmission/libtransmission/file.h"
#include "transmission/libtransmission/tr-getopt.h"
#include "transmission/libtransmission/variant.h"
#include "transmission/libtransmission/crypto-utils.h"
#include "transmission/libtransmission/makemeta.h"
#include "transmission/libtransmission/quark.h"
#include "transmission/libtransmission/utils.h"
#include "transmission/libtransmission/watchdir.h"
#include "transmission/libtransmission/rpcimpl.h"
#include "transmission/libtransmission/log.h"
#include "transmission/libtransmission/web.h"