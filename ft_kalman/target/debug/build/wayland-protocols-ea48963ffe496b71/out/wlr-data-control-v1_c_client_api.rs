# [ doc = "manager to control data devices\n\nThis interface is a manager that allows creating per-seat data device\ncontrols." ] pub mod zwlr_data_control_manager_v1 { use super :: { Proxy , NewProxy , AnonymousObject , Interface , MessageGroup , MessageDesc , ArgumentType , Object , Message , Argument , ObjectMetadata } ; use super :: sys :: common :: { wl_argument , wl_interface , wl_array } ; use super :: sys :: client :: * ; pub enum Request { # [ doc = "create a new data source\n\nCreate a new data source." ] CreateDataSource { id : Proxy < super :: zwlr_data_control_source_v1 :: ZwlrDataControlSourceV1 > , } , # [ doc = "get a data device for a seat\n\nCreate a data device that can be used to manage a seat's selection." ] GetDataDevice { id : Proxy < super :: zwlr_data_control_device_v1 :: ZwlrDataControlDeviceV1 > , seat : Proxy < super :: wl_seat :: WlSeat > , } , # [ doc = "destroy the manager\n\nAll objects created by the manager will still remain valid, until their\nappropriate destroy request has been called.\n\nThis is a destructor, once sent this object cannot be used any longer." ] Destroy , } impl super :: MessageGroup for Request { const MESSAGES : & 'static [ super :: MessageDesc ] = & [ super :: MessageDesc { name : "create_data_source" , since : 1 , signature : & [ super :: ArgumentType :: NewId , ] , } , super :: MessageDesc { name : "get_data_device" , since : 1 , signature : & [ super :: ArgumentType :: NewId , super :: ArgumentType :: Object , ] , } , super :: MessageDesc { name : "destroy" , since : 1 , signature : & [ ] , } , ] ; type Map = super :: ProxyMap ; fn is_destructor ( & self ) -> bool { match * self { Request :: Destroy => true , _ => false , } } fn opcode ( & self ) -> u16 { match * self { Request :: CreateDataSource { .. } => 0 , Request :: GetDataDevice { .. } => 1 , Request :: Destroy => 2 , } } fn child < Meta : ObjectMetadata > ( opcode : u16 , version : u32 , meta : & Meta ) -> Option < Object < Meta >> { match opcode { 0 => Some ( Object :: from_interface :: < super :: zwlr_data_control_source_v1 :: ZwlrDataControlSourceV1 > ( version , meta . child ( ) , ) ) , 1 => Some ( Object :: from_interface :: < super :: zwlr_data_control_device_v1 :: ZwlrDataControlDeviceV1 > ( version , meta . child ( ) , ) ) , _ => None , } } fn from_raw ( msg : Message , map : & mut Self :: Map ) -> Result < Self , ( ) > { panic ! ( "Request::from_raw can not be used Client-side." ) } fn into_raw ( self , sender_id : u32 ) -> Message { match self { Request :: CreateDataSource { id } => Message { sender_id : sender_id , opcode : 0 , args : vec ! [ Argument :: NewId ( id . id ( ) ) , ] , } , Request :: GetDataDevice { id , seat } => Message { sender_id : sender_id , opcode : 1 , args : vec ! [ Argument :: NewId ( id . id ( ) ) , Argument :: Object ( seat . id ( ) ) , ] , } , Request :: Destroy => Message { sender_id : sender_id , opcode : 2 , args : vec ! [ ] , } , } } unsafe fn from_raw_c ( obj : * mut :: std :: os :: raw :: c_void , opcode : u32 , args : * const wl_argument , ) -> Result < Request , ( ) > { panic ! ( "Request::from_raw_c can not be used Client-side." ) } fn as_raw_c_in < F , T > ( self , f : F ) -> T where F : FnOnce ( u32 , & mut [ wl_argument ] ) -> T { match self { Request :: CreateDataSource { id } => { let mut _args_array : [ wl_argument ; 1 ] = unsafe { :: std :: mem :: zeroed ( ) } ; _args_array [ 0 ] . o = id . c_ptr ( ) as * mut _ ; f ( 0 , & mut _args_array ) } , Request :: GetDataDevice { id , seat } => { let mut _args_array : [ wl_argument ; 2 ] = unsafe { :: std :: mem :: zeroed ( ) } ; _args_array [ 0 ] . o = id . c_ptr ( ) as * mut _ ; _args_array [ 1 ] . o = seat . c_ptr ( ) as * mut _ ; f ( 1 , & mut _args_array ) } , Request :: Destroy => { let mut _args_array : [ wl_argument ; 0 ] = unsafe { :: std :: mem :: zeroed ( ) } ; f ( 2 , & mut _args_array ) } , } } } pub enum Event { } impl super :: MessageGroup for Event { const MESSAGES : & 'static [ super :: MessageDesc ] = & [ ] ; type Map = super :: ProxyMap ; fn is_destructor ( & self ) -> bool { match * self { } } fn opcode ( & self ) -> u16 { match * self { } } fn child < Meta : ObjectMetadata > ( opcode : u16 , version : u32 , meta : & Meta ) -> Option < Object < Meta >> { match opcode { _ => None , } } fn from_raw ( msg : Message , map : & mut Self :: Map ) -> Result < Self , ( ) > { match msg . opcode { _ => Err ( ( ) ) , } } fn into_raw ( self , sender_id : u32 ) -> Message { panic ! ( "Event::into_raw can not be used Client-side." ) } unsafe fn from_raw_c ( obj : * mut :: std :: os :: raw :: c_void , opcode : u32 , args : * const wl_argument , ) -> Result < Event , ( ) > { match opcode { _ => return Err ( ( ) ) , } } fn as_raw_c_in < F , T > ( self , f : F ) -> T where F : FnOnce ( u32 , & mut [ wl_argument ] ) -> T { panic ! ( "Event::as_raw_c_in can not be used Client-side." ) } } pub struct ZwlrDataControlManagerV1 ; impl Interface for ZwlrDataControlManagerV1 { type Request = Request ; type Event = Event ; const NAME : & 'static str = "zwlr_data_control_manager_v1" ; const VERSION : u32 = 1 ; fn c_interface ( ) -> * const wl_interface { unsafe { & super :: super :: c_interfaces :: zwlr_data_control_manager_v1_interface } } } pub trait RequestsTrait { # [ doc = "create a new data source\n\nCreate a new data source." ] fn create_data_source < F > ( & self , implementor : F ) -> Result < Proxy < super :: zwlr_data_control_source_v1 :: ZwlrDataControlSourceV1 > , ( ) > where F : FnOnce ( NewProxy < super :: zwlr_data_control_source_v1 :: ZwlrDataControlSourceV1 > , ) -> Proxy < super :: zwlr_data_control_source_v1 :: ZwlrDataControlSourceV1 > ; # [ doc = "get a data device for a seat\n\nCreate a data device that can be used to manage a seat's selection." ] fn get_data_device < F > ( & self , seat : & Proxy < super :: wl_seat :: WlSeat > , implementor : F ) -> Result < Proxy < super :: zwlr_data_control_device_v1 :: ZwlrDataControlDeviceV1 > , ( ) > where F : FnOnce ( NewProxy < super :: zwlr_data_control_device_v1 :: ZwlrDataControlDeviceV1 > , ) -> Proxy < super :: zwlr_data_control_device_v1 :: ZwlrDataControlDeviceV1 > ; # [ doc = "destroy the manager\n\nAll objects created by the manager will still remain valid, until their\nappropriate destroy request has been called.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called." ] fn destroy ( & self , ) -> ( ) ; } impl RequestsTrait for Proxy < ZwlrDataControlManagerV1 > { fn create_data_source < F > ( & self , implementor : F ) -> Result < Proxy < super :: zwlr_data_control_source_v1 :: ZwlrDataControlSourceV1 > , ( ) > where F : FnOnce ( NewProxy < super :: zwlr_data_control_source_v1 :: ZwlrDataControlSourceV1 > , ) -> Proxy < super :: zwlr_data_control_source_v1 :: ZwlrDataControlSourceV1 > { let msg = Request :: CreateDataSource { id : self . child_placeholder ( ) } ; self . send_constructor ( msg , implementor , None ) } fn get_data_device < F > ( & self , seat : & Proxy < super :: wl_seat :: WlSeat > , implementor : F ) -> Result < Proxy < super :: zwlr_data_control_device_v1 :: ZwlrDataControlDeviceV1 > , ( ) > where F : FnOnce ( NewProxy < super :: zwlr_data_control_device_v1 :: ZwlrDataControlDeviceV1 > , ) -> Proxy < super :: zwlr_data_control_device_v1 :: ZwlrDataControlDeviceV1 > { let msg = Request :: GetDataDevice { id : self . child_placeholder ( ) , seat : seat . clone ( ) } ; self . send_constructor ( msg , implementor , None ) } fn destroy ( & self , ) -> ( ) { let msg = Request :: Destroy ; self . send ( msg ) ; } } # [ doc = r" The minimal object version supporting this request" ] pub const REQ_CREATE_DATA_SOURCE_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this request" ] pub const REQ_GET_DATA_DEVICE_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this request" ] pub const REQ_DESTROY_SINCE : u16 = 1u16 ; } # [ doc = "manage a data device for a seat\n\nThis interface allows a client to manage a seat's selection.\n\nWhen the seat is destroyed, this object becomes inert." ] pub mod zwlr_data_control_device_v1 { use super :: { Proxy , NewProxy , AnonymousObject , Interface , MessageGroup , MessageDesc , ArgumentType , Object , Message , Argument , ObjectMetadata } ; use super :: sys :: common :: { wl_argument , wl_interface , wl_array } ; use super :: sys :: client :: * ; pub enum Request { # [ doc = "copy data to the selection\n\nAll objects created by the device will still remain valid, until their\nappropriate destroy request has been called." ] SetSelection { source : Option < Proxy < super :: zwlr_data_control_source_v1 :: ZwlrDataControlSourceV1 > > , } , # [ doc = "destroy this data device\n\nDestroys the data device object.\n\nThis is a destructor, once sent this object cannot be used any longer." ] Destroy , } impl super :: MessageGroup for Request { const MESSAGES : & 'static [ super :: MessageDesc ] = & [ super :: MessageDesc { name : "set_selection" , since : 1 , signature : & [ super :: ArgumentType :: Object , ] , } , super :: MessageDesc { name : "destroy" , since : 1 , signature : & [ ] , } , ] ; type Map = super :: ProxyMap ; fn is_destructor ( & self ) -> bool { match * self { Request :: Destroy => true , _ => false , } } fn opcode ( & self ) -> u16 { match * self { Request :: SetSelection { .. } => 0 , Request :: Destroy => 1 , } } fn child < Meta : ObjectMetadata > ( opcode : u16 , version : u32 , meta : & Meta ) -> Option < Object < Meta >> { match opcode { _ => None , } } fn from_raw ( msg : Message , map : & mut Self :: Map ) -> Result < Self , ( ) > { panic ! ( "Request::from_raw can not be used Client-side." ) } fn into_raw ( self , sender_id : u32 ) -> Message { match self { Request :: SetSelection { source } => Message { sender_id : sender_id , opcode : 0 , args : vec ! [ Argument :: Object ( source . map ( | o | o . id ( ) ) . unwrap_or ( 0 ) ) , ] , } , Request :: Destroy => Message { sender_id : sender_id , opcode : 1 , args : vec ! [ ] , } , } } unsafe fn from_raw_c ( obj : * mut :: std :: os :: raw :: c_void , opcode : u32 , args : * const wl_argument , ) -> Result < Request , ( ) > { panic ! ( "Request::from_raw_c can not be used Client-side." ) } fn as_raw_c_in < F , T > ( self , f : F ) -> T where F : FnOnce ( u32 , & mut [ wl_argument ] ) -> T { match self { Request :: SetSelection { source } => { let mut _args_array : [ wl_argument ; 1 ] = unsafe { :: std :: mem :: zeroed ( ) } ; _args_array [ 0 ] . o = source . map ( | o | o . c_ptr ( ) as * mut _ ) . unwrap_or ( :: std :: ptr :: null_mut ( ) ) ; f ( 0 , & mut _args_array ) } , Request :: Destroy => { let mut _args_array : [ wl_argument ; 0 ] = unsafe { :: std :: mem :: zeroed ( ) } ; f ( 1 , & mut _args_array ) } , } } } pub enum Event { # [ doc = "introduce a new wlr_data_control_offer\n\nThe data_offer event introduces a new wlr_data_control_offer object,\nwhich will subsequently be used in the wlr_data_control_device.selection\nevent. Immediately following the wlr_data_control_device.data_offer\nevent, the new data_offer object will send out\nwlr_data_control_offer.offer events to describe the MIME types it\noffers.\n\nThis event replaces the previous data offer, which should be destroyed\nby the client." ] DataOffer { id : NewProxy < super :: zwlr_data_control_offer_v1 :: ZwlrDataControlOfferV1 > , } , # [ doc = "introduce a new wlr_data_control_offer\n\nThe selection event is sent out to notify the client of a new\nwlr_data_control_offer for the selection for this device. The\nwlr_data_control_device.data_offer and the wlr_data_control_offer.offer\nevents are sent out immediately before this event to introduce the data\noffer object. The selection event is sent to a client when a new\nselection is set. The wlr_data_control_offer is valid until a new\nwlr_data_control_offer or NULL is received. The client must destroy the\nprevious selection wlr_data_control_offer, if any, upon receiving this\nevent." ] Selection { id : Option < Proxy < super :: zwlr_data_control_offer_v1 :: ZwlrDataControlOfferV1 > > , } , # [ doc = "this data control is no longer valid\n\nThis data control object is no longer valid and should be destroyed by\nthe client." ] Finished , } impl super :: MessageGroup for Event { const MESSAGES : & 'static [ super :: MessageDesc ] = & [ super :: MessageDesc { name : "data_offer" , since : 1 , signature : & [ super :: ArgumentType :: NewId , ] , } , super :: MessageDesc { name : "selection" , since : 1 , signature : & [ super :: ArgumentType :: Object , ] , } , super :: MessageDesc { name : "finished" , since : 1 , signature : & [ ] , } , ] ; type Map = super :: ProxyMap ; fn is_destructor ( & self ) -> bool { match * self { _ => false , } } fn opcode ( & self ) -> u16 { match * self { Event :: DataOffer { .. } => 0 , Event :: Selection { .. } => 1 , Event :: Finished => 2 , } } fn child < Meta : ObjectMetadata > ( opcode : u16 , version : u32 , meta : & Meta ) -> Option < Object < Meta >> { match opcode { 0 => Some ( Object :: from_interface :: < super :: zwlr_data_control_offer_v1 :: ZwlrDataControlOfferV1 > ( version , meta . child ( ) , ) ) , _ => None , } } fn from_raw ( msg : Message , map : & mut Self :: Map ) -> Result < Self , ( ) > { match msg . opcode { 0 => { let mut args = msg . args . into_iter ( ) ; Ok ( Event :: DataOffer { id : { if let Some ( Argument :: NewId ( val ) ) = args . next ( ) { map . get_new ( val ) . ok_or ( ( ) ) ? } else { return Err ( ( ) ) ; } } , } ) } , 1 => { let mut args = msg . args . into_iter ( ) ; Ok ( Event :: Selection { id : { if let Some ( Argument :: Object ( val ) ) = args . next ( ) { if val == 0 { None } else { Some ( map . get ( val ) . ok_or ( ( ) ) ? ) } } else { return Err ( ( ) ) ; } } , } ) } , 2 => Ok ( Event :: Finished ) , _ => Err ( ( ) ) , } } fn into_raw ( self , sender_id : u32 ) -> Message { panic ! ( "Event::into_raw can not be used Client-side." ) } unsafe fn from_raw_c ( obj : * mut :: std :: os :: raw :: c_void , opcode : u32 , args : * const wl_argument , ) -> Result < Event , ( ) > { match opcode { 0 => { let _args = :: std :: slice :: from_raw_parts ( args , 1 ) ; Ok ( Event :: DataOffer { id : NewProxy :: < super :: zwlr_data_control_offer_v1 :: ZwlrDataControlOfferV1 > :: from_c_ptr ( _args [ 0 ] . o as * mut _ ) , } ) } , 1 => { let _args = :: std :: slice :: from_raw_parts ( args , 1 ) ; Ok ( Event :: Selection { id : if _args [ 0 ] . o . is_null ( ) { None } else { Some ( Proxy :: < super :: zwlr_data_control_offer_v1 :: ZwlrDataControlOfferV1 > :: from_c_ptr ( _args [ 0 ] . o as * mut _ , ) ) } , } ) } , 2 => { Ok ( Event :: Finished ) } , _ => return Err ( ( ) ) , } } fn as_raw_c_in < F , T > ( self , f : F ) -> T where F : FnOnce ( u32 , & mut [ wl_argument ] ) -> T { panic ! ( "Event::as_raw_c_in can not be used Client-side." ) } } pub struct ZwlrDataControlDeviceV1 ; impl Interface for ZwlrDataControlDeviceV1 { type Request = Request ; type Event = Event ; const NAME : & 'static str = "zwlr_data_control_device_v1" ; const VERSION : u32 = 1 ; fn c_interface ( ) -> * const wl_interface { unsafe { & super :: super :: c_interfaces :: zwlr_data_control_device_v1_interface } } } pub trait RequestsTrait { # [ doc = "copy data to the selection\n\nAll objects created by the device will still remain valid, until their\nappropriate destroy request has been called." ] fn set_selection ( & self , source : Option < & Proxy < super :: zwlr_data_control_source_v1 :: ZwlrDataControlSourceV1 > > ) -> ( ) ; # [ doc = "destroy this data device\n\nDestroys the data device object.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called." ] fn destroy ( & self , ) -> ( ) ; } impl RequestsTrait for Proxy < ZwlrDataControlDeviceV1 > { fn set_selection ( & self , source : Option < & Proxy < super :: zwlr_data_control_source_v1 :: ZwlrDataControlSourceV1 > > ) -> ( ) { let msg = Request :: SetSelection { source : source . map ( | o | o . clone ( ) ) } ; self . send ( msg ) ; } fn destroy ( & self , ) -> ( ) { let msg = Request :: Destroy ; self . send ( msg ) ; } } # [ doc = r" The minimal object version supporting this request" ] pub const REQ_SET_SELECTION_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this request" ] pub const REQ_DESTROY_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this event" ] pub const EVT_DATA_OFFER_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this event" ] pub const EVT_SELECTION_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this event" ] pub const EVT_FINISHED_SINCE : u16 = 1u16 ; } # [ doc = "offer to transfer data\n\nThe wlr_data_control_source object is the source side of a\nwlr_data_control_offer. It is created by the source client in a data\ntransfer and provides a way to describe the offered data and a way to\nrespond to requests to transfer the data." ] pub mod zwlr_data_control_source_v1 { use super :: { Proxy , NewProxy , AnonymousObject , Interface , MessageGroup , MessageDesc , ArgumentType , Object , Message , Argument , ObjectMetadata } ; use super :: sys :: common :: { wl_argument , wl_interface , wl_array } ; use super :: sys :: client :: * ; # [ repr ( u32 ) ] # [ derive ( Copy , Clone , Debug , PartialEq ) ] pub enum Error { # [ doc = "offer sent after wlr_data_control_device.set_selection" ] InvalidOffer = 1 , } impl Error { pub fn from_raw ( n : u32 ) -> Option < Error > { match n { 1 => Some ( Error :: InvalidOffer ) , _ => Option :: None } } pub fn to_raw ( & self ) -> u32 { * self as u32 } } pub enum Request { # [ doc = "add an offered MIME type\n\nThis request adds a MIME type to the set of MIME types advertised to\ntargets. Can be called several times to offer multiple types.\n\nCalling this after wlr_data_control_device.set_selection is a protocol\nerror." ] Offer { mime_type : String , } , # [ doc = "destroy this source\n\nDestroys the data source object.\n\nThis is a destructor, once sent this object cannot be used any longer." ] Destroy , } impl super :: MessageGroup for Request { const MESSAGES : & 'static [ super :: MessageDesc ] = & [ super :: MessageDesc { name : "offer" , since : 1 , signature : & [ super :: ArgumentType :: Str , ] , } , super :: MessageDesc { name : "destroy" , since : 1 , signature : & [ ] , } , ] ; type Map = super :: ProxyMap ; fn is_destructor ( & self ) -> bool { match * self { Request :: Destroy => true , _ => false , } } fn opcode ( & self ) -> u16 { match * self { Request :: Offer { .. } => 0 , Request :: Destroy => 1 , } } fn child < Meta : ObjectMetadata > ( opcode : u16 , version : u32 , meta : & Meta ) -> Option < Object < Meta >> { match opcode { _ => None , } } fn from_raw ( msg : Message , map : & mut Self :: Map ) -> Result < Self , ( ) > { panic ! ( "Request::from_raw can not be used Client-side." ) } fn into_raw ( self , sender_id : u32 ) -> Message { match self { Request :: Offer { mime_type } => Message { sender_id : sender_id , opcode : 0 , args : vec ! [ Argument :: Str ( unsafe { :: std :: ffi :: CString :: from_vec_unchecked ( mime_type . into ( ) ) } ) , ] , } , Request :: Destroy => Message { sender_id : sender_id , opcode : 1 , args : vec ! [ ] , } , } } unsafe fn from_raw_c ( obj : * mut :: std :: os :: raw :: c_void , opcode : u32 , args : * const wl_argument , ) -> Result < Request , ( ) > { panic ! ( "Request::from_raw_c can not be used Client-side." ) } fn as_raw_c_in < F , T > ( self , f : F ) -> T where F : FnOnce ( u32 , & mut [ wl_argument ] ) -> T { match self { Request :: Offer { mime_type } => { let mut _args_array : [ wl_argument ; 1 ] = unsafe { :: std :: mem :: zeroed ( ) } ; let _arg_0 = :: std :: ffi :: CString :: new ( mime_type ) . unwrap ( ) ; _args_array [ 0 ] . s = _arg_0 . as_ptr ( ) ; f ( 0 , & mut _args_array ) } , Request :: Destroy => { let mut _args_array : [ wl_argument ; 0 ] = unsafe { :: std :: mem :: zeroed ( ) } ; f ( 1 , & mut _args_array ) } , } } } pub enum Event { # [ doc = "send the data\n\nRequest for data from the client. Send the data as the specified MIME\ntype over the passed file descriptor, then close it." ] Send { mime_type : String , fd : :: std :: os :: unix :: io :: RawFd , } , # [ doc = "selection was cancelled\n\nThis data source is no longer valid. The data source has been replaced\nby another data source.\n\nThe client should clean up and destroy this data source." ] Cancelled , } impl super :: MessageGroup for Event { const MESSAGES : & 'static [ super :: MessageDesc ] = & [ super :: MessageDesc { name : "send" , since : 1 , signature : & [ super :: ArgumentType :: Str , super :: ArgumentType :: Fd , ] , } , super :: MessageDesc { name : "cancelled" , since : 1 , signature : & [ ] , } , ] ; type Map = super :: ProxyMap ; fn is_destructor ( & self ) -> bool { match * self { _ => false , } } fn opcode ( & self ) -> u16 { match * self { Event :: Send { .. } => 0 , Event :: Cancelled => 1 , } } fn child < Meta : ObjectMetadata > ( opcode : u16 , version : u32 , meta : & Meta ) -> Option < Object < Meta >> { match opcode { _ => None , } } fn from_raw ( msg : Message , map : & mut Self :: Map ) -> Result < Self , ( ) > { match msg . opcode { 0 => { let mut args = msg . args . into_iter ( ) ; Ok ( Event :: Send { mime_type : { if let Some ( Argument :: Str ( val ) ) = args . next ( ) { let s = String :: from_utf8 ( val . into_bytes ( ) ) . unwrap_or_else ( | e | String :: from_utf8_lossy ( & e . into_bytes ( ) ) . into ( ) ) ; s } else { return Err ( ( ) ) ; } } , fd : { if let Some ( Argument :: Fd ( val ) ) = args . next ( ) { val } else { return Err ( ( ) ) ; } } , } ) } , 1 => Ok ( Event :: Cancelled ) , _ => Err ( ( ) ) , } } fn into_raw ( self , sender_id : u32 ) -> Message { panic ! ( "Event::into_raw can not be used Client-side." ) } unsafe fn from_raw_c ( obj : * mut :: std :: os :: raw :: c_void , opcode : u32 , args : * const wl_argument , ) -> Result < Event , ( ) > { match opcode { 0 => { let _args = :: std :: slice :: from_raw_parts ( args , 2 ) ; Ok ( Event :: Send { mime_type : :: std :: ffi :: CStr :: from_ptr ( _args [ 0 ] . s ) . to_string_lossy ( ) . into_owned ( ) , fd : _args [ 1 ] . h , } ) } , 1 => { Ok ( Event :: Cancelled ) } , _ => return Err ( ( ) ) , } } fn as_raw_c_in < F , T > ( self , f : F ) -> T where F : FnOnce ( u32 , & mut [ wl_argument ] ) -> T { panic ! ( "Event::as_raw_c_in can not be used Client-side." ) } } pub struct ZwlrDataControlSourceV1 ; impl Interface for ZwlrDataControlSourceV1 { type Request = Request ; type Event = Event ; const NAME : & 'static str = "zwlr_data_control_source_v1" ; const VERSION : u32 = 1 ; fn c_interface ( ) -> * const wl_interface { unsafe { & super :: super :: c_interfaces :: zwlr_data_control_source_v1_interface } } } pub trait RequestsTrait { # [ doc = "add an offered MIME type\n\nThis request adds a MIME type to the set of MIME types advertised to\ntargets. Can be called several times to offer multiple types.\n\nCalling this after wlr_data_control_device.set_selection is a protocol\nerror." ] fn offer ( & self , mime_type : String ) -> ( ) ; # [ doc = "destroy this source\n\nDestroys the data source object.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called." ] fn destroy ( & self , ) -> ( ) ; } impl RequestsTrait for Proxy < ZwlrDataControlSourceV1 > { fn offer ( & self , mime_type : String ) -> ( ) { let msg = Request :: Offer { mime_type : mime_type } ; self . send ( msg ) ; } fn destroy ( & self , ) -> ( ) { let msg = Request :: Destroy ; self . send ( msg ) ; } } # [ doc = r" The minimal object version supporting this request" ] pub const REQ_OFFER_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this request" ] pub const REQ_DESTROY_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this event" ] pub const EVT_SEND_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this event" ] pub const EVT_CANCELLED_SINCE : u16 = 1u16 ; } # [ doc = "offer to transfer data\n\nA wlr_data_control_offer represents a piece of data offered for transfer\nby another client (the source client). The offer describes the different\nMIME types that the data can be converted to and provides the mechanism\nfor transferring the data directly from the source client." ] pub mod zwlr_data_control_offer_v1 { use super :: { Proxy , NewProxy , AnonymousObject , Interface , MessageGroup , MessageDesc , ArgumentType , Object , Message , Argument , ObjectMetadata } ; use super :: sys :: common :: { wl_argument , wl_interface , wl_array } ; use super :: sys :: client :: * ; pub enum Request { # [ doc = "request that the data is transferred\n\nTo transfer the offered data, the client issues this request and\nindicates the MIME type it wants to receive. The transfer happens\nthrough the passed file descriptor (typically created with the pipe\nsystem call). The source client writes the data in the MIME type\nrepresentation requested and then closes the file descriptor.\n\nThe receiving client reads from the read end of the pipe until EOF and\nthen closes its end, at which point the transfer is complete.\n\nThis request may happen multiple times for different MIME types." ] Receive { mime_type : String , fd : :: std :: os :: unix :: io :: RawFd , } , # [ doc = "destroy this offer\n\nDestroys the data offer object.\n\nThis is a destructor, once sent this object cannot be used any longer." ] Destroy , } impl super :: MessageGroup for Request { const MESSAGES : & 'static [ super :: MessageDesc ] = & [ super :: MessageDesc { name : "receive" , since : 1 , signature : & [ super :: ArgumentType :: Str , super :: ArgumentType :: Fd , ] , } , super :: MessageDesc { name : "destroy" , since : 1 , signature : & [ ] , } , ] ; type Map = super :: ProxyMap ; fn is_destructor ( & self ) -> bool { match * self { Request :: Destroy => true , _ => false , } } fn opcode ( & self ) -> u16 { match * self { Request :: Receive { .. } => 0 , Request :: Destroy => 1 , } } fn child < Meta : ObjectMetadata > ( opcode : u16 , version : u32 , meta : & Meta ) -> Option < Object < Meta >> { match opcode { _ => None , } } fn from_raw ( msg : Message , map : & mut Self :: Map ) -> Result < Self , ( ) > { panic ! ( "Request::from_raw can not be used Client-side." ) } fn into_raw ( self , sender_id : u32 ) -> Message { match self { Request :: Receive { mime_type , fd } => Message { sender_id : sender_id , opcode : 0 , args : vec ! [ Argument :: Str ( unsafe { :: std :: ffi :: CString :: from_vec_unchecked ( mime_type . into ( ) ) } ) , Argument :: Fd ( fd ) , ] , } , Request :: Destroy => Message { sender_id : sender_id , opcode : 1 , args : vec ! [ ] , } , } } unsafe fn from_raw_c ( obj : * mut :: std :: os :: raw :: c_void , opcode : u32 , args : * const wl_argument , ) -> Result < Request , ( ) > { panic ! ( "Request::from_raw_c can not be used Client-side." ) } fn as_raw_c_in < F , T > ( self , f : F ) -> T where F : FnOnce ( u32 , & mut [ wl_argument ] ) -> T { match self { Request :: Receive { mime_type , fd } => { let mut _args_array : [ wl_argument ; 2 ] = unsafe { :: std :: mem :: zeroed ( ) } ; let _arg_0 = :: std :: ffi :: CString :: new ( mime_type ) . unwrap ( ) ; _args_array [ 0 ] . s = _arg_0 . as_ptr ( ) ; _args_array [ 1 ] . h = fd ; f ( 0 , & mut _args_array ) } , Request :: Destroy => { let mut _args_array : [ wl_argument ; 0 ] = unsafe { :: std :: mem :: zeroed ( ) } ; f ( 1 , & mut _args_array ) } , } } } pub enum Event { # [ doc = "advertise offered MIME type\n\nSent immediately after creating the wlr_data_control_offer object.\nOne event per offered MIME type." ] Offer { mime_type : String , } , } impl super :: MessageGroup for Event { const MESSAGES : & 'static [ super :: MessageDesc ] = & [ super :: MessageDesc { name : "offer" , since : 1 , signature : & [ super :: ArgumentType :: Str , ] , } , ] ; type Map = super :: ProxyMap ; fn is_destructor ( & self ) -> bool { match * self { _ => false , } } fn opcode ( & self ) -> u16 { match * self { Event :: Offer { .. } => 0 , } } fn child < Meta : ObjectMetadata > ( opcode : u16 , version : u32 , meta : & Meta ) -> Option < Object < Meta >> { match opcode { _ => None , } } fn from_raw ( msg : Message , map : & mut Self :: Map ) -> Result < Self , ( ) > { match msg . opcode { 0 => { let mut args = msg . args . into_iter ( ) ; Ok ( Event :: Offer { mime_type : { if let Some ( Argument :: Str ( val ) ) = args . next ( ) { let s = String :: from_utf8 ( val . into_bytes ( ) ) . unwrap_or_else ( | e | String :: from_utf8_lossy ( & e . into_bytes ( ) ) . into ( ) ) ; s } else { return Err ( ( ) ) ; } } , } ) } , _ => Err ( ( ) ) , } } fn into_raw ( self , sender_id : u32 ) -> Message { panic ! ( "Event::into_raw can not be used Client-side." ) } unsafe fn from_raw_c ( obj : * mut :: std :: os :: raw :: c_void , opcode : u32 , args : * const wl_argument , ) -> Result < Event , ( ) > { match opcode { 0 => { let _args = :: std :: slice :: from_raw_parts ( args , 1 ) ; Ok ( Event :: Offer { mime_type : :: std :: ffi :: CStr :: from_ptr ( _args [ 0 ] . s ) . to_string_lossy ( ) . into_owned ( ) , } ) } , _ => return Err ( ( ) ) , } } fn as_raw_c_in < F , T > ( self , f : F ) -> T where F : FnOnce ( u32 , & mut [ wl_argument ] ) -> T { panic ! ( "Event::as_raw_c_in can not be used Client-side." ) } } pub struct ZwlrDataControlOfferV1 ; impl Interface for ZwlrDataControlOfferV1 { type Request = Request ; type Event = Event ; const NAME : & 'static str = "zwlr_data_control_offer_v1" ; const VERSION : u32 = 1 ; fn c_interface ( ) -> * const wl_interface { unsafe { & super :: super :: c_interfaces :: zwlr_data_control_offer_v1_interface } } } pub trait RequestsTrait { # [ doc = "request that the data is transferred\n\nTo transfer the offered data, the client issues this request and\nindicates the MIME type it wants to receive. The transfer happens\nthrough the passed file descriptor (typically created with the pipe\nsystem call). The source client writes the data in the MIME type\nrepresentation requested and then closes the file descriptor.\n\nThe receiving client reads from the read end of the pipe until EOF and\nthen closes its end, at which point the transfer is complete.\n\nThis request may happen multiple times for different MIME types." ] fn receive ( & self , mime_type : String , fd : :: std :: os :: unix :: io :: RawFd ) -> ( ) ; # [ doc = "destroy this offer\n\nDestroys the data offer object.\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called." ] fn destroy ( & self , ) -> ( ) ; } impl RequestsTrait for Proxy < ZwlrDataControlOfferV1 > { fn receive ( & self , mime_type : String , fd : :: std :: os :: unix :: io :: RawFd ) -> ( ) { let msg = Request :: Receive { mime_type : mime_type , fd : fd } ; self . send ( msg ) ; } fn destroy ( & self , ) -> ( ) { let msg = Request :: Destroy ; self . send ( msg ) ; } } # [ doc = r" The minimal object version supporting this request" ] pub const REQ_RECEIVE_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this request" ] pub const REQ_DESTROY_SINCE : u16 = 1u16 ; # [ doc = r" The minimal object version supporting this event" ] pub const EVT_OFFER_SINCE : u16 = 1u16 ; }