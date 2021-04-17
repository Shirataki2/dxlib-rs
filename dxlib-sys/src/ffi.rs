use winapi::{
    shared::{
        guiddef::GUID,
        minwindef::{HINSTANCE, HMODULE, HRGN},
        windef::{HICON, HWND},
    },
    um::winuser::WNDPROC,
};

use crate::data::*;

#[link(name = "DxLib_x64")]
extern "C" {
    pub fn dx_SetKeyInputStringColor(
        NmlStr: u64,
        NmlCur: u64,
        IMEStr: u64,
        IMECur: u64,
        IMELine: u64,
        IMESelectStr: u64,
        IMEModeStr: u64,
        NmlStrE: u64,
        IMESelectStrE: u64,
        IMEModeStrE: u64,
        IMESelectWinE: u64,
        IMESelectWinF: u64,
        SelectStrBackColor: u64,
        SelectStrColor: u64,
        SelectStrEdgeColor: u64,
    ) -> i32;
    pub fn dx_Paint(x: i32, y: i32, FillColor: u32, BoundaryColor: u64) -> i32;
    pub fn dx_MV1SetMaterialTypeParamAllS(
        MHandle: i32,
        Param0: f32,
        Param1: f32,
        Param2: f32,
        Param3: f32,
        Param4: f32,
        Param5: f32,
    ) -> i32;
    pub fn dx_MV1SetMaterialTypeParamS(
        MHandle: i32,
        MaterialIndex: i32,
        Param0: f32,
        Param1: f32,
        Param2: f32,
        Param3: f32,
        Param4: f32,
        Param5: f32,
    ) -> i32;
    pub fn dx_GraphFilterS(
        GrHandle: i32,
        FilterType: i32,
        Param0: i32,
        Param1: i32,
        Param2: i32,
        Param3: i32,
        Param4: i32,
        Param5: i32,
    ) -> i32;
    pub fn dx_GraphFilterBltS(
        SrcGrHandle: i32,
        DestGrHandle: i32,
        FilterType: i32,
        Param0: i32,
        Param1: i32,
        Param2: i32,
        Param3: i32,
        Param4: i32,
        Param5: i32,
    ) -> i32;
    pub fn dx_GraphFilterRectBltS(
        SrcGrHandle: i32,
        DestGrHandle: i32,
        SrcX1: i32,
        SrcY1: i32,
        SrcX2: i32,
        SrcY2: i32,
        DestX: i32,
        DestY: i32,
        FilterType: i32,
        Param0: i32,
        Param1: i32,
        Param2: i32,
        Param3: i32,
        Param4: i32,
        Param5: i32,
    ) -> i32;
    pub fn dx_GraphBlendS(
        GrHandle: i32,
        BlendGrHandle: i32,
        BlendRatio: i32,
        BlendType: i32,
        Param0: i32,
        Param1: i32,
        Param2: i32,
        Param3: i32,
        Param4: i32,
        Param5: i32,
    ) -> i32;
    pub fn dx_GraphBlendBltS(
        SrcGrHandle: i32,
        BlendGrHandle: i32,
        DestGrHandle: i32,
        BlendRatio: i32,
        BlendType: i32,
        Param0: i32,
        Param1: i32,
        Param2: i32,
        Param3: i32,
        Param4: i32,
        Param5: i32,
    ) -> i32;
    pub fn dx_GraphBlendRectBltS(
        SrcGrHandle: i32,
        BlendGrHandle: i32,
        DestGrHandle: i32,
        SrcX1: i32,
        SrcY1: i32,
        SrcX2: i32,
        SrcY2: i32,
        BlendX: i32,
        BlendY: i32,
        DestX: i32,
        DestY: i32,
        BlendRatio: i32,
        BlendType: i32,
        Param0: i32,
        Param1: i32,
        Param2: i32,
        Param3: i32,
        Param4: i32,
        Param5: i32,
    ) -> i32;
    pub fn dx_SetBlendGraphParamS(
        BlendGraph: i32,
        BlendType: i32,
        Param0: i32,
        Param1: i32,
        Param2: i32,
        Param3: i32,
        Param4: i32,
        Param5: i32,
    ) -> i32;
    pub fn dx_MGetTranslateElem(InM: Matrix) -> Vector;
    pub fn dx_MGetTranslateElemD(InM: DMatrix) -> DVector;
    pub fn dx_VConvFtoD(In: Vector) -> DVector;
    pub fn dx_VConvDtoF(In: DVector) -> Vector;
    pub fn dx_VGet(x: f32, y: f32, z: f32) -> Vector;
    pub fn dx_VGetD(x: f64, y: f64, z: f64) -> DVector;
    pub fn dx_F2Get(u: f32, v: f32) -> Float2;
    pub fn dx_F4Get(x: f32, y: f32, z: f32, w: f32) -> Float4;
    pub fn dx_D4Get(x: f64, y: f64, z: f64, w: f64) -> Double4;
    pub fn dx_VAdd(In1: Vector, In2: Vector) -> Vector;
    pub fn dx_VAddD(In1: DVector, In2: DVector) -> DVector;
    pub fn dx_VSub(In1: Vector, In2: Vector) -> Vector;
    pub fn dx_VSubD(In1: DVector, In2: DVector) -> DVector;
    pub fn dx_F2Add(In1: Float2, In2: Float2) -> Float2;
    pub fn dx_F4Add(In1: Float4, In2: Float4) -> Float4;
    pub fn dx_D4Add(In1: Double4, In2: Double4) -> Double4;
    pub fn dx_F2Sub(In1: Float2, In2: Float2) -> Float2;
    pub fn dx_F4Sub(In1: Float4, In2: Float4) -> Float4;
    pub fn dx_D4Sub(In1: Double4, In2: Double4) -> Double4;
    pub fn dx_VDot(In1: Vector, In2: Vector) -> f32;
    pub fn dx_VDotD(In1: DVector, In2: DVector) -> f64;
    pub fn dx_VCross(In1: Vector, In2: Vector) -> Vector;
    pub fn dx_VCrossD(In1: DVector, In2: DVector) -> DVector;
    pub fn dx_VScale(In: Vector, Scale: f32) -> Vector;
    pub fn dx_VScaleD(In: DVector, Scale: f64) -> DVector;
    pub fn dx_F2Scale(In: Float2, Scale: f32) -> Float2;
    pub fn dx_F4Scale(In: Float4, Scale: f32) -> Float4;
    pub fn dx_D4Scale(In: Double4, Scale: f64) -> Double4;
    pub fn dx_VSquareSize(In: Vector) -> f32;
    pub fn dx_VSquareSizeD(In: DVector) -> f64;
    pub fn dx_VTransform(InV: Vector, InM: Matrix) -> Vector;
    pub fn dx_VTransformD(InV: DVector, InM: DMatrix) -> DVector;
    pub fn dx_VTransformSR(InV: Vector, InM: Matrix) -> Vector;
    pub fn dx_VTransformSRD(InV: DVector, InM: DMatrix) -> DVector;
    pub fn dx_QTCross(A: Float4, B: Float4) -> Float4;
    pub fn dx_QTCrossD(A: Double4, B: Double4) -> Double4;
    pub fn dx_QTConj(A: Float4) -> Float4;
    pub fn dx_QTConjD(A: Double4) -> Double4;
    pub fn dx_DxLib_Init() -> i32;
    pub fn dx_DxLib_End() -> i32;
    pub fn dx_DxLib_GlobalStructInitialize() -> i32;
    pub fn dx_DxLib_IsInit() -> i32;
    pub fn dx_GetLastErrorCode() -> i32;
    pub fn dx_GetLastErrorMessage(StringBuffer: *mut u8, StringBufferBytes: i32) -> i32;
    pub fn dx_ProcessMessage() -> i32;
    pub fn dx_SetAlwaysRunFlag(Flag: i32) -> i32;
    pub fn dx_WaitTimer(WaitTime: i32) -> i32;
    pub fn dx_WaitKey() -> i32;
    pub fn dx_GetNowCount(UseRDTSCFlag: i32) -> i32;
    pub fn dx_GetNowHiPerformanceCount(UseRDTSCFlag: i32) -> i64;
    pub fn dx_GetNowSysPerformanceCount() -> u64;
    pub fn dx_GetSysPerformanceFrequency() -> u64;
    pub fn dx_ConvSysPerformanceCountToSeconds(Count: u64) -> u64;
    pub fn dx_ConvSysPerformanceCountToMilliSeconds(Count: u64) -> u64;
    pub fn dx_ConvSysPerformanceCountToMicroSeconds(Count: u64) -> u64;
    pub fn dx_ConvSysPerformanceCountToNanoSeconds(Count: u64) -> u64;
    pub fn dx_ConvSecondsToSysPerformanceCount(Seconds: u64) -> u64;
    pub fn dx_ConvMilliSecondsToSysPerformanceCount(MilliSeconds: u64) -> u64;
    pub fn dx_ConvMicroSecondsToSysPerformanceCount(MicroSeconds: u64) -> u64;
    pub fn dx_ConvNanoSecondsToSysPerformanceCount(NanoSeconds: u64) -> u64;
    pub fn dx_GetDateTime(DateBuf: *mut DateData) -> i32;
    pub fn dx_GetRand(RandMax: i32) -> i32;
    pub fn dx_SRand(Seed: i32) -> i32;
    pub fn dx_GetBatteryLifePercent() -> i32;
    pub fn dx_GetClipboardText(DestBuffer: *mut u8) -> i32;
    pub fn dx_SetClipboardText(Text: *const i8) -> i32;
    pub fn dx_SetClipboardTextWithStrLen(Text: *const i8, TextLength: usize) -> i32;
    pub fn dx_GetPrivateProfileStringDx(
        AppName: *const i8,
        KeyName: *const i8,
        Default: *const i8,
        ReturnedStringBuffer: *mut u8,
        ReturnedStringBufferBytes: usize,
        IniFilePath: *const i8,
        IniFileCharCodeFormat: i32,
    ) -> i32;
    pub fn dx_GetPrivateProfileStringDxWithStrLen(
        AppName: *const i8,
        AppNameLength: usize,
        KeyName: *const i8,
        KeyNameLength: usize,
        Default: *const i8,
        DefaultLength: usize,
        ReturnedStringBuffer: *mut u8,
        ReturnedStringBufferBytes: usize,
        IniFilePath: *const i8,
        IniFilePathLength: usize,
        IniFileCharCodeFormat: i32,
    ) -> i32;
    pub fn dx_GetPrivateProfileIntDx(
        AppName: *const i8,
        KeyName: *const i8,
        Default: i32,
        IniFilePath: *const i8,
        IniFileCharCodeFormat: i32,
    ) -> i32;
    pub fn dx_GetPrivateProfileIntDxWithStrLen(
        AppName: *const i8,
        AppNameLength: usize,
        KeyName: *const i8,
        KeyNameLength: usize,
        Default: i32,
        IniFilePath: *const i8,
        IniFilePathLength: usize,
        IniFileCharCodeFormat: i32,
    ) -> i32;
    pub fn dx_GetPrivateProfileStringDxForMem(
        AppName: *const i8,
        KeyName: *const i8,
        Default: *const i8,
        ReturnedStringBuffer: *mut u8,
        ReturnedStringBufferBytes: usize,
        IniFileImage: *const libc::c_void,
        IniFileImageBytes: usize,
        IniFileCharCodeFormat: i32,
    ) -> i32;
    pub fn dx_GetPrivateProfileStringDxForMemWithStrLen(
        AppName: *const i8,
        AppNameLength: usize,
        KeyName: *const i8,
        KeyNameLength: usize,
        Default: *const i8,
        DefaultLength: usize,
        ReturnedStringBuffer: *mut u8,
        ReturnedStringBufferBytes: usize,
        IniFileImage: *const libc::c_void,
        IniFileImageBytes: usize,
        IniFileCharCodeFormat: i32,
    ) -> i32;
    pub fn dx_GetPrivateProfileIntDxForMem(
        AppName: *const i8,
        KeyName: *const i8,
        Default: i32,
        IniFileImage: *const libc::c_void,
        IniFileImageBytes: usize,
        IniFileCharCodeFormat: i32,
    ) -> i32;
    pub fn dx_GetPrivateProfileIntDxForMemWithStrLen(
        AppName: *const i8,
        AppNameLength: usize,
        KeyName: *const i8,
        KeyNameLength: usize,
        Default: i32,
        IniFileImage: *const libc::c_void,
        IniFileImageBytes: usize,
        IniFileCharCodeFormat: i32,
    ) -> i32;
    pub fn dx_LogFileAdd(String: *const i8) -> i32;
    pub fn dx_LogFileAddWithStrLen(String: *const i8, StringLength: usize) -> i32;
    pub fn dx_LogFileTabAdd() -> i32;
    pub fn dx_LogFileTabSub() -> i32;
    pub fn dx_ErrorLogAdd(String: *const i8) -> i32;
    pub fn dx_ErrorLogTabAdd() -> i32;
    pub fn dx_ErrorLogTabSub() -> i32;
    pub fn dx_SetUseTimeStampFlag(UseFlag: i32) -> i32;
    pub fn dx_SetOutApplicationLogValidFlag(Flag: i32) -> i32;
    pub fn dx_SetApplicationLogFileName(FileName: *const i8) -> i32;
    pub fn dx_SetApplicationLogFileNameWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
    ) -> i32;
    pub fn dx_SetApplicationLogSaveDirectory(DirectoryPath: *const i8) -> i32;
    pub fn dx_SetApplicationLogSaveDirectoryWithStrLen(
        DirectoryPath: *const i8,
        DirectoryPathLength: usize,
    ) -> i32;
    pub fn dx_SetUseDateNameLogFile(Flag: i32) -> i32;
    pub fn dx_SetLogDrawOutFlag(DrawFlag: i32) -> i32;
    pub fn dx_GetLogDrawFlag() -> i32;
    pub fn dx_SetLogFontSize(Size: i32) -> i32;
    pub fn dx_SetLogFontHandle(FontHandle: i32) -> i32;
    pub fn dx_SetLogDrawArea(x1: i32, y1: i32, x2: i32, y2: i32) -> i32;
    pub fn dx_putsDx(String: *const i8, NewLine: i32) -> i32;
    pub fn dx_putsDxWithStrLen(String: *const i8, StringLength: usize, NewLine: i32) -> i32;
    pub fn dx_clsDx() -> i32;
    pub fn dx_SetUseASyncLoadFlag(Flag: i32) -> i32;
    pub fn dx_GetUseASyncLoadFlag() -> i32;
    pub fn dx_CheckHandleASyncLoad(Handle: i32) -> i32;
    pub fn dx_GetHandleASyncLoadResult(Handle: i32) -> i32;
    pub fn dx_SetASyncLoadFinishDeleteFlag(Handle: i32) -> i32;
    pub fn dx_GetASyncLoadNum() -> i32;
    pub fn dx_SetASyncLoadThreadNum(ThreadNum: i32) -> i32;
    pub fn dx_SetDeleteHandleFlag(Handle: i32, DeleteFlag: *mut i32) -> i32;
    pub fn dx_SetMouseDispFlag(DispFlag: i32) -> i32;
    pub fn dx_GetMousePoint(XBuf: *mut i32, YBuf: *mut i32) -> i32;
    pub fn dx_SetMousePoint(PointX: i32, PointY: i32) -> i32;
    pub fn dx_GetMouseInput() -> i32;
    pub fn dx_GetMouseWheelRotVol(CounterReset: i32) -> i32;
    pub fn dx_GetMouseHWheelRotVol(CounterReset: i32) -> i32;
    pub fn dx_GetMouseWheelRotVolF(CounterReset: i32) -> f32;
    pub fn dx_GetMouseHWheelRotVolF(CounterReset: i32) -> f32;
    pub fn dx_GetMouseInputLog(
        Button: *mut i32,
        ClickX: *mut i32,
        ClickY: *mut i32,
        LogDelete: i32,
    ) -> i32;
    pub fn dx_GetMouseInputLog2(
        Button: *mut i32,
        ClickX: *mut i32,
        ClickY: *mut i32,
        LogType: *mut i32,
        LogDelete: i32,
    ) -> i32;
    pub fn dx_GetTouchInputNum() -> i32;
    pub fn dx_GetTouchInput(
        InputNo: i32,
        PositionX: *mut i32,
        PositionY: *mut i32,
        ID: *mut i32,
        Device: *mut i32,
    ) -> i32;
    pub fn dx_GetTouchInputLogNum() -> i32;
    pub fn dx_ClearTouchInputLog() -> i32;
    pub fn dx_GetTouchInputLogOne(PeekFlag: i32) -> TouchInputData;
    pub fn dx_GetTouchInputLog(TouchData: *mut TouchInputData, GetNum: i32, PeekFlag: i32) -> i32;
    pub fn dx_GetTouchInputDownLogNum() -> i32;
    pub fn dx_ClearTouchInputDownLog() -> i32;
    pub fn dx_GetTouchInputDownLogOne(PeekFlag: i32) -> TouchInputData;
    pub fn dx_GetTouchInputDownLog(
        PointData: *mut TouchInputData,
        GetNum: i32,
        PeekFlag: i32,
    ) -> i32;
    pub fn dx_GetTouchInputUpLogNum() -> i32;
    pub fn dx_ClearTouchInputUpLog() -> i32;
    pub fn dx_GetTouchInputUpLogOne(PeekFlag: i32) -> TouchInputPoint;
    pub fn dx_GetTouchInputUpLog(
        PointData: *mut TouchInputPoint,
        GetNum: i32,
        PeekFlag: i32,
    ) -> i32;
    pub fn dx_DxFree(Memory: *mut libc::c_void) -> ();
    pub fn dx_DxSetAllocSizeTrap(Size: usize) -> usize;
    pub fn dx_DxSetAllocPrintFlag(Flag: i32) -> i32;
    pub fn dx_DxGetAllocSize() -> usize;
    pub fn dx_DxGetAllocNum() -> i32;
    pub fn dx_DxDumpAlloc() -> ();
    pub fn dx_DxDrawAlloc(x: i32, y: i32, Width: i32, Height: i32) -> ();
    pub fn dx_DxErrorCheckAlloc() -> i32;
    pub fn dx_DxSetAllocSizeOutFlag(Flag: i32) -> i32;
    pub fn dx_DxSetAllocMemoryErrorCheckFlag(Flag: i32) -> i32;
    pub fn dx_GetCharBytes(CharCodeFormat: i32, String: *const libc::c_void) -> i32;
    pub fn dx_ConvertStringCharCodeFormat(
        SrcCharCodeFormat: i32,
        SrcString: *const libc::c_void,
        DestCharCodeFormat: i32,
        DestStringBuffer: *mut libc::c_void,
    ) -> i32;
    pub fn dx_SetUseCharCodeFormat(CharCodeFormat: i32) -> i32;
    pub fn dx_GetUseCharCodeFormat() -> i32;
    pub fn dx_Get_wchar_t_CharCodeFormat() -> i32;
    pub fn dx_strcpyDx(Dest: *mut u8, Src: *const i8) -> ();
    pub fn dx_strcpy_sDx(Dest: *mut u8, DestBytes: usize, Src: *const i8) -> ();
    pub fn dx_strpcpyDx(Dest: *mut u8, Src: *const i8, Pos: i32) -> ();
    pub fn dx_strpcpy_sDx(Dest: *mut u8, DestBytes: usize, Src: *const i8, Pos: i32) -> ();
    pub fn dx_strpcpy2Dx(Dest: *mut u8, Src: *const i8, Pos: i32) -> ();
    pub fn dx_strpcpy2_sDx(Dest: *mut u8, DestBytes: usize, Src: *const i8, Pos: i32) -> ();
    pub fn dx_strncpyDx(Dest: *mut u8, Src: *const i8, Num: i32) -> ();
    pub fn dx_strncpy_sDx(Dest: *mut u8, DestBytes: usize, Src: *const i8, Num: i32) -> ();
    pub fn dx_strncpy2Dx(Dest: *mut u8, Src: *const i8, Num: i32) -> ();
    pub fn dx_strncpy2_sDx(Dest: *mut u8, DestBytes: usize, Src: *const i8, Num: i32) -> ();
    pub fn dx_strrncpyDx(Dest: *mut u8, Src: *const i8, Num: i32) -> ();
    pub fn dx_strrncpy_sDx(Dest: *mut u8, DestBytes: usize, Src: *const i8, Num: i32) -> ();
    pub fn dx_strrncpy2Dx(Dest: *mut u8, Src: *const i8, Num: i32) -> ();
    pub fn dx_strrncpy2_sDx(Dest: *mut u8, DestBytes: usize, Src: *const i8, Num: i32) -> ();
    pub fn dx_strpncpyDx(Dest: *mut u8, Src: *const i8, Pos: i32, Num: i32) -> ();
    pub fn dx_strpncpy_sDx(
        Dest: *mut u8,
        DestBytes: usize,
        Src: *const i8,
        Pos: i32,
        Num: i32,
    ) -> ();
    pub fn dx_strpncpy2Dx(Dest: *mut u8, Src: *const i8, Pos: i32, Num: i32) -> ();
    pub fn dx_strpncpy2_sDx(
        Dest: *mut u8,
        DestBytes: usize,
        Src: *const i8,
        Pos: i32,
        Num: i32,
    ) -> ();
    pub fn dx_strcatDx(Dest: *mut u8, Src: *const i8) -> ();
    pub fn dx_strcat_sDx(Dest: *mut u8, DestBytes: usize, Src: *const i8) -> ();
    pub fn dx_strlenDx(Str: *const i8) -> usize;
    pub fn dx_strlen2Dx(Str: *const i8) -> usize;
    pub fn dx_strcmpDx(Str1: *const i8, Str2: *const i8) -> i32;
    pub fn dx_stricmpDx(Str1: *const i8, Str2: *const i8) -> i32;
    pub fn dx_strncmpDx(Str1: *const i8, Str2: *const i8, Num: i32) -> i32;
    pub fn dx_strncmp2Dx(Str1: *const i8, Str2: *const i8, Num: i32) -> i32;
    pub fn dx_strpncmpDx(Str1: *const i8, Str2: *const i8, Pos: i32, Num: i32) -> i32;
    pub fn dx_strpncmp2Dx(Str1: *const i8, Str2: *const i8, Pos: i32, Num: i32) -> i32;
    pub fn dx_strgetchrDx(Str: *const i8, Pos: i32, CharNums: *mut i32) -> u32;
    pub fn dx_strgetchr2Dx(Str: *const i8, Pos: i32, CharNums: *mut i32) -> u32;
    pub fn dx_strputchrDx(Str: *mut u8, Pos: i32, CharCode: u32) -> i32;
    pub fn dx_strputchr2Dx(Str: *mut u8, Pos: i32, CharCode: u32) -> i32;
    pub fn dx_strstr2Dx(Str1: *const i8, Str2: *const i8) -> i32;
    pub fn dx_strrstr2Dx(Str1: *const i8, Str2: *const i8) -> i32;
    pub fn dx_strchr2Dx(Str: *const i8, CharCode: u32) -> i32;
    pub fn dx_strrchr2Dx(Str: *const i8, CharCode: u32) -> i32;
    pub fn dx_vsprintfDx(Buffer: *mut u8, FormatString: *const i8, Arg: *mut u8) -> i32;
    pub fn dx_vsnprintfDx(
        Buffer: *mut u8,
        BufferSize: usize,
        FormatString: *const i8,
        Arg: *mut u8,
    ) -> i32;
    pub fn dx_atoiDx(Str: *const i8) -> i32;
    pub fn dx_atofDx(Str: *const i8) -> f64;
    pub fn dx_vsscanfDx(String: *const i8, FormatString: *const i8, Arg: *mut u8) -> i32;
    pub fn dx_ProcessNetMessage(RunReleaseProcess: i32) -> i32;
    pub fn dx_GetHostIPbyName(
        HostName: *const i8,
        IPDataBuf: *mut IpData,
        IPDataBufLength: i32,
        IPDataGetNum: *mut i32,
    ) -> i32;
    pub fn dx_GetHostIPbyNameWithStrLen(
        HostName: *const i8,
        HostNameLength: usize,
        IPDataBuf: *mut IpData,
        IPDataBufLength: i32,
        IPDataGetNum: *mut i32,
    ) -> i32;
    pub fn dx_GetHostIPbyName_IPv6(
        HostName: *const i8,
        IPDataBuf: *mut IpDataV6,
        IPDataBufLength: i32,
        IPDataGetNum: *mut i32,
    ) -> i32;
    pub fn dx_GetHostIPbyName_IPv6WithStrLen(
        HostName: *const i8,
        HostNameLength: usize,
        IPDataBuf: *mut IpDataV6,
        IPDataBufLength: i32,
        IPDataGetNum: *mut i32,
    ) -> i32;
    pub fn dx_ConnectNetWork(IPData: IpData, Port: i32) -> i32;
    pub fn dx_ConnectNetWork_IPv6(IPData: IpDataV6, Port: i32) -> i32;
    pub fn dx_ConnectNetWork_ASync(IPData: IpData, Port: i32) -> i32;
    pub fn dx_ConnectNetWork_IPv6_ASync(IPData: IpDataV6, Port: i32) -> i32;
    pub fn dx_PreparationListenNetWork(Port: i32) -> i32;
    pub fn dx_PreparationListenNetWork_IPv6(Port: i32) -> i32;
    pub fn dx_StopListenNetWork() -> i32;
    pub fn dx_CloseNetWork(NetHandle: i32) -> i32;
    pub fn dx_GetNetWorkAcceptState(NetHandle: i32) -> i32;
    pub fn dx_GetNetWorkDataLength(NetHandle: i32) -> i32;
    pub fn dx_GetNetWorkSendDataLength(NetHandle: i32) -> i32;
    pub fn dx_GetNewAcceptNetWork() -> i32;
    pub fn dx_GetLostNetWork() -> i32;
    pub fn dx_GetNetWorkIP(NetHandle: i32, IpBuf: *mut IpData) -> i32;
    pub fn dx_GetNetWorkIP_IPv6(NetHandle: i32, IpBuf: *mut IpDataV6) -> i32;
    pub fn dx_GetMyIPAddress(IpBuf: *mut IpData, IpBufLength: i32, IpNum: *mut i32) -> i32;
    pub fn dx_GetMyIPAddress_IPv6(IpBuf: *mut IpDataV6, IpBufLength: i32, IpNum: *mut i32) -> i32;
    pub fn dx_SetConnectTimeOutWait(Time: i32) -> i32;
    pub fn dx_SetUseDXNetWorkProtocol(Flag: i32) -> i32;
    pub fn dx_GetUseDXNetWorkProtocol() -> i32;
    pub fn dx_SetUseDXProtocol(Flag: i32) -> i32;
    pub fn dx_GetUseDXProtocol() -> i32;
    pub fn dx_SetNetWorkCloseAfterLostFlag(Flag: i32) -> i32;
    pub fn dx_GetNetWorkCloseAfterLostFlag() -> i32;
    pub fn dx_NetWorkRecv(NetHandle: i32, Buffer: *mut libc::c_void, Length: i32) -> i32;
    pub fn dx_NetWorkRecvToPeek(NetHandle: i32, Buffer: *mut libc::c_void, Length: i32) -> i32;
    pub fn dx_NetWorkRecvBufferClear(NetHandle: i32) -> i32;
    pub fn dx_NetWorkSend(NetHandle: i32, Buffer: *const libc::c_void, Length: i32) -> i32;
    pub fn dx_MakeUDPSocket(RecvPort: i32) -> i32;
    pub fn dx_MakeUDPSocket_IPv6(RecvPort: i32) -> i32;
    pub fn dx_DeleteUDPSocket(NetUDPHandle: i32) -> i32;
    pub fn dx_NetWorkSendUDP(
        NetUDPHandle: i32,
        SendIP: IpData,
        SendPort: i32,
        Buffer: *const libc::c_void,
        Length: i32,
    ) -> i32;
    pub fn dx_NetWorkSendUDP_IPv6(
        NetUDPHandle: i32,
        SendIP: IpDataV6,
        SendPort: i32,
        Buffer: *const libc::c_void,
        Length: i32,
    ) -> i32;
    pub fn dx_NetWorkRecvUDP(
        NetUDPHandle: i32,
        RecvIP: *mut IpData,
        RecvPort: *mut i32,
        Buffer: *mut libc::c_void,
        Length: i32,
        Peek: i32,
    ) -> i32;
    pub fn dx_NetWorkRecvUDP_IPv6(
        NetUDPHandle: i32,
        RecvIP: *mut IpDataV6,
        RecvPort: *mut i32,
        Buffer: *mut libc::c_void,
        Length: i32,
        Peek: i32,
    ) -> i32;
    pub fn dx_CheckNetWorkRecvUDP(NetUDPHandle: i32) -> i32;
    pub fn dx_StockInputChar(CharCode: u8) -> i32;
    pub fn dx_ClearInputCharBuf() -> i32;
    pub fn dx_GetInputChar(DeleteFlag: i32) -> u8;
    pub fn dx_GetInputCharWait(DeleteFlag: i32) -> u8;
    pub fn dx_GetOneChar(CharBuffer: *mut u8, DeleteFlag: i32) -> i32;
    pub fn dx_GetOneCharWait(CharBuffer: *mut u8, DeleteFlag: i32) -> i32;
    pub fn dx_GetCtrlCodeCmp(Char: u8) -> i32;
    pub fn dx_DrawIMEInputString(
        x: i32,
        y: i32,
        SelectStringNum: i32,
        DrawCandidateList: i32,
    ) -> i32;
    pub fn dx_SetUseIMEFlag(UseFlag: i32) -> i32;
    pub fn dx_GetUseIMEFlag() -> i32;
    pub fn dx_SetInputStringMaxLengthIMESync(Flag: i32) -> i32;
    pub fn dx_SetIMEInputStringMaxLength(Length: i32) -> i32;
    pub fn dx_GetStringPoint(String: *const i8, Point: i32) -> i32;
    pub fn dx_GetStringPointWithStrLen(String: *const i8, StringLength: usize, Point: i32) -> i32;
    pub fn dx_GetStringPoint2(String: *const i8, Point: i32) -> i32;
    pub fn dx_GetStringPoint2WithStrLen(String: *const i8, StringLength: usize, Point: i32) -> i32;
    pub fn dx_GetStringLength(String: *const i8) -> i32;
    pub fn dx_DrawObtainsString(
        x: i32,
        y: i32,
        AddY: i32,
        String: *const i8,
        StrColor: u32,
        StrEdgeColor: u32,
        FontHandle: i32,
        SelectBackColor: u32,
        SelectStrColor: u32,
        SelectStrEdgeColor: u32,
        SelectStart: i32,
        SelectEnd: i32,
    ) -> i32;
    pub fn dx_DrawObtainsNString(
        x: i32,
        y: i32,
        AddY: i32,
        String: *const i8,
        StringLength: usize,
        StrColor: u32,
        StrEdgeColor: u32,
        FontHandle: i32,
        SelectBackColor: u32,
        SelectStrColor: u32,
        SelectStrEdgeColor: u32,
        SelectStart: i32,
        SelectEnd: i32,
    ) -> i32;
    pub fn dx_DrawObtainsString_CharClip(
        x: i32,
        y: i32,
        AddY: i32,
        String: *const i8,
        StrColor: u32,
        StrEdgeColor: u32,
        FontHandle: i32,
        SelectBackColor: u32,
        SelectStrColor: u32,
        SelectStrEdgeColor: u32,
        SelectStart: i32,
        SelectEnd: i32,
    ) -> i32;
    pub fn dx_DrawObtainsNString_CharClip(
        x: i32,
        y: i32,
        AddY: i32,
        String: *const i8,
        StringLength: usize,
        StrColor: u32,
        StrEdgeColor: u32,
        FontHandle: i32,
        SelectBackColor: u32,
        SelectStrColor: u32,
        SelectStrEdgeColor: u32,
        SelectStart: i32,
        SelectEnd: i32,
    ) -> i32;
    pub fn dx_GetObtainsStringCharPosition(
        x: i32,
        y: i32,
        AddY: i32,
        String: *const i8,
        StrLen: i32,
        PosX: *mut i32,
        PosY: *mut i32,
        FontHandle: i32,
    ) -> i32;
    pub fn dx_GetObtainsStringCharPosition_CharClip(
        x: i32,
        y: i32,
        AddY: i32,
        String: *const i8,
        StrLen: i32,
        PosX: *mut i32,
        PosY: *mut i32,
        FontHandle: i32,
    ) -> i32;
    pub fn dx_DrawObtainsBox(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        AddY: i32,
        Color: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_InputStringToCustom(
        x: i32,
        y: i32,
        BufLength: usize,
        StrBuffer: *mut u8,
        CancelValidFlag: i32,
        SingleCharOnlyFlag: i32,
        NumCharOnlyFlag: i32,
        DoubleCharOnlyFlag: i32,
        EnableNewLineFlag: i32,
        DisplayCandidateList: i32,
    ) -> i32;
    pub fn dx_KeyInputString(
        x: i32,
        y: i32,
        CharMaxLength: usize,
        StrBuffer: *mut u8,
        CancelValidFlag: i32,
    ) -> i32;
    pub fn dx_KeyInputSingleCharString(
        x: i32,
        y: i32,
        CharMaxLength: usize,
        StrBuffer: *mut u8,
        CancelValidFlag: i32,
    ) -> i32;
    pub fn dx_KeyInputNumber(x: i32, y: i32, MaxNum: i32, MinNum: i32, CancelValidFlag: i32)
        -> i32;
    pub fn dx_GetIMEInputModeStr(GetBuffer: *mut u8) -> i32;
    pub fn dx_SetKeyInputStringColor2(TargetColor: i32, Color: u32) -> i32;
    pub fn dx_ResetKeyInputStringColor2(TargetColor: i32) -> i32;
    pub fn dx_SetKeyInputStringFont(FontHandle: i32) -> i32;
    pub fn dx_SetKeyInputStringEndCharaMode(EndCharaMode: i32) -> i32;
    pub fn dx_DrawKeyInputModeString(x: i32, y: i32) -> i32;
    pub fn dx_InitKeyInput() -> i32;
    pub fn dx_MakeKeyInput(
        MaxStrLength: usize,
        CancelValidFlag: i32,
        SingleCharOnlyFlag: i32,
        NumCharOnlyFlag: i32,
        DoubleCharOnlyFlag: i32,
        EnableNewLineFlag: i32,
    ) -> i32;
    pub fn dx_DeleteKeyInput(InputHandle: i32) -> i32;
    pub fn dx_SetActiveKeyInput(InputHandle: i32) -> i32;
    pub fn dx_GetActiveKeyInput() -> i32;
    pub fn dx_CheckKeyInput(InputHandle: i32) -> i32;
    pub fn dx_ReStartKeyInput(InputHandle: i32) -> i32;
    pub fn dx_ProcessActKeyInput() -> i32;
    pub fn dx_DrawKeyInputString(x: i32, y: i32, InputHandle: i32, DrawCandidateList: i32) -> i32;
    pub fn dx_SetKeyInputDrawArea(x1: i32, y1: i32, x2: i32, y2: i32, InputHandle: i32) -> i32;
    pub fn dx_SetKeyInputSelectArea(SelectStart: i32, SelectEnd: i32, InputHandle: i32) -> i32;
    pub fn dx_GetKeyInputSelectArea(
        SelectStart: *mut i32,
        SelectEnd: *mut i32,
        InputHandle: i32,
    ) -> i32;
    pub fn dx_SetKeyInputDrawStartPos(DrawStartPos: i32, InputHandle: i32) -> i32;
    pub fn dx_GetKeyInputDrawStartPos(InputHandle: i32) -> i32;
    pub fn dx_SetKeyInputCursorBrinkTime(Time: i32) -> i32;
    pub fn dx_SetKeyInputCursorBrinkFlag(Flag: i32) -> i32;
    pub fn dx_SetKeyInputString(String: *const i8, InputHandle: i32) -> i32;
    pub fn dx_SetKeyInputStringWithStrLen(
        String: *const i8,
        StringLength: usize,
        InputHandle: i32,
    ) -> i32;
    pub fn dx_SetKeyInputNumber(Number: i32, InputHandle: i32) -> i32;
    pub fn dx_SetKeyInputNumberToFloat(Number: f32, InputHandle: i32) -> i32;
    pub fn dx_GetKeyInputString(StrBuffer: *mut u8, InputHandle: i32) -> i32;
    pub fn dx_GetKeyInputNumber(InputHandle: i32) -> i32;
    pub fn dx_GetKeyInputNumberToFloat(InputHandle: i32) -> f32;
    pub fn dx_SetKeyInputCursorPosition(Position: i32, InputHandle: i32) -> i32;
    pub fn dx_GetKeyInputCursorPosition(InputHandle: i32) -> i32;
    pub fn dx_FileRead_open(FilePath: *const i8, ASync: i32) -> i32;
    pub fn dx_FileRead_open_WithStrLen(
        FilePath: *const i8,
        FilePathLength: usize,
        ASync: i32,
    ) -> i32;
    pub fn dx_FileRead_open_mem(FileImage: *const libc::c_void, FileImageSize: usize) -> i32;
    pub fn dx_FileRead_size(FilePath: *const i8) -> i64;
    pub fn dx_FileRead_size_WithStrLen(FilePath: *const i8, FilePathLength: usize) -> i64;
    pub fn dx_FileRead_close(FileHandle: i32) -> i32;
    pub fn dx_FileRead_tell(FileHandle: i32) -> i64;
    pub fn dx_FileRead_seek(FileHandle: i32, Offset: i64, Origin: i32) -> i32;
    pub fn dx_FileRead_read(Buffer: *mut libc::c_void, ReadSize: i32, FileHandle: i32) -> i32;
    pub fn dx_FileRead_idle_chk(FileHandle: i32) -> i32;
    pub fn dx_FileRead_eof(FileHandle: i32) -> i32;
    pub fn dx_FileRead_set_format(FileHandle: i32, CharCodeFormat: i32) -> i32;
    pub fn dx_FileRead_gets(Buffer: *mut u8, BufferSize: i32, FileHandle: i32) -> i32;
    pub fn dx_FileRead_getc(FileHandle: i32) -> u8;
    pub fn dx_FileRead_createInfo(ObjectPath: *const i8) -> u32;
    pub fn dx_FileRead_createInfo_WithStrLen(ObjectPath: *const i8, ObjectPathLength: usize)
        -> u32;
    pub fn dx_FileRead_getInfoNum(FileInfoHandle: u32) -> i32;
    pub fn dx_FileRead_getInfo(Index: i32, Buffer: *mut FileInfo, FileInfoHandle: u32) -> i32;
    pub fn dx_FileRead_deleteInfo(FileInfoHandle: u32) -> i32;
    pub fn dx_FileRead_findFirst(FilePath: *const i8, Buffer: *mut FileInfo) -> u32;
    pub fn dx_FileRead_findFirst_WithStrLen(
        FilePath: *const i8,
        FilePathLength: usize,
        Buffer: *mut FileInfo,
    ) -> u32;
    pub fn dx_FileRead_findNext(FindHandle: u32, Buffer: *mut FileInfo) -> i32;
    pub fn dx_FileRead_findClose(FindHandle: u32) -> i32;
    pub fn dx_FileRead_fullyLoad(FilePath: *const i8) -> i32;
    pub fn dx_FileRead_fullyLoad_WithStrLen(FilePath: *const i8, FilePathLength: usize) -> i32;
    pub fn dx_FileRead_fullyLoad_delete(FLoadHandle: i32) -> i32;
    pub fn dx_FileRead_fullyLoad_getSize(FLoadHandle: i32) -> i64;
    pub fn dx_GetStreamFunctionDefault() -> i32;
    pub fn dx_ChangeStreamFunction(StreamThread: *const StreamDataShredType2) -> i32;
    pub fn dx_ChangeStreamFunctionW(StreamThreadW: *const StreamDataShredType2W) -> i32;
    pub fn dx_ConvertFullPath(Src: *const i8, Dest: *mut u8, CurrentDir: *const i8) -> i32;
    pub fn dx_ConvertFullPathWithStrLen(
        Src: *const i8,
        SrcLength: usize,
        Dest: *mut u8,
        CurrentDir: *const i8,
        CurrentDirLength: usize,
    ) -> i32;
    pub fn dx_CheckHitKey(KeyCode: i32) -> i32;
    pub fn dx_CheckHitKeyAll(CheckType: i32) -> i32;
    pub fn dx_GetHitKeyStateAll(KeyStateArray: *mut u8) -> i32;
    pub fn dx_GetJoypadNum() -> i32;
    pub fn dx_GetJoypadButtonNum(InputType: i32) -> i32;
    pub fn dx_GetJoypadInputState(InputType: i32) -> i32;
    pub fn dx_GetJoypadAnalogInput(XBuf: *mut i32, YBuf: *mut i32, InputType: i32) -> i32;
    pub fn dx_GetJoypadAnalogInputRight(XBuf: *mut i32, YBuf: *mut i32, InputType: i32) -> i32;
    pub fn dx_GetJoypadDirectInputState(InputType: i32, DInputState: *mut DInputJoyState) -> i32;
    pub fn dx_CheckJoypadXInput(InputType: i32) -> i32;
    pub fn dx_GetJoypadXInputState(InputType: i32, XInputState: *mut XInputState) -> i32;
    pub fn dx_SetJoypadInputToKeyInput(
        InputType: i32,
        PadInput: i32,
        KeyInput1: i32,
        KeyInput2: i32,
        KeyInput3: i32,
        KeyInput4: i32,
    ) -> i32;
    pub fn dx_SetJoypadDeadZone(InputType: i32, Zone: f64) -> i32;
    pub fn dx_GetJoypadDeadZone(InputType: i32) -> f64;
    pub fn dx_StartJoypadVibration(InputType: i32, Power: i32, Time: i32, EffectIndex: i32) -> i32;
    pub fn dx_StopJoypadVibration(InputType: i32, EffectIndex: i32) -> i32;
    pub fn dx_GetJoypadPOVState(InputType: i32, POVNumber: i32) -> i32;
    pub fn dx_ReSetupJoypad() -> i32;
    pub fn dx_SetUseJoypadVibrationFlag(Flag: i32) -> i32;
    pub fn dx_MakeGraph(SizeX: i32, SizeY: i32, NotUse3DFlag: i32) -> i32;
    pub fn dx_MakeScreen(SizeX: i32, SizeY: i32, UseAlphaChannel: i32) -> i32;
    pub fn dx_DerivationGraph(
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        SrcGraphHandle: i32,
    ) -> i32;
    pub fn dx_DerivationGraphF(
        SrcX: f32,
        SrcY: f32,
        Width: f32,
        Height: f32,
        SrcGraphHandle: i32,
    ) -> i32;
    pub fn dx_DeleteGraph(GrHandle: i32, LogOutFlag: i32) -> i32;
    pub fn dx_DeleteSharingGraph(GrHandle: i32) -> i32;
    pub fn dx_GetGraphNum() -> i32;
    pub fn dx_FillGraph(GrHandle: i32, Red: i32, Green: i32, Blue: i32, Alpha: i32) -> i32;
    pub fn dx_FillRectGraph(
        GrHandle: i32,
        x: i32,
        y: i32,
        Width: i32,
        Height: i32,
        Red: i32,
        Green: i32,
        Blue: i32,
        Alpha: i32,
    ) -> i32;
    pub fn dx_SetGraphLostFlag(GrHandle: i32, LostFlag: *mut i32) -> i32;
    pub fn dx_InitGraph(LogOutFlag: i32) -> i32;
    pub fn dx_ReloadFileGraphAll() -> i32;
    pub fn dx_MakeShadowMap(SizeX: i32, SizeY: i32) -> i32;
    pub fn dx_DeleteShadowMap(SmHandle: i32) -> i32;
    pub fn dx_SetShadowMapLightDirection(SmHandle: i32, Direction: Vector) -> i32;
    pub fn dx_ShadowMap_DrawSetup(SmHandle: i32) -> i32;
    pub fn dx_ShadowMap_DrawEnd() -> i32;
    pub fn dx_SetUseShadowMap(SmSlotIndex: i32, SmHandle: i32) -> i32;
    pub fn dx_SetShadowMapDrawArea(SmHandle: i32, MinPosition: Vector, MaxPosition: Vector) -> i32;
    pub fn dx_ResetShadowMapDrawArea(SmHandle: i32) -> i32;
    pub fn dx_SetShadowMapAdjustDepth(SmHandle: i32, Depth: f32) -> i32;
    pub fn dx_GetShadowMapViewProjectionMatrix(SmHandle: i32, MatrixBuffer: *mut Matrix) -> i32;
    pub fn dx_TestDrawShadowMap(SmHandle: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> i32;
    pub fn dx_BltBmpToGraph(
        BmpColorData: *const ColorData,
        RgbBmp: *const libc::c_void,
        AlphaBmp: *const libc::c_void,
        CopyPointX: i32,
        CopyPointY: i32,
        GrHandle: i32,
    ) -> i32;
    pub fn dx_BltBmpToDivGraph(
        BmpColorData: *const ColorData,
        RgbBmp: *const libc::c_void,
        AlphaBmp: *const libc::c_void,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        Width: i32,
        Height: i32,
        GrHandle: *const i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_BltBmpOrGraphImageToGraph(
        BmpColorData: *const ColorData,
        RgbBmp: *const libc::c_void,
        AlphaBmp: *const libc::c_void,
        BmpFlag: i32,
        RgbBaseImage: *const BaseImage,
        AlphaBaseImage: *const BaseImage,
        CopyPointX: i32,
        CopyPointY: i32,
        GrHandle: i32,
    ) -> i32;
    pub fn dx_BltBmpOrGraphImageToGraph2(
        BmpColorData: *const ColorData,
        RgbBmp: *const libc::c_void,
        AlphaBmp: *const libc::c_void,
        BmpFlag: i32,
        RgbBaseImage: *const BaseImage,
        AlphaBaseImage: *const BaseImage,
        SrcRect: *const Rect,
        DestX: i32,
        DestY: i32,
        GrHandle: i32,
    ) -> i32;
    pub fn dx_BltBmpOrGraphImageToDivGraph(
        BmpColorData: *const ColorData,
        RgbBmp: *const libc::c_void,
        AlphaBmp: *const libc::c_void,
        BmpFlag: i32,
        RgbBaseImage: *const BaseImage,
        AlphaBaseImage: *const BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        Width: i32,
        Height: i32,
        GrHandle: *const i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_BltBmpOrGraphImageToDivGraphF(
        BmpColorData: *const ColorData,
        RgbBmp: *const libc::c_void,
        AlphaBmp: *const libc::c_void,
        BmpFlag: i32,
        RgbBaseImage: *const BaseImage,
        AlphaBaseImage: *const BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        Width: f32,
        Height: f32,
        GrHandle: *const i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_LoadBmpToGraph(
        FileName: *const i8,
        TextureFlag: i32,
        ReverseFlag: i32,
        SurfaceMode: i32,
    ) -> i32;
    pub fn dx_LoadBmpToGraphWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        TextureFlag: i32,
        ReverseFlag: i32,
        SurfaceMode: i32,
    ) -> i32;
    pub fn dx_LoadGraph(FileName: *const i8, NotUse3DFlag: i32) -> i32;
    pub fn dx_LoadGraphWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        NotUse3DFlag: i32,
    ) -> i32;
    pub fn dx_LoadReverseGraph(FileName: *const i8, NotUse3DFlag: i32) -> i32;
    pub fn dx_LoadReverseGraphWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        NotUse3DFlag: i32,
    ) -> i32;
    pub fn dx_LoadDivGraph(
        FileName: *const i8,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        HandleArray: *mut i32,
        NotUse3DFlag: i32,
    ) -> i32;
    pub fn dx_LoadDivGraphWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        HandleArray: *mut i32,
        NotUse3DFlag: i32,
    ) -> i32;
    pub fn dx_LoadDivGraphF(
        FileName: *const i8,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: f32,
        YSize: f32,
        HandleArray: *mut i32,
        NotUse3DFlag: i32,
    ) -> i32;
    pub fn dx_LoadDivGraphFWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: f32,
        YSize: f32,
        HandleArray: *mut i32,
        NotUse3DFlag: i32,
    ) -> i32;
    pub fn dx_LoadDivBmpToGraph(
        FileName: *const i8,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *mut i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_LoadDivBmpToGraphWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *mut i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_LoadDivBmpToGraphF(
        FileName: *const i8,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *mut i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_LoadDivBmpToGraphFWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *mut i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_LoadReverseDivGraph(
        FileName: *const i8,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        HandleArray: *mut i32,
        NotUse3DFlag: i32,
    ) -> i32;
    pub fn dx_LoadReverseDivGraphWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        HandleArray: *mut i32,
        NotUse3DFlag: i32,
    ) -> i32;
    pub fn dx_LoadReverseDivGraphF(
        FileName: *const i8,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: f32,
        YSize: f32,
        HandleArray: *mut i32,
        NotUse3DFlag: i32,
    ) -> i32;
    pub fn dx_LoadReverseDivGraphFWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: f32,
        YSize: f32,
        HandleArray: *mut i32,
        NotUse3DFlag: i32,
    ) -> i32;
    pub fn dx_LoadBlendGraph(FileName: *const i8) -> i32;
    pub fn dx_LoadBlendGraphWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_CreateGraphFromMem(
        RGBFileImage: *const libc::c_void,
        RGBFileImageSize: i32,
        AlphaFileImage: *const libc::c_void,
        AlphaFileImageSize: i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReCreateGraphFromMem(
        RGBFileImage: *const libc::c_void,
        RGBFileImageSize: i32,
        GrHandle: i32,
        AlphaFileImage: *const libc::c_void,
        AlphaFileImageSize: i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateDivGraphFromMem(
        RGBFileImage: *const libc::c_void,
        RGBFileImageSize: i32,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *mut i32,
        TextureFlag: i32,
        ReverseFlag: i32,
        AlphaFileImage: *const libc::c_void,
        AlphaFileImageSize: i32,
    ) -> i32;
    pub fn dx_CreateDivGraphFFromMem(
        RGBFileImage: *const libc::c_void,
        RGBFileImageSize: i32,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *mut i32,
        TextureFlag: i32,
        ReverseFlag: i32,
        AlphaFileImage: *const libc::c_void,
        AlphaFileImageSize: i32,
    ) -> i32;
    pub fn dx_ReCreateDivGraphFromMem(
        RGBFileImage: *const libc::c_void,
        RGBFileImageSize: i32,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *const i32,
        TextureFlag: i32,
        ReverseFlag: i32,
        AlphaFileImage: *const libc::c_void,
        AlphaFileImageSize: i32,
    ) -> i32;
    pub fn dx_ReCreateDivGraphFFromMem(
        RGBFileImage: *const libc::c_void,
        RGBFileImageSize: i32,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *const i32,
        TextureFlag: i32,
        ReverseFlag: i32,
        AlphaFileImage: *const libc::c_void,
        AlphaFileImageSize: i32,
    ) -> i32;
    pub fn dx_CreateGraphFromBmp(
        RGBBmpInfo: *const BitMapInfo,
        RGBBmpImage: *const libc::c_void,
        AlphaBmpInfo: *const BitMapInfo,
        AlphaBmpImage: *const libc::c_void,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReCreateGraphFromBmp(
        RGBBmpInfo: *const BitMapInfo,
        RGBBmpImage: *const libc::c_void,
        GrHandle: i32,
        AlphaBmpInfo: *const BitMapInfo,
        AlphaBmpImage: *const libc::c_void,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateDivGraphFromBmp(
        RGBBmpInfo: *const BitMapInfo,
        RGBBmpImage: *const libc::c_void,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *mut i32,
        TextureFlag: i32,
        ReverseFlag: i32,
        AlphaBmpInfo: *const BitMapInfo,
        AlphaBmpImage: *const libc::c_void,
    ) -> i32;
    pub fn dx_CreateDivGraphFFromBmp(
        RGBBmpInfo: *const BitMapInfo,
        RGBBmpImage: *const libc::c_void,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *mut i32,
        TextureFlag: i32,
        ReverseFlag: i32,
        AlphaBmpInfo: *const BitMapInfo,
        AlphaBmpImage: *const libc::c_void,
    ) -> i32;
    pub fn dx_ReCreateDivGraphFromBmp(
        RGBBmpInfo: *const BitMapInfo,
        RGBBmpImage: *const libc::c_void,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *const i32,
        TextureFlag: i32,
        ReverseFlag: i32,
        AlphaBmpInfo: *const BitMapInfo,
        AlphaBmpImage: *const libc::c_void,
    ) -> i32;
    pub fn dx_ReCreateDivGraphFFromBmp(
        RGBBmpInfo: *const BitMapInfo,
        RGBBmpImage: *const libc::c_void,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *const i32,
        TextureFlag: i32,
        ReverseFlag: i32,
        AlphaBmpInfo: *const BitMapInfo,
        AlphaBmpImage: *const libc::c_void,
    ) -> i32;
    pub fn dx_CreateDXGraph(
        RgbBaseImage: *const BaseImage,
        AlphaBaseImage: *const BaseImage,
        TextureFlag: i32,
    ) -> i32;
    pub fn dx_CreateGraphFromGraphImage(
        RgbBaseImage: *const BaseImage,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateGraphFromGraphImage_2(
        RgbBaseImage: *const BaseImage,
        AlphaBaseImage: *const BaseImage,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReCreateGraphFromGraphImage(
        RgbBaseImage: *const BaseImage,
        GrHandle: i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReCreateGraphFromGraphImage_2(
        RgbBaseImage: *const BaseImage,
        AlphaBaseImage: *const BaseImage,
        GrHandle: i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateDivGraphFromGraphImage(
        RgbBaseImage: *mut BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *mut i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateDivGraphFFromGraphImage(
        RgbBaseImage: *mut BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *mut i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateDivGraphFromGraphImage_2(
        RgbBaseImage: *mut BaseImage,
        AlphaBaseImage: *const BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *mut i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateDivGraphFFromGraphImage_2(
        RgbBaseImage: *mut BaseImage,
        AlphaBaseImage: *const BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *mut i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReCreateDivGraphFromGraphImage(
        RgbBaseImage: *mut BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *const i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReCreateDivGraphFFromGraphImage(
        RgbBaseImage: *mut BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *const i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReCreateDivGraphFromGraphImage_2(
        RgbBaseImage: *mut BaseImage,
        AlphaBaseImage: *const BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *const i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReCreateDivGraphFFromGraphImage_2(
        RgbBaseImage: *mut BaseImage,
        AlphaBaseImage: *const BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *const i32,
        TextureFlag: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateGraph(
        Width: i32,
        Height: i32,
        Pitch: i32,
        RGBImage: *const libc::c_void,
        AlphaImage: *const libc::c_void,
        GrHandle: i32,
    ) -> i32;
    pub fn dx_CreateDivGraph(
        Width: i32,
        Height: i32,
        Pitch: i32,
        RGBImage: *const libc::c_void,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *mut i32,
        AlphaImage: *const libc::c_void,
    ) -> i32;
    pub fn dx_CreateDivGraphF(
        Width: i32,
        Height: i32,
        Pitch: i32,
        RGBImage: *const libc::c_void,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *mut i32,
        AlphaImage: *const libc::c_void,
    ) -> i32;
    pub fn dx_ReCreateGraph(
        Width: i32,
        Height: i32,
        Pitch: i32,
        RGBImage: *const libc::c_void,
        GrHandle: i32,
        AlphaImage: *const libc::c_void,
    ) -> i32;
    pub fn dx_CreateBlendGraphFromSoftImage(SIHandle: i32) -> i32;
    pub fn dx_CreateGraphFromSoftImage(SIHandle: i32) -> i32;
    pub fn dx_CreateGraphFromRectSoftImage(
        SIHandle: i32,
        x: i32,
        y: i32,
        SizeX: i32,
        SizeY: i32,
    ) -> i32;
    pub fn dx_ReCreateGraphFromSoftImage(SIHandle: i32, GrHandle: i32) -> i32;
    pub fn dx_ReCreateGraphFromRectSoftImage(
        SIHandle: i32,
        x: i32,
        y: i32,
        SizeX: i32,
        SizeY: i32,
        GrHandle: i32,
    ) -> i32;
    pub fn dx_CreateDivGraphFromSoftImage(
        SIHandle: i32,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *mut i32,
    ) -> i32;
    pub fn dx_CreateDivGraphFFromSoftImage(
        SIHandle: i32,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *mut i32,
    ) -> i32;
    pub fn dx_ReCreateDivGraphFromSoftImage(
        SIHandle: i32,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *const i32,
    ) -> i32;
    pub fn dx_ReCreateDivGraphFFromSoftImage(
        SIHandle: i32,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *const i32,
    ) -> i32;
    pub fn dx_CreateGraphFromBaseImage(BaseImage: *const BaseImage) -> i32;
    pub fn dx_CreateGraphFromRectBaseImage(
        BaseImage: *const BaseImage,
        x: i32,
        y: i32,
        SizeX: i32,
        SizeY: i32,
    ) -> i32;
    pub fn dx_ReCreateGraphFromBaseImage(BaseImage: *const BaseImage, GrHandle: i32) -> i32;
    pub fn dx_ReCreateGraphFromRectBaseImage(
        BaseImage: *const BaseImage,
        x: i32,
        y: i32,
        SizeX: i32,
        SizeY: i32,
        GrHandle: i32,
    ) -> i32;
    pub fn dx_CreateDivGraphFromBaseImage(
        BaseImage: *mut BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *mut i32,
    ) -> i32;
    pub fn dx_CreateDivGraphFFromBaseImage(
        BaseImage: *mut BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *mut i32,
    ) -> i32;
    pub fn dx_ReCreateDivGraphFromBaseImage(
        BaseImage: *mut BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: i32,
        SizeY: i32,
        HandleArray: *const i32,
    ) -> i32;
    pub fn dx_ReCreateDivGraphFFromBaseImage(
        BaseImage: *mut BaseImage,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        SizeX: f32,
        SizeY: f32,
        HandleArray: *const i32,
    ) -> i32;
    pub fn dx_ReloadGraph(FileName: *const i8, GrHandle: i32, ReverseFlag: i32) -> i32;
    pub fn dx_ReloadGraphWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        GrHandle: i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReloadDivGraph(
        FileName: *const i8,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        HandleArray: *const i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReloadDivGraphWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        HandleArray: *const i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReloadDivGraphF(
        FileName: *const i8,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: f32,
        YSize: f32,
        HandleArray: *const i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReloadDivGraphFWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: f32,
        YSize: f32,
        HandleArray: *const i32,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReloadReverseGraph(FileName: *const i8, GrHandle: i32) -> i32;
    pub fn dx_ReloadReverseGraphWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        GrHandle: i32,
    ) -> i32;
    pub fn dx_ReloadReverseDivGraph(
        FileName: *const i8,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        HandleArray: *const i32,
    ) -> i32;
    pub fn dx_ReloadReverseDivGraphWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        HandleArray: *const i32,
    ) -> i32;
    pub fn dx_ReloadReverseDivGraphF(
        FileName: *const i8,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: f32,
        YSize: f32,
        HandleArray: *const i32,
    ) -> i32;
    pub fn dx_ReloadReverseDivGraphFWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: f32,
        YSize: f32,
        HandleArray: *const i32,
    ) -> i32;
    pub fn dx_SetGraphColorBitDepth(ColorBitDepth: i32) -> i32;
    pub fn dx_GetGraphColorBitDepth() -> i32;
    pub fn dx_SetCreateGraphColorBitDepth(BitDepth: i32) -> i32;
    pub fn dx_GetCreateGraphColorBitDepth() -> i32;
    pub fn dx_SetCreateGraphChannelBitDepth(BitDepth: i32) -> i32;
    pub fn dx_GetCreateGraphChannelBitDepth() -> i32;
    pub fn dx_SetDrawValidGraphCreateFlag(Flag: i32) -> i32;
    pub fn dx_GetDrawValidGraphCreateFlag() -> i32;
    pub fn dx_SetDrawValidFlagOf3DGraph(Flag: i32) -> i32;
    pub fn dx_SetLeftUpColorIsTransColorFlag(Flag: i32) -> i32;
    pub fn dx_SetUsePaletteGraphFlag(Flag: i32) -> i32;
    pub fn dx_SetUseBlendGraphCreateFlag(Flag: i32) -> i32;
    pub fn dx_GetUseBlendGraphCreateFlag() -> i32;
    pub fn dx_SetUseAlphaTestGraphCreateFlag(Flag: i32) -> i32;
    pub fn dx_GetUseAlphaTestGraphCreateFlag() -> i32;
    pub fn dx_SetUseAlphaTestFlag(Flag: i32) -> i32;
    pub fn dx_GetUseAlphaTestFlag() -> i32;
    pub fn dx_SetCubeMapTextureCreateFlag(Flag: i32) -> i32;
    pub fn dx_GetCubeMapTextureCreateFlag() -> i32;
    pub fn dx_SetUseNoBlendModeParam(Flag: i32) -> i32;
    pub fn dx_SetDrawValidAlphaChannelGraphCreateFlag(Flag: i32) -> i32;
    pub fn dx_GetDrawValidAlphaChannelGraphCreateFlag() -> i32;
    pub fn dx_SetDrawValidFloatTypeGraphCreateFlag(Flag: i32) -> i32;
    pub fn dx_GetDrawValidFloatTypeGraphCreateFlag() -> i32;
    pub fn dx_SetDrawValidGraphCreateZBufferFlag(Flag: i32) -> i32;
    pub fn dx_GetDrawValidGraphCreateZBufferFlag() -> i32;
    pub fn dx_SetCreateDrawValidGraphZBufferBitDepth(BitDepth: i32) -> i32;
    pub fn dx_GetCreateDrawValidGraphZBufferBitDepth() -> i32;
    pub fn dx_SetCreateDrawValidGraphMipLevels(MipLevels: i32) -> i32;
    pub fn dx_GetCreateDrawValidGraphMipLevels() -> i32;
    pub fn dx_SetCreateDrawValidGraphChannelNum(ChannelNum: i32) -> i32;
    pub fn dx_GetCreateDrawValidGraphChannelNum() -> i32;
    pub fn dx_SetCreateDrawValidGraphMultiSample(Samples: i32, Quality: i32) -> i32;
    pub fn dx_SetDrawValidMultiSample(Samples: i32, Quality: i32) -> i32;
    pub fn dx_GetMultiSampleQuality(Samples: i32) -> i32;
    pub fn dx_SetUseTransColor(Flag: i32) -> i32;
    pub fn dx_SetUseTransColorGraphCreateFlag(Flag: i32) -> i32;
    pub fn dx_SetUseGraphAlphaChannel(Flag: i32) -> i32;
    pub fn dx_GetUseGraphAlphaChannel() -> i32;
    pub fn dx_SetUseAlphaChannelGraphCreateFlag(Flag: i32) -> i32;
    pub fn dx_GetUseAlphaChannelGraphCreateFlag() -> i32;
    pub fn dx_SetUseNotManageTextureFlag(Flag: i32) -> i32;
    pub fn dx_GetUseNotManageTextureFlag() -> i32;
    pub fn dx_SetUsePlatformTextureFormat(PlatformTextureFormat: i32) -> i32;
    pub fn dx_GetUsePlatformTextureFormat() -> i32;
    pub fn dx_SetTransColor(Red: i32, Green: i32, Blue: i32) -> i32;
    pub fn dx_GetTransColor(Red: *mut i32, Green: *mut i32, Blue: *mut i32) -> i32;
    pub fn dx_SetUseDivGraphFlag(Flag: i32) -> i32;
    pub fn dx_SetUseAlphaImageLoadFlag(Flag: i32) -> i32;
    pub fn dx_SetUseMaxTextureSize(Size: i32) -> i32;
    pub fn dx_SetUseGraphBaseDataBackup(Flag: i32) -> i32;
    pub fn dx_GetUseGraphBaseDataBackup() -> i32;
    pub fn dx_SetUseSystemMemGraphCreateFlag(Flag: i32) -> i32;
    pub fn dx_GetUseSystemMemGraphCreateFlag() -> i32;
    pub fn dx_GraphLock(
        GrHandle: i32,
        PitchBuf: *mut i32,
        DataPoi32Buf: *mut *mut libc::c_void,
        ColorDataPP: *mut *mut ColorData,
        WriteOnly: i32,
    ) -> i32;
    pub fn dx_GraphUnLock(GrHandle: i32) -> i32;
    pub fn dx_SetUseGraphZBuffer(GrHandle: i32, UseFlag: i32, BitDepth: i32) -> i32;
    pub fn dx_CopyGraphZBufferImage(DestGrHandle: i32, SrcGrHandle: i32) -> i32;
    pub fn dx_SetDeviceLostDeleteGraphFlag(GrHandle: i32, DeleteFlag: i32) -> i32;
    pub fn dx_GetGraphSize(GrHandle: i32, SizeXBuf: *mut i32, SizeYBuf: *mut i32) -> i32;
    pub fn dx_GetGraphSizeF(GrHandle: i32, SizeXBuf: *mut f32, SizeYBuf: *mut f32) -> i32;
    pub fn dx_GetGraphTextureSize(GrHandle: i32, SizeXBuf: *mut i32, SizeYBuf: *mut i32) -> i32;
    pub fn dx_GetGraphUseBaseGraphArea(
        GrHandle: i32,
        UseX: *mut i32,
        UseY: *mut i32,
        UseSizeX: *mut i32,
        UseSizeY: *mut i32,
    ) -> i32;
    pub fn dx_GetGraphMipmapCount(GrHandle: i32) -> i32;
    pub fn dx_GetGraphFilePath(GrHandle: i32, FilePathBuffer: *mut u8) -> i32;
    pub fn dx_CheckDrawValidGraph(GrHandle: i32) -> i32;
    pub fn dx_GetMaxGraphTextureSize(SizeX: *mut i32, SizeY: *mut i32) -> i32;
    pub fn dx_GetValidRestoreShredPoi32() -> i32;
    pub fn dx_GetCreateGraphColorData(
        ColorData: *mut ColorData,
        Format: *mut ImageFormatDesc,
    ) -> i32;
    pub fn dx_GetGraphPalette(
        GrHandle: i32,
        ColorIndex: i32,
        Red: *mut i32,
        Green: *mut i32,
        Blue: *mut i32,
    ) -> i32;
    pub fn dx_GetGraphOriginalPalette(
        GrHandle: i32,
        ColorIndex: i32,
        Red: *mut i32,
        Green: *mut i32,
        Blue: *mut i32,
    ) -> i32;
    pub fn dx_SetGraphPalette(GrHandle: i32, ColorIndex: i32, Color: u32) -> i32;
    pub fn dx_ResetGraphPalette(GrHandle: i32) -> i32;
    pub fn dx_DrawLine(x1: i32, y1: i32, x2: i32, y2: i32, Color: u32, Thickness: i32) -> i32;
    pub fn dx_DrawLineAA(x1: f32, y1: f32, x2: f32, y2: f32, Color: u32, Thickness: f32) -> i32;
    pub fn dx_DrawBox(x1: i32, y1: i32, x2: i32, y2: i32, Color: u32, FillFlag: i32) -> i32;
    pub fn dx_DrawBoxAA(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        Color: u32,
        FillFlag: i32,
        LineThickness: f32,
    ) -> i32;
    pub fn dx_DrawFillBox(x1: i32, y1: i32, x2: i32, y2: i32, Color: u32) -> i32;
    pub fn dx_DrawLineBox(x1: i32, y1: i32, x2: i32, y2: i32, Color: u32) -> i32;
    pub fn dx_DrawCircle(
        x: i32,
        y: i32,
        r: i32,
        Color: u32,
        FillFlag: i32,
        LineThickness: i32,
    ) -> i32;
    pub fn dx_DrawCircleAA(
        x: f32,
        y: f32,
        r: f32,
        posnum: i32,
        Color: u32,
        FillFlag: i32,
        LineThickness: f32,
    ) -> i32;
    pub fn dx_DrawOval(
        x: i32,
        y: i32,
        rx: i32,
        ry: i32,
        Color: u32,
        FillFlag: i32,
        LineThickness: i32,
    ) -> i32;
    pub fn dx_DrawOvalAA(
        x: f32,
        y: f32,
        rx: f32,
        ry: f32,
        posnum: i32,
        Color: u32,
        FillFlag: i32,
        LineThickness: f32,
    ) -> i32;
    pub fn dx_DrawOval_Rect(x1: i32, y1: i32, x2: i32, y2: i32, Color: u32, FillFlag: i32) -> i32;
    pub fn dx_DrawTriangle(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        Color: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_DrawTriangleAA(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        Color: u32,
        FillFlag: i32,
        LineThickness: f32,
    ) -> i32;
    pub fn dx_DrawQuadrangle(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        x4: i32,
        y4: i32,
        Color: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_DrawQuadrangleAA(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
        Color: u32,
        FillFlag: i32,
        LineThickness: f32,
    ) -> i32;
    pub fn dx_DrawRoundRect(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        rx: i32,
        ry: i32,
        Color: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_DrawRoundRectAA(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        rx: f32,
        ry: f32,
        posnum: i32,
        Color: u32,
        FillFlag: i32,
        LineThickness: f32,
    ) -> i32;
    pub fn dx_BeginAADraw() -> i32;
    pub fn dx_EndAADraw() -> i32;
    pub fn dx_DrawPixel(x: i32, y: i32, Color: u32) -> i32;
    pub fn dx_DrawPixelSet(Poi32DataArray: *const PointData, Num: i32) -> i32;
    pub fn dx_DrawLineSet(LineDataArray: *const LineData, Num: i32) -> i32;
    pub fn dx_DrawPixel3D(Pos: Vector, Color: u32) -> i32;
    pub fn dx_DrawPixel3DD(Pos: DVector, Color: u32) -> i32;
    pub fn dx_DrawLine3D(Pos1: Vector, Pos2: Vector, Color: u32) -> i32;
    pub fn dx_DrawLine3DD(Pos1: DVector, Pos2: DVector, Color: u32) -> i32;
    pub fn dx_DrawTriangle3D(
        Pos1: Vector,
        Pos2: Vector,
        Pos3: Vector,
        Color: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_DrawTriangle3DD(
        Pos1: DVector,
        Pos2: DVector,
        Pos3: DVector,
        Color: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_DrawCube3D(
        Pos1: Vector,
        Pos2: Vector,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_DrawCube3DD(
        Pos1: DVector,
        Pos2: DVector,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_DrawCubeSet3D(CubeDataArray: *mut CubeData, Num: i32, FillFlag: i32) -> i32;
    pub fn dx_DrawSphere3D(
        CenterPos: Vector,
        r: f32,
        DivNum: i32,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_DrawSphere3DD(
        CenterPos: DVector,
        r: f64,
        DivNum: i32,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_DrawCapsule3D(
        Pos1: Vector,
        Pos2: Vector,
        r: f32,
        DivNum: i32,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_DrawCapsule3DD(
        Pos1: DVector,
        Pos2: DVector,
        r: f64,
        DivNum: i32,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_DrawCone3D(
        TopPos: Vector,
        BottomPos: Vector,
        r: f32,
        DivNum: i32,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_DrawCone3DD(
        TopPos: DVector,
        BottomPos: DVector,
        r: f64,
        DivNum: i32,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_LoadGraphScreen(x: i32, y: i32, GraphName: *const i8, TransFlag: i32) -> i32;
    pub fn dx_LoadGraphScreenWithStrLen(
        x: i32,
        y: i32,
        GraphName: *const i8,
        GraphNameLength: usize,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawGraph(x: i32, y: i32, GrHandle: i32, TransFlag: i32) -> i32;
    pub fn dx_DrawExtendGraph(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraph(
        x: i32,
        y: i32,
        ExRate: f64,
        Angle: f64,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraph2(
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        ExtRate: f64,
        Angle: f64,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraph3(
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        ExtRateX: f64,
        ExtRateY: f64,
        Angle: f64,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraphFast(
        x: i32,
        y: i32,
        ExRate: f32,
        Angle: f32,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraphFast2(
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        ExtRate: f32,
        Angle: f32,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraphFast3(
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        ExtRateX: f32,
        ExtRateY: f32,
        Angle: f32,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawModiGraph(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        x4: i32,
        y4: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawTurnGraph(x: i32, y: i32, GrHandle: i32, TransFlag: i32) -> i32;
    pub fn dx_DrawReverseGraph(
        x: i32,
        y: i32,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawGraphF(xf: f32, yf: f32, GrHandle: i32, TransFlag: i32) -> i32;
    pub fn dx_DrawExtendGraphF(
        x1f: f32,
        y1f: f32,
        x2f: f32,
        y2: f32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraphF(
        xf: f32,
        yf: f32,
        ExRate: f64,
        Angle: f64,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraph2F(
        xf: f32,
        yf: f32,
        cxf: f32,
        cyf: f32,
        ExtRate: f64,
        Angle: f64,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraph3F(
        xf: f32,
        yf: f32,
        cxf: f32,
        cyf: f32,
        ExtRateX: f64,
        ExtRateY: f64,
        Angle: f64,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraphFastF(
        xf: f32,
        yf: f32,
        ExRate: f32,
        Angle: f32,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraphFast2F(
        xf: f32,
        yf: f32,
        cxf: f32,
        cyf: f32,
        ExtRate: f32,
        Angle: f32,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraphFast3F(
        xf: f32,
        yf: f32,
        cxf: f32,
        cyf: f32,
        ExtRateX: f32,
        ExtRateY: f32,
        Angle: f32,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawModiGraphF(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawTurnGraphF(xf: f32, yf: f32, GrHandle: i32, TransFlag: i32) -> i32;
    pub fn dx_DrawReverseGraphF(
        xf: f32,
        yf: f32,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawChipMap(
        Sx: i32,
        Sy: i32,
        XNum: i32,
        YNum: i32,
        MapData: *const i32,
        ChipTypeNum: i32,
        MapDataPitch: i32,
        ChipGrHandle: *const i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawChipMap_2(
        MapWidth: i32,
        MapHeight: i32,
        MapData: *const i32,
        ChipTypeNum: i32,
        ChipGrHandle: *const i32,
        TransFlag: i32,
        MapDrawPoi32X: i32,
        MapDrawPoi32Y: i32,
        MapDrawWidth: i32,
        MapDrawHeight: i32,
        ScreenX: i32,
        ScreenY: i32,
    ) -> i32;
    pub fn dx_DrawTile(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        Tx: i32,
        Ty: i32,
        ExtRate: f64,
        Angle: f64,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectGraph(
        DestX: i32,
        DestY: i32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectExtendGraph(
        DestX1: i32,
        DestY1: i32,
        DestX2: i32,
        DestY2: i32,
        SrcX: i32,
        SrcY: i32,
        SrcWidth: i32,
        SrcHeight: i32,
        GraphHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectRotaGraph(
        x: i32,
        y: i32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        ExtRate: f64,
        Angle: f64,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectRotaGraph2(
        x: i32,
        y: i32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        cx: i32,
        cy: i32,
        ExtRate: f64,
        Angle: f64,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectRotaGraph3(
        x: i32,
        y: i32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        cx: i32,
        cy: i32,
        ExtRateX: f64,
        ExtRateY: f64,
        Angle: f64,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectRotaGraphFast(
        x: i32,
        y: i32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        ExtRate: f32,
        Angle: f32,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectRotaGraphFast2(
        x: i32,
        y: i32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        cx: i32,
        cy: i32,
        ExtRate: f32,
        Angle: f32,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectRotaGraphFast3(
        x: i32,
        y: i32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        cx: i32,
        cy: i32,
        ExtRateX: f32,
        ExtRateY: f32,
        Angle: f32,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectModiGraph(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        x4: i32,
        y4: i32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        GraphHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectGraphF(
        DestX: f32,
        DestY: f32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectExtendGraphF(
        DestX1: f32,
        DestY1: f32,
        DestX2: f32,
        DestY2: f32,
        SrcX: i32,
        SrcY: i32,
        SrcWidth: i32,
        SrcHeight: i32,
        GraphHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectRotaGraphF(
        x: f32,
        y: f32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        ExtRate: f64,
        Angle: f64,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectRotaGraph2F(
        x: f32,
        y: f32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        cxf: f32,
        cyf: f32,
        ExtRate: f64,
        Angle: f64,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectRotaGraph3F(
        x: f32,
        y: f32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        cxf: f32,
        cyf: f32,
        ExtRateX: f64,
        ExtRateY: f64,
        Angle: f64,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectRotaGraphFastF(
        x: f32,
        y: f32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        ExtRate: f32,
        Angle: f32,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectRotaGraphFast2F(
        x: f32,
        y: f32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        cxf: f32,
        cyf: f32,
        ExtRate: f32,
        Angle: f32,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectRotaGraphFast3F(
        x: f32,
        y: f32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        cxf: f32,
        cyf: f32,
        ExtRateX: f32,
        ExtRateY: f32,
        Angle: f32,
        GraphHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRectModiGraphF(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
        SrcX: i32,
        SrcY: i32,
        Width: i32,
        Height: i32,
        GraphHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawBlendGraph(
        x: i32,
        y: i32,
        GrHandle: i32,
        TransFlag: i32,
        BlendGraph: i32,
        BorderParam: i32,
        BorderRange: i32,
    ) -> i32;
    pub fn dx_DrawBlendGraphPos(
        x: i32,
        y: i32,
        GrHandle: i32,
        TransFlag: i32,
        bx: i32,
        by: i32,
        BlendGraph: i32,
        BorderParam: i32,
        BorderRange: i32,
    ) -> i32;
    pub fn dx_DrawCircleGauge(
        CenterX: i32,
        CenterY: i32,
        Percent: f64,
        GrHandle: i32,
        StartPercent: f64,
        Scale: f64,
        ReverseX: i32,
        ReverseY: i32,
    ) -> i32;
    pub fn dx_DrawCircleGaugeF(
        CenterX: f32,
        CenterY: f32,
        Percent: f64,
        GrHandle: i32,
        StartPercent: f64,
        Scale: f64,
        ReverseX: i32,
        ReverseY: i32,
    ) -> i32;
    pub fn dx_DrawGraphToZBuffer(X: i32, Y: i32, GrHandle: i32, WriteZMode: i32) -> i32;
    pub fn dx_DrawTurnGraphToZBuffer(x: i32, y: i32, GrHandle: i32, WriteZMode: i32) -> i32;
    pub fn dx_DrawReverseGraphToZBuffer(
        x: i32,
        y: i32,
        GrHandle: i32,
        WriteZMode: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawExtendGraphToZBuffer(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        GrHandle: i32,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraphToZBuffer(
        x: i32,
        y: i32,
        ExRate: f64,
        Angle: f64,
        GrHandle: i32,
        WriteZMode: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraph2ToZBuffer(
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        ExtRate: f64,
        Angle: f64,
        GrHandle: i32,
        WriteZMode: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraph3ToZBuffer(
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        ExtRateX: f64,
        ExtRateY: f64,
        Angle: f64,
        GrHandle: i32,
        WriteZMode: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraphFastToZBuffer(
        x: i32,
        y: i32,
        ExRate: f32,
        Angle: f32,
        GrHandle: i32,
        WriteZMode: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraphFast2ToZBuffer(
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        ExtRate: f32,
        Angle: f32,
        GrHandle: i32,
        WriteZMode: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraphFast3ToZBuffer(
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        ExtRateX: f32,
        ExtRateY: f32,
        Angle: f32,
        GrHandle: i32,
        WriteZMode: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawModiGraphToZBuffer(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        x4: i32,
        y4: i32,
        GrHandle: i32,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawBoxToZBuffer(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FillFlag: i32,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawCircleToZBuffer(x: i32, y: i32, r: i32, FillFlag: i32, WriteZMode: i32) -> i32;
    pub fn dx_DrawTriangleToZBuffer(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        FillFlag: i32,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawQuadrangleToZBuffer(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        x4: i32,
        y4: i32,
        FillFlag: i32,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawRoundRectToZBuffer(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        rx: i32,
        ry: i32,
        FillFlag: i32,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawPolygon(
        VertexArray: *const Vertex,
        PolygonNum: i32,
        GrHandle: i32,
        TransFlag: i32,
        UVScaling: i32,
    ) -> i32;
    pub fn dx_DrawPolygon2D(
        VertexArray: *const Vertex2d,
        PolygonNum: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPolygon3D(
        VertexArray: *const Vertex3d,
        PolygonNum: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPolygonIndexed2D(
        VertexArray: *const Vertex2d,
        VertexNum: i32,
        IndexArray: *const u16,
        PolygonNum: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPolygonIndexed3D(
        VertexArray: *const Vertex3d,
        VertexNum: i32,
        IndexArray: *const u16,
        PolygonNum: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPolygonIndexed3DBase(
        VertexArray: *const Vertex3dCompat,
        VertexNum: i32,
        IndexArray: *const u16,
        IndexNum: i32,
        PrimitiveType: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPolygon3DBase(
        VertexArray: *const Vertex3dCompat,
        VertexNum: i32,
        PrimitiveType: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPolygon3D_2(
        VertexArray: *const Vertex3dCompat,
        PolygonNum: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPolygonBase(
        VertexArray: *const Vertex,
        VertexNum: i32,
        PrimitiveType: i32,
        GrHandle: i32,
        TransFlag: i32,
        UVScaling: i32,
    ) -> i32;
    pub fn dx_DrawPrimitive2D(
        VertexArray: *const Vertex2d,
        VertexNum: i32,
        PrimitiveType: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPrimitive3D(
        VertexArray: *const Vertex3d,
        VertexNum: i32,
        PrimitiveType: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPrimitiveIndexed2D(
        VertexArray: *const Vertex2d,
        VertexNum: i32,
        IndexArray: *const u16,
        IndexNum: i32,
        PrimitiveType: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPrimitiveIndexed3D(
        VertexArray: *const Vertex3d,
        VertexNum: i32,
        IndexArray: *const u16,
        IndexNum: i32,
        PrimitiveType: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPolygon3D_UseVertexBuffer(
        VertexBufHandle: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPrimitive3D_UseVertexBuffer(
        VertexBufHandle: i32,
        PrimitiveType: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPrimitive3D_UseVertexBuffer2(
        VertexBufHandle: i32,
        PrimitiveType: i32,
        StartVertex: i32,
        UseVertexNum: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPolygonIndexed3D_UseVertexBuffer(
        VertexBufHandle: i32,
        IndexBufHandle: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPrimitiveIndexed3D_UseVertexBuffer(
        VertexBufHandle: i32,
        IndexBufHandle: i32,
        PrimitiveType: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPrimitiveIndexed3D_UseVertexBuffer2(
        VertexBufHandle: i32,
        IndexBufHandle: i32,
        PrimitiveType: i32,
        BaseVertex: i32,
        StartVertex: i32,
        UseVertexNum: i32,
        StartIndex: i32,
        UseIndexNum: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawGraph3D(x: f32, y: f32, z: f32, GrHandle: i32, TransFlag: i32) -> i32;
    pub fn dx_DrawExtendGraph3D(
        x: f32,
        y: f32,
        z: f32,
        ExRateX: f64,
        ExRateY: f64,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawRotaGraph3D(
        x: f32,
        y: f32,
        z: f32,
        ExRate: f64,
        Angle: f64,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawRota2Graph3D(
        x: f32,
        y: f32,
        z: f32,
        cx: f32,
        cy: f32,
        ExtRateX: f64,
        ExtRateY: f64,
        Angle: f64,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawModiBillboard3D(
        Pos: Vector,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawBillboard3D(
        Pos: Vector,
        cx: f32,
        cy: f32,
        Size: f32,
        Angle: f32,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_SetDrawMode(DrawMode: i32) -> i32;
    pub fn dx_GetDrawMode() -> i32;
    pub fn dx_SetDrawBlendMode(BlendMode: i32, BlendParam: i32) -> i32;
    pub fn dx_GetDrawBlendMode(BlendMode: *mut i32, BlendParam: *mut i32) -> i32;
    pub fn dx_SetDrawAlphaTest(TestMode: i32, TestParam: i32) -> i32;
    pub fn dx_GetDrawAlphaTest(TestMode: *mut i32, TestParam: *mut i32) -> i32;
    pub fn dx_SetBlendGraph(BlendGraph: i32, BorderParam: i32, BorderRange: i32) -> i32;
    pub fn dx_SetBlendGraphPosition(x: i32, y: i32) -> i32;
    pub fn dx_SetBlendGraphPositionMode(BlendGraphPositionMode: i32) -> i32;
    pub fn dx_SetDrawBright(RedBright: i32, GreenBright: i32, BlueBright: i32) -> i32;
    pub fn dx_GetDrawBright(Red: *mut i32, Green: *mut i32, Blue: *mut i32) -> i32;
    pub fn dx_SetWriteAlphaChannelFlag(Flag: i32) -> i32;
    pub fn dx_GetWriteAlphaChannelFlag() -> i32;
    pub fn dx_CheckSeparateAlphaBlendEnable() -> i32;
    pub fn dx_SetIgnoreDrawGraphColor(EnableFlag: i32) -> i32;
    pub fn dx_SetMaxAnisotropy(MaxAnisotropy: i32) -> i32;
    pub fn dx_GetMaxAnisotropy() -> i32;
    pub fn dx_SetUseLarge3DPositionSupport(UseFlag: i32) -> i32;
    pub fn dx_SetUseZBufferFlag(Flag: i32) -> i32;
    pub fn dx_SetWriteZBufferFlag(Flag: i32) -> i32;
    pub fn dx_SetZBufferCmpType(CmpType: i32) -> i32;
    pub fn dx_SetZBias(Bias: i32) -> i32;
    pub fn dx_SetUseZBuffer3D(Flag: i32) -> i32;
    pub fn dx_SetWriteZBuffer3D(Flag: i32) -> i32;
    pub fn dx_SetZBufferCmpType3D(CmpType: i32) -> i32;
    pub fn dx_SetZBias3D(Bias: i32) -> i32;
    pub fn dx_SetDrawZ(Z: f32) -> i32;
    pub fn dx_SetDrawArea(x1: i32, y1: i32, x2: i32, y2: i32) -> i32;
    pub fn dx_GetDrawArea(Rect: *mut Rect) -> i32;
    pub fn dx_SetDrawAreaFull() -> i32;
    pub fn dx_SetDraw3DScale(Scale: f32) -> i32;
    pub fn dx_SetRestoreShredPoint(ShredPoint: *const libc::c_void) -> i32;
    pub fn dx_SetRestoreGraphCallback(Callback: *const libc::c_void) -> i32;
    pub fn dx_RunRestoreShred() -> i32;
    pub fn dx_SetGraphicsDeviceRestoreCallbackFunction(
        Callback: *const libc::c_void,
        CallbackData: *mut libc::c_void,
    ) -> i32;
    pub fn dx_SetGraphicsDeviceLostCallbackFunction(
        Callback: *const libc::c_void,
        CallbackData: *mut libc::c_void,
    ) -> i32;
    pub fn dx_SetTransformTo2D(Matrix: *const Matrix) -> i32;
    pub fn dx_SetTransformTo2DD(Matrix: *const DMatrix) -> i32;
    pub fn dx_ResetTransformTo2D() -> i32;
    pub fn dx_SetTransformToWorld(Matrix: *const Matrix) -> i32;
    pub fn dx_SetTransformToWorldD(Matrix: *const DMatrix) -> i32;
    pub fn dx_GetTransformToWorldMatrix(MatBuf: *mut Matrix) -> i32;
    pub fn dx_GetTransformToWorldMatrixD(MatBuf: *mut DMatrix) -> i32;
    pub fn dx_SetTransformToView(Matrix: *const Matrix) -> i32;
    pub fn dx_SetTransformToViewD(Matrix: *const DMatrix) -> i32;
    pub fn dx_GetTransformToViewMatrix(MatBuf: *mut Matrix) -> i32;
    pub fn dx_GetTransformToViewMatrixD(MatBuf: *mut DMatrix) -> i32;
    pub fn dx_SetTransformToProjection(Matrix: *const Matrix) -> i32;
    pub fn dx_SetTransformToProjectionD(Matrix: *const DMatrix) -> i32;
    pub fn dx_GetTransformToProjectionMatrix(MatBuf: *mut Matrix) -> i32;
    pub fn dx_GetTransformToProjectionMatrixD(MatBuf: *mut DMatrix) -> i32;
    pub fn dx_SetTransformToViewport(Matrix: *const Matrix) -> i32;
    pub fn dx_SetTransformToViewportD(Matrix: *const DMatrix) -> i32;
    pub fn dx_GetTransformToViewportMatrix(MatBuf: *mut Matrix) -> i32;
    pub fn dx_GetTransformToViewportMatrixD(MatBuf: *mut DMatrix) -> i32;
    pub fn dx_GetTransformToAPIViewportMatrix(MatBuf: *mut Matrix) -> i32;
    pub fn dx_GetTransformToAPIViewportMatrixD(MatBuf: *mut DMatrix) -> i32;
    pub fn dx_SetDefTransformMatrix() -> i32;
    pub fn dx_GetTransformPosition(LocalPos: *mut Vector, x: *mut f32, y: *mut f32) -> i32;
    pub fn dx_GetTransformPositionD(LocalPos: *mut DVector, x: *mut f64, y: *mut f64) -> i32;
    pub fn dx_GetBillboardPixelSize(WorldPos: Vector, WorldSize: f32) -> f32;
    pub fn dx_GetBillboardPixelSizeD(WorldPos: DVector, WorldSize: f64) -> f64;
    pub fn dx_ConvWorldPosToViewPos(WorldPos: Vector) -> Vector;
    pub fn dx_ConvWorldPosToViewPosD(WorldPos: DVector) -> DVector;
    pub fn dx_ConvWorldPosToScreenPos(WorldPos: Vector) -> Vector;
    pub fn dx_ConvWorldPosToScreenPosD(WorldPos: DVector) -> DVector;
    pub fn dx_ConvWorldPosToScreenPosPlusW(WorldPos: Vector) -> Float4;
    pub fn dx_ConvWorldPosToScreenPosPlusWD(WorldPos: DVector) -> Double4;
    pub fn dx_ConvScreenPosToWorldPos(ScreenPos: Vector) -> Vector;
    pub fn dx_ConvScreenPosToWorldPosD(ScreenPos: DVector) -> DVector;
    pub fn dx_ConvScreenPosToWorldPos_ZLinear(ScreenPos: Vector) -> Vector;
    pub fn dx_ConvScreenPosToWorldPos_ZLinearD(ScreenPos: DVector) -> DVector;
    pub fn dx_SetUseCullingFlag(Flag: i32) -> i32;
    pub fn dx_SetUseBackCulling(Flag: i32) -> i32;
    pub fn dx_GetUseBackCulling() -> i32;
    pub fn dx_SetTextureAddressMode(Mode: i32, Stage: i32) -> i32;
    pub fn dx_SetTextureAddressModeUV(ModeU: i32, ModeV: i32, Stage: i32) -> i32;
    pub fn dx_SetTextureAddressTransform(
        TransU: f32,
        TransV: f32,
        ScaleU: f32,
        ScaleV: f32,
        RotCenterU: f32,
        RotCenterV: f32,
        Rotate: f32,
    ) -> i32;
    pub fn dx_SetTextureAddressTransformMatrix(Matrix: Matrix) -> i32;
    pub fn dx_ResetTextureAddressTransform() -> i32;
    pub fn dx_SetFogEnable(Flag: i32) -> i32;
    pub fn dx_GetFogEnable() -> i32;
    pub fn dx_SetFogMode(Mode: i32) -> i32;
    pub fn dx_GetFogMode() -> i32;
    pub fn dx_SetFogColor(r: i32, g: i32, b: i32) -> i32;
    pub fn dx_GetFogColor(r: *mut i32, g: *mut i32, b: *mut i32) -> i32;
    pub fn dx_SetFogStartEnd(start: f32, end: f32) -> i32;
    pub fn dx_GetFogStartEnd(start: *mut f32, end: *mut f32) -> i32;
    pub fn dx_SetFogDensity(density: f32) -> i32;
    pub fn dx_GetFogDensity() -> f32;
    pub fn dx_GetPixelF(x: i32, y: i32) -> ColorF32;
    pub fn dx_SetBackgroundColor(Red: i32, Green: i32, Blue: i32, Alpha: i32) -> i32;
    pub fn dx_GetBackgroundColor(
        Red: *mut i32,
        Green: *mut i32,
        Blue: *mut i32,
        Alpha: *mut i32,
    ) -> i32;
    pub fn dx_GetDrawScreenGraph(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        GrHandle: i32,
        UseClientFlag: i32,
    ) -> i32;
    pub fn dx_BltDrawValidGraph(
        TargetDrawValidGrHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        DestX: i32,
        DestY: i32,
        DestGrHandle: i32,
    ) -> i32;
    pub fn dx_ScreenFlip() -> i32;
    pub fn dx_ScreenCopy() -> i32;
    pub fn dx_WaitVSync(SyncNum: i32) -> i32;
    pub fn dx_ClearDrawScreen(ClearRect: *const Rect) -> i32;
    pub fn dx_ClearDrawScreenZBuffer(ClearRect: *const Rect) -> i32;
    pub fn dx_ClsDrawScreen() -> i32;
    pub fn dx_SetDrawScreen(DrawScreen: i32) -> i32;
    pub fn dx_GetDrawScreen() -> i32;
    pub fn dx_GetActiveGraph() -> i32;
    pub fn dx_SetUseSetDrawScreenSettingReset(UseFlag: i32) -> i32;
    pub fn dx_GetUseSetDrawScreenSettingReset() -> i32;
    pub fn dx_SetDrawZBuffer(DrawScreen: i32) -> i32;
    pub fn dx_SetGraphMode(
        ScreenSizeX: i32,
        ScreenSizeY: i32,
        ColorBitDepth: i32,
        RefreshRate: i32,
    ) -> i32;
    pub fn dx_SetUserScreenImage(Image: *mut libc::c_void, PixelFormat: i32) -> i32;
    pub fn dx_SetFullScreenResolutionMode(ResolutionMode: i32) -> i32;
    pub fn dx_GetFullScreenResolutionMode(
        ResolutionMode: *mut i32,
        UseResolutionMode: *mut i32,
    ) -> i32;
    pub fn dx_SetFullScreenScalingMode(ScalingMode: i32, FitScaling: i32) -> i32;
    pub fn dx_SetEmulation320x240(Flag: i32) -> i32;
    pub fn dx_SetZBufferSize(ZBufferSizeX: i32, ZBufferSizeY: i32) -> i32;
    pub fn dx_SetZBufferBitDepth(BitDepth: i32) -> i32;
    pub fn dx_SetWaitVSyncFlag(Flag: i32) -> i32;
    pub fn dx_GetWaitVSyncFlag() -> i32;
    pub fn dx_SetFullSceneAntiAliasingMode(Samples: i32, Quality: i32) -> i32;
    pub fn dx_SetGraphDisplayArea(x1: i32, y1: i32, x2: i32, y2: i32) -> i32;
    pub fn dx_SetChangeScreenModeGraphicsSystemResetFlag(Flag: i32) -> i32;
    pub fn dx_GetScreenState(SizeX: *mut i32, SizeY: *mut i32, ColorBitDepth: *mut i32) -> i32;
    pub fn dx_GetDrawScreenSize(XBuf: *mut i32, YBuf: *mut i32) -> i32;
    pub fn dx_GetScreenBitDepth() -> i32;
    pub fn dx_GetColorBitDepth() -> i32;
    pub fn dx_GetChangeDisplayFlag() -> i32;
    pub fn dx_GetVideoMemorySize(AllSize: *mut i32, FreeSize: *mut i32) -> i32;
    pub fn dx_GetRefreshRate() -> i32;
    pub fn dx_GetDisplayNum() -> i32;
    pub fn dx_GetDisplayInfo(
        DisplayIndex: i32,
        DesktopRectX: *mut i32,
        DesktopRectY: *mut i32,
        DesktopSizeX: *mut i32,
        DesktopSizeY: *mut i32,
        IsPrimary: *mut i32,
    ) -> i32;
    pub fn dx_GetDisplayModeNum(DisplayIndex: i32) -> i32;
    pub fn dx_GetDisplayMode(ModeIndex: i32, DisplayIndex: i32) -> DisplayModeData;
    pub fn dx_GetDisplayMaxResolution(SizeX: *mut i32, SizeY: *mut i32, DisplayIndex: i32) -> i32;
    pub fn dx_GetMultiDrawScreenNum() -> i32;
    pub fn dx_GetDrawFloatCoordType() -> i32;
    pub fn dx_SetUseNormalDrawShader(Flag: i32) -> i32;
    pub fn dx_SetUseSoftwareRenderModeFlag(Flag: i32) -> i32;
    pub fn dx_SetNotUse3DFlag(Flag: i32) -> i32;
    pub fn dx_SetUse3DFlag(Flag: i32) -> i32;
    pub fn dx_GetUse3DFlag() -> i32;
    pub fn dx_SetScreenMemToVramFlag(Flag: i32) -> i32;
    pub fn dx_GetScreenMemToSystemMemFlag() -> i32;
    pub fn dx_SetWindowDrawRect(DrawRect: *const Rect) -> i32;
    pub fn dx_RestoreGraphSystem() -> i32;
    pub fn dx_SetUseHardwareVertexProcessing(Flag: i32) -> i32;
    pub fn dx_SetUsePixelLighting(Flag: i32) -> i32;
    pub fn dx_SetUseOldDrawModiGraphCodeFlag(Flag: i32) -> i32;
    pub fn dx_SetUseVramFlag(Flag: i32) -> i32;
    pub fn dx_GetUseVramFlag() -> i32;
    pub fn dx_SetBasicBlendFlag(Flag: i32) -> i32;
    pub fn dx_SetUseBasicGraphDraw3DDeviceMethodFlag(Flag: i32) -> i32;
    pub fn dx_SetUseDisplayIndex(Index: i32) -> i32;
    pub fn dx_RenderVertex() -> i32;
    pub fn dx_GetDrawCallCount() -> i32;
    pub fn dx_GetFPS() -> f32;
    pub fn dx_SaveDrawScreen(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        SaveType: i32,
        Jpeg_Quality: i32,
        Jpeg_Sample2x1: i32,
        Png_CompressionLevel: i32,
    ) -> i32;
    pub fn dx_SaveDrawScreenWithStrLen(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        FileNameLength: usize,
        SaveType: i32,
        Jpeg_Quality: i32,
        Jpeg_Sample2x1: i32,
        Png_CompressionLevel: i32,
    ) -> i32;
    pub fn dx_SaveDrawScreenToBMP(x1: i32, y1: i32, x2: i32, y2: i32, FileName: *const i8) -> i32;
    pub fn dx_SaveDrawScreenToBMPWithStrLen(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        FileNameLength: usize,
    ) -> i32;
    pub fn dx_SaveDrawScreenToDDS(x1: i32, y1: i32, x2: i32, y2: i32, FileName: *const i8) -> i32;
    pub fn dx_SaveDrawScreenToDDSWithStrLen(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        FileNameLength: usize,
    ) -> i32;
    pub fn dx_SaveDrawScreenToJPEG(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        Quality: i32,
        Sample2x1: i32,
    ) -> i32;
    pub fn dx_SaveDrawScreenToJPEGWithStrLen(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        FileNameLength: usize,
        Quality: i32,
        Sample2x1: i32,
    ) -> i32;
    pub fn dx_SaveDrawScreenToPNG(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        CompressionLevel: i32,
    ) -> i32;
    pub fn dx_SaveDrawScreenToPNGWithStrLen(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        FileNameLength: usize,
        CompressionLevel: i32,
    ) -> i32;
    pub fn dx_SaveDrawValidGraph(
        GrHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        SaveType: i32,
        Jpeg_Quality: i32,
        Jpeg_Sample2x1: i32,
        Png_CompressionLevel: i32,
    ) -> i32;
    pub fn dx_SaveDrawValidGraphWithStrLen(
        GrHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        FileNameLength: usize,
        SaveType: i32,
        Jpeg_Quality: i32,
        Jpeg_Sample2x1: i32,
        Png_CompressionLevel: i32,
    ) -> i32;
    pub fn dx_SaveDrawValidGraphToBMP(
        GrHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
    ) -> i32;
    pub fn dx_SaveDrawValidGraphToBMPWithStrLen(
        GrHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        FileNameLength: usize,
    ) -> i32;
    pub fn dx_SaveDrawValidGraphToDDS(
        GrHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
    ) -> i32;
    pub fn dx_SaveDrawValidGraphToDDSWithStrLen(
        GrHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        FileNameLength: usize,
    ) -> i32;
    pub fn dx_SaveDrawValidGraphToJPEG(
        GrHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        Quality: i32,
        Sample2x1: i32,
    ) -> i32;
    pub fn dx_SaveDrawValidGraphToJPEGWithStrLen(
        GrHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        FileNameLength: usize,
        Quality: i32,
        Sample2x1: i32,
    ) -> i32;
    pub fn dx_SaveDrawValidGraphToPNG(
        GrHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        CompressionLevel: i32,
    ) -> i32;
    pub fn dx_SaveDrawValidGraphToPNGWithStrLen(
        GrHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        FileName: *const i8,
        FileNameLength: usize,
        CompressionLevel: i32,
    ) -> i32;
    pub fn dx_CreateVertexBuffer(VertexNum: i32, VertexType: i32) -> i32;
    pub fn dx_DeleteVertexBuffer(VertexBufHandle: i32) -> i32;
    pub fn dx_InitVertexBuffer() -> i32;
    pub fn dx_SetVertexBufferData(
        SetIndex: i32,
        VertexArray: *const libc::c_void,
        VertexNum: i32,
        VertexBufHandle: i32,
    ) -> i32;
    pub fn dx_UpdateVertexBuffer(
        VertexBufHandle: i32,
        UpdateStartIndex: i32,
        UpdateVertexNum: i32,
    ) -> i32;
    pub fn dx_CreateIndexBuffer(IndexNum: i32, IndexType: i32) -> i32;
    pub fn dx_DeleteIndexBuffer(IndexBufHandle: i32) -> i32;
    pub fn dx_InitIndexBuffer() -> i32;
    pub fn dx_SetIndexBufferData(
        SetIndex: i32,
        IndexArray: *const libc::c_void,
        IndexNum: i32,
        IndexBufHandle: i32,
    ) -> i32;
    pub fn dx_UpdateIndexBuffer(
        IndexBufHandle: i32,
        UpdateStartIndex: i32,
        UpdateIndexNum: i32,
    ) -> i32;
    pub fn dx_GetMaxPrimitiveCount() -> i32;
    pub fn dx_GetMaxVertexIndex() -> i32;
    pub fn dx_GetValidShaderVersion() -> i32;
    pub fn dx_LoadVertexShader(FileName: *const i8) -> i32;
    pub fn dx_LoadVertexShaderWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_LoadGeometryShader(FileName: *const i8) -> i32;
    pub fn dx_LoadGeometryShaderWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_LoadPixelShader(FileName: *const i8) -> i32;
    pub fn dx_LoadPixelShaderWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_LoadVertexShaderFromMem(ImageAddress: *const libc::c_void, ImageSize: i32) -> i32;
    pub fn dx_LoadGeometryShaderFromMem(ImageAddress: *const libc::c_void, ImageSize: i32) -> i32;
    pub fn dx_LoadPixelShaderFromMem(ImageAddress: *const libc::c_void, ImageSize: i32) -> i32;
    pub fn dx_DeleteShader(ShaderHandle: i32) -> i32;
    pub fn dx_InitShader() -> i32;
    pub fn dx_GetConstIndexToShader(ConstantName: *const i8, ShaderHandle: i32) -> i32;
    pub fn dx_GetConstIndexToShaderWithStrLen(
        ConstantName: *const i8,
        ConstantNameLength: usize,
        ShaderHandle: i32,
    ) -> i32;
    pub fn dx_GetConstCountToShader(ConstantName: *const i8, ShaderHandle: i32) -> i32;
    pub fn dx_GetConstCountToShaderWithStrLen(
        ConstantName: *const i8,
        ConstantNameLength: usize,
        ShaderHandle: i32,
    ) -> i32;
    pub fn dx_SetVSConstSF(ConstantIndex: i32, Param: f32) -> i32;
    pub fn dx_SetVSConstF(ConstantIndex: i32, Param: Float4) -> i32;
    pub fn dx_SetVSConstFMtx(ConstantIndex: i32, Param: Matrix) -> i32;
    pub fn dx_SetVSConstFMtxT(ConstantIndex: i32, Param: Matrix) -> i32;
    pub fn dx_SetVSConstSI(ConstantIndex: i32, Param: i32) -> i32;
    pub fn dx_SetVSConstI(ConstantIndex: i32, Param: Int4) -> i32;
    pub fn dx_SetVSConstB(ConstantIndex: i32, Param: i32) -> i32;
    pub fn dx_SetVSConstSFArray(ConstantIndex: i32, ParamArray: *const f32, ParamNum: i32) -> i32;
    pub fn dx_SetVSConstFArray(ConstantIndex: i32, ParamArray: *const Float4, ParamNum: i32)
        -> i32;
    pub fn dx_SetVSConstFMtxArray(
        ConstantIndex: i32,
        ParamArray: *const Matrix,
        ParamNum: i32,
    ) -> i32;
    pub fn dx_SetVSConstFMtxTArray(
        ConstantIndex: i32,
        ParamArray: *const Matrix,
        ParamNum: i32,
    ) -> i32;
    pub fn dx_SetVSConstSIArray(ConstantIndex: i32, ParamArray: *const i32, ParamNum: i32) -> i32;
    pub fn dx_SetVSConstIArray(ConstantIndex: i32, ParamArray: *const Int4, ParamNum: i32) -> i32;
    pub fn dx_SetVSConstBArray(ConstantIndex: i32, ParamArray: *const i32, ParamNum: i32) -> i32;
    pub fn dx_ResetVSConstF(ConstantIndex: i32, ParamNum: i32) -> i32;
    pub fn dx_ResetVSConstI(ConstantIndex: i32, ParamNum: i32) -> i32;
    pub fn dx_ResetVSConstB(ConstantIndex: i32, ParamNum: i32) -> i32;
    pub fn dx_SetPSConstSF(ConstantIndex: i32, Param: f32) -> i32;
    pub fn dx_SetPSConstF(ConstantIndex: i32, Param: Float4) -> i32;
    pub fn dx_SetPSConstFMtx(ConstantIndex: i32, Param: Matrix) -> i32;
    pub fn dx_SetPSConstFMtxT(ConstantIndex: i32, Param: Matrix) -> i32;
    pub fn dx_SetPSConstSI(ConstantIndex: i32, Param: i32) -> i32;
    pub fn dx_SetPSConstI(ConstantIndex: i32, Param: Int4) -> i32;
    pub fn dx_SetPSConstB(ConstantIndex: i32, Param: i32) -> i32;
    pub fn dx_SetPSConstSFArray(ConstantIndex: i32, ParamArray: *const f32, ParamNum: i32) -> i32;
    pub fn dx_SetPSConstFArray(ConstantIndex: i32, ParamArray: *const Float4, ParamNum: i32)
        -> i32;
    pub fn dx_SetPSConstFMtxArray(
        ConstantIndex: i32,
        ParamArray: *const Matrix,
        ParamNum: i32,
    ) -> i32;
    pub fn dx_SetPSConstFMtxTArray(
        ConstantIndex: i32,
        ParamArray: *const Matrix,
        ParamNum: i32,
    ) -> i32;
    pub fn dx_SetPSConstSIArray(ConstantIndex: i32, ParamArray: *const i32, ParamNum: i32) -> i32;
    pub fn dx_SetPSConstIArray(ConstantIndex: i32, ParamArray: *const Int4, ParamNum: i32) -> i32;
    pub fn dx_SetPSConstBArray(ConstantIndex: i32, ParamArray: *const i32, ParamNum: i32) -> i32;
    pub fn dx_ResetPSConstF(ConstantIndex: i32, ParamNum: i32) -> i32;
    pub fn dx_ResetPSConstI(ConstantIndex: i32, ParamNum: i32) -> i32;
    pub fn dx_ResetPSConstB(ConstantIndex: i32, ParamNum: i32) -> i32;
    pub fn dx_SetRenderTargetToShader(
        TargetIndex: i32,
        DrawScreen: i32,
        SurfaceIndex: i32,
        MipLevel: i32,
    ) -> i32;
    pub fn dx_SetUseTextureToShader(StageIndex: i32, GraphHandle: i32) -> i32;
    pub fn dx_SetUseVertexShader(ShaderHandle: i32) -> i32;
    pub fn dx_SetUseGeometryShader(ShaderHandle: i32) -> i32;
    pub fn dx_SetUsePixelShader(ShaderHandle: i32) -> i32;
    pub fn dx_CalcPolygonBinormalAndTangentsToShader(
        VertexArray: *mut Vertex3dShader,
        PolygonNum: i32,
    ) -> i32;
    pub fn dx_CalcPolygonIndexedBinormalAndTangentsToShader(
        VertexArray: *mut Vertex3dShader,
        VertexNum: i32,
        IndexArray: *const u16,
        PolygonNum: i32,
    ) -> i32;
    pub fn dx_DrawBillboard3DToShader(
        Pos: Vector,
        cx: f32,
        cy: f32,
        Size: f32,
        Angle: f32,
        GrHandle: i32,
        TransFlag: i32,
        ReverseXFlag: i32,
        ReverseYFlag: i32,
    ) -> i32;
    pub fn dx_DrawPolygon2DToShader(VertexArray: *const Vertex2dShader, PolygonNum: i32) -> i32;
    pub fn dx_DrawPolygon3DToShader(VertexArray: *const Vertex3dShader, PolygonNum: i32) -> i32;
    pub fn dx_DrawPolygonIndexed2DToShader(
        VertexArray: *const Vertex2dShader,
        VertexNum: i32,
        IndexArray: *const u16,
        PolygonNum: i32,
    ) -> i32;
    pub fn dx_DrawPolygonIndexed3DToShader(
        VertexArray: *const Vertex3dShader,
        VertexNum: i32,
        IndexArray: *const u16,
        PolygonNum: i32,
    ) -> i32;
    pub fn dx_DrawPrimitive2DToShader(
        VertexArray: *const Vertex2dShader,
        VertexNum: i32,
        PrimitiveType: i32,
    ) -> i32;
    pub fn dx_DrawPrimitive3DToShader(
        VertexArray: *const Vertex3dShader,
        VertexNum: i32,
        PrimitiveType: i32,
    ) -> i32;
    pub fn dx_DrawPrimitiveIndexed2DToShader(
        VertexArray: *const Vertex2dShader,
        VertexNum: i32,
        IndexArray: *const u16,
        IndexNum: i32,
        PrimitiveType: i32,
    ) -> i32;
    pub fn dx_DrawPrimitiveIndexed3DToShader(
        VertexArray: *const Vertex3dShader,
        VertexNum: i32,
        IndexArray: *const u16,
        IndexNum: i32,
        PrimitiveType: i32,
    ) -> i32;
    pub fn dx_DrawPolygon3DToShader_UseVertexBuffer(VertexBufHandle: i32) -> i32;
    pub fn dx_DrawPolygonIndexed3DToShader_UseVertexBuffer(
        VertexBufHandle: i32,
        IndexBufHandle: i32,
    ) -> i32;
    pub fn dx_DrawPrimitive3DToShader_UseVertexBuffer(
        VertexBufHandle: i32,
        PrimitiveType: i32,
    ) -> i32;
    pub fn dx_DrawPrimitive3DToShader_UseVertexBuffer2(
        VertexBufHandle: i32,
        PrimitiveType: i32,
        StartVertex: i32,
        UseVertexNum: i32,
    ) -> i32;
    pub fn dx_DrawPrimitiveIndexed3DToShader_UseVertexBuffer(
        VertexBufHandle: i32,
        IndexBufHandle: i32,
        PrimitiveType: i32,
    ) -> i32;
    pub fn dx_DrawPrimitiveIndexed3DToShader_UseVertexBuffer2(
        VertexBufHandle: i32,
        IndexBufHandle: i32,
        PrimitiveType: i32,
        BaseVertex: i32,
        StartVertex: i32,
        UseVertexNum: i32,
        StartIndex: i32,
        UseIndexNum: i32,
    ) -> i32;
    pub fn dx_InitShaderConstantBuffer() -> i32;
    pub fn dx_CreateShaderConstantBuffer(BufferSize: i32) -> i32;
    pub fn dx_DeleteShaderConstantBuffer(SConstBufHandle: i32) -> i32;
    pub fn dx_UpdateShaderConstantBuffer(SConstBufHandle: i32) -> i32;
    pub fn dx_SetShaderConstantBuffer(SConstBufHandle: i32, TargetShader: i32, Slot: i32) -> i32;
    pub fn dx_SetGraphFilterBltBlendMode(BlendMode: i32) -> i32;
    pub fn dx_PlayMovie(FileName: *const i8, ExRate: i32, PlayType: i32) -> i32;
    pub fn dx_PlayMovieWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        ExRate: i32,
        PlayType: i32,
    ) -> i32;
    pub fn dx_GetMovieImageSize_File(FileName: *const i8, SizeX: *mut i32, SizeY: *mut i32) -> i32;
    pub fn dx_GetMovieImageSize_File_WithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        SizeX: *mut i32,
        SizeY: *mut i32,
    ) -> i32;
    pub fn dx_GetMovieImageSize_Mem(
        FileImage: *const libc::c_void,
        FileImageSize: i32,
        SizeX: *mut i32,
        SizeY: *mut i32,
    ) -> i32;
    pub fn dx_OpenMovieToGraph(FileName: *const i8, FullColor: i32) -> i32;
    pub fn dx_OpenMovieToGraphWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        FullColor: i32,
    ) -> i32;
    pub fn dx_PlayMovieToGraph(GraphHandle: i32, PlayType: i32, SysPlay: i32) -> i32;
    pub fn dx_PauseMovieToGraph(GraphHandle: i32, SysPause: i32) -> i32;
    pub fn dx_AddMovieFrameToGraph(GraphHandle: i32, FrameNum: u32) -> i32;
    pub fn dx_SeekMovieToGraph(GraphHandle: i32, Time: i32) -> i32;
    pub fn dx_SetPlaySpeedRateMovieToGraph(GraphHandle: i32, SpeedRate: f64) -> i32;
    pub fn dx_GetMovieStateToGraph(GraphHandle: i32) -> i32;
    pub fn dx_SetMovieVolumeToGraph(Volume: i32, GraphHandle: i32) -> i32;
    pub fn dx_ChangeMovieVolumeToGraph(Volume: i32, GraphHandle: i32) -> i32;
    pub fn dx_GetMovieTotalFrameToGraph(GraphHandle: i32) -> i32;
    pub fn dx_TellMovieToGraph(GraphHandle: i32) -> i32;
    pub fn dx_TellMovieToGraphToFrame(GraphHandle: i32) -> i32;
    pub fn dx_SeekMovieToGraphToFrame(GraphHandle: i32, Frame: i32) -> i32;
    pub fn dx_GetOneFrameTimeMovieToGraph(GraphHandle: i32) -> i64;
    pub fn dx_GetLastUpdateTimeMovieToGraph(GraphHandle: i32) -> i32;
    pub fn dx_SetMovieRightImageAlphaFlag(Flag: i32) -> i32;
    pub fn dx_SetMovieColorA8R8G8B8Flag(Flag: i32) -> i32;
    pub fn dx_SetMovieUseYUVFormatSurfaceFlag(Flag: i32) -> i32;
    pub fn dx_SetCameraNearFar(Near: f32, Far: f32) -> i32;
    pub fn dx_SetCameraNearFarD(Near: f64, Far: f64) -> i32;
    pub fn dx_SetCameraPositionAndTarget_UpVecY(Position: Vector, Target: Vector) -> i32;
    pub fn dx_SetCameraPositionAndTarget_UpVecYD(Position: DVector, Target: DVector) -> i32;
    pub fn dx_SetCameraPositionAndTargetAndUpVec(
        Position: Vector,
        TargetPosition: Vector,
        UpVector: Vector,
    ) -> i32;
    pub fn dx_SetCameraPositionAndTargetAndUpVecD(
        Position: DVector,
        TargetPosition: DVector,
        UpVector: DVector,
    ) -> i32;
    pub fn dx_SetCameraPositionAndAngle(
        Position: Vector,
        VRotate: f32,
        HRotate: f32,
        TRotate: f32,
    ) -> i32;
    pub fn dx_SetCameraPositionAndAngleD(
        Position: DVector,
        VRotate: f64,
        HRotate: f64,
        TRotate: f64,
    ) -> i32;
    pub fn dx_SetCameraViewMatrix(ViewMatrix: Matrix) -> i32;
    pub fn dx_SetCameraViewMatrixD(ViewMatrix: DMatrix) -> i32;
    pub fn dx_SetCameraScreenCenter(x: f32, y: f32) -> i32;
    pub fn dx_SetCameraScreenCenterD(x: f64, y: f64) -> i32;
    pub fn dx_SetupCamera_Perspective(Fov: f32) -> i32;
    pub fn dx_SetupCamera_PerspectiveD(Fov: f64) -> i32;
    pub fn dx_SetupCamera_Ortho(Size: f32) -> i32;
    pub fn dx_SetupCamera_OrthoD(Size: f64) -> i32;
    pub fn dx_SetupCamera_ProjectionMatrix(ProjectionMatrix: Matrix) -> i32;
    pub fn dx_SetupCamera_ProjectionMatrixD(ProjectionMatrix: DMatrix) -> i32;
    pub fn dx_SetCameraDotAspect(DotAspect: f32) -> i32;
    pub fn dx_SetCameraDotAspectD(DotAspect: f64) -> i32;
    pub fn dx_CheckCameraViewClip(CheckPos: Vector) -> i32;
    pub fn dx_CheckCameraViewClipD(CheckPos: DVector) -> i32;
    pub fn dx_CheckCameraViewClip_Dir(CheckPos: Vector) -> i32;
    pub fn dx_CheckCameraViewClip_DirD(CheckPos: DVector) -> i32;
    pub fn dx_CheckCameraViewClip_Box(BoxPos1: Vector, BoxPos2: Vector) -> i32;
    pub fn dx_CheckCameraViewClip_BoxD(BoxPos1: DVector, BoxPos2: DVector) -> i32;
    pub fn dx_GetCameraNear() -> f32;
    pub fn dx_GetCameraNearD() -> f64;
    pub fn dx_GetCameraFar() -> f32;
    pub fn dx_GetCameraFarD() -> f64;
    pub fn dx_GetCameraPosition() -> Vector;
    pub fn dx_GetCameraPositionD() -> DVector;
    pub fn dx_GetCameraTarget() -> Vector;
    pub fn dx_GetCameraTargetD() -> DVector;
    pub fn dx_GetCameraUpVector() -> Vector;
    pub fn dx_GetCameraUpVectorD() -> DVector;
    pub fn dx_GetCameraDownVector() -> Vector;
    pub fn dx_GetCameraDownVectorD() -> DVector;
    pub fn dx_GetCameraRightVector() -> Vector;
    pub fn dx_GetCameraRightVectorD() -> DVector;
    pub fn dx_GetCameraLeftVector() -> Vector;
    pub fn dx_GetCameraLeftVectorD() -> DVector;
    pub fn dx_GetCameraFrontVector() -> Vector;
    pub fn dx_GetCameraFrontVectorD() -> DVector;
    pub fn dx_GetCameraBackVector() -> Vector;
    pub fn dx_GetCameraBackVectorD() -> DVector;
    pub fn dx_GetCameraAngleHRotate() -> f32;
    pub fn dx_GetCameraAngleHRotateD() -> f64;
    pub fn dx_GetCameraAngleVRotate() -> f32;
    pub fn dx_GetCameraAngleVRotateD() -> f64;
    pub fn dx_GetCameraAngleTRotate() -> f32;
    pub fn dx_GetCameraAngleTRotateD() -> f64;
    pub fn dx_GetCameraViewMatrix() -> Matrix;
    pub fn dx_GetCameraViewMatrixD() -> DMatrix;
    pub fn dx_GetCameraBillboardMatrix() -> Matrix;
    pub fn dx_GetCameraBillboardMatrixD() -> DMatrix;
    pub fn dx_GetCameraScreenCenter(x: *mut f32, y: *mut f32) -> i32;
    pub fn dx_GetCameraScreenCenterD(x: *mut f64, y: *mut f64) -> i32;
    pub fn dx_GetCameraFov() -> f32;
    pub fn dx_GetCameraFovD() -> f64;
    pub fn dx_GetCameraSize() -> f32;
    pub fn dx_GetCameraSizeD() -> f64;
    pub fn dx_GetCameraProjectionMatrix() -> Matrix;
    pub fn dx_GetCameraProjectionMatrixD() -> DMatrix;
    pub fn dx_GetCameraDotAspect() -> f32;
    pub fn dx_GetCameraDotAspectD() -> f64;
    pub fn dx_GetCameraViewportMatrix() -> Matrix;
    pub fn dx_GetCameraViewportMatrixD() -> DMatrix;
    pub fn dx_GetCameraAPIViewportMatrix() -> Matrix;
    pub fn dx_GetCameraAPIViewportMatrixD() -> DMatrix;
    pub fn dx_SetUseLighting(Flag: i32) -> i32;
    pub fn dx_SetMaterialUseVertDifColor(UseFlag: i32) -> i32;
    pub fn dx_SetMaterialUseVertSpcColor(UseFlag: i32) -> i32;
    pub fn dx_SetMaterialParam(Material: MaterialParam) -> i32;
    pub fn dx_SetUseSpecular(UseFlag: i32) -> i32;
    pub fn dx_SetGlobalAmbientLight(Color: ColorF32) -> i32;
    pub fn dx_ChangeLightTypeDir(Direction: Vector) -> i32;
    pub fn dx_ChangeLightTypeSpot(
        Position: Vector,
        Direction: Vector,
        OutAngle: f32,
        InAngle: f32,
        Range: f32,
        Atten0: f32,
        Atten1: f32,
        Atten2: f32,
    ) -> i32;
    pub fn dx_ChangeLightTypePoint(
        Position: Vector,
        Range: f32,
        Atten0: f32,
        Atten1: f32,
        Atten2: f32,
    ) -> i32;
    pub fn dx_GetLightType() -> i32;
    pub fn dx_SetLightEnable(EnableFlag: i32) -> i32;
    pub fn dx_GetLightEnable() -> i32;
    pub fn dx_SetLightDifColor(Color: ColorF32) -> i32;
    pub fn dx_GetLightDifColor() -> ColorF32;
    pub fn dx_SetLightSpcColor(Color: ColorF32) -> i32;
    pub fn dx_GetLightSpcColor() -> ColorF32;
    pub fn dx_SetLightAmbColor(Color: ColorF32) -> i32;
    pub fn dx_GetLightAmbColor() -> ColorF32;
    pub fn dx_SetLightDirection(Direction: Vector) -> i32;
    pub fn dx_GetLightDirection() -> Vector;
    pub fn dx_SetLightPosition(Position: Vector) -> i32;
    pub fn dx_GetLightPosition() -> Vector;
    pub fn dx_SetLightRangeAtten(Range: f32, Atten0: f32, Atten1: f32, Atten2: f32) -> i32;
    pub fn dx_GetLightRangeAtten(
        Range: *mut f32,
        Atten0: *mut f32,
        Atten1: *mut f32,
        Atten2: *mut f32,
    ) -> i32;
    pub fn dx_SetLightAngle(OutAngle: f32, InAngle: f32) -> i32;
    pub fn dx_GetLightAngle(OutAngle: *mut f32, InAngle: *mut f32) -> i32;
    pub fn dx_SetLightUseShadowMap(SmSlotIndex: i32, UseFlag: i32) -> i32;
    pub fn dx_CreateDirLightHandle(Direction: Vector) -> i32;
    pub fn dx_CreateSpotLightHandle(
        Position: Vector,
        Direction: Vector,
        OutAngle: f32,
        InAngle: f32,
        Range: f32,
        Atten0: f32,
        Atten1: f32,
        Atten2: f32,
    ) -> i32;
    pub fn dx_CreatePointLightHandle(
        Position: Vector,
        Range: f32,
        Atten0: f32,
        Atten1: f32,
        Atten2: f32,
    ) -> i32;
    pub fn dx_DeleteLightHandle(LHandle: i32) -> i32;
    pub fn dx_DeleteLightHandleAll() -> i32;
    pub fn dx_SetLightTypeHandle(LHandle: i32, LightType: i32) -> i32;
    pub fn dx_SetLightEnableHandle(LHandle: i32, EnableFlag: i32) -> i32;
    pub fn dx_SetLightDifColorHandle(LHandle: i32, Color: ColorF32) -> i32;
    pub fn dx_SetLightSpcColorHandle(LHandle: i32, Color: ColorF32) -> i32;
    pub fn dx_SetLightAmbColorHandle(LHandle: i32, Color: ColorF32) -> i32;
    pub fn dx_SetLightDirectionHandle(LHandle: i32, Direction: Vector) -> i32;
    pub fn dx_SetLightPositionHandle(LHandle: i32, Position: Vector) -> i32;
    pub fn dx_SetLightRangeAttenHandle(
        LHandle: i32,
        Range: f32,
        Atten0: f32,
        Atten1: f32,
        Atten2: f32,
    ) -> i32;
    pub fn dx_SetLightAngleHandle(LHandle: i32, OutAngle: f32, InAngle: f32) -> i32;
    pub fn dx_SetLightUseShadowMapHandle(LHandle: i32, SmSlotIndex: i32, UseFlag: i32) -> i32;
    pub fn dx_GetLightTypeHandle(LHandle: i32) -> i32;
    pub fn dx_GetLightEnableHandle(LHandle: i32) -> i32;
    pub fn dx_GetLightDifColorHandle(LHandle: i32) -> ColorF32;
    pub fn dx_GetLightSpcColorHandle(LHandle: i32) -> ColorF32;
    pub fn dx_GetLightAmbColorHandle(LHandle: i32) -> ColorF32;
    pub fn dx_GetLightDirectionHandle(LHandle: i32) -> Vector;
    pub fn dx_GetLightPositionHandle(LHandle: i32) -> Vector;
    pub fn dx_GetLightRangeAttenHandle(
        LHandle: i32,
        Range: *mut f32,
        Atten0: *mut f32,
        Atten1: *mut f32,
        Atten2: *mut f32,
    ) -> i32;
    pub fn dx_GetLightAngleHandle(LHandle: i32, OutAngle: *mut f32, InAngle: *mut f32) -> i32;
    pub fn dx_GetEnableLightHandleNum() -> i32;
    pub fn dx_GetEnableLightHandle(Index: i32) -> i32;
    pub fn dx_GetTexFormatIndex(Format: *const ImageFormatDesc) -> i32;
    pub fn dx_CreateMaskScreen() -> i32;
    pub fn dx_DeleteMaskScreen() -> i32;
    pub fn dx_DrawMaskToDirectData(
        x: i32,
        y: i32,
        Width: i32,
        Height: i32,
        MaskData: *const libc::c_void,
        TransMode: i32,
    ) -> i32;
    pub fn dx_DrawFillMaskToDirectData(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        Width: i32,
        Height: i32,
        MaskData: *const libc::c_void,
    ) -> i32;
    pub fn dx_SetUseMaskScreenFlag(ValidFlag: i32) -> i32;
    pub fn dx_GetUseMaskScreenFlag() -> i32;
    pub fn dx_FillMaskScreen(Flag: i32) -> i32;
    pub fn dx_SetMaskScreenGraph(GraphHandle: i32) -> i32;
    pub fn dx_SetMaskScreenGraphUseChannel(UseChannel: i32) -> i32;
    pub fn dx_InitMask() -> i32;
    pub fn dx_MakeMask(Width: i32, Height: i32) -> i32;
    pub fn dx_GetMaskSize(WidthBuf: *mut i32, HeightBuf: *mut i32, MaskHandle: i32) -> i32;
    pub fn dx_GetMaskBaseImageInfo(BaseImage: *mut BaseImage, MaskHandle: i32) -> i32;
    pub fn dx_SetDataToMask(
        Width: i32,
        Height: i32,
        MaskData: *const libc::c_void,
        MaskHandle: i32,
    ) -> i32;
    pub fn dx_DeleteMask(MaskHandle: i32) -> i32;
    pub fn dx_GraphImageBltToMask(
        BaseImage: *const BaseImage,
        ImageX: i32,
        ImageY: i32,
        MaskHandle: i32,
    ) -> i32;
    pub fn dx_LoadMask(FileName: *const i8) -> i32;
    pub fn dx_LoadMaskWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_LoadDivMask(
        FileName: *const i8,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        HandleArray: *mut i32,
    ) -> i32;
    pub fn dx_LoadDivMaskWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        HandleArray: *mut i32,
    ) -> i32;
    pub fn dx_CreateMaskFromMem(FileImage: *const libc::c_void, FileImageSize: i32) -> i32;
    pub fn dx_CreateDivMaskFromMem(
        FileImage: *const libc::c_void,
        FileImageSize: i32,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        HandleArray: *mut i32,
    ) -> i32;
    pub fn dx_DrawMask(x: i32, y: i32, MaskHandle: i32, TransMode: i32) -> i32;
    pub fn dx_DrawStringMask(x: i32, y: i32, Flag: i32, String: *const i8) -> i32;
    pub fn dx_DrawNStringMask(
        x: i32,
        y: i32,
        Flag: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_DrawStringMaskToHandle(
        x: i32,
        y: i32,
        Flag: i32,
        FontHandle: i32,
        String: *const i8,
    ) -> i32;
    pub fn dx_DrawNStringMaskToHandle(
        x: i32,
        y: i32,
        Flag: i32,
        FontHandle: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_DrawFillMask(x1: i32, y1: i32, x2: i32, y2: i32, MaskHandle: i32) -> i32;
    pub fn dx_SetMaskReverseEffectFlag(ReverseFlag: i32) -> i32;
    pub fn dx_GetMaskScreenData(x1: i32, y1: i32, x2: i32, y2: i32, MaskHandle: i32) -> i32;
    pub fn dx_GetMaskUseFlag() -> i32;
    pub fn dx_EnumFontName(NameBuffer: *mut u8, NameBufferNum: i32, JapanOnlyFlag: i32) -> i32;
    pub fn dx_EnumFontNameEx(NameBuffer: *mut u8, NameBufferNum: i32, CharSet: i32) -> i32;
    pub fn dx_EnumFontNameEx2(
        NameBuffer: *mut u8,
        NameBufferNum: i32,
        EnumFontName: *const i8,
        CharSet: i32,
    ) -> i32;
    pub fn dx_EnumFontNameEx2WithStrLen(
        NameBuffer: *mut u8,
        NameBufferNum: i32,
        EnumFontName: *const i8,
        EnumFontNameLength: usize,
        CharSet: i32,
    ) -> i32;
    pub fn dx_CheckFontName(FontName: *const i8, CharSet: i32) -> i32;
    pub fn dx_CheckFontNameWithStrLen(
        FontName: *const i8,
        FontNameLength: usize,
        CharSet: i32,
    ) -> i32;
    pub fn dx_InitFontToHandle() -> i32;
    pub fn dx_CreateFontToHandle(
        FontName: *const i8,
        Size: i32,
        Thick: i32,
        FontType: i32,
        CharSet: i32,
        EdgeSize: i32,
        Italic: i32,
        Handle: i32,
    ) -> i32;
    pub fn dx_CreateFontToHandleWithStrLen(
        FontName: *const i8,
        FontNameLength: usize,
        Size: i32,
        Thick: i32,
        FontType: i32,
        CharSet: i32,
        EdgeSize: i32,
        Italic: i32,
        Handle: i32,
    ) -> i32;
    pub fn dx_LoadFontDataToHandle(FileName: *const i8, EdgeSize: i32) -> i32;
    pub fn dx_LoadFontDataToHandleWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        EdgeSize: i32,
    ) -> i32;
    pub fn dx_LoadFontDataFromMemToHandle(
        FontDataImage: *const libc::c_void,
        FontDataImageSize: i32,
        EdgeSize: i32,
    ) -> i32;
    pub fn dx_SetFontSpaceToHandle(Pixel: i32, FontHandle: i32) -> i32;
    pub fn dx_SetFontLineSpaceToHandle(Pixel: i32, FontHandle: i32) -> i32;
    pub fn dx_SetFontCharCodeFormatToHandle(CharCodeFormat: i32, FontHandle: i32) -> i32;
    pub fn dx_DeleteFontToHandle(FontHandle: i32) -> i32;
    pub fn dx_SetFontLostFlag(FontHandle: i32, LostFlag: *mut i32) -> i32;
    pub fn dx_AddFontImageToHandle(
        FontHandle: i32,
        Char: *const i8,
        GrHandle: i32,
        DrawX: i32,
        DrawY: i32,
        AddX: i32,
    ) -> i32;
    pub fn dx_AddFontImageToHandleWithStrLen(
        FontHandle: i32,
        Char: *const i8,
        CharLength: usize,
        GrHandle: i32,
        DrawX: i32,
        DrawY: i32,
        AddX: i32,
    ) -> i32;
    pub fn dx_SubFontImageToHandle(FontHandle: i32, Char: *const i8) -> i32;
    pub fn dx_SubFontImageToHandleWithStrLen(
        FontHandle: i32,
        Char: *const i8,
        CharLength: usize,
    ) -> i32;
    pub fn dx_AddSubstitutionFontToHandle(
        FontHandle: i32,
        SubstitutionFontHandle: i32,
        DrawX: i32,
        DrawY: i32,
    ) -> i32;
    pub fn dx_SubSubstitutionFontToHandle(FontHandle: i32, SubstitutionFontHandle: i32) -> i32;
    pub fn dx_ChangeFont(FontName: *const i8, CharSet: i32) -> i32;
    pub fn dx_ChangeFontWithStrLen(FontName: *const i8, FontNameLength: usize, CharSet: i32)
        -> i32;
    pub fn dx_ChangeFontType(FontType: i32) -> i32;
    pub fn dx_SetFontSize(FontSize: i32) -> i32;
    pub fn dx_GetFontSize() -> i32;
    pub fn dx_GetFontEdgeSize() -> i32;
    pub fn dx_SetFontThickness(ThickPal: i32) -> i32;
    pub fn dx_SetFontSpace(Pixel: i32) -> i32;
    pub fn dx_GetFontSpace() -> i32;
    pub fn dx_SetFontLineSpace(Pixel: i32) -> i32;
    pub fn dx_GetFontLineSpace() -> i32;
    pub fn dx_SetFontCharCodeFormat(CharCodeFormat: i32) -> i32;
    pub fn dx_SetDefaultFontState(
        FontName: *const i8,
        Size: i32,
        Thick: i32,
        FontType: i32,
        CharSet: i32,
        EdgeSize: i32,
        Italic: i32,
    ) -> i32;
    pub fn dx_SetDefaultFontStateWithStrLen(
        FontName: *const i8,
        FontNameLength: usize,
        Size: i32,
        Thick: i32,
        FontType: i32,
        CharSet: i32,
        EdgeSize: i32,
        Italic: i32,
    ) -> i32;
    pub fn dx_GetDefaultFontHandle() -> i32;
    pub fn dx_GetFontMaxCacheCharNum() -> i32;
    pub fn dx_GetFontMaxWidth() -> i32;
    pub fn dx_GetFontAscent() -> i32;
    pub fn dx_GetDrawStringWidth(String: *const i8, StrLen: i32, VerticalFlag: i32) -> i32;
    pub fn dx_GetDrawNStringWidth(String: *const i8, StringLength: usize, VerticalFlag: i32)
        -> i32;
    pub fn dx_GetDrawExtendStringWidth(
        ExRateX: f64,
        String: *const i8,
        StrLen: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawExtendNStringWidth(
        ExRateX: f64,
        String: *const i8,
        StringLength: usize,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawStringSize(
        SizeX: *mut i32,
        SizeY: *mut i32,
        LineCount: *mut i32,
        String: *const i8,
        StrLen: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawNStringSize(
        SizeX: *mut i32,
        SizeY: *mut i32,
        LineCount: *mut i32,
        String: *const i8,
        StringLength: usize,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawExtendStringSize(
        SizeX: *mut i32,
        SizeY: *mut i32,
        LineCount: *mut i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StrLen: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawExtendNStringSize(
        SizeX: *mut i32,
        SizeY: *mut i32,
        LineCount: *mut i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawStringCharInfo(
        InfoBuffer: *mut DrawCharInfo,
        InfoBufferSize: usize,
        String: *const i8,
        StrLen: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawNStringCharInfo(
        InfoBuffer: *mut DrawCharInfo,
        InfoBufferSize: usize,
        String: *const i8,
        StringLength: usize,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawFormatStringCharInfo(
        InfoBuffer: *mut DrawCharInfo,
        InfoBufferSize: usize,
        FormatString: *const i8,
    ) -> i32;
    pub fn dx_GetDrawExtendStringCharInfo(
        InfoBuffer: *mut DrawCharInfo,
        InfoBufferSize: usize,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StrLen: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawExtendNStringCharInfo(
        InfoBuffer: *mut DrawCharInfo,
        InfoBufferSize: usize,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawExtendFormatStringCharInfo(
        InfoBuffer: *mut DrawCharInfo,
        InfoBufferSize: usize,
        ExRateX: f64,
        ExRateY: f64,
        FormatString: *const i8,
    ) -> i32;
    pub fn dx_GetDrawStringKerningPairInfo(PairChar: *const i8, KernAmount: *mut i32) -> i32;
    pub fn dx_GetDrawStringKerningPairInfoWithStrLen(
        PairChar: *const i8,
        PairCharLength: usize,
        KernAmount: *mut i32,
    ) -> i32;
    pub fn dx_GetFontMaxCacheCharNumToHandle(FontHandle: i32) -> i32;
    pub fn dx_GetFontMaxWidthToHandle(FontHandle: i32) -> i32;
    pub fn dx_GetFontAscentToHandle(FontHandle: i32) -> i32;
    pub fn dx_GetFontSizeToHandle(FontHandle: i32) -> i32;
    pub fn dx_GetFontEdgeSizeToHandle(FontHandle: i32) -> i32;
    pub fn dx_GetFontSpaceToHandle(FontHandle: i32) -> i32;
    pub fn dx_GetFontLineSpaceToHandle(FontHandle: i32) -> i32;
    pub fn dx_GetFontCharInfo(
        FontHandle: i32,
        Char: *const i8,
        DrawX: *mut i32,
        DrawY: *mut i32,
        NextCharX: *mut i32,
        SizeX: *mut i32,
        SizeY: *mut i32,
    ) -> i32;
    pub fn dx_GetFontCharInfoWithStrLen(
        FontHandle: i32,
        Char: *const i8,
        CharLength: usize,
        DrawX: *mut i32,
        DrawY: *mut i32,
        NextCharX: *mut i32,
        SizeX: *mut i32,
        SizeY: *mut i32,
    ) -> i32;
    pub fn dx_GetDrawStringWidthToHandle(
        String: *const i8,
        StrLen: i32,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawNStringWidthToHandle(
        String: *const i8,
        StringLength: usize,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawExtendStringWidthToHandle(
        ExRateX: f64,
        String: *const i8,
        StrLen: i32,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawExtendNStringWidthToHandle(
        ExRateX: f64,
        String: *const i8,
        StringLength: usize,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawStringSizeToHandle(
        SizeX: *mut i32,
        SizeY: *mut i32,
        LineCount: *mut i32,
        String: *const i8,
        StrLen: i32,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawNStringSizeToHandle(
        SizeX: *mut i32,
        SizeY: *mut i32,
        LineCount: *mut i32,
        String: *const i8,
        StringLength: usize,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawExtendStringSizeToHandle(
        SizeX: *mut i32,
        SizeY: *mut i32,
        LineCount: *mut i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StrLen: i32,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawExtendNStringSizeToHandle(
        SizeX: *mut i32,
        SizeY: *mut i32,
        LineCount: *mut i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawStringCharInfoToHandle(
        InfoBuffer: *mut DrawCharInfo,
        InfoBufferSize: usize,
        String: *const i8,
        StrLen: i32,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawNStringCharInfoToHandle(
        InfoBuffer: *mut DrawCharInfo,
        InfoBufferSize: usize,
        String: *const i8,
        StringLength: usize,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawFormatStringCharInfoToHandle(
        InfoBuffer: *mut DrawCharInfo,
        InfoBufferSize: usize,
        FontHandle: i32,
        FormatString: *const i8,
    ) -> i32;
    pub fn dx_GetDrawExtendStringCharInfoToHandle(
        InfoBuffer: *mut DrawCharInfo,
        InfoBufferSize: usize,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StrLen: i32,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawExtendNStringCharInfoToHandle(
        InfoBuffer: *mut DrawCharInfo,
        InfoBufferSize: usize,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_GetDrawExtendFormatStringCharInfoToHandle(
        InfoBuffer: *mut DrawCharInfo,
        InfoBufferSize: usize,
        ExRateX: f64,
        ExRateY: f64,
        FontHandle: i32,
        FormatString: *const i8,
    ) -> i32;
    pub fn dx_GetDrawStringKerningPairInfoToHandle(
        PairChar: *const i8,
        KernAmount: *mut i32,
        FontHandle: i32,
    ) -> i32;
    pub fn dx_GetDrawStringKerningPairInfoToHandleWithStrLen(
        PairChar: *const i8,
        PairCharLength: usize,
        KernAmount: *mut i32,
        FontHandle: i32,
    ) -> i32;
    pub fn dx_GetFontStateToHandle(
        FontName: *mut u8,
        Size: *mut i32,
        Thick: *mut i32,
        FontHandle: i32,
        FontType: *mut i32,
        CharSet: *mut i32,
        EdgeSize: *mut i32,
        Italic: *mut i32,
    ) -> i32;
    pub fn dx_CheckFontCacheToTextureFlag(FontHandle: i32) -> i32;
    pub fn dx_CheckFontChacheToTextureFlag(FontHandle: i32) -> i32;
    pub fn dx_CheckFontHandleValid(FontHandle: i32) -> i32;
    pub fn dx_ClearFontCacheToHandle(FontHandle: i32) -> i32;
    pub fn dx_SetFontCacheToTextureFlag(Flag: i32) -> i32;
    pub fn dx_GetFontCacheToTextureFlag() -> i32;
    pub fn dx_SetFontChacheToTextureFlag(Flag: i32) -> i32;
    pub fn dx_GetFontChacheToTextureFlag() -> i32;
    pub fn dx_SetFontCacheTextureColorBitDepth(ColorBitDepth: i32) -> i32;
    pub fn dx_GetFontCacheTextureColorBitDepth() -> i32;
    pub fn dx_SetFontCacheCharNum(CharNum: i32) -> i32;
    pub fn dx_GetFontCacheCharNum() -> i32;
    pub fn dx_SetFontCacheUsePremulAlphaFlag(Flag: i32) -> i32;
    pub fn dx_GetFontCacheUsePremulAlphaFlag() -> i32;
    pub fn dx_SetFontUseAdjustSizeFlag(Flag: i32) -> i32;
    pub fn dx_GetFontUseAdjustSizeFlag() -> i32;
    pub fn dx_FontCacheStringDrawToHandle(
        x: i32,
        y: i32,
        StrData: *const i8,
        Color: u32,
        EdgeColor: u32,
        DestImage: *mut BaseImage,
        ClipRect: *const Rect,
        FontHandle: i32,
        VerticalFlag: i32,
        DrawSizeP: *mut usize,
    ) -> i32;
    pub fn dx_FontCacheStringDrawToHandleWithStrLen(
        x: i32,
        y: i32,
        StrData: *const i8,
        StrDataLength: usize,
        Color: u32,
        EdgeColor: u32,
        DestImage: *mut BaseImage,
        ClipRect: *const Rect,
        FontHandle: i32,
        VerticalFlag: i32,
        DrawSizeP: *mut usize,
    ) -> i32;
    pub fn dx_FontBaseImageBlt(
        x: i32,
        y: i32,
        StrData: *const i8,
        DestImage: *mut BaseImage,
        DestEdgeImage: *mut BaseImage,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_FontBaseImageBltWithStrLen(
        x: i32,
        y: i32,
        StrData: *const i8,
        StrDataLength: usize,
        DestImage: *mut BaseImage,
        DestEdgeImage: *mut BaseImage,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_FontBaseImageBltToHandle(
        x: i32,
        y: i32,
        StrData: *const i8,
        DestImage: *mut BaseImage,
        DestEdgeImage: *mut BaseImage,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_FontBaseImageBltToHandleWithStrLen(
        x: i32,
        y: i32,
        StrData: *const i8,
        StrDataLength: usize,
        DestImage: *mut BaseImage,
        DestEdgeImage: *mut BaseImage,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_MultiByteCharCheck(Buf: *const i8, CharSet: i32) -> i32;
    pub fn dx_DrawString(x: i32, y: i32, String: *const i8, Color: u32, EdgeColor: u32) -> i32;
    pub fn dx_DrawNString(
        x: i32,
        y: i32,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawVString(x: i32, y: i32, String: *const i8, Color: u32, EdgeColor: u32) -> i32;
    pub fn dx_DrawNVString(
        x: i32,
        y: i32,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawExtendString(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawExtendNString(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawExtendVString(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawExtendNVString(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawRotaString(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        RotCenterX: f64,
        RotCenterY: f64,
        RotAngle: f64,
        Color: u32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
    ) -> i32;
    pub fn dx_DrawRotaNString(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        RotCenterX: f64,
        RotCenterY: f64,
        RotAngle: f64,
        Color: u32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_DrawModiString(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        x4: i32,
        y4: i32,
        Color: u32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
    ) -> i32;
    pub fn dx_DrawModiNString(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        x4: i32,
        y4: i32,
        Color: u32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_DrawStringF(x: f32, y: f32, String: *const i8, Color: u32, EdgeColor: u32) -> i32;
    pub fn dx_DrawNStringF(
        x: f32,
        y: f32,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawVStringF(x: f32, y: f32, String: *const i8, Color: u32, EdgeColor: u32) -> i32;
    pub fn dx_DrawNVStringF(
        x: f32,
        y: f32,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawExtendStringF(
        x: f32,
        y: f32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawExtendNStringF(
        x: f32,
        y: f32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawExtendVStringF(
        x: f32,
        y: f32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawExtendNVStringF(
        x: f32,
        y: f32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawRotaStringF(
        x: f32,
        y: f32,
        ExRateX: f64,
        ExRateY: f64,
        RotCenterX: f64,
        RotCenterY: f64,
        RotAngle: f64,
        Color: u32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
    ) -> i32;
    pub fn dx_DrawRotaNStringF(
        x: f32,
        y: f32,
        ExRateX: f64,
        ExRateY: f64,
        RotCenterX: f64,
        RotCenterY: f64,
        RotAngle: f64,
        Color: u32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_DrawModiStringF(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
        Color: u32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
    ) -> i32;
    pub fn dx_DrawModiNStringF(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
        Color: u32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_DrawNumberToI(
        x: i32,
        y: i32,
        Num: i32,
        RisesNum: i32,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawNumberToF(
        x: i32,
        y: i32,
        Num: f64,
        Length: i32,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawNumberPlusToI(
        x: i32,
        y: i32,
        NoteString: *const i8,
        Num: i32,
        RisesNum: i32,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawNumberPlusToF(
        x: i32,
        y: i32,
        NoteString: *const i8,
        Num: f64,
        Length: i32,
        Color: u32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawStringToZBuffer(x: i32, y: i32, String: *const i8, WriteZMode: i32) -> i32;
    pub fn dx_DrawNStringToZBuffer(
        x: i32,
        y: i32,
        String: *const i8,
        StringLength: usize,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawVStringToZBuffer(x: i32, y: i32, String: *const i8, WriteZMode: i32) -> i32;
    pub fn dx_DrawNVStringToZBuffer(
        x: i32,
        y: i32,
        String: *const i8,
        StringLength: usize,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawExtendStringToZBuffer(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawExtendNStringToZBuffer(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawExtendVStringToZBuffer(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawExtendNVStringToZBuffer(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawRotaStringToZBuffer(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        RotCenterX: f64,
        RotCenterY: f64,
        RotAngle: f64,
        WriteZMode: i32,
        VerticalFlag: i32,
        String: *const i8,
    ) -> i32;
    pub fn dx_DrawRotaNStringToZBuffer(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        RotCenterX: f64,
        RotCenterY: f64,
        RotAngle: f64,
        WriteZMode: i32,
        VerticalFlag: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_DrawModiStringToZBuffer(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        x4: i32,
        y4: i32,
        WriteZMode: i32,
        VerticalFlag: i32,
        String: *const i8,
    ) -> i32;
    pub fn dx_DrawModiNStringToZBuffer(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        x4: i32,
        y4: i32,
        WriteZMode: i32,
        VerticalFlag: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_DrawStringToHandle(
        x: i32,
        y: i32,
        String: *const i8,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_DrawNStringToHandle(
        x: i32,
        y: i32,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_DrawVStringToHandle(
        x: i32,
        y: i32,
        String: *const i8,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawNVStringToHandle(
        x: i32,
        y: i32,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawExtendStringToHandle(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_DrawExtendNStringToHandle(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_DrawExtendVStringToHandle(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawExtendNVStringToHandle(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawRotaStringToHandle(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        RotCenterX: f64,
        RotCenterY: f64,
        RotAngle: f64,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
    ) -> i32;
    pub fn dx_DrawRotaNStringToHandle(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        RotCenterX: f64,
        RotCenterY: f64,
        RotAngle: f64,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_DrawModiStringToHandle(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        x4: i32,
        y4: i32,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
    ) -> i32;
    pub fn dx_DrawModiNStringToHandle(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        x4: i32,
        y4: i32,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_DrawStringFToHandle(
        x: f32,
        y: f32,
        String: *const i8,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_DrawNStringFToHandle(
        x: f32,
        y: f32,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_DrawVStringFToHandle(
        x: f32,
        y: f32,
        String: *const i8,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawNVStringFToHandle(
        x: f32,
        y: f32,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawExtendStringFToHandle(
        x: f32,
        y: f32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_DrawExtendNStringFToHandle(
        x: f32,
        y: f32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_DrawExtendVStringFToHandle(
        x: f32,
        y: f32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawExtendNVStringFToHandle(
        x: f32,
        y: f32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawRotaStringFToHandle(
        x: f32,
        y: f32,
        ExRateX: f64,
        ExRateY: f64,
        RotCenterX: f64,
        RotCenterY: f64,
        RotAngle: f64,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
    ) -> i32;
    pub fn dx_DrawRotaNStringFToHandle(
        x: f32,
        y: f32,
        ExRateX: f64,
        ExRateY: f64,
        RotCenterX: f64,
        RotCenterY: f64,
        RotAngle: f64,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_DrawModiStringFToHandle(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
    ) -> i32;
    pub fn dx_DrawModiNStringFToHandle(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
        VerticalFlag: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_DrawNumberToIToHandle(
        x: i32,
        y: i32,
        Num: i32,
        RisesNum: i32,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawNumberToFToHandle(
        x: i32,
        y: i32,
        Num: f64,
        Length: i32,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawNumberPlusToIToHandle(
        x: i32,
        y: i32,
        NoteString: *const i8,
        Num: i32,
        RisesNum: i32,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawNumberPlusToFToHandle(
        x: i32,
        y: i32,
        NoteString: *const i8,
        Num: f64,
        Length: i32,
        Color: u32,
        FontHandle: i32,
        EdgeColor: u32,
    ) -> i32;
    pub fn dx_DrawStringToHandleToZBuffer(
        x: i32,
        y: i32,
        String: *const i8,
        FontHandle: i32,
        WriteZMode: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_DrawNStringToHandleToZBuffer(
        x: i32,
        y: i32,
        String: *const i8,
        StringLength: usize,
        FontHandle: i32,
        WriteZMode: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_DrawVStringToHandleToZBuffer(
        x: i32,
        y: i32,
        String: *const i8,
        FontHandle: i32,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawNVStringToHandleToZBuffer(
        x: i32,
        y: i32,
        String: *const i8,
        StringLength: usize,
        FontHandle: i32,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawExtendStringToHandleToZBuffer(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        FontHandle: i32,
        WriteZMode: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_DrawExtendNStringToHandleToZBuffer(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        FontHandle: i32,
        WriteZMode: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_DrawExtendVStringToHandleToZBuffer(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        FontHandle: i32,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawExtendNVStringToHandleToZBuffer(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        String: *const i8,
        StringLength: usize,
        FontHandle: i32,
        WriteZMode: i32,
    ) -> i32;
    pub fn dx_DrawRotaStringToHandleToZBuffer(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        RotCenterX: f64,
        RotCenterY: f64,
        RotAngle: f64,
        FontHandle: i32,
        WriteZMode: i32,
        VerticalFlag: i32,
        String: *const i8,
    ) -> i32;
    pub fn dx_DrawRotaNStringToHandleToZBuffer(
        x: i32,
        y: i32,
        ExRateX: f64,
        ExRateY: f64,
        RotCenterX: f64,
        RotCenterY: f64,
        RotAngle: f64,
        FontHandle: i32,
        WriteZMode: i32,
        VerticalFlag: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_DrawModiStringToHandleToZBuffer(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        x4: i32,
        y4: i32,
        FontHandle: i32,
        WriteZMode: i32,
        VerticalFlag: i32,
        String: *const i8,
    ) -> i32;
    pub fn dx_DrawModiNStringToHandleToZBuffer(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        x4: i32,
        y4: i32,
        FontHandle: i32,
        WriteZMode: i32,
        VerticalFlag: i32,
        String: *const i8,
        StringLength: usize,
    ) -> i32;
    pub fn dx_ConvertMatrixFtoD(Out: *mut DMatrix, In: *const Matrix) -> i32;
    pub fn dx_ConvertMatrixDtoF(Out: *mut Matrix, In: *const DMatrix) -> i32;
    pub fn dx_CreateIdentityMatrix(Out: *mut Matrix) -> i32;
    pub fn dx_CreateIdentityMatrixD(Out: *mut DMatrix) -> i32;
    pub fn dx_CreateLookAtMatrix(
        Out: *mut Matrix,
        Eye: *const Vector,
        At: *const Vector,
        Up: *const Vector,
    ) -> i32;
    pub fn dx_CreateLookAtMatrixD(
        Out: *mut DMatrix,
        Eye: *const DVector,
        At: *const DVector,
        Up: *const DVector,
    ) -> i32;
    pub fn dx_CreateLookAtMatrix2(
        Out: *mut Matrix,
        Eye: *const Vector,
        XZAngle: f64,
        Oira: f64,
    ) -> i32;
    pub fn dx_CreateLookAtMatrix2D(
        Out: *mut DMatrix,
        Eye: *const DVector,
        XZAngle: f64,
        Oira: f64,
    ) -> i32;
    pub fn dx_CreateLookAtMatrixRH(
        Out: *mut Matrix,
        Eye: *const Vector,
        At: *const Vector,
        Up: *const Vector,
    ) -> i32;
    pub fn dx_CreateLookAtMatrixRHD(
        Out: *mut DMatrix,
        Eye: *const DVector,
        At: *const DVector,
        Up: *const DVector,
    ) -> i32;
    pub fn dx_CreateMultiplyMatrix(Out: *mut Matrix, In1: *const Matrix, In2: *const Matrix)
        -> i32;
    pub fn dx_CreateMultiplyMatrixD(
        Out: *mut DMatrix,
        In1: *const DMatrix,
        In2: *const DMatrix,
    ) -> i32;
    pub fn dx_CreatePerspectiveFovMatrix(
        Out: *mut Matrix,
        fov: f32,
        zn: f32,
        zf: f32,
        aspect: f32,
    ) -> i32;
    pub fn dx_CreatePerspectiveFovMatrixD(
        Out: *mut DMatrix,
        fov: f64,
        zn: f64,
        zf: f64,
        aspect: f64,
    ) -> i32;
    pub fn dx_CreatePerspectiveFovMatrixRH(
        Out: *mut Matrix,
        fov: f32,
        zn: f32,
        zf: f32,
        aspect: f32,
    ) -> i32;
    pub fn dx_CreatePerspectiveFovMatrixRHD(
        Out: *mut DMatrix,
        fov: f64,
        zn: f64,
        zf: f64,
        aspect: f64,
    ) -> i32;
    pub fn dx_CreateOrthoMatrix(Out: *mut Matrix, size: f32, zn: f32, zf: f32, aspect: f32) -> i32;
    pub fn dx_CreateOrthoMatrixD(
        Out: *mut DMatrix,
        size: f64,
        zn: f64,
        zf: f64,
        aspect: f64,
    ) -> i32;
    pub fn dx_CreateOrthoMatrixRH(
        Out: *mut Matrix,
        size: f32,
        zn: f32,
        zf: f32,
        aspect: f32,
    ) -> i32;
    pub fn dx_CreateOrthoMatrixRHD(
        Out: *mut DMatrix,
        size: f64,
        zn: f64,
        zf: f64,
        aspect: f64,
    ) -> i32;
    pub fn dx_CreateScalingMatrix(Out: *mut Matrix, sx: f32, sy: f32, sz: f32) -> i32;
    pub fn dx_CreateScalingMatrixD(Out: *mut DMatrix, sx: f64, sy: f64, sz: f64) -> i32;
    pub fn dx_CreateRotationXMatrix(Out: *mut Matrix, Angle: f32) -> i32;
    pub fn dx_CreateRotationXMatrixD(Out: *mut DMatrix, Angle: f64) -> i32;
    pub fn dx_CreateRotationYMatrix(Out: *mut Matrix, Angle: f32) -> i32;
    pub fn dx_CreateRotationYMatrixD(Out: *mut DMatrix, Angle: f64) -> i32;
    pub fn dx_CreateRotationZMatrix(Out: *mut Matrix, Angle: f32) -> i32;
    pub fn dx_CreateRotationZMatrixD(Out: *mut DMatrix, Angle: f64) -> i32;
    pub fn dx_CreateTranslationMatrix(Out: *mut Matrix, x: f32, y: f32, z: f32) -> i32;
    pub fn dx_CreateTranslationMatrixD(Out: *mut DMatrix, x: f64, y: f64, z: f64) -> i32;
    pub fn dx_CreateTransposeMatrix(Out: *mut Matrix, In: *const Matrix) -> i32;
    pub fn dx_CreateTransposeMatrixD(Out: *mut DMatrix, In: *const DMatrix) -> i32;
    pub fn dx_CreateInverseMatrix(Out: *mut Matrix, In: *const Matrix) -> i32;
    pub fn dx_CreateInverseMatrixD(Out: *mut DMatrix, In: *const DMatrix) -> i32;
    pub fn dx_CreateViewportMatrix(
        Out: *mut Matrix,
        CenterX: f32,
        CenterY: f32,
        Width: f32,
        Height: f32,
    ) -> i32;
    pub fn dx_CreateViewportMatrixD(
        Out: *mut DMatrix,
        CenterX: f64,
        CenterY: f64,
        Width: f64,
        Height: f64,
    ) -> i32;
    pub fn dx_CreateRotationXYZMatrix(Out: *mut Matrix, XRot: f32, YRot: f32, ZRot: f32) -> i32;
    pub fn dx_CreateRotationXYZMatrixD(Out: *mut DMatrix, XRot: f64, YRot: f64, ZRot: f64) -> i32;
    pub fn dx_CreateRotationXZYMatrix(Out: *mut Matrix, XRot: f32, YRot: f32, ZRot: f32) -> i32;
    pub fn dx_CreateRotationXZYMatrixD(Out: *mut DMatrix, XRot: f64, YRot: f64, ZRot: f64) -> i32;
    pub fn dx_CreateRotationYXZMatrix(Out: *mut Matrix, XRot: f32, YRot: f32, ZRot: f32) -> i32;
    pub fn dx_CreateRotationYXZMatrixD(Out: *mut DMatrix, XRot: f64, YRot: f64, ZRot: f64) -> i32;
    pub fn dx_CreateRotationYZXMatrix(Out: *mut Matrix, XRot: f32, YRot: f32, ZRot: f32) -> i32;
    pub fn dx_CreateRotationYZXMatrixD(Out: *mut DMatrix, XRot: f64, YRot: f64, ZRot: f64) -> i32;
    pub fn dx_CreateRotationZXYMatrix(Out: *mut Matrix, XRot: f32, YRot: f32, ZRot: f32) -> i32;
    pub fn dx_CreateRotationZXYMatrixD(Out: *mut DMatrix, XRot: f64, YRot: f64, ZRot: f64) -> i32;
    pub fn dx_CreateRotationZYXMatrix(Out: *mut Matrix, XRot: f32, YRot: f32, ZRot: f32) -> i32;
    pub fn dx_CreateRotationZYXMatrixD(Out: *mut DMatrix, XRot: f64, YRot: f64, ZRot: f64) -> i32;
    pub fn dx_GetMatrixXYZRotation(
        In: *const Matrix,
        OutXRot: *mut f32,
        OutYRot: *mut f32,
        OutZRot: *mut f32,
    ) -> i32;
    pub fn dx_GetMatrixXYZRotationD(
        In: *const DMatrix,
        OutXRot: *mut f64,
        OutYRot: *mut f64,
        OutZRot: *mut f64,
    ) -> i32;
    pub fn dx_GetMatrixXZYRotation(
        In: *const Matrix,
        OutXRot: *mut f32,
        OutYRot: *mut f32,
        OutZRot: *mut f32,
    ) -> i32;
    pub fn dx_GetMatrixXZYRotationD(
        In: *const DMatrix,
        OutXRot: *mut f64,
        OutYRot: *mut f64,
        OutZRot: *mut f64,
    ) -> i32;
    pub fn dx_GetMatrixYXZRotation(
        In: *const Matrix,
        OutXRot: *mut f32,
        OutYRot: *mut f32,
        OutZRot: *mut f32,
    ) -> i32;
    pub fn dx_GetMatrixYXZRotationD(
        In: *const DMatrix,
        OutXRot: *mut f64,
        OutYRot: *mut f64,
        OutZRot: *mut f64,
    ) -> i32;
    pub fn dx_GetMatrixYZXRotation(
        In: *const Matrix,
        OutXRot: *mut f32,
        OutYRot: *mut f32,
        OutZRot: *mut f32,
    ) -> i32;
    pub fn dx_GetMatrixYZXRotationD(
        In: *const DMatrix,
        OutXRot: *mut f64,
        OutYRot: *mut f64,
        OutZRot: *mut f64,
    ) -> i32;
    pub fn dx_GetMatrixZXYRotation(
        In: *const Matrix,
        OutXRot: *mut f32,
        OutYRot: *mut f32,
        OutZRot: *mut f32,
    ) -> i32;
    pub fn dx_GetMatrixZXYRotationD(
        In: *const DMatrix,
        OutXRot: *mut f64,
        OutYRot: *mut f64,
        OutZRot: *mut f64,
    ) -> i32;
    pub fn dx_GetMatrixZYXRotation(
        In: *const Matrix,
        OutXRot: *mut f32,
        OutYRot: *mut f32,
        OutZRot: *mut f32,
    ) -> i32;
    pub fn dx_GetMatrixZYXRotationD(
        In: *const DMatrix,
        OutXRot: *mut f64,
        OutYRot: *mut f64,
        OutZRot: *mut f64,
    ) -> i32;
    pub fn dx_VectorConvertFtoD(Out: *mut DVector, In: *const Vector) -> i32;
    pub fn dx_VectorConvertDtoF(Out: *mut Vector, In: *const DVector) -> i32;
    pub fn dx_VectorNormalize(Out: *mut Vector, In: *const Vector) -> i32;
    pub fn dx_VectorNormalizeD(Out: *mut DVector, In: *const DVector) -> i32;
    pub fn dx_VectorScale(Out: *mut Vector, In: *const Vector, Scale: f32) -> i32;
    pub fn dx_VectorScaleD(Out: *mut DVector, In: *const DVector, Scale: f64) -> i32;
    pub fn dx_VectorMultiply(Out: *mut Vector, In1: *const Vector, In2: *const Vector) -> i32;
    pub fn dx_VectorMultiplyD(Out: *mut DVector, In1: *const DVector, In2: *const DVector) -> i32;
    pub fn dx_VectorSub(Out: *mut Vector, In1: *const Vector, In2: *const Vector) -> i32;
    pub fn dx_VectorSubD(Out: *mut DVector, In1: *const DVector, In2: *const DVector) -> i32;
    pub fn dx_VectorAdd(Out: *mut Vector, In1: *const Vector, In2: *const Vector) -> i32;
    pub fn dx_VectorAddD(Out: *mut DVector, In1: *const DVector, In2: *const DVector) -> i32;
    pub fn dx_VectorOuterProduct(Out: *mut Vector, In1: *const Vector, In2: *const Vector) -> i32;
    pub fn dx_VectorOuterProductD(
        Out: *mut DVector,
        In1: *const DVector,
        In2: *const DVector,
    ) -> i32;
    pub fn dx_VectorInnerProduct(In1: *const Vector, In2: *const Vector) -> f32;
    pub fn dx_VectorInnerProductD(In1: *const DVector, In2: *const DVector) -> f64;
    pub fn dx_VectorRotationX(Out: *mut Vector, In: *const Vector, Angle: f64) -> i32;
    pub fn dx_VectorRotationXD(Out: *mut DVector, In: *const DVector, Angle: f64) -> i32;
    pub fn dx_VectorRotationY(Out: *mut Vector, In: *const Vector, Angle: f64) -> i32;
    pub fn dx_VectorRotationYD(Out: *mut DVector, In: *const DVector, Angle: f64) -> i32;
    pub fn dx_VectorRotationZ(Out: *mut Vector, In: *const Vector, Angle: f64) -> i32;
    pub fn dx_VectorRotationZD(Out: *mut DVector, In: *const DVector, Angle: f64) -> i32;
    pub fn dx_VectorTransform(
        Out: *mut Vector,
        InVec: *const Vector,
        InMatrix: *const Matrix,
    ) -> i32;
    pub fn dx_VectorTransformD(
        Out: *mut DVector,
        InVec: *const DVector,
        InMatrix: *const DMatrix,
    ) -> i32;
    pub fn dx_VectorTransformSR(
        Out: *mut Vector,
        InVec: *const Vector,
        InMatrix: *const Matrix,
    ) -> i32;
    pub fn dx_VectorTransformSRD(
        Out: *mut DVector,
        InVec: *const DVector,
        InMatrix: *const DMatrix,
    ) -> i32;
    pub fn dx_VectorTransform4(
        Out: *mut Vector,
        V4Out: *mut f32,
        InVec: *const Vector,
        V4In: *const f32,
        InMatrix: *const Matrix,
    ) -> i32;
    pub fn dx_VectorTransform4D(
        Out: *mut DVector,
        V4Out: *mut f64,
        InVec: *const DVector,
        V4In: *const f64,
        InMatrix: *const DMatrix,
    ) -> i32;
    pub fn dx_Segment_Segment_Analyse(
        SegmentAPos1: *const Vector,
        SegmentAPos2: *const Vector,
        SegmentBPos1: *const Vector,
        SegmentBPos2: *const Vector,
        Result: *mut SegmentSegmentResult,
    ) -> i32;
    pub fn dx_Segment_Segment_AnalyseD(
        SegmentAPos1: *const DVector,
        SegmentAPos2: *const DVector,
        SegmentBPos1: *const DVector,
        SegmentBPos2: *const DVector,
        Result: *mut SegmentSegmentResultDouble,
    ) -> i32;
    pub fn dx_Segment_Point_Analyse(
        SegmentPos1: *const Vector,
        SegmentPos2: *const Vector,
        PointPos: *const Vector,
        Result: *mut SegmentPointResult,
    ) -> i32;
    pub fn dx_Segment_Point_AnalyseD(
        SegmentPos1: *const DVector,
        SegmentPos2: *const DVector,
        PointPos: *const DVector,
        Result: *mut SegmentPointResultDouble,
    ) -> i32;
    pub fn dx_Segment_Triangle_Analyse(
        SegmentPos1: *const Vector,
        SegmentPos2: *const Vector,
        TrianglePos1: *const Vector,
        TrianglePos2: *const Vector,
        TrianglePos3: *const Vector,
        Result: *mut SegmentTriangleResult,
    ) -> i32;
    pub fn dx_Segment_Triangle_AnalyseD(
        SegmentPos1: *const DVector,
        SegmentPos2: *const DVector,
        TrianglePos1: *const DVector,
        TrianglePos2: *const DVector,
        TrianglePos3: *const DVector,
        Result: *mut SegmentTriangleResultDouble,
    ) -> i32;
    pub fn dx_Triangle_Point_Analyse(
        TrianglePos1: *const Vector,
        TrianglePos2: *const Vector,
        TrianglePos3: *const Vector,
        PointPos: *const Vector,
        Result: *mut TrianglePointResult,
    ) -> i32;
    pub fn dx_Triangle_Point_AnalyseD(
        TrianglePos1: *const DVector,
        TrianglePos2: *const DVector,
        TrianglePos3: *const DVector,
        PointPos: *const DVector,
        Result: *mut TrianglePointResultDouble,
    ) -> i32;
    pub fn dx_Plane_Point_Analyse(
        PlanePos: *const Vector,
        PlaneNormal: *const Vector,
        PointPos: *const Vector,
        Result: *mut PlanePointResult,
    ) -> i32;
    pub fn dx_Plane_Point_AnalyseD(
        PlanePos: *const DVector,
        PlaneNormal: *const DVector,
        PointPos: *const DVector,
        Result: *mut PlanePointResultDouble,
    ) -> i32;
    pub fn dx_TriangleBarycenter(
        TrianglePos1: Vector,
        TrianglePos2: Vector,
        TrianglePos3: Vector,
        Position: Vector,
        TrianglePos1Weight: *mut f32,
        TrianglePos2Weight: *mut f32,
        TrianglePos3Weight: *mut f32,
    ) -> ();
    pub fn dx_TriangleBarycenterD(
        TrianglePos1: DVector,
        TrianglePos2: DVector,
        TrianglePos3: DVector,
        Position: DVector,
        TrianglePos1Weight: *mut f64,
        TrianglePos2Weight: *mut f64,
        TrianglePos3Weight: *mut f64,
    ) -> ();
    pub fn dx_Segment_Segment_MinLength(
        SegmentAPos1: Vector,
        SegmentAPos2: Vector,
        SegmentBPos1: Vector,
        SegmentBPos2: Vector,
    ) -> f32;
    pub fn dx_Segment_Segment_MinLengthD(
        SegmentAPos1: DVector,
        SegmentAPos2: DVector,
        SegmentBPos1: DVector,
        SegmentBPos2: DVector,
    ) -> f64;
    pub fn dx_Segment_Segment_MinLength_Square(
        SegmentAPos1: Vector,
        SegmentAPos2: Vector,
        SegmentBPos1: Vector,
        SegmentBPos2: Vector,
    ) -> f32;
    pub fn dx_Segment_Segment_MinLength_SquareD(
        SegmentAPos1: DVector,
        SegmentAPos2: DVector,
        SegmentBPos1: DVector,
        SegmentBPos2: DVector,
    ) -> f64;
    pub fn dx_Segment_Triangle_MinLength(
        SegmentPos1: Vector,
        SegmentPos2: Vector,
        TrianglePos1: Vector,
        TrianglePos2: Vector,
        TrianglePos3: Vector,
    ) -> f32;
    pub fn dx_Segment_Triangle_MinLengthD(
        SegmentPos1: DVector,
        SegmentPos2: DVector,
        TrianglePos1: DVector,
        TrianglePos2: DVector,
        TrianglePos3: DVector,
    ) -> f64;
    pub fn dx_Segment_Triangle_MinLength_Square(
        SegmentPos1: Vector,
        SegmentPos2: Vector,
        TrianglePos1: Vector,
        TrianglePos2: Vector,
        TrianglePos3: Vector,
    ) -> f32;
    pub fn dx_Segment_Triangle_MinLength_SquareD(
        SegmentPos1: DVector,
        SegmentPos2: DVector,
        TrianglePos1: DVector,
        TrianglePos2: DVector,
        TrianglePos3: DVector,
    ) -> f64;
    pub fn dx_Segment_Point_MinLength(
        SegmentPos1: Vector,
        SegmentPos2: Vector,
        PointPos: Vector,
    ) -> f32;
    pub fn dx_Segment_Point_MinLengthD(
        SegmentPos1: DVector,
        SegmentPos2: DVector,
        PointPos: DVector,
    ) -> f64;
    pub fn dx_Segment_Point_MinLength_Square(
        SegmentPos1: Vector,
        SegmentPos2: Vector,
        PointPos: Vector,
    ) -> f32;
    pub fn dx_Segment_Point_MinLength_SquareD(
        SegmentPos1: DVector,
        SegmentPos2: DVector,
        PointPos: DVector,
    ) -> f64;
    pub fn dx_Triangle_Point_MinLength(
        TrianglePos1: Vector,
        TrianglePos2: Vector,
        TrianglePos3: Vector,
        PointPos: Vector,
    ) -> f32;
    pub fn dx_Triangle_Point_MinLengthD(
        TrianglePos1: DVector,
        TrianglePos2: DVector,
        TrianglePos3: DVector,
        PointPos: DVector,
    ) -> f64;
    pub fn dx_Triangle_Point_MinLength_Square(
        TrianglePos1: Vector,
        TrianglePos2: Vector,
        TrianglePos3: Vector,
        PointPos: Vector,
    ) -> f32;
    pub fn dx_Triangle_Point_MinLength_SquareD(
        TrianglePos1: DVector,
        TrianglePos2: DVector,
        TrianglePos3: DVector,
        PointPos: DVector,
    ) -> f64;
    pub fn dx_Triangle_Triangle_MinLength(
        Triangle1Pos1: Vector,
        Triangle1Pos2: Vector,
        Triangle1Pos3: Vector,
        Triangle2Pos1: Vector,
        Triangle2Pos2: Vector,
        Triangle2Pos3: Vector,
    ) -> f32;
    pub fn dx_Triangle_Triangle_MinLengthD(
        Triangle1Pos1: DVector,
        Triangle1Pos2: DVector,
        Triangle1Pos3: DVector,
        Triangle2Pos1: DVector,
        Triangle2Pos2: DVector,
        Triangle2Pos3: DVector,
    ) -> f64;
    pub fn dx_Triangle_Triangle_MinLength_Square(
        Triangle1Pos1: Vector,
        Triangle1Pos2: Vector,
        Triangle1Pos3: Vector,
        Triangle2Pos1: Vector,
        Triangle2Pos2: Vector,
        Triangle2Pos3: Vector,
    ) -> f32;
    pub fn dx_Triangle_Triangle_MinLength_SquareD(
        Triangle1Pos1: DVector,
        Triangle1Pos2: DVector,
        Triangle1Pos3: DVector,
        Triangle2Pos1: DVector,
        Triangle2Pos2: DVector,
        Triangle2Pos3: DVector,
    ) -> f64;
    pub fn dx_Plane_Point_MinLength_Position(
        PlanePos: Vector,
        PlaneNormal: Vector,
        PointPos: Vector,
    ) -> Vector;
    pub fn dx_Plane_Point_MinLength_PositionD(
        PlanePos: DVector,
        PlaneNormal: DVector,
        PointPos: DVector,
    ) -> DVector;
    pub fn dx_Plane_Point_MinLength(PlanePos: Vector, PlaneNormal: Vector, PointPos: Vector)
        -> f32;
    pub fn dx_Plane_Point_MinLengthD(
        PlanePos: DVector,
        PlaneNormal: DVector,
        PointPos: DVector,
    ) -> f64;
    pub fn dx_HitCheck_Line_Triangle(
        LinePos1: Vector,
        LinePos2: Vector,
        TrianglePos1: Vector,
        TrianglePos2: Vector,
        TrianglePos3: Vector,
    ) -> HitResultLine;
    pub fn dx_HitCheck_Line_TriangleD(
        LinePos1: DVector,
        LinePos2: DVector,
        TrianglePos1: DVector,
        TrianglePos2: DVector,
        TrianglePos3: DVector,
    ) -> HitResultLineDouble;
    pub fn dx_HitCheck_Triangle_Triangle(
        Triangle1Pos1: Vector,
        Triangle1Pos2: Vector,
        Triangle1Pos3: Vector,
        Triangle2Pos1: Vector,
        Triangle2Pos2: Vector,
        Triangle2Pos3: Vector,
    ) -> i32;
    pub fn dx_HitCheck_Triangle_TriangleD(
        Triangle1Pos1: DVector,
        Triangle1Pos2: DVector,
        Triangle1Pos3: DVector,
        Triangle2Pos1: DVector,
        Triangle2Pos2: DVector,
        Triangle2Pos3: DVector,
    ) -> i32;
    pub fn dx_HitCheck_Triangle_Triangle_2D(
        Triangle1Pos1: Vector,
        Triangle1Pos2: Vector,
        Triangle1Pos3: Vector,
        Triangle2Pos1: Vector,
        Triangle2Pos2: Vector,
        Triangle2Pos3: Vector,
    ) -> i32;
    pub fn dx_HitCheck_Triangle_TriangleD_2D(
        Triangle1Pos1: DVector,
        Triangle1Pos2: DVector,
        Triangle1Pos3: DVector,
        Triangle2Pos1: DVector,
        Triangle2Pos2: DVector,
        Triangle2Pos3: DVector,
    ) -> i32;
    pub fn dx_HitCheck_Line_Cube(
        LinePos1: Vector,
        LinePos2: Vector,
        CubePos1: Vector,
        CubePos2: Vector,
    ) -> HitResultLine;
    pub fn dx_HitCheck_Line_CubeD(
        LinePos1: DVector,
        LinePos2: DVector,
        CubePos1: DVector,
        CubePos2: DVector,
    ) -> HitResultLineDouble;
    pub fn dx_HitCheck_Point_Cone(
        PointPos: Vector,
        ConeTopPos: Vector,
        ConeBottomPos: Vector,
        ConeR: f32,
    ) -> i32;
    pub fn dx_HitCheck_Point_ConeD(
        PointPos: DVector,
        ConeTopPos: DVector,
        ConeBottomPos: DVector,
        ConeR: f64,
    ) -> i32;
    pub fn dx_HitCheck_Line_Sphere(
        LinePos1: Vector,
        LinePos2: Vector,
        SphereCenterPos: Vector,
        SphereR: f32,
    ) -> i32;
    pub fn dx_HitCheck_Line_SphereD(
        LinePos1: DVector,
        LinePos2: DVector,
        SphereCenterPos: DVector,
        SphereR: f64,
    ) -> i32;
    pub fn dx_HitCheck_Sphere_Sphere(
        Sphere1CenterPos: Vector,
        Sphere1R: f32,
        Sphere2CenterPos: Vector,
        Sphere2R: f32,
    ) -> i32;
    pub fn dx_HitCheck_Sphere_SphereD(
        Sphere1CenterPos: DVector,
        Sphere1R: f64,
        Sphere2CenterPos: DVector,
        Sphere2R: f64,
    ) -> i32;
    pub fn dx_HitCheck_Sphere_Capsule(
        SphereCenterPos: Vector,
        SphereR: f32,
        CapPos1: Vector,
        CapPos2: Vector,
        CapR: f32,
    ) -> i32;
    pub fn dx_HitCheck_Sphere_CapsuleD(
        SphereCenterPos: DVector,
        SphereR: f64,
        CapPos1: DVector,
        CapPos2: DVector,
        CapR: f64,
    ) -> i32;
    pub fn dx_HitCheck_Sphere_Triangle(
        SphereCenterPos: Vector,
        SphereR: f32,
        TrianglePos1: Vector,
        TrianglePos2: Vector,
        TrianglePos3: Vector,
    ) -> i32;
    pub fn dx_HitCheck_Sphere_TriangleD(
        SphereCenterPos: DVector,
        SphereR: f64,
        TrianglePos1: DVector,
        TrianglePos2: DVector,
        TrianglePos3: DVector,
    ) -> i32;
    pub fn dx_HitCheck_Capsule_Capsule(
        Cap1Pos1: Vector,
        Cap1Pos2: Vector,
        Cap1R: f32,
        Cap2Pos1: Vector,
        Cap2Pos2: Vector,
        Cap2R: f32,
    ) -> i32;
    pub fn dx_HitCheck_Capsule_CapsuleD(
        Cap1Pos1: DVector,
        Cap1Pos2: DVector,
        Cap1R: f64,
        Cap2Pos1: DVector,
        Cap2Pos2: DVector,
        Cap2R: f64,
    ) -> i32;
    pub fn dx_HitCheck_Capsule_Triangle(
        CapPos1: Vector,
        CapPos2: Vector,
        CapR: f32,
        TrianglePos1: Vector,
        TrianglePos2: Vector,
        TrianglePos3: Vector,
    ) -> i32;
    pub fn dx_HitCheck_Capsule_TriangleD(
        CapPos1: DVector,
        CapPos2: DVector,
        CapR: f64,
        TrianglePos1: DVector,
        TrianglePos2: DVector,
        TrianglePos3: DVector,
    ) -> i32;
    pub fn dx_RectClipping(Rect: *mut Rect, ClippuRect: *const Rect) -> i32;
    pub fn dx_RectAdjust(Rect: *mut Rect) -> i32;
    pub fn dx_GetRectSize(Rect: *const Rect, Width: *mut i32, Height: *mut i32) -> i32;
    pub fn dx_MGetIdent() -> Matrix;
    pub fn dx_MGetIdentD() -> DMatrix;
    pub fn dx_MMult(In1: Matrix, In2: Matrix) -> Matrix;
    pub fn dx_MMultD(In1: DMatrix, In2: DMatrix) -> DMatrix;
    pub fn dx_MScale(InM: Matrix, Scale: f32) -> Matrix;
    pub fn dx_MScaleD(InM: DMatrix, Scale: f64) -> DMatrix;
    pub fn dx_MAdd(In1: Matrix, In2: Matrix) -> Matrix;
    pub fn dx_MAddD(In1: DMatrix, In2: DMatrix) -> DMatrix;
    pub fn dx_MGetScale(Scale: Vector) -> Matrix;
    pub fn dx_MGetScaleD(Scale: DVector) -> DMatrix;
    pub fn dx_MGetRotX(XAxisRotate: f32) -> Matrix;
    pub fn dx_MGetRotXD(XAxisRotate: f64) -> DMatrix;
    pub fn dx_MGetRotY(YAxisRotate: f32) -> Matrix;
    pub fn dx_MGetRotYD(YAxisRotate: f64) -> DMatrix;
    pub fn dx_MGetRotZ(ZAxisRotate: f32) -> Matrix;
    pub fn dx_MGetRotZD(ZAxisRotate: f64) -> DMatrix;
    pub fn dx_MGetRotAxis(RotateAxis: Vector, Rotate: f32) -> Matrix;
    pub fn dx_MGetRotAxisD(RotateAxis: DVector, Rotate: f64) -> DMatrix;
    pub fn dx_MGetRotVec2(In1: Vector, In2: Vector) -> Matrix;
    pub fn dx_MGetRotVec2D(In1: DVector, In2: DVector) -> DMatrix;
    pub fn dx_MGetTranslate(Trans: Vector) -> Matrix;
    pub fn dx_MGetTranslateD(Trans: DVector) -> DMatrix;
    pub fn dx_MGetAxis1(XAxis: Vector, YAxis: Vector, ZAxis: Vector, Pos: Vector) -> Matrix;
    pub fn dx_MGetAxis1D(XAxis: DVector, YAxis: DVector, ZAxis: DVector, Pos: DVector) -> DMatrix;
    pub fn dx_MGetAxis2(XAxis: Vector, YAxis: Vector, ZAxis: Vector, Pos: Vector) -> Matrix;
    pub fn dx_MGetAxis2D(XAxis: DVector, YAxis: DVector, ZAxis: DVector, Pos: DVector) -> DMatrix;
    pub fn dx_MTranspose(InM: Matrix) -> Matrix;
    pub fn dx_MTransposeD(InM: DMatrix) -> DMatrix;
    pub fn dx_MInverse(InM: Matrix) -> Matrix;
    pub fn dx_MInverseD(InM: DMatrix) -> DMatrix;
    pub fn dx_MGetSize(InM: Matrix) -> Vector;
    pub fn dx_MGetSizeD(InM: DMatrix) -> DVector;
    pub fn dx_MGetRotElem(InM: Matrix) -> Matrix;
    pub fn dx_MGetRotElemD(InM: DMatrix) -> DMatrix;
    pub fn dx_VNorm(In: Vector) -> Vector;
    pub fn dx_VNormD(In: DVector) -> DVector;
    pub fn dx_VSize(In: Vector) -> f32;
    pub fn dx_VSizeD(In: DVector) -> f64;
    pub fn dx_VCos(In1: Vector, In2: Vector) -> f32;
    pub fn dx_VCosD(In1: DVector, In2: DVector) -> f64;
    pub fn dx_VRad(In1: Vector, In2: Vector) -> f32;
    pub fn dx_VRadD(In1: DVector, In2: DVector) -> f64;
    pub fn dx_QTRot(Axis: Vector, Angle: f32) -> Float4;
    pub fn dx_QTRotD(Axis: DVector, Angle: f64) -> Double4;
    pub fn dx_VRotQ(P: Vector, Axis: Vector, Angle: f32) -> Vector;
    pub fn dx_VRotQD(P: DVector, Axis: DVector, Angle: f64) -> DVector;
    pub fn dx_CreateGraphImageOrDIBGraph(
        FileName: *const i8,
        DataImage: *const libc::c_void,
        DataImageSize: i32,
        DataImageType: i32,
        BmpFlag: i32,
        ReverseFlag: i32,
        BaseImage: *mut BaseImage,
        BmpInfo: *mut *mut BitMapInfo,
        GraphData: *mut *mut libc::c_void,
    ) -> i32;
    pub fn dx_CreateGraphImageOrDIBGraphWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        DataImage: *const libc::c_void,
        DataImageSize: i32,
        DataImageType: i32,
        BmpFlag: i32,
        ReverseFlag: i32,
        BaseImage: *mut BaseImage,
        BmpInfo: *mut *mut BitMapInfo,
        GraphData: *mut *mut libc::c_void,
    ) -> i32;
    pub fn dx_CreateGraphImageType2(Src: *mut StreamData, Dest: *mut BaseImage) -> i32;
    pub fn dx_CreateBmpInfo(
        BmpInfo: *mut BitMapInfo,
        Width: i32,
        Height: i32,
        Pitch: i32,
        SrcGrData: *const libc::c_void,
        DestGrData: *mut *mut libc::c_void,
    ) -> i32;
    pub fn dx_GetImageSize_File(FileName: *const i8, SizeX: *mut i32, SizeY: *mut i32) -> i32;
    pub fn dx_GetImageSize_FileWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        SizeX: *mut i32,
        SizeY: *mut i32,
    ) -> i32;
    pub fn dx_GetImageSize_Mem(
        FileImage: *const libc::c_void,
        FileImageSize: i32,
        SizeX: *mut i32,
        SizeY: *mut i32,
    ) -> i32;
    pub fn dx_CreateGraphImage_plus_Alpha(
        FileName: *const i8,
        RgbImage: *const libc::c_void,
        RgbImageSize: i32,
        RgbImageType: i32,
        AlphaImage: *const libc::c_void,
        AlphaImageSize: i32,
        AlphaImageType: i32,
        RgbGraphImage: *mut BaseImage,
        AlphaGraphImage: *mut BaseImage,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateGraphImage_plus_AlphaWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        RgbImage: *const libc::c_void,
        RgbImageSize: i32,
        RgbImageType: i32,
        AlphaImage: *const libc::c_void,
        AlphaImageSize: i32,
        AlphaImageType: i32,
        RgbGraphImage: *mut BaseImage,
        AlphaGraphImage: *mut BaseImage,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_ReverseGraphImage(GraphImage: *mut BaseImage) -> i32;
    pub fn dx_AddUserGraphLoadFunction4(Src: *mut libc::c_void) -> i32;
    pub fn dx_SubUserGraphLoadFunction4(Src: *mut libc::c_void) -> i32;
    pub fn dx_SetUseFastLoadFlag(Flag: i32) -> i32;
    pub fn dx_SetGraphDataShavedMode(ShavedMode: i32) -> i32;
    pub fn dx_GetGraphDataShavedMode() -> i32;
    pub fn dx_SetUsePremulAlphaConvertLoad(UseFlag: i32) -> i32;
    pub fn dx_CreateBaseImage(
        FileName: *const i8,
        FileImage: *const libc::c_void,
        FileImageSize: i32,
        DataType: i32,
        BaseImage: *mut BaseImage,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateBaseImageWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        FileImage: *const libc::c_void,
        FileImageSize: i32,
        DataType: i32,
        BaseImage: *mut BaseImage,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateGraphImage(
        FileName: *const i8,
        DataImage: *const libc::c_void,
        DataImageSize: i32,
        DataImageType: i32,
        GraphImage: *mut BaseImage,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateBaseImageToFile(
        FileName: *const i8,
        BaseImage: *mut BaseImage,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateBaseImageToFileWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        BaseImage: *mut BaseImage,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateBaseImageToMem(
        FileImage: *const libc::c_void,
        FileImageSize: i32,
        BaseImage: *mut BaseImage,
        ReverseFlag: i32,
    ) -> i32;
    pub fn dx_CreateARGBF32ColorBaseImage(SizeX: i32, SizeY: i32, BaseImage: *mut BaseImage)
        -> i32;
    pub fn dx_CreateARGBF16ColorBaseImage(SizeX: i32, SizeY: i32, BaseImage: *mut BaseImage)
        -> i32;
    pub fn dx_CreateARGB8ColorBaseImage(SizeX: i32, SizeY: i32, BaseImage: *mut BaseImage) -> i32;
    pub fn dx_CreateXRGB8ColorBaseImage(SizeX: i32, SizeY: i32, BaseImage: *mut BaseImage) -> i32;
    pub fn dx_CreateRGB8ColorBaseImage(SizeX: i32, SizeY: i32, BaseImage: *mut BaseImage) -> i32;
    pub fn dx_CreateARGB4ColorBaseImage(SizeX: i32, SizeY: i32, BaseImage: *mut BaseImage) -> i32;
    pub fn dx_CreateA1R5G5B5ColorBaseImage(
        SizeX: i32,
        SizeY: i32,
        BaseImage: *mut BaseImage,
    ) -> i32;
    pub fn dx_CreateX1R5G5B5ColorBaseImage(
        SizeX: i32,
        SizeY: i32,
        BaseImage: *mut BaseImage,
    ) -> i32;
    pub fn dx_CreateR5G5B5A1ColorBaseImage(
        SizeX: i32,
        SizeY: i32,
        BaseImage: *mut BaseImage,
    ) -> i32;
    pub fn dx_CreateR5G6B5ColorBaseImage(SizeX: i32, SizeY: i32, BaseImage: *mut BaseImage) -> i32;
    pub fn dx_CreatePAL8ColorBaseImage(
        SizeX: i32,
        SizeY: i32,
        BaseImage: *mut BaseImage,
        UseAlpha: i32,
    ) -> i32;
    pub fn dx_CreateColorDataBaseImage(
        SizeX: i32,
        SizeY: i32,
        ColorData: *const ColorData,
        BaseImage: *mut BaseImage,
    ) -> i32;
    pub fn dx_GetBaseImageGraphDataSize(BaseImage: *const BaseImage) -> i32;
    pub fn dx_DerivationBaseImage(
        BaseImage: *const BaseImage,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        NewBaseImage: *mut BaseImage,
    ) -> i32;
    pub fn dx_ReleaseBaseImage(BaseImage: *mut BaseImage) -> i32;
    pub fn dx_ReleaseGraphImage(GraphImage: *mut BaseImage) -> i32;
    pub fn dx_ConvertNormalFormatBaseImage(
        BaseImage: *mut BaseImage,
        ReleaseOrigGraphData: i32,
    ) -> i32;
    pub fn dx_ConvertPremulAlphaBaseImage(BaseImage: *mut BaseImage) -> i32;
    pub fn dx_ConvertInterpAlphaBaseImage(BaseImage: *mut BaseImage) -> i32;
    pub fn dx_GetDrawScreenBaseImage(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        BaseImage: *mut BaseImage,
    ) -> i32;
    pub fn dx_GetDrawScreenBaseImageDestPos(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        BaseImage: *mut BaseImage,
        DestX: i32,
        DestY: i32,
    ) -> i32;
    pub fn dx_FillBaseImage(BaseImage: *mut BaseImage, r: i32, g: i32, b: i32, a: i32) -> i32;
    pub fn dx_FillRectBaseImage(
        BaseImage: *mut BaseImage,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
    ) -> i32;
    pub fn dx_ClearRectBaseImage(BaseImage: *mut BaseImage, x: i32, y: i32, w: i32, h: i32) -> i32;
    pub fn dx_GetPaletteBaseImage(
        BaseImage: *const BaseImage,
        PaletteNo: i32,
        r: *mut i32,
        g: *mut i32,
        b: *mut i32,
        a: *mut i32,
    ) -> i32;
    pub fn dx_SetPaletteBaseImage(
        BaseImage: *mut BaseImage,
        PaletteNo: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
    ) -> i32;
    pub fn dx_SetPixelPalCodeBaseImage(
        BaseImage: *mut BaseImage,
        x: i32,
        y: i32,
        palNo: i32,
    ) -> i32;
    pub fn dx_GetPixelPalCodeBaseImage(BaseImage: *const BaseImage, x: i32, y: i32) -> i32;
    pub fn dx_SetPixelBaseImage(
        BaseImage: *mut BaseImage,
        x: i32,
        y: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
    ) -> i32;
    pub fn dx_SetPixelBaseImageF(
        BaseImage: *mut BaseImage,
        x: i32,
        y: i32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> i32;
    pub fn dx_GetPixelBaseImage(
        BaseImage: *const BaseImage,
        x: i32,
        y: i32,
        r: *mut i32,
        g: *mut i32,
        b: *mut i32,
        a: *mut i32,
    ) -> i32;
    pub fn dx_GetPixelBaseImageF(
        BaseImage: *const BaseImage,
        x: i32,
        y: i32,
        r: *mut f32,
        g: *mut f32,
        b: *mut f32,
        a: *mut f32,
    ) -> i32;
    pub fn dx_DrawLineBaseImage(
        BaseImage: *mut BaseImage,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
    ) -> i32;
    pub fn dx_DrawCircleBaseImage(
        BaseImage: *mut BaseImage,
        x: i32,
        y: i32,
        radius: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_BltBaseImage(
        SrcX: i32,
        SrcY: i32,
        SrcSizeX: i32,
        SrcSizeY: i32,
        DestX: i32,
        DestY: i32,
        SrcBaseImage: *mut BaseImage,
        DestBaseImage: *mut BaseImage,
    ) -> i32;
    pub fn dx_BltBaseImage_2(
        DestX: i32,
        DestY: i32,
        SrcBaseImage: *mut BaseImage,
        DestBaseImage: *mut BaseImage,
    ) -> i32;
    pub fn dx_BltBaseImageWithTransColor(
        SrcX: i32,
        SrcY: i32,
        SrcSizeX: i32,
        SrcSizeY: i32,
        DestX: i32,
        DestY: i32,
        SrcBaseImage: *mut BaseImage,
        DestBaseImage: *mut BaseImage,
        Tr: i32,
        Tg: i32,
        Tb: i32,
        Ta: i32,
    ) -> i32;
    pub fn dx_BltBaseImageWithAlphaBlend(
        SrcX: i32,
        SrcY: i32,
        SrcSizeX: i32,
        SrcSizeY: i32,
        DestX: i32,
        DestY: i32,
        SrcBaseImage: *mut BaseImage,
        DestBaseImage: *mut BaseImage,
        Opacity: i32,
    ) -> i32;
    pub fn dx_ReverseBaseImageH(BaseImage: *mut BaseImage) -> i32;
    pub fn dx_ReverseBaseImageV(BaseImage: *mut BaseImage) -> i32;
    pub fn dx_ReverseBaseImage(BaseImage: *mut BaseImage) -> i32;
    pub fn dx_CheckPixelAlphaBaseImage(BaseImage: *const BaseImage) -> i32;
    pub fn dx_GetBaseImageUseMaxPaletteNo(BaseImage: *const BaseImage) -> i32;
    pub fn dx_ReadJpegExif(
        JpegFilePath: *const i8,
        ExifBuffer_Array: *mut u8,
        ExifBufferSize: usize,
    ) -> i32;
    pub fn dx_ReadJpegExifWithStrLen(
        JpegFilePath: *const i8,
        JpegFilePathLength: usize,
        ExifBuffer_Array: *mut u8,
        ExifBufferSize: usize,
    ) -> i32;
    pub fn dx_SaveBaseImageToBmp(FilePath: *const i8, BaseImage: *const BaseImage) -> i32;
    pub fn dx_SaveBaseImageToBmpWithStrLen(
        FilePath: *const i8,
        FilePathLength: usize,
        BaseImage: *const BaseImage,
    ) -> i32;
    pub fn dx_SaveBaseImageToDds(FilePath: *const i8, BaseImage: *const BaseImage) -> i32;
    pub fn dx_SaveBaseImageToDdsWithStrLen(
        FilePath: *const i8,
        FilePathLength: usize,
        BaseImage: *const BaseImage,
    ) -> i32;
    pub fn dx_SaveBaseImageToPng(
        FilePath: *const i8,
        BaseImage: *mut BaseImage,
        CompressionLevel: i32,
    ) -> i32;
    pub fn dx_SaveBaseImageToPngWithStrLen(
        FilePath: *const i8,
        FilePathLength: usize,
        BaseImage: *mut BaseImage,
        CompressionLevel: i32,
    ) -> i32;
    pub fn dx_SaveBaseImageToJpeg(
        FilePath: *const i8,
        BaseImage: *mut BaseImage,
        Quality: i32,
        Sample2x1: i32,
    ) -> i32;
    pub fn dx_SaveBaseImageToJpegWithStrLen(
        FilePath: *const i8,
        FilePathLength: usize,
        BaseImage: *mut BaseImage,
        Quality: i32,
        Sample2x1: i32,
    ) -> i32;
    pub fn dx_DrawBaseImage(x: i32, y: i32, BaseImage: *mut BaseImage) -> i32;
    pub fn dx_GraphColorMatchBltVer2(
        DestGraphData: *mut libc::c_void,
        DestPitch: i32,
        DestColorData: *const ColorData,
        SrcGraphData: *const libc::c_void,
        SrcPitch: i32,
        SrcColorData: *const ColorData,
        AlphaMask: *const libc::c_void,
        AlphaPitch: i32,
        AlphaColorData: *const ColorData,
        DestPoint: Point,
        SrcRect: *const Rect,
        ReverseFlag: i32,
        TransColorAlphaTestFlag: i32,
        TransColor: u32,
        ImageShavedMode: i32,
        AlphaOnlyFlag: i32,
        RedIsAlphaFlag: i32,
        TransColorNoMoveFlag: i32,
        Pal8ColorMatch: i32,
    ) -> i32;
    pub fn dx_GetColorF(Red: f32, Green: f32, Blue: f32, Alpha: f32) -> ColorF32;
    pub fn dx_GetColorU8(Red: i32, Green: i32, Blue: i32, Alpha: i32) -> ColorU8;
    pub fn dx_GetColor(Red: i32, Green: i32, Blue: i32) -> u32;
    pub fn dx_GetColor2(Color: u32, Red: *mut i32, Green: *mut i32, Blue: *mut i32) -> i32;
    pub fn dx_GetColor5(
        ColorData: *const ColorData,
        Color: u32,
        Red: *mut i32,
        Green: *mut i32,
        Blue: *mut i32,
        Alpha: *mut i32,
    ) -> i32;
    pub fn dx_CreatePaletteColorData(ColorDataBuf: *mut ColorData) -> i32;
    pub fn dx_CreateARGBF32ColorData(ColorDataBuf: *mut ColorData) -> i32;
    pub fn dx_CreateARGBF16ColorData(ColorDataBuf: *mut ColorData) -> i32;
    pub fn dx_CreateXRGB8ColorData(ColorDataBuf: *mut ColorData) -> i32;
    pub fn dx_CreateARGB8ColorData(ColorDataBuf: *mut ColorData) -> i32;
    pub fn dx_CreateARGB4ColorData(ColorDataBuf: *mut ColorData) -> i32;
    pub fn dx_CreateA1R5G5B5ColorData(ColorDataBuf: *mut ColorData) -> i32;
    pub fn dx_CreateX1R5G5B5ColorData(ColorDataBuf: *mut ColorData) -> i32;
    pub fn dx_CreateR5G5B5A1ColorData(ColorDataBuf: *mut ColorData) -> i32;
    pub fn dx_CreateR5G6B5ColorData(ColorDataBuf: *mut ColorData) -> i32;
    pub fn dx_CreateFullColorData(ColorDataBuf: *mut ColorData) -> i32;
    pub fn dx_CreateGrayColorData(ColorDataBuf: *mut ColorData) -> i32;
    pub fn dx_CreatePal8ColorData(ColorDataBuf: *mut ColorData, UseAlpha: i32) -> i32;
    pub fn dx_CreateColorData(
        ColorDataBuf: *mut ColorData,
        ColorBitDepth: i32,
        RedMask: u32,
        GreenMask: u32,
        BlueMask: u32,
        AlphaMask: u32,
        ChannelNum: i32,
        ChannelBitDepth: i32,
        f32TypeFlag: i32,
    ) -> i32;
    pub fn dx_SetColorDataNoneMask(ColorData: *mut ColorData) -> ();
    pub fn dx_CmpColorData(ColorData1: *const ColorData, ColorData2: *const ColorData) -> i32;
    pub fn dx_InitSoftImage() -> i32;
    pub fn dx_LoadSoftImage(FileName: *const i8) -> i32;
    pub fn dx_LoadSoftImageWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_LoadARGB8ColorSoftImage(FileName: *const i8) -> i32;
    pub fn dx_LoadARGB8ColorSoftImageWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_LoadXRGB8ColorSoftImage(FileName: *const i8) -> i32;
    pub fn dx_LoadXRGB8ColorSoftImageWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_LoadSoftImageToMem(FileImage: *const libc::c_void, FileImageSize: i32) -> i32;
    pub fn dx_LoadARGB8ColorSoftImageToMem(
        FileImage: *const libc::c_void,
        FileImageSize: i32,
    ) -> i32;
    pub fn dx_LoadXRGB8ColorSoftImageToMem(
        FileImage: *const libc::c_void,
        FileImageSize: i32,
    ) -> i32;
    pub fn dx_MakeSoftImage(SizeX: i32, SizeY: i32) -> i32;
    pub fn dx_MakeARGBF32ColorSoftImage(SizeX: i32, SizeY: i32) -> i32;
    pub fn dx_MakeARGBF16ColorSoftImage(SizeX: i32, SizeY: i32) -> i32;
    pub fn dx_MakeARGB8ColorSoftImage(SizeX: i32, SizeY: i32) -> i32;
    pub fn dx_MakeXRGB8ColorSoftImage(SizeX: i32, SizeY: i32) -> i32;
    pub fn dx_MakeARGB4ColorSoftImage(SizeX: i32, SizeY: i32) -> i32;
    pub fn dx_MakeA1R5G5B5ColorSoftImage(SizeX: i32, SizeY: i32) -> i32;
    pub fn dx_MakeX1R5G5B5ColorSoftImage(SizeX: i32, SizeY: i32) -> i32;
    pub fn dx_MakeR5G5B5A1ColorSoftImage(SizeX: i32, SizeY: i32) -> i32;
    pub fn dx_MakeR5G6B5ColorSoftImage(SizeX: i32, SizeY: i32) -> i32;
    pub fn dx_MakeRGB8ColorSoftImage(SizeX: i32, SizeY: i32) -> i32;
    pub fn dx_MakePAL8ColorSoftImage(SizeX: i32, SizeY: i32, UseAlpha: i32) -> i32;
    pub fn dx_DeleteSoftImage(SIHandle: i32) -> i32;
    pub fn dx_GetSoftImageSize(SIHandle: i32, Width: *mut i32, Height: *mut i32) -> i32;
    pub fn dx_CheckPaletteSoftImage(SIHandle: i32) -> i32;
    pub fn dx_CheckAlphaSoftImage(SIHandle: i32) -> i32;
    pub fn dx_CheckPixelAlphaSoftImage(SIHandle: i32) -> i32;
    pub fn dx_GetDrawScreenSoftImage(x1: i32, y1: i32, x2: i32, y2: i32, SIHandle: i32) -> i32;
    pub fn dx_GetDrawScreenSoftImageDestPos(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        SIHandle: i32,
        DestX: i32,
        DestY: i32,
    ) -> i32;
    pub fn dx_FillSoftImage(SIHandle: i32, r: i32, g: i32, b: i32, a: i32) -> i32;
    pub fn dx_ClearRectSoftImage(SIHandle: i32, x: i32, y: i32, w: i32, h: i32) -> i32;
    pub fn dx_GetPaletteSoftImage(
        SIHandle: i32,
        PaletteNo: i32,
        r: *mut i32,
        g: *mut i32,
        b: *mut i32,
        a: *mut i32,
    ) -> i32;
    pub fn dx_SetPaletteSoftImage(
        SIHandle: i32,
        PaletteNo: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
    ) -> i32;
    pub fn dx_DrawPixelPalCodeSoftImage(SIHandle: i32, x: i32, y: i32, palNo: i32) -> i32;
    pub fn dx_GetPixelPalCodeSoftImage(SIHandle: i32, x: i32, y: i32) -> i32;
    pub fn dx_GetPitchSoftImage(SIHandle: i32) -> i32;
    pub fn dx_DrawPixelSoftImage(
        SIHandle: i32,
        x: i32,
        y: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
    ) -> i32;
    pub fn dx_DrawPixelSoftImageF(
        SIHandle: i32,
        x: i32,
        y: i32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> i32;
    pub fn dx_DrawPixelSoftImage_Unsafe_XRGB8(
        SIHandle: i32,
        x: i32,
        y: i32,
        r: i32,
        g: i32,
        b: i32,
    ) -> ();
    pub fn dx_DrawPixelSoftImage_Unsafe_ARGB8(
        SIHandle: i32,
        x: i32,
        y: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
    ) -> ();
    pub fn dx_GetPixelSoftImage(
        SIHandle: i32,
        x: i32,
        y: i32,
        r: *mut i32,
        g: *mut i32,
        b: *mut i32,
        a: *mut i32,
    ) -> i32;
    pub fn dx_GetPixelSoftImageF(
        SIHandle: i32,
        x: i32,
        y: i32,
        r: *mut f32,
        g: *mut f32,
        b: *mut f32,
        a: *mut f32,
    ) -> i32;
    pub fn dx_GetPixelSoftImage_Unsafe_XRGB8(
        SIHandle: i32,
        x: i32,
        y: i32,
        r: *mut i32,
        g: *mut i32,
        b: *mut i32,
    ) -> ();
    pub fn dx_GetPixelSoftImage_Unsafe_ARGB8(
        SIHandle: i32,
        x: i32,
        y: i32,
        r: *mut i32,
        g: *mut i32,
        b: *mut i32,
        a: *mut i32,
    ) -> ();
    pub fn dx_DrawLineSoftImage(
        SIHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
    ) -> i32;
    pub fn dx_DrawCircleSoftImage(
        SIHandle: i32,
        x: i32,
        y: i32,
        radius: i32,
        r: i32,
        g: i32,
        b: i32,
        a: i32,
        FillFlag: i32,
    ) -> i32;
    pub fn dx_BltSoftImage(
        SrcX: i32,
        SrcY: i32,
        SrcSizeX: i32,
        SrcSizeY: i32,
        SrcSIHandle: i32,
        DestX: i32,
        DestY: i32,
        DestSIHandle: i32,
    ) -> i32;
    pub fn dx_BltSoftImageWithTransColor(
        SrcX: i32,
        SrcY: i32,
        SrcSizeX: i32,
        SrcSizeY: i32,
        SrcSIHandle: i32,
        DestX: i32,
        DestY: i32,
        DestSIHandle: i32,
        Tr: i32,
        Tg: i32,
        Tb: i32,
        Ta: i32,
    ) -> i32;
    pub fn dx_BltSoftImageWithAlphaBlend(
        SrcX: i32,
        SrcY: i32,
        SrcSizeX: i32,
        SrcSizeY: i32,
        SrcSIHandle: i32,
        DestX: i32,
        DestY: i32,
        DestSIHandle: i32,
        Opacity: i32,
    ) -> i32;
    pub fn dx_ReverseSoftImageH(SIHandle: i32) -> i32;
    pub fn dx_ReverseSoftImageV(SIHandle: i32) -> i32;
    pub fn dx_ReverseSoftImage(SIHandle: i32) -> i32;
    pub fn dx_BltStringSoftImage(
        x: i32,
        y: i32,
        StrData: *const i8,
        DestSIHandle: i32,
        DestEdgeSIHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_BltStringSoftImageWithStrLen(
        x: i32,
        y: i32,
        StrData: *const i8,
        StrDataLength: usize,
        DestSIHandle: i32,
        DestEdgeSIHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_BltStringSoftImageToHandle(
        x: i32,
        y: i32,
        StrData: *const i8,
        DestSIHandle: i32,
        DestEdgeSIHandle: i32,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_BltStringSoftImageToHandleWithStrLen(
        x: i32,
        y: i32,
        StrData: *const i8,
        StrDataLength: usize,
        DestSIHandle: i32,
        DestEdgeSIHandle: i32,
        FontHandle: i32,
        VerticalFlag: i32,
    ) -> i32;
    pub fn dx_DrawSoftImage(x: i32, y: i32, SIHandle: i32) -> i32;
    pub fn dx_SaveSoftImageToBmp(FilePath: *const i8, SIHandle: i32) -> i32;
    pub fn dx_SaveSoftImageToBmpWithStrLen(
        FilePath: *const i8,
        FilePathLength: usize,
        SIHandle: i32,
    ) -> i32;
    pub fn dx_SaveSoftImageToDds(FilePath: *const i8, SIHandle: i32) -> i32;
    pub fn dx_SaveSoftImageToDdsWithStrLen(
        FilePath: *const i8,
        FilePathLength: usize,
        SIHandle: i32,
    ) -> i32;
    pub fn dx_SaveSoftImageToPng(FilePath: *const i8, SIHandle: i32, CompressionLevel: i32) -> i32;
    pub fn dx_SaveSoftImageToPngWithStrLen(
        FilePath: *const i8,
        FilePathLength: usize,
        SIHandle: i32,
        CompressionLevel: i32,
    ) -> i32;
    pub fn dx_SaveSoftImageToJpeg(
        FilePath: *const i8,
        SIHandle: i32,
        Quality: i32,
        Sample2x1: i32,
    ) -> i32;
    pub fn dx_SaveSoftImageToJpegWithStrLen(
        FilePath: *const i8,
        FilePathLength: usize,
        SIHandle: i32,
        Quality: i32,
        Sample2x1: i32,
    ) -> i32;
    pub fn dx_InitSoundMem(LogOutFlag: i32) -> i32;
    pub fn dx_AddSoundData(Handle: i32) -> i32;
    pub fn dx_AddStreamSoundMem(
        Stream: *mut StreamData,
        LoopNum: i32,
        SoundHandle: i32,
        StreamDataType: i32,
        CanStreamCloseFlag: *mut i32,
        UnionHandle: i32,
    ) -> i32;
    pub fn dx_AddStreamSoundMemToMem(
        FileImage: *const libc::c_void,
        FileImageSize: usize,
        LoopNum: i32,
        SoundHandle: i32,
        StreamDataType: i32,
        UnionHandle: i32,
    ) -> i32;
    pub fn dx_AddStreamSoundMemToFile(
        WaveFile: *const i8,
        LoopNum: i32,
        SoundHandle: i32,
        StreamDataType: i32,
        UnionHandle: i32,
    ) -> i32;
    pub fn dx_AddStreamSoundMemToFileWithStrLen(
        WaveFile: *const i8,
        WaveFilePathLength: usize,
        LoopNum: i32,
        SoundHandle: i32,
        StreamDataType: i32,
        UnionHandle: i32,
    ) -> i32;
    pub fn dx_SetupStreamSoundMem(SoundHandle: i32) -> i32;
    pub fn dx_PlayStreamSoundMem(SoundHandle: i32, PlayType: i32, TopPositionFlag: i32) -> i32;
    pub fn dx_CheckStreamSoundMem(SoundHandle: i32) -> i32;
    pub fn dx_StopStreamSoundMem(SoundHandle: i32) -> i32;
    pub fn dx_SetStreamSoundCurrentPosition(Byte: i64, SoundHandle: i32) -> i32;
    pub fn dx_GetStreamSoundCurrentPosition(SoundHandle: i32) -> i64;
    pub fn dx_SetStreamSoundCurrentTime(Time: i64, SoundHandle: i32) -> i32;
    pub fn dx_GetStreamSoundCurrentTime(SoundHandle: i32) -> i64;
    pub fn dx_ProcessStreamSoundMem(SoundHandle: i32) -> i32;
    pub fn dx_ProcessStreamSoundMemAll() -> i32;
    pub fn dx_LoadSoundMem2(FileName1: *const i8, FileName2: *const i8) -> i32;
    pub fn dx_LoadSoundMem2WithStrLen(
        FileName1: *const i8,
        FileName1Length: usize,
        FileName2: *const i8,
        FileName2Length: usize,
    ) -> i32;
    pub fn dx_LoadBGM(FileName: *const i8) -> i32;
    pub fn dx_LoadBGMWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_LoadSoundMemBase(FileName: *const i8, BufferNum: i32, UnionHandle: i32) -> i32;
    pub fn dx_LoadSoundMemBaseWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        BufferNum: i32,
        UnionHandle: i32,
    ) -> i32;
    pub fn dx_LoadSoundMem(FileName: *const i8, BufferNum: i32, UnionHandle: i32) -> i32;
    pub fn dx_LoadSoundMemWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        BufferNum: i32,
        UnionHandle: i32,
    ) -> i32;
    pub fn dx_LoadSoundMemToBufNumSitei(FileName: *const i8, BufferNum: i32) -> i32;
    pub fn dx_LoadSoundMemToBufNumSiteiWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        BufferNum: i32,
    ) -> i32;
    pub fn dx_DuplicateSoundMem(SrcSoundHandle: i32, BufferNum: i32) -> i32;
    pub fn dx_LoadSoundMemByMemImageBase(
        FileImage: *const libc::c_void,
        FileImageSize: usize,
        BufferNum: i32,
        UnionHandle: i32,
    ) -> i32;
    pub fn dx_LoadSoundMemByMemImage(
        FileImage: *const libc::c_void,
        FileImageSize: usize,
        BufferNum: i32,
        UnionHandle: i32,
    ) -> i32;
    pub fn dx_LoadSoundMemByMemImage2(
        WaveImage: *const libc::c_void,
        WaveImageSize: usize,
        WaveFormat: *const WaveFormatEx,
        WaveHeaderSize: usize,
    ) -> i32;
    pub fn dx_LoadSoundMemByMemImageToBufNumSitei(
        FileImage: *const libc::c_void,
        FileImageSize: usize,
        BufferNum: i32,
    ) -> i32;
    pub fn dx_LoadSoundMem2ByMemImage(
        FileImage1: *const libc::c_void,
        FileImageSize1: usize,
        FileImage2: *const libc::c_void,
        FileImageSize2: usize,
    ) -> i32;
    pub fn dx_LoadSoundMemFromSoftSound(SoftSoundHandle: i32, BufferNum: i32) -> i32;
    pub fn dx_DeleteSoundMem(SoundHandle: i32, LogOutFlag: i32) -> i32;
    pub fn dx_PlaySoundMem(SoundHandle: i32, PlayType: i32, TopPositionFlag: i32) -> i32;
    pub fn dx_StopSoundMem(SoundHandle: i32) -> i32;
    pub fn dx_CheckSoundMem(SoundHandle: i32) -> i32;
    pub fn dx_SetPanSoundMem(PanPal: i32, SoundHandle: i32) -> i32;
    pub fn dx_ChangePanSoundMem(PanPal: i32, SoundHandle: i32) -> i32;
    pub fn dx_GetPanSoundMem(SoundHandle: i32) -> i32;
    pub fn dx_SetVolumeSoundMem(VolumePal: i32, SoundHandle: i32) -> i32;
    pub fn dx_ChangeVolumeSoundMem(VolumePal: i32, SoundHandle: i32) -> i32;
    pub fn dx_GetVolumeSoundMem(SoundHandle: i32) -> i32;
    pub fn dx_GetVolumeSoundMem2(SoundHandle: i32) -> i32;
    pub fn dx_SetChannelVolumeSoundMem(Channel: i32, VolumePal: i32, SoundHandle: i32) -> i32;
    pub fn dx_ChangeChannelVolumeSoundMem(Channel: i32, VolumePal: i32, SoundHandle: i32) -> i32;
    pub fn dx_GetChannelVolumeSoundMem(Channel: i32, SoundHandle: i32) -> i32;
    pub fn dx_GetChannelVolumeSoundMem2(Channel: i32, SoundHandle: i32) -> i32;
    pub fn dx_SetFrequencySoundMem(FrequencyPal: i32, SoundHandle: i32) -> i32;
    pub fn dx_GetFrequencySoundMem(SoundHandle: i32) -> i32;
    pub fn dx_ResetFrequencySoundMem(SoundHandle: i32) -> i32;
    pub fn dx_SetNextPlayPanSoundMem(PanPal: i32, SoundHandle: i32) -> i32;
    pub fn dx_ChangeNextPlayPanSoundMem(PanPal: i32, SoundHandle: i32) -> i32;
    pub fn dx_SetNextPlayVolumeSoundMem(VolumePal: i32, SoundHandle: i32) -> i32;
    pub fn dx_ChangeNextPlayVolumeSoundMem(VolumePal: i32, SoundHandle: i32) -> i32;
    pub fn dx_SetNextPlayChannelVolumeSoundMem(
        Channel: i32,
        VolumePal: i32,
        SoundHandle: i32,
    ) -> i32;
    pub fn dx_ChangeNextPlayChannelVolumeSoundMem(
        Channel: i32,
        VolumePal: i32,
        SoundHandle: i32,
    ) -> i32;
    pub fn dx_SetNextPlayFrequencySoundMem(FrequencyPal: i32, SoundHandle: i32) -> i32;
    pub fn dx_SetCurrentPositionSoundMem(SamplePosition: i64, SoundHandle: i32) -> i32;
    pub fn dx_GetCurrentPositionSoundMem(SoundHandle: i32) -> i64;
    pub fn dx_SetSoundCurrentPosition(Byte: i64, SoundHandle: i32) -> i32;
    pub fn dx_GetSoundCurrentPosition(SoundHandle: i32) -> i64;
    pub fn dx_SetSoundCurrentTime(Time: i64, SoundHandle: i32) -> i32;
    pub fn dx_GetSoundCurrentTime(SoundHandle: i32) -> i64;
    pub fn dx_GetSoundTotalSample(SoundHandle: i32) -> i64;
    pub fn dx_GetSoundTotalTime(SoundHandle: i32) -> i64;
    pub fn dx_SetLoopPosSoundMem(LoopTime: i64, SoundHandle: i32) -> i32;
    pub fn dx_SetLoopTimePosSoundMem(LoopTime: i64, SoundHandle: i32) -> i32;
    pub fn dx_SetLoopSamplePosSoundMem(LoopSamplePosition: i64, SoundHandle: i32) -> i32;
    pub fn dx_SetLoopStartTimePosSoundMem(LoopStartTime: i64, SoundHandle: i32) -> i32;
    pub fn dx_SetLoopStartSamplePosSoundMem(LoopStartSamplePosition: i64, SoundHandle: i32) -> i32;
    pub fn dx_SetLoopAreaTimePosSoundMem(
        LoopStartTime: i64,
        LoopEndTime: i64,
        SoundHandle: i32,
    ) -> i32;
    pub fn dx_GetLoopAreaTimePosSoundMem(
        LoopStartTime: *mut i64,
        LoopEndTime: *mut i64,
        SoundHandle: i32,
    ) -> i32;
    pub fn dx_SetLoopAreaSamplePosSoundMem(
        LoopStartSamplePosition: i64,
        LoopEndSamplePosition: i64,
        SoundHandle: i32,
    ) -> i32;
    pub fn dx_GetLoopAreaSamplePosSoundMem(
        LoopStartSamplePosition: *mut i64,
        LoopEndSamplePosition: *mut i64,
        SoundHandle: i32,
    ) -> i32;
    pub fn dx_SetPlayFinishDeleteSoundMem(DeleteFlag: i32, SoundHandle: i32) -> i32;
    pub fn dx_Set3DReverbParamSoundMem(Param: *const Sound3dReverbParam, SoundHandle: i32) -> i32;
    pub fn dx_Set3DPresetReverbParamSoundMem(PresetNo: i32, SoundHandle: i32) -> i32;
    pub fn dx_Set3DReverbParamSoundMemAll(
        Param: *const Sound3dReverbParam,
        PlaySoundOnly: i32,
    ) -> i32;
    pub fn dx_Set3DPresetReverbParamSoundMemAll(PresetNo: i32, PlaySoundOnly: i32) -> i32;
    pub fn dx_Get3DReverbParamSoundMem(
        ParamBuffer: *mut Sound3dReverbParam,
        SoundHandle: i32,
    ) -> i32;
    pub fn dx_Get3DPresetReverbParamSoundMem(
        ParamBuffer: *mut Sound3dReverbParam,
        PresetNo: i32,
    ) -> i32;
    pub fn dx_Set3DPositionSoundMem(Position: Vector, SoundHandle: i32) -> i32;
    pub fn dx_Set3DRadiusSoundMem(Radius: f32, SoundHandle: i32) -> i32;
    pub fn dx_Set3DVelocitySoundMem(Velocity: Vector, SoundHandle: i32) -> i32;
    pub fn dx_SetNextPlay3DPositionSoundMem(Position: Vector, SoundHandle: i32) -> i32;
    pub fn dx_SetNextPlay3DRadiusSoundMem(Radius: f32, SoundHandle: i32) -> i32;
    pub fn dx_SetNextPlay3DVelocitySoundMem(Velocity: Vector, SoundHandle: i32) -> i32;
    pub fn dx_GetMP3TagInfo(
        FileName: *const i8,
        TitleBuffer: *mut u8,
        TitleBufferBytes: usize,
        ArtistBuffer: *mut u8,
        ArtistBufferBytes: usize,
        AlbumBuffer: *mut u8,
        AlbumBufferBytes: usize,
        YearBuffer: *mut u8,
        YearBufferBytes: usize,
        CommentBuffer: *mut u8,
        CommentBufferBytes: usize,
        TrackBuffer: *mut u8,
        TrackBufferBytes: usize,
        GenreBuffer: *mut u8,
        GenreBufferBytes: usize,
        PictureGrHandle: *mut i32,
    ) -> i32;
    pub fn dx_GetMP3TagInfoWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        TitleBuffer: *mut u8,
        TitleBufferBytes: usize,
        ArtistBuffer: *mut u8,
        ArtistBufferBytes: usize,
        AlbumBuffer: *mut u8,
        AlbumBufferBytes: usize,
        YearBuffer: *mut u8,
        YearBufferBytes: usize,
        CommentBuffer: *mut u8,
        CommentBufferBytes: usize,
        TrackBuffer: *mut u8,
        TrackBufferBytes: usize,
        GenreBuffer: *mut u8,
        GenreBufferBytes: usize,
        PictureGrHandle: *mut i32,
    ) -> i32;
    pub fn dx_GetOggCommentNum(FileName: *const i8) -> i32;
    pub fn dx_GetOggCommentNumWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_GetOggComment(
        FileName: *const i8,
        CommentIndex: i32,
        CommentNameBuffer: *mut u8,
        CommentNameBufferBytes: usize,
        CommentBuffer: *mut u8,
        CommentBufferBytes: usize,
    ) -> i32;
    pub fn dx_GetOggCommentWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        CommentIndex: i32,
        CommentNameBuffer: *mut u8,
        CommentNameBufferBytes: usize,
        CommentBuffer: *mut u8,
        CommentBufferBytes: usize,
    ) -> i32;
    pub fn dx_SetCreateSoundDataType(SoundDataType: i32) -> i32;
    pub fn dx_GetCreateSoundDataType() -> i32;
    pub fn dx_SetCreateSoundPitchRate(Cents: f32) -> i32;
    pub fn dx_GetCreateSoundPitchRate() -> f32;
    pub fn dx_SetCreateSoundTimeStretchRate(Rate: f32) -> i32;
    pub fn dx_GetCreateSoundTimeStretchRate() -> f32;
    pub fn dx_SetCreateSoundLoopAreaTimePos(LoopStartTime: i64, LoopEndTime: i64) -> i32;
    pub fn dx_GetCreateSoundLoopAreaTimePos(LoopStartTime: *mut i64, LoopEndTime: *mut i64) -> i32;
    pub fn dx_SetCreateSoundLoopAreaSamplePos(
        LoopStartSamplePosition: i64,
        LoopEndSamplePosition: i64,
    ) -> i32;
    pub fn dx_GetCreateSoundLoopAreaSamplePos(
        LoopStartSamplePosition: *mut i64,
        LoopEndSamplePosition: *mut i64,
    ) -> i32;
    pub fn dx_SetCreateSoundIgnoreLoopAreaInfo(IgnoreFlag: i32) -> i32;
    pub fn dx_GetCreateSoundIgnoreLoopAreaInfo() -> i32;
    pub fn dx_SetDisableReadSoundFunctionMask(Mask: i32) -> i32;
    pub fn dx_GetDisableReadSoundFunctionMask() -> i32;
    pub fn dx_SetEnableSoundCaptureFlag(Flag: i32) -> i32;
    pub fn dx_SetUseOldVolumeCalcFlag(Flag: i32) -> i32;
    pub fn dx_SetSoundCurrentTimeType(Type: i32) -> i32;
    pub fn dx_GetSoundCurrentTimeType() -> i32;
    pub fn dx_SetCreate3DSoundFlag(Flag: i32) -> i32;
    pub fn dx_Set3DSoundOneMetre(Distance: f32) -> i32;
    pub fn dx_Set3DSoundListenerPosAndFrontPos_UpVecY(
        Position: Vector,
        FrontPosition: Vector,
    ) -> i32;
    pub fn dx_Set3DSoundListenerPosAndFrontPosAndUpVec(
        Position: Vector,
        FrontPosition: Vector,
        UpVector: Vector,
    ) -> i32;
    pub fn dx_Set3DSoundListenerVelocity(Velocity: Vector) -> i32;
    pub fn dx_Set3DSoundListenerConeAngle(InnerAngle: f32, OuterAngle: f32) -> i32;
    pub fn dx_Set3DSoundListenerConeVolume(InnerAngleVolume: f32, OuterAngleVolume: f32) -> i32;
    pub fn dx_PlaySoundFile(FileName: *const i8, PlayType: i32) -> i32;
    pub fn dx_PlaySoundFileWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        PlayType: i32,
    ) -> i32;
    pub fn dx_PlaySound(FileName: *const i8, PlayType: i32) -> i32;
    pub fn dx_PlaySoundWithStrLen(FileName: *const i8, FileNameLength: usize, PlayType: i32)
        -> i32;
    pub fn dx_CheckSoundFile() -> i32;
    pub fn dx_CheckSound() -> i32;
    pub fn dx_StopSoundFile() -> i32;
    pub fn dx_StopSound() -> i32;
    pub fn dx_SetVolumeSoundFile(VolumePal: i32) -> i32;
    pub fn dx_SetVolumeSound(VolumePal: i32) -> i32;
    pub fn dx_InitSoftSound() -> i32;
    pub fn dx_LoadSoftSound(FileName: *const i8) -> i32;
    pub fn dx_LoadSoftSoundWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_LoadSoftSoundFromMemImage(
        FileImage: *const libc::c_void,
        FileImageSize: usize,
    ) -> i32;
    pub fn dx_MakeSoftSound(UseFormat_SoftSoundHandle: i32, SampleNum: i64) -> i32;
    pub fn dx_MakeSoftSound2Ch16Bit44KHz(SampleNum: i64) -> i32;
    pub fn dx_MakeSoftSound2Ch16Bit22KHz(SampleNum: i64) -> i32;
    pub fn dx_MakeSoftSound2Ch8Bit44KHz(SampleNum: i64) -> i32;
    pub fn dx_MakeSoftSound2Ch8Bit22KHz(SampleNum: i64) -> i32;
    pub fn dx_MakeSoftSound1Ch16Bit44KHz(SampleNum: i64) -> i32;
    pub fn dx_MakeSoftSound1Ch16Bit22KHz(SampleNum: i64) -> i32;
    pub fn dx_MakeSoftSound1Ch8Bit44KHz(SampleNum: i64) -> i32;
    pub fn dx_MakeSoftSound1Ch8Bit22KHz(SampleNum: i64) -> i32;
    pub fn dx_MakeSoftSoundCustom(
        ChannelNum: i32,
        BitsPerSample: i32,
        SamplesPerSec: i32,
        SampleNum: i64,
        IsFloatType: i32,
    ) -> i32;
    pub fn dx_DeleteSoftSound(SoftSoundHandle: i32) -> i32;
    pub fn dx_SaveSoftSound(SoftSoundHandle: i32, FileName: *const i8) -> i32;
    pub fn dx_SaveSoftSoundWithStrLen(
        SoftSoundHandle: i32,
        FileName: *const i8,
        FileNameLength: usize,
    ) -> i32;
    pub fn dx_GetSoftSoundSampleNum(SoftSoundHandle: i32) -> i64;
    pub fn dx_GetSoftSoundFormat(
        SoftSoundHandle: i32,
        Channels: *mut i32,
        BitsPerSample: *mut i32,
        SamplesPerSec: *mut i32,
        IsFloatType: *mut i32,
    ) -> i32;
    pub fn dx_ReadSoftSoundData(
        SoftSoundHandle: i32,
        SamplePosition: i64,
        Channel1: *mut i32,
        Channel2: *mut i32,
    ) -> i32;
    pub fn dx_ReadSoftSoundDataF(
        SoftSoundHandle: i32,
        SamplePosition: i64,
        Channel1: *mut f32,
        Channel2: *mut f32,
    ) -> i32;
    pub fn dx_WriteSoftSoundData(
        SoftSoundHandle: i32,
        SamplePosition: i64,
        Channel1: i32,
        Channel2: i32,
    ) -> i32;
    pub fn dx_WriteSoftSoundDataF(
        SoftSoundHandle: i32,
        SamplePosition: i64,
        Channel1: f32,
        Channel2: f32,
    ) -> i32;
    pub fn dx_WriteTimeStretchSoftSoundData(
        SrcSoftSoundHandle: i32,
        DestSoftSoundHandle: i32,
    ) -> i32;
    pub fn dx_WritePitchShiftSoftSoundData(
        SrcSoftSoundHandle: i32,
        DestSoftSoundHandle: i32,
    ) -> i32;
    pub fn dx_GetFFTVibrationSoftSound(
        SoftSoundHandle: i32,
        Channel: i32,
        SamplePosition: i64,
        SampleNum: i32,
        Buffer_Array: *mut f32,
        BufferLength: i32,
    ) -> i32;
    pub fn dx_GetFFTVibrationSoftSoundBase(
        SoftSoundHandle: i32,
        Channel: i32,
        SamplePosition: i64,
        SampleNum: i32,
        RealBuffer_Array: *mut f32,
        ImagBuffer_Array: *mut f32,
        BufferLength: i32,
    ) -> i32;
    pub fn dx_InitSoftSoundPlayer() -> i32;
    pub fn dx_MakeSoftSoundPlayer(UseFormat_SoftSoundHandle: i32) -> i32;
    pub fn dx_MakeSoftSoundPlayer2Ch16Bit44KHz() -> i32;
    pub fn dx_MakeSoftSoundPlayer2Ch16Bit22KHz() -> i32;
    pub fn dx_MakeSoftSoundPlayer2Ch8Bit44KHz() -> i32;
    pub fn dx_MakeSoftSoundPlayer2Ch8Bit22KHz() -> i32;
    pub fn dx_MakeSoftSoundPlayer1Ch16Bit44KHz() -> i32;
    pub fn dx_MakeSoftSoundPlayer1Ch16Bit22KHz() -> i32;
    pub fn dx_MakeSoftSoundPlayer1Ch8Bit44KHz() -> i32;
    pub fn dx_MakeSoftSoundPlayer1Ch8Bit22KHz() -> i32;
    pub fn dx_MakeSoftSoundPlayerCustom(
        ChannelNum: i32,
        BitsPerSample: i32,
        SamplesPerSec: i32,
    ) -> i32;
    pub fn dx_DeleteSoftSoundPlayer(SSoundPlayerHandle: i32) -> i32;
    pub fn dx_AddDataSoftSoundPlayer(
        SSoundPlayerHandle: i32,
        SoftSoundHandle: i32,
        AddSamplePosition: i64,
        AddSampleNum: i32,
    ) -> i32;
    pub fn dx_AddDirectDataSoftSoundPlayer(
        SSoundPlayerHandle: i32,
        SoundData: *const libc::c_void,
        AddSampleNum: i32,
    ) -> i32;
    pub fn dx_AddOneDataSoftSoundPlayer(
        SSoundPlayerHandle: i32,
        Channel1: i32,
        Channel2: i32,
    ) -> i32;
    pub fn dx_GetSoftSoundPlayerFormat(
        SSoundPlayerHandle: i32,
        Channels: *mut i32,
        BitsPerSample: *mut i32,
        SamplesPerSec: *mut i32,
    ) -> i32;
    pub fn dx_StartSoftSoundPlayer(SSoundPlayerHandle: i32) -> i32;
    pub fn dx_CheckStartSoftSoundPlayer(SSoundPlayerHandle: i32) -> i32;
    pub fn dx_StopSoftSoundPlayer(SSoundPlayerHandle: i32) -> i32;
    pub fn dx_ResetSoftSoundPlayer(SSoundPlayerHandle: i32) -> i32;
    pub fn dx_GetStockDataLengthSoftSoundPlayer(SSoundPlayerHandle: i32) -> i32;
    pub fn dx_CheckSoftSoundPlayerNoneData(SSoundPlayerHandle: i32) -> i32;
    pub fn dx_DeleteMusicMem(MusicHandle: i32) -> i32;
    pub fn dx_LoadMusicMem(FileName: *const i8) -> i32;
    pub fn dx_LoadMusicMemWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_LoadMusicMemByMemImage(FileImage: *const libc::c_void, FileImageSize: usize) -> i32;
    pub fn dx_PlayMusicMem(MusicHandle: i32, PlayType: i32) -> i32;
    pub fn dx_StopMusicMem(MusicHandle: i32) -> i32;
    pub fn dx_CheckMusicMem(MusicHandle: i32) -> i32;
    pub fn dx_SetVolumeMusicMem(Volume: i32, MusicHandle: i32) -> i32;
    pub fn dx_GetMusicMemPosition(MusicHandle: i32) -> i32;
    pub fn dx_InitMusicMem() -> i32;
    pub fn dx_ProcessMusicMem() -> i32;
    pub fn dx_PlayMusic(FileName: *const i8, PlayType: i32) -> i32;
    pub fn dx_PlayMusicWithStrLen(FileName: *const i8, FileNameLength: usize, PlayType: i32)
        -> i32;
    pub fn dx_PlayMusicByMemImage(
        FileImage: *const libc::c_void,
        FileImageSize: usize,
        PlayType: i32,
    ) -> i32;
    pub fn dx_SetVolumeMusic(Volume: i32) -> i32;
    pub fn dx_StopMusic() -> i32;
    pub fn dx_CheckMusic() -> i32;
    pub fn dx_GetMusicPosition() -> i32;
    pub fn dx_SelectMidiMode(Mode: i32) -> i32;
    pub fn dx_SetUseDXArchiveFlag(Flag: i32) -> i32;
    pub fn dx_SetDXArchivePriority(Priority: i32) -> i32;
    pub fn dx_SetDXArchiveExtension(Extension: *const i8) -> i32;
    pub fn dx_SetDXArchiveExtensionWithStrLen(Extension: *const i8, ExtensionLength: usize) -> i32;
    pub fn dx_SetDXArchiveKeyString(KeyString: *const i8) -> i32;
    pub fn dx_SetDXArchiveKeyStringWithStrLen(KeyString: *const i8, KeyStringLength: usize) -> i32;
    pub fn dx_DXArchivePreLoad(FilePath: *const i8, ASync: i32) -> i32;
    pub fn dx_DXArchivePreLoadWithStrLen(
        FilePath: *const i8,
        FilePathLength: usize,
        ASync: i32,
    ) -> i32;
    pub fn dx_DXArchiveCheckIdle(FilePath: *const i8) -> i32;
    pub fn dx_DXArchiveCheckIdleWithStrLen(FilePath: *const i8, FilePathLength: usize) -> i32;
    pub fn dx_DXArchiveRelease(FilePath: *const i8) -> i32;
    pub fn dx_DXArchiveReleaseWithStrLen(FilePath: *const i8, FilePathLength: usize) -> i32;
    pub fn dx_DXArchiveCheckFile(FilePath: *const i8, TargetFilePath: *const i8) -> i32;
    pub fn dx_DXArchiveCheckFileWithStrLen(
        FilePath: *const i8,
        FilePathLength: usize,
        TargetFilePath: *const i8,
        TargetFilePathLength: usize,
    ) -> i32;
    pub fn dx_DXArchiveSetMemImage(
        ArchiveImage: *mut libc::c_void,
        ArchiveImageSize: i32,
        EmulateFilePath: *const i8,
        ArchiveImageCopyFlag: i32,
        ArchiveImageReadOnly: i32,
    ) -> i32;
    pub fn dx_DXArchiveSetMemImageWithStrLen(
        ArchiveImage: *mut libc::c_void,
        ArchiveImageSize: i32,
        EmulateFilePath: *const i8,
        EmulateFilePathLength: usize,
        ArchiveImageCopyFlag: i32,
        ArchiveImageReadOnly: i32,
    ) -> i32;
    pub fn dx_DXArchiveReleaseMemImage(ArchiveImage: *mut libc::c_void) -> i32;
    pub fn dx_HashCRC32(SrcData: *const libc::c_void, SrcDataSize: usize) -> u32;
    pub fn dx_MV1LoadModel(FileName: *const i8) -> i32;
    pub fn dx_MV1LoadModelWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_MV1LoadModelFromMem(
        FileImage: *const libc::c_void,
        FileSize: i32,
        FilePath: *mut libc::c_void,
        FileReleaseFunc: *mut libc::c_void,
        FileReadFuncData: *mut libc::c_void,
    ) -> i32;
    pub fn dx_MV1DuplicateModel(SrcMHandle: i32) -> i32;
    pub fn dx_MV1CreateCloneModel(SrcMHandle: i32) -> i32;
    pub fn dx_MV1DeleteModel(MHandle: i32) -> i32;
    pub fn dx_MV1InitModel() -> i32;
    pub fn dx_MV1SetLoadModelReMakeNormal(Flag: i32) -> i32;
    pub fn dx_MV1SetLoadModelReMakeNormalSmoothingAngle(SmoothingAngle: f32) -> i32;
    pub fn dx_MV1SetLoadModelIgnoreScaling(Flag: i32) -> i32;
    pub fn dx_MV1SetLoadModelPositionOptimize(Flag: i32) -> i32;
    pub fn dx_MV1SetLoadModelNotEqNormalSide_AddZeroAreaPolygon(Flag: i32) -> i32;
    pub fn dx_MV1SetLoadModelUsePhysicsMode(PhysicsMode: i32) -> i32;
    pub fn dx_MV1SetLoadModelPhysicsWorldGravity(Gravity: f32) -> i32;
    pub fn dx_MV1GetLoadModelPhysicsWorldGravity() -> f32;
    pub fn dx_MV1SetLoadCalcPhysicsWorldGravity(GravityNo: i32, Gravity: Vector) -> i32;
    pub fn dx_MV1GetLoadCalcPhysicsWorldGravity(GravityNo: i32) -> Vector;
    pub fn dx_MV1SetLoadModelPhysicsCalcPrecision(Precision: i32) -> i32;
    pub fn dx_MV1SetLoadModel_PMD_PMX_AnimationFPSMode(FPSMode: i32) -> i32;
    pub fn dx_MV1AddLoadModelDisablePhysicsNameWord(NameWord: *const i8) -> i32;
    pub fn dx_MV1AddLoadModelDisablePhysicsNameWordWithStrLen(
        NameWord: *const i8,
        NameWordLength: usize,
    ) -> i32;
    pub fn dx_MV1ResetLoadModelDisablePhysicsNameWord() -> i32;
    pub fn dx_MV1SetLoadModelDisablePhysicsNameWordMode(DisableNameWordMode: i32) -> i32;
    pub fn dx_MV1SetLoadModelAnimFilePath(FileName: *const i8) -> i32;
    pub fn dx_MV1SetLoadModelAnimFilePathWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
    ) -> i32;
    pub fn dx_MV1SetLoadModelUsePackDraw(Flag: i32) -> i32;
    pub fn dx_MV1SetLoadModelTriangleListUseMaxBoneNum(UseMaxBoneNum: i32) -> i32;
    pub fn dx_MV1SaveModelToMV1File(
        MHandle: i32,
        FileName: *const i8,
        SaveType: i32,
        AnimMHandle: i32,
        AnimNameCheck: i32,
        Normal8BitFlag: i32,
        Position16BitFlag: i32,
        Weight8BitFlag: i32,
        Anim16BitFlag: i32,
    ) -> i32;
    pub fn dx_MV1SaveModelToMV1FileWithStrLen(
        MHandle: i32,
        FileName: *const i8,
        FileNameLength: usize,
        SaveType: i32,
        AnimMHandle: i32,
        AnimNameCheck: i32,
        Normal8BitFlag: i32,
        Position16BitFlag: i32,
        Weight8BitFlag: i32,
        Anim16BitFlag: i32,
    ) -> i32;
    pub fn dx_MV1SaveModelToXFile(
        MHandle: i32,
        FileName: *const i8,
        SaveType: i32,
        AnimMHandle: i32,
        AnimNameCheck: i32,
    ) -> i32;
    pub fn dx_MV1SaveModelToXFileWithStrLen(
        MHandle: i32,
        FileName: *const i8,
        FileNameLength: usize,
        SaveType: i32,
        AnimMHandle: i32,
        AnimNameCheck: i32,
    ) -> i32;
    pub fn dx_MV1DrawModel(MHandle: i32) -> i32;
    pub fn dx_MV1DrawFrame(MHandle: i32, FrameIndex: i32) -> i32;
    pub fn dx_MV1DrawMesh(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1DrawTriangleList(MHandle: i32, TriangleListIndex: i32) -> i32;
    pub fn dx_MV1DrawModelDebug(
        MHandle: i32,
        Color: u32,
        IsNormalLine: i32,
        NormalLineLength: f32,
        IsPolyLine: i32,
        IsCollisionBox: i32,
    ) -> i32;
    pub fn dx_MV1SetUseOrigShader(UseFlag: i32) -> i32;
    pub fn dx_MV1SetSemiTransDrawMode(DrawMode: i32) -> i32;
    pub fn dx_MV1GetLocalWorldMatrix(MHandle: i32) -> Matrix;
    pub fn dx_MV1GetLocalWorldMatrixD(MHandle: i32) -> DMatrix;
    pub fn dx_MV1SetPosition(MHandle: i32, Position: Vector) -> i32;
    pub fn dx_MV1SetPositionD(MHandle: i32, Position: DVector) -> i32;
    pub fn dx_MV1GetPosition(MHandle: i32) -> Vector;
    pub fn dx_MV1GetPositionD(MHandle: i32) -> DVector;
    pub fn dx_MV1SetScale(MHandle: i32, Scale: Vector) -> i32;
    pub fn dx_MV1GetScale(MHandle: i32) -> Vector;
    pub fn dx_MV1SetRotationXYZ(MHandle: i32, Rotate: Vector) -> i32;
    pub fn dx_MV1GetRotationXYZ(MHandle: i32) -> Vector;
    pub fn dx_MV1SetRotationZYAxis(
        MHandle: i32,
        ZAxisDirection: Vector,
        YAxisDirection: Vector,
        ZAxisTwistRotate: f32,
    ) -> i32;
    pub fn dx_MV1SetRotationYUseDir(MHandle: i32, Direction: Vector, OffsetYAngle: f32) -> i32;
    pub fn dx_MV1SetRotationMatrix(MHandle: i32, Matrix: Matrix) -> i32;
    pub fn dx_MV1GetRotationMatrix(MHandle: i32) -> Matrix;
    pub fn dx_MV1SetMatrix(MHandle: i32, Matrix: Matrix) -> i32;
    pub fn dx_MV1SetMatrixD(MHandle: i32, Matrix: DMatrix) -> i32;
    pub fn dx_MV1GetMatrix(MHandle: i32) -> Matrix;
    pub fn dx_MV1GetMatrixD(MHandle: i32) -> DMatrix;
    pub fn dx_MV1SetVisible(MHandle: i32, VisibleFlag: i32) -> i32;
    pub fn dx_MV1GetVisible(MHandle: i32) -> i32;
    pub fn dx_MV1SetMeshCategoryVisible(MHandle: i32, MeshCategory: i32, VisibleFlag: i32) -> i32;
    pub fn dx_MV1GetMeshCategoryVisible(MHandle: i32, MeshCategory: i32) -> i32;
    pub fn dx_MV1SetDifColorScale(MHandle: i32, Scale: ColorF32) -> i32;
    pub fn dx_MV1GetDifColorScale(MHandle: i32) -> ColorF32;
    pub fn dx_MV1SetSpcColorScale(MHandle: i32, Scale: ColorF32) -> i32;
    pub fn dx_MV1GetSpcColorScale(MHandle: i32) -> ColorF32;
    pub fn dx_MV1SetEmiColorScale(MHandle: i32, Scale: ColorF32) -> i32;
    pub fn dx_MV1GetEmiColorScale(MHandle: i32) -> ColorF32;
    pub fn dx_MV1SetAmbColorScale(MHandle: i32, Scale: ColorF32) -> i32;
    pub fn dx_MV1GetAmbColorScale(MHandle: i32) -> ColorF32;
    pub fn dx_MV1GetSemiTransState(MHandle: i32) -> i32;
    pub fn dx_MV1SetOpacityRate(MHandle: i32, Rate: f32) -> i32;
    pub fn dx_MV1GetOpacityRate(MHandle: i32) -> f32;
    pub fn dx_MV1SetUseDrawMulAlphaColor(MHandle: i32, Flag: i32) -> i32;
    pub fn dx_MV1GetUseDrawMulAlphaColor(MHandle: i32) -> i32;
    pub fn dx_MV1SetUseZBuffer(MHandle: i32, Flag: i32) -> i32;
    pub fn dx_MV1SetWriteZBuffer(MHandle: i32, Flag: i32) -> i32;
    pub fn dx_MV1SetZBufferCmpType(MHandle: i32, CmpType: i32) -> i32;
    pub fn dx_MV1SetZBias(MHandle: i32, Bias: i32) -> i32;
    pub fn dx_MV1SetUseVertDifColor(MHandle: i32, UseFlag: i32) -> i32;
    pub fn dx_MV1SetUseVertSpcColor(MHandle: i32, UseFlag: i32) -> i32;
    pub fn dx_MV1SetSampleFilterMode(MHandle: i32, FilterMode: i32) -> i32;
    pub fn dx_MV1SetMaxAnisotropy(MHandle: i32, MaxAnisotropy: i32) -> i32;
    pub fn dx_MV1SetWireFrameDrawFlag(MHandle: i32, Flag: i32) -> i32;
    pub fn dx_MV1RefreshVertColorFromMaterial(MHandle: i32) -> i32;
    pub fn dx_MV1SetPhysicsWorldGravity(MHandle: i32, Gravity: Vector) -> i32;
    pub fn dx_MV1PhysicsCalculation(MHandle: i32, MillisecondTime: f32) -> i32;
    pub fn dx_MV1PhysicsResetState(MHandle: i32) -> i32;
    pub fn dx_MV1SetUseShapeFlag(MHandle: i32, UseFlag: i32) -> i32;
    pub fn dx_MV1GetMaterialNumberOrderFlag(MHandle: i32) -> i32;
    pub fn dx_MV1AttachAnim(
        MHandle: i32,
        AnimIndex: i32,
        AnimSrcMHandle: i32,
        NameCheck: i32,
    ) -> i32;
    pub fn dx_MV1DetachAnim(MHandle: i32, AttachIndex: i32) -> i32;
    pub fn dx_MV1SetAttachAnimTime(MHandle: i32, AttachIndex: i32, Time: f32) -> i32;
    pub fn dx_MV1GetAttachAnimTime(MHandle: i32, AttachIndex: i32) -> f32;
    pub fn dx_MV1GetAttachAnimTotalTime(MHandle: i32, AttachIndex: i32) -> f32;
    pub fn dx_MV1SetAttachAnimBlendRate(MHandle: i32, AttachIndex: i32, Rate: f32) -> i32;
    pub fn dx_MV1GetAttachAnimBlendRate(MHandle: i32, AttachIndex: i32) -> f32;
    pub fn dx_MV1SetAttachAnimBlendRateToFrame(
        MHandle: i32,
        AttachIndex: i32,
        FrameIndex: i32,
        Rate: f32,
        SetChild: i32,
    ) -> i32;
    pub fn dx_MV1GetAttachAnimBlendRateToFrame(
        MHandle: i32,
        AttachIndex: i32,
        FrameIndex: i32,
    ) -> f32;
    pub fn dx_MV1GetAttachAnim(MHandle: i32, AttachIndex: i32) -> i32;
    pub fn dx_MV1SetAttachAnimUseShapeFlag(MHandle: i32, AttachIndex: i32, UseFlag: i32) -> i32;
    pub fn dx_MV1GetAttachAnimUseShapeFlag(MHandle: i32, AttachIndex: i32) -> i32;
    pub fn dx_MV1GetAttachAnimFrameLocalPosition(
        MHandle: i32,
        AttachIndex: i32,
        FrameIndex: i32,
    ) -> Vector;
    pub fn dx_MV1GetAttachAnimFrameLocalMatrix(
        MHandle: i32,
        AttachIndex: i32,
        FrameIndex: i32,
    ) -> Matrix;
    pub fn dx_MV1GetAnimNum(MHandle: i32) -> i32;
    pub fn dx_MV1SetAnimName(MHandle: i32, AnimIndex: i32, AnimName: *const i8) -> i32;
    pub fn dx_MV1SetAnimNameWithStrLen(
        MHandle: i32,
        AnimIndex: i32,
        AnimName: *const i8,
        AnimNameLength: usize,
    ) -> i32;
    pub fn dx_MV1GetAnimIndex(MHandle: i32, AnimName: *const i8) -> i32;
    pub fn dx_MV1GetAnimIndexWithStrLen(
        MHandle: i32,
        AnimName: *const i8,
        AnimNameLength: usize,
    ) -> i32;
    pub fn dx_MV1GetAnimTotalTime(MHandle: i32, AnimIndex: i32) -> f32;
    pub fn dx_MV1GetAnimTargetFrameNum(MHandle: i32, AnimIndex: i32) -> i32;
    pub fn dx_MV1GetAnimTargetFrame(MHandle: i32, AnimIndex: i32, AnimFrameIndex: i32) -> i32;
    pub fn dx_MV1GetAnimTargetFrameKeySetNum(
        MHandle: i32,
        AnimIndex: i32,
        AnimFrameIndex: i32,
    ) -> i32;
    pub fn dx_MV1GetAnimTargetFrameKeySet(
        MHandle: i32,
        AnimIndex: i32,
        AnimFrameIndex: i32,
        Index: i32,
    ) -> i32;
    pub fn dx_MV1GetAnimKeySetNum(MHandle: i32) -> i32;
    pub fn dx_MV1GetAnimKeySetType(MHandle: i32, AnimKeySetIndex: i32) -> i32;
    pub fn dx_MV1GetAnimKeySetDataType(MHandle: i32, AnimKeySetIndex: i32) -> i32;
    pub fn dx_MV1GetAnimKeySetTimeType(MHandle: i32, AnimKeySetIndex: i32) -> i32;
    pub fn dx_MV1GetAnimKeySetDataNum(MHandle: i32, AnimKeySetIndex: i32) -> i32;
    pub fn dx_MV1GetAnimKeyDataTime(MHandle: i32, AnimKeySetIndex: i32, Index: i32) -> f32;
    pub fn dx_MV1GetAnimKeyDataIndexFromTime(MHandle: i32, AnimKeySetIndex: i32, Time: f32) -> i32;
    pub fn dx_MV1GetAnimKeyDataToQuaternion(
        MHandle: i32,
        AnimKeySetIndex: i32,
        Index: i32,
    ) -> Float4;
    pub fn dx_MV1GetAnimKeyDataToQuaternionFromTime(
        MHandle: i32,
        AnimKeySetIndex: i32,
        Time: f32,
    ) -> Float4;
    pub fn dx_MV1GetAnimKeyDataToVector(MHandle: i32, AnimKeySetIndex: i32, Index: i32) -> Vector;
    pub fn dx_MV1GetAnimKeyDataToVectorFromTime(
        MHandle: i32,
        AnimKeySetIndex: i32,
        Time: f32,
    ) -> Vector;
    pub fn dx_MV1GetAnimKeyDataToMatrix(MHandle: i32, AnimKeySetIndex: i32, Index: i32) -> Matrix;
    pub fn dx_MV1GetAnimKeyDataToMatrixFromTime(
        MHandle: i32,
        AnimKeySetIndex: i32,
        Time: f32,
    ) -> Matrix;
    pub fn dx_MV1GetAnimKeyDataToFlat(MHandle: i32, AnimKeySetIndex: i32, Index: i32) -> f32;
    pub fn dx_MV1GetAnimKeyDataToFlatFromTime(MHandle: i32, AnimKeySetIndex: i32, Time: f32)
        -> f32;
    pub fn dx_MV1GetAnimKeyDataToLinear(MHandle: i32, AnimKeySetIndex: i32, Index: i32) -> f32;
    pub fn dx_MV1GetAnimKeyDataToLinearFromTime(
        MHandle: i32,
        AnimKeySetIndex: i32,
        Time: f32,
    ) -> f32;
    pub fn dx_MV1GetMaterialNum(MHandle: i32) -> i32;
    pub fn dx_MV1SetMaterialTypeAll(MHandle: i32, Type: i32) -> i32;
    pub fn dx_MV1SetMaterialType(MHandle: i32, MaterialIndex: i32, Type: i32) -> i32;
    pub fn dx_MV1GetMaterialType(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1SetMaterialDifColor(MHandle: i32, MaterialIndex: i32, Color: ColorF32) -> i32;
    pub fn dx_MV1GetMaterialDifColor(MHandle: i32, MaterialIndex: i32) -> ColorF32;
    pub fn dx_MV1SetMaterialSpcColor(MHandle: i32, MaterialIndex: i32, Color: ColorF32) -> i32;
    pub fn dx_MV1GetMaterialSpcColor(MHandle: i32, MaterialIndex: i32) -> ColorF32;
    pub fn dx_MV1SetMaterialEmiColor(MHandle: i32, MaterialIndex: i32, Color: ColorF32) -> i32;
    pub fn dx_MV1GetMaterialEmiColor(MHandle: i32, MaterialIndex: i32) -> ColorF32;
    pub fn dx_MV1SetMaterialAmbColor(MHandle: i32, MaterialIndex: i32, Color: ColorF32) -> i32;
    pub fn dx_MV1GetMaterialAmbColor(MHandle: i32, MaterialIndex: i32) -> ColorF32;
    pub fn dx_MV1SetMaterialSpcPower(MHandle: i32, MaterialIndex: i32, Power: f32) -> i32;
    pub fn dx_MV1GetMaterialSpcPower(MHandle: i32, MaterialIndex: i32) -> f32;
    pub fn dx_MV1SetMaterialDifMapTexture(MHandle: i32, MaterialIndex: i32, TexIndex: i32) -> i32;
    pub fn dx_MV1GetMaterialDifMapTexture(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1SetMaterialSubDifMapTexture(
        MHandle: i32,
        MaterialIndex: i32,
        TexIndex: i32,
    ) -> i32;
    pub fn dx_MV1GetMaterialSubDifMapTexture(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1SetMaterialSpcMapTexture(MHandle: i32, MaterialIndex: i32, TexIndex: i32) -> i32;
    pub fn dx_MV1GetMaterialSpcMapTexture(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1GetMaterialNormalMapTexture(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1SetMaterialDifGradTexture(MHandle: i32, MaterialIndex: i32, TexIndex: i32) -> i32;
    pub fn dx_MV1GetMaterialDifGradTexture(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1SetMaterialSpcGradTexture(MHandle: i32, MaterialIndex: i32, TexIndex: i32) -> i32;
    pub fn dx_MV1GetMaterialSpcGradTexture(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1SetMaterialSphereMapTexture(
        MHandle: i32,
        MaterialIndex: i32,
        TexIndex: i32,
    ) -> i32;
    pub fn dx_MV1GetMaterialSphereMapTexture(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1SetMaterialDifGradBlendTypeAll(MHandle: i32, BlendType: i32) -> i32;
    pub fn dx_MV1SetMaterialDifGradBlendType(
        MHandle: i32,
        MaterialIndex: i32,
        BlendType: i32,
    ) -> i32;
    pub fn dx_MV1GetMaterialDifGradBlendType(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1SetMaterialSpcGradBlendTypeAll(MHandle: i32, BlendType: i32) -> i32;
    pub fn dx_MV1SetMaterialSpcGradBlendType(
        MHandle: i32,
        MaterialIndex: i32,
        BlendType: i32,
    ) -> i32;
    pub fn dx_MV1GetMaterialSpcGradBlendType(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1SetMaterialSphereMapBlendTypeAll(MHandle: i32, BlendType: i32) -> i32;
    pub fn dx_MV1SetMaterialSphereMapBlendType(
        MHandle: i32,
        MaterialIndex: i32,
        BlendType: i32,
    ) -> i32;
    pub fn dx_MV1GetMaterialSphereMapBlendType(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1SetMaterialOutLineWidthAll(MHandle: i32, Width: f32) -> i32;
    pub fn dx_MV1SetMaterialOutLineWidth(MHandle: i32, MaterialIndex: i32, Width: f32) -> i32;
    pub fn dx_MV1GetMaterialOutLineWidth(MHandle: i32, MaterialIndex: i32) -> f32;
    pub fn dx_MV1SetMaterialOutLineDotWidthAll(MHandle: i32, Width: f32) -> i32;
    pub fn dx_MV1SetMaterialOutLineDotWidth(MHandle: i32, MaterialIndex: i32, Width: f32) -> i32;
    pub fn dx_MV1GetMaterialOutLineDotWidth(MHandle: i32, MaterialIndex: i32) -> f32;
    pub fn dx_MV1SetMaterialOutLineColorAll(MHandle: i32, Color: ColorF32) -> i32;
    pub fn dx_MV1SetMaterialOutLineColor(MHandle: i32, MaterialIndex: i32, Color: ColorF32) -> i32;
    pub fn dx_MV1GetMaterialOutLineColor(MHandle: i32, MaterialIndex: i32) -> ColorF32;
    pub fn dx_MV1SetMaterialDrawBlendModeAll(MHandle: i32, BlendMode: i32) -> i32;
    pub fn dx_MV1SetMaterialDrawBlendMode(MHandle: i32, MaterialIndex: i32, BlendMode: i32) -> i32;
    pub fn dx_MV1GetMaterialDrawBlendMode(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1SetMaterialDrawBlendParamAll(MHandle: i32, BlendParam: i32) -> i32;
    pub fn dx_MV1SetMaterialDrawBlendParam(
        MHandle: i32,
        MaterialIndex: i32,
        BlendParam: i32,
    ) -> i32;
    pub fn dx_MV1GetMaterialDrawBlendParam(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1SetMaterialDrawAlphaTestAll(
        MHandle: i32,
        Enable: i32,
        Mode: i32,
        Param: i32,
    ) -> i32;
    pub fn dx_MV1SetMaterialDrawAlphaTest(
        MHandle: i32,
        MaterialIndex: i32,
        Enable: i32,
        Mode: i32,
        Param: i32,
    ) -> i32;
    pub fn dx_MV1GetMaterialDrawAlphaTestEnable(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1GetMaterialDrawAlphaTestMode(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1GetMaterialDrawAlphaTestParam(MHandle: i32, MaterialIndex: i32) -> i32;
    pub fn dx_MV1GetTextureNum(MHandle: i32) -> i32;
    pub fn dx_MV1SetTextureColorFilePath(MHandle: i32, TexIndex: i32, FilePath: *const i8) -> i32;
    pub fn dx_MV1SetTextureColorFilePathWithStrLen(
        MHandle: i32,
        TexIndex: i32,
        FilePath: *const i8,
        FilePathLength: usize,
    ) -> i32;
    pub fn dx_MV1SetTextureAlphaFilePath(MHandle: i32, TexIndex: i32, FilePath: *const i8) -> i32;
    pub fn dx_MV1SetTextureAlphaFilePathWithStrLen(
        MHandle: i32,
        TexIndex: i32,
        FilePath: *const i8,
        FilePathLength: usize,
    ) -> i32;
    pub fn dx_MV1SetTextureGraphHandle(
        MHandle: i32,
        TexIndex: i32,
        GrHandle: i32,
        SemiTransFlag: i32,
    ) -> i32;
    pub fn dx_MV1GetTextureGraphHandle(MHandle: i32, TexIndex: i32) -> i32;
    pub fn dx_MV1SetTextureAddressMode(
        MHandle: i32,
        TexIndex: i32,
        AddrUMode: i32,
        AddrVMode: i32,
    ) -> i32;
    pub fn dx_MV1GetTextureAddressModeU(MHandle: i32, TexIndex: i32) -> i32;
    pub fn dx_MV1GetTextureAddressModeV(MHandle: i32, TexIndex: i32) -> i32;
    pub fn dx_MV1GetTextureWidth(MHandle: i32, TexIndex: i32) -> i32;
    pub fn dx_MV1GetTextureHeight(MHandle: i32, TexIndex: i32) -> i32;
    pub fn dx_MV1GetTextureSemiTransState(MHandle: i32, TexIndex: i32) -> i32;
    pub fn dx_MV1SetTextureBumpImageFlag(MHandle: i32, TexIndex: i32, Flag: i32) -> i32;
    pub fn dx_MV1GetTextureBumpImageFlag(MHandle: i32, TexIndex: i32) -> i32;
    pub fn dx_MV1SetTextureBumpImageNextPixelLength(
        MHandle: i32,
        TexIndex: i32,
        Length: f32,
    ) -> i32;
    pub fn dx_MV1GetTextureBumpImageNextPixelLength(MHandle: i32, TexIndex: i32) -> f32;
    pub fn dx_MV1SetTextureSampleFilterMode(MHandle: i32, TexIndex: i32, FilterMode: i32) -> i32;
    pub fn dx_MV1GetTextureSampleFilterMode(MHandle: i32, TexIndex: i32) -> i32;
    pub fn dx_MV1LoadTexture(FilePath: *const i8) -> i32;
    pub fn dx_MV1LoadTextureWithStrLen(FilePath: *const i8, FilePathLength: usize) -> i32;
    pub fn dx_MV1GetFrameNum(MHandle: i32) -> i32;
    pub fn dx_MV1SearchFrame(MHandle: i32, FrameName: *const i8) -> i32;
    pub fn dx_MV1SearchFrameWithStrLen(
        MHandle: i32,
        FrameName: *const i8,
        FrameNameLength: usize,
    ) -> i32;
    pub fn dx_MV1SearchFrameChild(MHandle: i32, FrameIndex: i32, ChildName: *const i8) -> i32;
    pub fn dx_MV1SearchFrameChildWithStrLen(
        MHandle: i32,
        FrameIndex: i32,
        ChildName: *const i8,
        ChildNameLength: usize,
    ) -> i32;
    pub fn dx_MV1GetFrameName2(MHandle: i32, FrameIndex: i32, StrBuffer: *mut u8) -> i32;
    pub fn dx_MV1GetFrameParent(MHandle: i32, FrameIndex: i32) -> i32;
    pub fn dx_MV1GetFrameChildNum(MHandle: i32, FrameIndex: i32) -> i32;
    pub fn dx_MV1GetFrameChild(MHandle: i32, FrameIndex: i32, ChildIndex: i32) -> i32;
    pub fn dx_MV1GetFramePosition(MHandle: i32, FrameIndex: i32) -> Vector;
    pub fn dx_MV1GetFramePositionD(MHandle: i32, FrameIndex: i32) -> DVector;
    pub fn dx_MV1GetFrameBaseLocalMatrix(MHandle: i32, FrameIndex: i32) -> Matrix;
    pub fn dx_MV1GetFrameBaseLocalMatrixD(MHandle: i32, FrameIndex: i32) -> DMatrix;
    pub fn dx_MV1GetFrameLocalMatrix(MHandle: i32, FrameIndex: i32) -> Matrix;
    pub fn dx_MV1GetFrameLocalMatrixD(MHandle: i32, FrameIndex: i32) -> DMatrix;
    pub fn dx_MV1GetFrameLocalWorldMatrix(MHandle: i32, FrameIndex: i32) -> Matrix;
    pub fn dx_MV1GetFrameLocalWorldMatrixD(MHandle: i32, FrameIndex: i32) -> DMatrix;
    pub fn dx_MV1SetFrameUserLocalMatrix(MHandle: i32, FrameIndex: i32, Matrix: Matrix) -> i32;
    pub fn dx_MV1SetFrameUserLocalMatrixD(MHandle: i32, FrameIndex: i32, Matrix: DMatrix) -> i32;
    pub fn dx_MV1ResetFrameUserLocalMatrix(MHandle: i32, FrameIndex: i32) -> i32;
    pub fn dx_MV1SetFrameUserLocalWorldMatrix(MHandle: i32, FrameIndex: i32, Matrix: Matrix)
        -> i32;
    pub fn dx_MV1SetFrameUserLocalWorldMatrixD(
        MHandle: i32,
        FrameIndex: i32,
        Matrix: DMatrix,
    ) -> i32;
    pub fn dx_MV1ResetFrameUserLocalWorldMatrix(MHandle: i32, FrameIndex: i32) -> i32;
    pub fn dx_MV1GetFrameMaxVertexLocalPosition(MHandle: i32, FrameIndex: i32) -> Vector;
    pub fn dx_MV1GetFrameMaxVertexLocalPositionD(MHandle: i32, FrameIndex: i32) -> DVector;
    pub fn dx_MV1GetFrameMinVertexLocalPosition(MHandle: i32, FrameIndex: i32) -> Vector;
    pub fn dx_MV1GetFrameMinVertexLocalPositionD(MHandle: i32, FrameIndex: i32) -> DVector;
    pub fn dx_MV1GetFrameAvgVertexLocalPosition(MHandle: i32, FrameIndex: i32) -> Vector;
    pub fn dx_MV1GetFrameAvgVertexLocalPositionD(MHandle: i32, FrameIndex: i32) -> DVector;
    pub fn dx_MV1GetFrameVertexNum(MHandle: i32, FrameIndex: i32) -> i32;
    pub fn dx_MV1GetFrameTriangleNum(MHandle: i32, FrameIndex: i32) -> i32;
    pub fn dx_MV1GetFrameMeshNum(MHandle: i32, FrameIndex: i32) -> i32;
    pub fn dx_MV1GetFrameMesh(MHandle: i32, FrameIndex: i32, Index: i32) -> i32;
    pub fn dx_MV1SetFrameVisible(MHandle: i32, FrameIndex: i32, VisibleFlag: i32) -> i32;
    pub fn dx_MV1GetFrameVisible(MHandle: i32, FrameIndex: i32) -> i32;
    pub fn dx_MV1SetFrameDifColorScale(MHandle: i32, FrameIndex: i32, Scale: ColorF32) -> i32;
    pub fn dx_MV1SetFrameSpcColorScale(MHandle: i32, FrameIndex: i32, Scale: ColorF32) -> i32;
    pub fn dx_MV1SetFrameEmiColorScale(MHandle: i32, FrameIndex: i32, Scale: ColorF32) -> i32;
    pub fn dx_MV1SetFrameAmbColorScale(MHandle: i32, FrameIndex: i32, Scale: ColorF32) -> i32;
    pub fn dx_MV1GetFrameDifColorScale(MHandle: i32, FrameIndex: i32) -> ColorF32;
    pub fn dx_MV1GetFrameSpcColorScale(MHandle: i32, FrameIndex: i32) -> ColorF32;
    pub fn dx_MV1GetFrameEmiColorScale(MHandle: i32, FrameIndex: i32) -> ColorF32;
    pub fn dx_MV1GetFrameAmbColorScale(MHandle: i32, FrameIndex: i32) -> ColorF32;
    pub fn dx_MV1GetFrameSemiTransState(MHandle: i32, FrameIndex: i32) -> i32;
    pub fn dx_MV1SetFrameOpacityRate(MHandle: i32, FrameIndex: i32, Rate: f32) -> i32;
    pub fn dx_MV1GetFrameOpacityRate(MHandle: i32, FrameIndex: i32) -> f32;
    pub fn dx_MV1SetFrameBaseVisible(MHandle: i32, FrameIndex: i32, VisibleFlag: i32) -> i32;
    pub fn dx_MV1GetFrameBaseVisible(MHandle: i32, FrameIndex: i32) -> i32;
    pub fn dx_MV1SetFrameTextureAddressTransform(
        MHandle: i32,
        FrameIndex: i32,
        TransU: f32,
        TransV: f32,
        ScaleU: f32,
        ScaleV: f32,
        RotCenterU: f32,
        RotCenterV: f32,
        Rotate: f32,
    ) -> i32;
    pub fn dx_MV1SetFrameTextureAddressTransformMatrix(
        MHandle: i32,
        FrameIndex: i32,
        Matrix: Matrix,
    ) -> i32;
    pub fn dx_MV1ResetFrameTextureAddressTransform(MHandle: i32, FrameIndex: i32) -> i32;
    pub fn dx_MV1GetMeshNum(MHandle: i32) -> i32;
    pub fn dx_MV1GetMeshMaterial(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1GetMeshVertexNum(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1GetMeshTriangleNum(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1SetMeshVisible(MHandle: i32, MeshIndex: i32, VisibleFlag: i32) -> i32;
    pub fn dx_MV1GetMeshVisible(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1SetMeshDifColorScale(MHandle: i32, MeshIndex: i32, Scale: ColorF32) -> i32;
    pub fn dx_MV1SetMeshSpcColorScale(MHandle: i32, MeshIndex: i32, Scale: ColorF32) -> i32;
    pub fn dx_MV1SetMeshEmiColorScale(MHandle: i32, MeshIndex: i32, Scale: ColorF32) -> i32;
    pub fn dx_MV1SetMeshAmbColorScale(MHandle: i32, MeshIndex: i32, Scale: ColorF32) -> i32;
    pub fn dx_MV1GetMeshDifColorScale(MHandle: i32, MeshIndex: i32) -> ColorF32;
    pub fn dx_MV1GetMeshSpcColorScale(MHandle: i32, MeshIndex: i32) -> ColorF32;
    pub fn dx_MV1GetMeshEmiColorScale(MHandle: i32, MeshIndex: i32) -> ColorF32;
    pub fn dx_MV1GetMeshAmbColorScale(MHandle: i32, MeshIndex: i32) -> ColorF32;
    pub fn dx_MV1SetMeshOpacityRate(MHandle: i32, MeshIndex: i32, Rate: f32) -> i32;
    pub fn dx_MV1GetMeshOpacityRate(MHandle: i32, MeshIndex: i32) -> f32;
    pub fn dx_MV1SetMeshDrawBlendMode(MHandle: i32, MeshIndex: i32, BlendMode: i32) -> i32;
    pub fn dx_MV1SetMeshDrawBlendParam(MHandle: i32, MeshIndex: i32, BlendParam: i32) -> i32;
    pub fn dx_MV1GetMeshDrawBlendMode(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1GetMeshDrawBlendParam(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1SetMeshBaseVisible(MHandle: i32, MeshIndex: i32, VisibleFlag: i32) -> i32;
    pub fn dx_MV1GetMeshBaseVisible(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1SetMeshBackCulling(MHandle: i32, MeshIndex: i32, CullingFlag: i32) -> i32;
    pub fn dx_MV1GetMeshBackCulling(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1GetMeshMaxPosition(MHandle: i32, MeshIndex: i32) -> Vector;
    pub fn dx_MV1GetMeshMinPosition(MHandle: i32, MeshIndex: i32) -> Vector;
    pub fn dx_MV1GetMeshTListNum(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1GetMeshTList(MHandle: i32, MeshIndex: i32, Index: i32) -> i32;
    pub fn dx_MV1GetMeshSemiTransState(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1SetMeshUseVertDifColor(MHandle: i32, MeshIndex: i32, UseFlag: i32) -> i32;
    pub fn dx_MV1SetMeshUseVertSpcColor(MHandle: i32, MeshIndex: i32, UseFlag: i32) -> i32;
    pub fn dx_MV1GetMeshUseVertDifColor(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1GetMeshUseVertSpcColor(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1GetMeshShapeFlag(MHandle: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1GetShapeNum(MHandle: i32) -> i32;
    pub fn dx_MV1SearchShape(MHandle: i32, ShapeName: *const i8) -> i32;
    pub fn dx_MV1SearchShapeWithStrLen(
        MHandle: i32,
        ShapeName: *const i8,
        ShapeNameLength: usize,
    ) -> i32;
    pub fn dx_MV1GetShapeTargetMeshNum(MHandle: i32, ShapeIndex: i32) -> i32;
    pub fn dx_MV1GetShapeTargetMesh(MHandle: i32, ShapeIndex: i32, Index: i32) -> i32;
    pub fn dx_MV1SetShapeRate(MHandle: i32, ShapeIndex: i32, Rate: f32, Type: i32) -> i32;
    pub fn dx_MV1GetShapeRate(MHandle: i32, ShapeIndex: i32) -> f32;
    pub fn dx_MV1GetShapeApplyRate(MHandle: i32, ShapeIndex: i32) -> f32;
    pub fn dx_MV1GetTriangleListNum(MHandle: i32) -> i32;
    pub fn dx_MV1GetTriangleListVertexType(MHandle: i32, TListIndex: i32) -> i32;
    pub fn dx_MV1GetTriangleListPolygonNum(MHandle: i32, TListIndex: i32) -> i32;
    pub fn dx_MV1GetTriangleListVertexNum(MHandle: i32, TListIndex: i32) -> i32;
    pub fn dx_MV1GetTriangleListLocalWorldMatrixNum(MHandle: i32, TListIndex: i32) -> i32;
    pub fn dx_MV1GetTriangleListLocalWorldMatrix(
        MHandle: i32,
        TListIndex: i32,
        LWMatrixIndex: i32,
    ) -> Matrix;
    pub fn dx_MV1GetTriangleListPolygonVertexPosition(
        MHandle: i32,
        TListIndex: i32,
        PolygonIndex: i32,
        VertexPositionArray: *mut Vector,
        MatrixWeightArray: *mut f32,
    ) -> i32;
    pub fn dx_MV1GetTriangleListUseMaterial(MHandle: i32, TListIndex: i32) -> i32;
    pub fn dx_MV1SetupCollInfo(
        MHandle: i32,
        FrameIndex: i32,
        XDivNum: i32,
        YDivNum: i32,
        ZDivNum: i32,
        MeshIndex: i32,
    ) -> i32;
    pub fn dx_MV1TerminateCollInfo(MHandle: i32, FrameIndex: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1RefreshCollInfo(MHandle: i32, FrameIndex: i32, MeshIndex: i32) -> i32;
    pub fn dx_MV1CollCheck_Line(
        MHandle: i32,
        FrameIndex: i32,
        PosStart: Vector,
        PosEnd: Vector,
        MeshIndex: i32,
    ) -> Mv1CollResultPoly;
    pub fn dx_MV1CollCheck_LineDim(
        MHandle: i32,
        FrameIndex: i32,
        PosStart: Vector,
        PosEnd: Vector,
        MeshIndex: i32,
    ) -> Mv1CollResultPolyDim;
    pub fn dx_MV1CollCheck_Sphere(
        MHandle: i32,
        FrameIndex: i32,
        CenterPos: Vector,
        r: f32,
        MeshIndex: i32,
    ) -> Mv1CollResultPolyDim;
    pub fn dx_MV1CollCheck_Capsule(
        MHandle: i32,
        FrameIndex: i32,
        Pos1: Vector,
        Pos2: Vector,
        r: f32,
        MeshIndex: i32,
    ) -> Mv1CollResultPolyDim;
    pub fn dx_MV1CollCheck_Triangle(
        MHandle: i32,
        FrameIndex: i32,
        Pos1: Vector,
        Pos2: Vector,
        Pos3: Vector,
        MeshIndex: i32,
    ) -> Mv1CollResultPolyDim;
    pub fn dx_MV1CollCheck_GetResultPoly(
        ResultPolyDim: Mv1CollResultPolyDim,
        PolyNo: i32,
    ) -> Mv1CollResultPoly;
    pub fn dx_MV1CollResultPolyDimTerminate(ResultPolyDim: Mv1CollResultPolyDim) -> i32;
    pub fn dx_MV1SetupReferenceMesh(
        MHandle: i32,
        FrameIndex: i32,
        IsTransform: i32,
        IsPositionOnly: i32,
        MeshIndex: i32,
    ) -> i32;
    pub fn dx_MV1TerminateReferenceMesh(
        MHandle: i32,
        FrameIndex: i32,
        IsTransform: i32,
        IsPositionOnly: i32,
        MeshIndex: i32,
    ) -> i32;
    pub fn dx_MV1RefreshReferenceMesh(
        MHandle: i32,
        FrameIndex: i32,
        IsTransform: i32,
        IsPositionOnly: i32,
        MeshIndex: i32,
    ) -> i32;
    pub fn dx_MV1GetReferenceMesh(
        MHandle: i32,
        FrameIndex: i32,
        IsTransform: i32,
        IsPositionOnly: i32,
        MeshIndex: i32,
    ) -> Mv1RefPolygonList;
    pub fn dx_Live2D_SetCubism4CoreDLLPath(CoreDLLFilePath: *const i8) -> i32;
    pub fn dx_Live2D_SetCubism4CoreDLLPathWithStrLen(
        CoreDLLFilePath: *const i8,
        CoreDLLFilePathLength: usize,
    ) -> i32;
    pub fn dx_Live2D_SetCubism3CoreDLLPath(CoreDLLFilePath: *const i8) -> i32;
    pub fn dx_Live2D_SetCubism3CoreDLLPathWithStrLen(
        CoreDLLFilePath: *const i8,
        CoreDLLFilePathLength: usize,
    ) -> i32;
    pub fn dx_Live2D_RenderBegin() -> i32;
    pub fn dx_Live2D_RenderEnd() -> i32;
    pub fn dx_Live2D_LoadModel(FilePath: *const i8) -> i32;
    pub fn dx_Live2D_LoadModelWithStrLen(FilePath: *const i8, FilePathLength: usize) -> i32;
    pub fn dx_Live2D_DeleteModel(Live2DModelHandle: i32) -> i32;
    pub fn dx_Live2D_InitModel() -> i32;
    pub fn dx_Live2D_Model_Update(Live2DModelHandle: i32, DeltaTimeSeconds: f32) -> i32;
    pub fn dx_Live2D_Model_SetTranslate(Live2DModelHandle: i32, x: f32, y: f32) -> i32;
    pub fn dx_Live2D_Model_SetExtendRate(Live2DModelHandle: i32, ExRateX: f32, ExRateY: f32)
        -> i32;
    pub fn dx_Live2D_Model_SetRotate(Live2DModelHandle: i32, RotAngle: f32) -> i32;
    pub fn dx_Live2D_Model_Draw(Live2DModelHandle: i32) -> i32;
    pub fn dx_Live2D_Model_StartMotion(Live2DModelHandle: i32, group: *const i8, no: i32) -> i32;
    pub fn dx_Live2D_Model_StartMotionWithStrLen(
        Live2DModelHandle: i32,
        group: *const i8,
        groupLength: usize,
        no: i32,
    ) -> i32;
    pub fn dx_Live2D_Model_IsMotionFinished(Live2DModelHandle: i32) -> i32;
    pub fn dx_Live2D_Model_SetExpression(Live2DModelHandle: i32, expressionID: *const i8) -> i32;
    pub fn dx_Live2D_Model_SetExpressionWithStrLen(
        Live2DModelHandle: i32,
        expressionID: *const i8,
        expressionIDLength: usize,
    ) -> i32;
    pub fn dx_Live2D_Model_HitTest(
        Live2DModelHandle: i32,
        hitAreaName: *const i8,
        x: f32,
        y: f32,
    ) -> i32;
    pub fn dx_Live2D_Model_HitTestWithStrLen(
        Live2DModelHandle: i32,
        hitAreaName: *const i8,
        hitAreaNameLength: usize,
        x: f32,
        y: f32,
    ) -> i32;
    pub fn dx_Live2D_Model_GetParameterCount(Live2DModelHandle: i32) -> i32;
    pub fn dx_Live2D_Model_GetParameterValue(Live2DModelHandle: i32, parameterId: *const i8)
        -> f32;
    pub fn dx_Live2D_Model_GetParameterValueWithStrLen(
        Live2DModelHandle: i32,
        parameterId: *const i8,
        parameterIdLength: usize,
    ) -> f32;
    pub fn dx_Live2D_Model_SetParameterValue(
        Live2DModelHandle: i32,
        parameterId: *const i8,
        value: f32,
    ) -> i32;
    pub fn dx_Live2D_Model_SetParameterValueWithStrLen(
        Live2DModelHandle: i32,
        parameterId: *const i8,
        parameterIdLength: usize,
        value: f32,
    ) -> i32;
    pub fn dx_Live2D_Model_GetHitAreasCount(Live2DModelHandle: i32) -> i32;
    pub fn dx_Live2D_Model_GetExpressionCount(Live2DModelHandle: i32) -> i32;
    pub fn dx_Live2D_Model_GetMotionGroupCount(Live2DModelHandle: i32) -> i32;
    pub fn dx_Live2D_Model_GetMotionCount(Live2DModelHandle: i32, groupName: *const i8) -> i32;
    pub fn dx_Live2D_Model_GetMotionCountWithStrLen(
        Live2DModelHandle: i32,
        groupName: *const i8,
        groupNameLength: usize,
    ) -> i32;
    pub fn dx_Live2D_Model_GetMotionFadeInTimeValue(
        Live2DModelHandle: i32,
        groupName: *const i8,
        index: i32,
    ) -> f32;
    pub fn dx_Live2D_Model_GetMotionFadeInTimeValueWithStrLen(
        Live2DModelHandle: i32,
        groupName: *const i8,
        groupNameLength: usize,
        index: i32,
    ) -> f32;
    pub fn dx_Live2D_Model_GetMotionFadeOutTimeValue(
        Live2DModelHandle: i32,
        groupName: *const i8,
        index: i32,
    ) -> f32;
    pub fn dx_Live2D_Model_GetMotionFadeOutTimeValueWithStrLen(
        Live2DModelHandle: i32,
        groupName: *const i8,
        groupNameLength: usize,
        index: i32,
    ) -> f32;
    pub fn dx_Live2D_Model_GetEyeBlinkParameterCount(Live2DModelHandle: i32) -> i32;
    pub fn dx_Live2D_Model_GetLipSyncParameterCount(Live2DModelHandle: i32) -> i32;
    pub fn dx_GetResourceInfo(
        ResourceName: *const i8,
        ResourceType: *const i8,
        DataPointerP: *mut *mut libc::c_void,
        DataSizeP: *mut usize,
    ) -> i32;
    pub fn dx_GetResourceInfoWithStrLen(
        ResourceName: *const i8,
        ResourceNameLength: usize,
        ResourceType: *const i8,
        ResourceTypeLength: usize,
        DataPointerP: *mut *mut libc::c_void,
        DataSizeP: *mut usize,
    ) -> i32;
    pub fn dx_GetWindowCRect(RectBuf: *mut Rect) -> i32;
    pub fn dx_GetWindowClientRect(RectBuf: *mut Rect) -> i32;
    pub fn dx_GetWindowFrameRect(RectBuf: *mut Rect) -> i32;
    pub fn dx_GetWindowActiveFlag() -> i32;
    pub fn dx_GetWindowMinSizeFlag() -> i32;
    pub fn dx_GetWindowMaxSizeFlag() -> i32;
    pub fn dx_GetActiveFlag() -> i32;
    pub fn dx_GetMainWindowHandle() -> HWND;
    pub fn dx_GetWindowModeFlag() -> i32;
    pub fn dx_GetDefaultState(
        SizeX: *mut i32,
        SizeY: *mut i32,
        ColorBitDepth: *mut i32,
        RefreshRate: *mut i32,
        LeftTopX: *mut i32,
        LeftTopY: *mut i32,
        PixelSizeX: *mut i32,
        PixelSizeY: *mut i32,
        XDpi: *mut i32,
        YDpi: *mut i32,
    ) -> i32;
    pub fn dx_GetMonitorDpi(XDpi: *mut i32, YDpi: *mut i32, MonitorIndex: i32) -> i32;
    pub fn dx_GetNoActiveState(ResetFlag: i32) -> i32;
    pub fn dx_GetMouseDispFlag() -> i32;
    pub fn dx_GetAlwaysRunFlag() -> i32;
    pub fn dx__GetSystemInfo(DxLibVer: *mut i32, DirectXVer: *mut i32, WindowsVer: *mut i32)
        -> i32;
    pub fn dx_GetPcInfo(
        OSString: *mut u8,
        DirectXString: *mut u8,
        CPUString: *mut u8,
        CPUSpeed: *mut i32,
        FreeMemorySize: *mut f64,
        TotalMemorySize: *mut f64,
        VideoDriverFileName: *mut u8,
        VideoDriverString: *mut u8,
        FreeVideoMemorySize: *mut f64,
        TotalVideoMemorySize: *mut f64,
    ) -> i32;
    pub fn dx_GetUseMMXFlag() -> i32;
    pub fn dx_GetUseSSEFlag() -> i32;
    pub fn dx_GetUseSSE2Flag() -> i32;
    pub fn dx_GetWindowCloseFlag() -> i32;
    pub fn dx_GetTaskInstance() -> HINSTANCE;
    pub fn dx_GetUseWindowRgnFlag() -> i32;
    pub fn dx_GetWindowSizeChangeEnableFlag(FitScreen: *mut i32) -> i32;
    pub fn dx_GetWindowSizeExtendRate(ExRateX: *mut f64, ExRateY: *mut f64) -> f64;
    pub fn dx_GetWindowSize(Width: *mut i32, Height: *mut i32) -> i32;
    pub fn dx_GetWindowEdgeWidth(
        LeftWidth: *mut i32,
        RightWidth: *mut i32,
        TopWidth: *mut i32,
        BottomWidth: *mut i32,
    ) -> i32;
    pub fn dx_GetWindowPosition(x: *mut i32, y: *mut i32) -> i32;
    pub fn dx_GetWindowUserCloseFlag(StateResetFlag: i32) -> i32;
    pub fn dx_CheckWindowMaximizeButtonInput(StateResetFlag: i32) -> i32;
    pub fn dx_GetNotDrawFlag() -> i32;
    pub fn dx_GetPaintMessageFlag() -> i32;
    pub fn dx_GetValidHiPerformanceCounter() -> i32;
    pub fn dx_GetInputSystemChar(DeleteFlag: i32) -> u8;
    pub fn dx_ChangeWindowMode(Flag: i32) -> i32;
    pub fn dx_SetUseCharSet(CharSet: i32) -> i32;
    pub fn dx_LoadPauseGraph(FileName: *const i8) -> i32;
    pub fn dx_LoadPauseGraphWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_LoadPauseGraphFromMem(MemImage: *const libc::c_void, MemImageSize: i32) -> i32;
    pub fn dx_SetActiveStateChangeCallBackFunction(
        ActiveState: *mut libc::c_void,
        UserData: *mut libc::c_void,
    ) -> i32;
    pub fn dx_SetWindowText(WindowText: *const i8) -> i32;
    pub fn dx_SetWindowTextWithStrLen(WindowText: *const i8, WindowTextLength: usize) -> i32;
    pub fn dx_SetMainWindowText(WindowText: *const i8) -> i32;
    pub fn dx_SetMainWindowTextWithStrLen(WindowText: *const i8, WindowTextLength: usize) -> i32;
    pub fn dx_SetMainWindowClassName(ClassName: *const i8) -> i32;
    pub fn dx_SetMainWindowClassNameWithStrLen(ClassName: *const i8, ClassNameLength: usize)
        -> i32;
    pub fn dx_SetWindowIconID(ID: i32) -> i32;
    pub fn dx_SetWindowIconHandle(Icon: HICON) -> i32;
    pub fn dx_SetUseASyncChangeWindowModeFunction(
        Flag: i32,
        CallbackFunction: *mut libc::c_void,
        Data: *mut libc::c_void,
    ) -> i32;
    pub fn dx_SetShutdownCallbackFunction(
        CallbackFunction: *mut libc::c_void,
        Data: *mut libc::c_void,
        Message: *const i8,
    ) -> i32;
    pub fn dx_SetWindowStyleMode(Mode: i32) -> i32;
    pub fn dx_SetWindowZOrder(ZType: i32, WindowActivateFlag: i32) -> i32;
    pub fn dx_SetWindowSizeChangeEnableFlag(Flag: i32, FitScreen: i32) -> i32;
    pub fn dx_SetWindowSizeExtendRate(ExRateX: f64, ExRateY: f64) -> i32;
    pub fn dx_SetWindowSize(Width: i32, Height: i32) -> i32;
    pub fn dx_SetWindowMaxSize(MaxWidth: i32, MaxHeight: i32) -> i32;
    pub fn dx_SetWindowMinSize(MinWidth: i32, MinHeight: i32) -> i32;
    pub fn dx_SetWindowPosition(x: i32, y: i32) -> i32;
    pub fn dx_SetSysCommandOffFlag(Flag: i32, HookDllPath: *const i8) -> i32;
    pub fn dx_SetSysCommandOffFlagWithStrLen(
        Flag: i32,
        HookDllPath: *const i8,
        HookDllPathLength: usize,
    ) -> i32;
    pub fn dx_SetWindowMaximizeButtonBehavior(BehaviorType: i32) -> i32;
    pub fn dx_SetHookWinProc(WinProc: WNDPROC) -> i32;
    pub fn dx_SetUseHookWinProcReturnValue(UseFlag: i32) -> i32;
    pub fn dx_SetDoubleStartValidFlag(Flag: i32) -> i32;
    pub fn dx_CheckDoubleStart() -> i32;
    pub fn dx_AddMessageTakeOverWindow(Window: HWND) -> i32;
    pub fn dx_SubMessageTakeOverWindow(Window: HWND) -> i32;
    pub fn dx_SetWindowInitPosition(x: i32, y: i32) -> i32;
    pub fn dx_SetNotWinFlag(Flag: i32) -> i32;
    pub fn dx_SetNotDrawFlag(Flag: i32) -> i32;
    pub fn dx_SetNotSoundFlag(Flag: i32) -> i32;
    pub fn dx_SetNotInputFlag(Flag: i32) -> i32;
    pub fn dx_SetDialogBoxHandle(WindowHandle: HWND) -> i32;
    pub fn dx_SetWindowVisibleFlag(Flag: i32) -> i32;
    pub fn dx_SetWindowMinimizeFlag(Flag: i32) -> i32;
    pub fn dx_SetWindowUserCloseEnableFlag(Flag: i32) -> i32;
    pub fn dx_SetDxLibEndPostQuitMessageFlag(Flag: i32) -> i32;
    pub fn dx_SetUserWindow(WindowHandle: HWND) -> i32;
    pub fn dx_SetUserChildWindow(WindowHandle: HWND) -> i32;
    pub fn dx_SetUserWindowMessageProcessDXLibFlag(Flag: i32) -> i32;
    pub fn dx_SetUseFPUPreserveFlag(Flag: i32) -> i32;
    pub fn dx_SetValidMousePointerWindowOutClientAreaMoveFlag(Flag: i32) -> i32;
    pub fn dx_SetUseBackBufferTransColorFlag(Flag: i32) -> i32;
    pub fn dx_SetUseUpdateLayerdWindowFlag(Flag: i32) -> i32;
    pub fn dx_SetResourceModule(ResourceModule: HMODULE) -> i32;
    pub fn dx_SetUseDxLibWM_PAINTProcess(Flag: i32) -> i32;
    pub fn dx_SetWindows10_WM_CHAR_CancelTime(MilliSecond: i32) -> i32;
    pub fn dx_SetDragFileValidFlag(Flag: i32) -> i32;
    pub fn dx_DragFileInfoClear() -> i32;
    pub fn dx_GetDragFilePath(FilePathBuffer: *mut u8) -> i32;
    pub fn dx_GetDragFileNum() -> i32;
    pub fn dx_CreateRgnFromGraph(
        Width: i32,
        Height: i32,
        MaskData: *const libc::c_void,
        Pitch: i32,
        Byte: i32,
    ) -> HRGN;
    pub fn dx_CreateRgnFromBaseImage(
        BaseImage: *mut BaseImage,
        TransColorR: i32,
        TransColorG: i32,
        TransColorB: i32,
    ) -> HRGN;
    pub fn dx_SetWindowRgnGraph(FileName: *const i8) -> i32;
    pub fn dx_SetWindowRgnGraphWithStrLen(FileName: *const i8, FileNameLength: usize) -> i32;
    pub fn dx_UpdateTransColorWindowRgn() -> i32;
    pub fn dx_SetupToolBar(BitmapName: *const i8, DivNum: i32, ResourceID: i32) -> i32;
    pub fn dx_SetupToolBarWithStrLen(
        BitmapName: *const i8,
        BitmapNameLength: usize,
        DivNum: i32,
        ResourceID: i32,
    ) -> i32;
    pub fn dx_AddToolBarButton(Type: i32, State: i32, ImageIndex: i32, ID: i32) -> i32;
    pub fn dx_AddToolBarSep() -> i32;
    pub fn dx_GetToolBarButtonState(ID: i32) -> i32;
    pub fn dx_SetToolBarButtonState(ID: i32, State: i32) -> i32;
    pub fn dx_DeleteAllToolBarButton() -> i32;
    pub fn dx_SetUseMenuFlag(Flag: i32) -> i32;
    pub fn dx_SetUseKeyAccelFlag(Flag: i32) -> i32;
    pub fn dx_AddKeyAccel(
        ItemName: *const i8,
        ItemID: i32,
        KeyCode: i32,
        CtrlFlag: i32,
        AltFlag: i32,
        ShiftFlag: i32,
    ) -> i32;
    pub fn dx_AddKeyAccelWithStrLen(
        ItemName: *const i8,
        ItemNameLength: usize,
        ItemID: i32,
        KeyCode: i32,
        CtrlFlag: i32,
        AltFlag: i32,
        ShiftFlag: i32,
    ) -> i32;
    pub fn dx_AddKeyAccel_Name(
        ItemName: *const i8,
        KeyCode: i32,
        CtrlFlag: i32,
        AltFlag: i32,
        ShiftFlag: i32,
    ) -> i32;
    pub fn dx_AddKeyAccel_NameWithStrLen(
        ItemName: *const i8,
        ItemNameLength: usize,
        KeyCode: i32,
        CtrlFlag: i32,
        AltFlag: i32,
        ShiftFlag: i32,
    ) -> i32;
    pub fn dx_AddKeyAccel_ID(
        ItemID: i32,
        KeyCode: i32,
        CtrlFlag: i32,
        AltFlag: i32,
        ShiftFlag: i32,
    ) -> i32;
    pub fn dx_ClearKeyAccel() -> i32;
    pub fn dx_AddMenuItem(
        AddType: i32,
        ItemName: *const i8,
        ItemID: i32,
        SeparatorFlag: i32,
        NewItemName: *const i8,
        NewItemID: i32,
    ) -> i32;
    pub fn dx_AddMenuItemWithStrLen(
        AddType: i32,
        ItemName: *const i8,
        ItemNameLength: usize,
        ItemID: i32,
        SeparatorFlag: i32,
        NewItemName: *const i8,
        NewItemNameLength: usize,
        NewItemID: i32,
    ) -> i32;
    pub fn dx_DeleteMenuItem(ItemName: *const i8, ItemID: i32) -> i32;
    pub fn dx_DeleteMenuItemWithStrLen(
        ItemName: *const i8,
        ItemNameLength: usize,
        ItemID: i32,
    ) -> i32;
    pub fn dx_CheckMenuItemSelect(ItemName: *const i8, ItemID: i32) -> i32;
    pub fn dx_CheckMenuItemSelectWithStrLen(
        ItemName: *const i8,
        ItemNameLength: usize,
        ItemID: i32,
    ) -> i32;
    pub fn dx_SetMenuItemEnable(ItemName: *const i8, ItemID: i32, EnableFlag: i32) -> i32;
    pub fn dx_SetMenuItemEnableWithStrLen(
        ItemName: *const i8,
        ItemNameLength: usize,
        ItemID: i32,
        EnableFlag: i32,
    ) -> i32;
    pub fn dx_SetMenuItemMark(ItemName: *const i8, ItemID: i32, Mark: i32) -> i32;
    pub fn dx_SetMenuItemMarkWithStrLen(
        ItemName: *const i8,
        ItemNameLength: usize,
        ItemID: i32,
        Mark: i32,
    ) -> i32;
    pub fn dx_CheckMenuItemSelectAll() -> i32;
    pub fn dx_AddMenuItem_Name(ParentItemName: *const i8, NewItemName: *const i8) -> i32;
    pub fn dx_AddMenuItem_NameWithStrLen(
        ParentItemName: *const i8,
        ParentItemNameLength: usize,
        NewItemName: *const i8,
        NewItemNameLength: usize,
    ) -> i32;
    pub fn dx_AddMenuLine_Name(ParentItemName: *const i8) -> i32;
    pub fn dx_AddMenuLine_NameWithStrLen(
        ParentItemName: *const i8,
        ParentItemNameLength: usize,
    ) -> i32;
    pub fn dx_InsertMenuItem_Name(ItemName: *const i8, NewItemName: *const i8) -> i32;
    pub fn dx_InsertMenuItem_NameWithStrLen(
        ItemName: *const i8,
        ItemNameLength: usize,
        NewItemName: *const i8,
        NewItemNameLength: usize,
    ) -> i32;
    pub fn dx_InsertMenuLine_Name(ItemName: *const i8) -> i32;
    pub fn dx_InsertMenuLine_NameWithStrLen(ItemName: *const i8, ItemNameLength: usize) -> i32;
    pub fn dx_DeleteMenuItem_Name(ItemName: *const i8) -> i32;
    pub fn dx_DeleteMenuItem_NameWithStrLen(ItemName: *const i8, ItemNameLength: usize) -> i32;
    pub fn dx_CheckMenuItemSelect_Name(ItemName: *const i8) -> i32;
    pub fn dx_CheckMenuItemSelect_NameWithStrLen(ItemName: *const i8, ItemNameLength: usize)
        -> i32;
    pub fn dx_SetMenuItemEnable_Name(ItemName: *const i8, EnableFlag: i32) -> i32;
    pub fn dx_SetMenuItemEnable_NameWithStrLen(
        ItemName: *const i8,
        ItemNameLength: usize,
        EnableFlag: i32,
    ) -> i32;
    pub fn dx_SetMenuItemMark_Name(ItemName: *const i8, Mark: i32) -> i32;
    pub fn dx_SetMenuItemMark_NameWithStrLen(
        ItemName: *const i8,
        ItemNameLength: usize,
        Mark: i32,
    ) -> i32;
    pub fn dx_AddMenuItem_ID(ParentItemID: i32, NewItemName: *const i8, NewItemID: i32) -> i32;
    pub fn dx_AddMenuItem_IDWithStrLen(
        ParentItemID: i32,
        NewItemName: *const i8,
        NewItemNameLength: usize,
        NewItemID: i32,
    ) -> i32;
    pub fn dx_AddMenuLine_ID(ParentItemID: i32) -> i32;
    pub fn dx_InsertMenuItem_ID(ItemID: i32, NewItemID: i32) -> i32;
    pub fn dx_InsertMenuLine_ID(ItemID: i32, NewItemID: i32) -> i32;
    pub fn dx_DeleteMenuItem_ID(ItemID: i32) -> i32;
    pub fn dx_CheckMenuItemSelect_ID(ItemID: i32) -> i32;
    pub fn dx_SetMenuItemEnable_ID(ItemID: i32, EnableFlag: i32) -> i32;
    pub fn dx_SetMenuItemMark_ID(ItemID: i32, Mark: i32) -> i32;
    pub fn dx_DeleteMenuItemAll() -> i32;
    pub fn dx_ClearMenuItemSelect() -> i32;
    pub fn dx_GetMenuItemID(ItemName: *const i8) -> i32;
    pub fn dx_GetMenuItemIDWithStrLen(ItemName: *const i8, ItemNameLength: usize) -> i32;
    pub fn dx_GetMenuItemName(ItemID: i32, NameBuffer: *mut u8) -> i32;
    pub fn dx_LoadMenuResource(MenuResourceID: i32) -> i32;
    pub fn dx_SetMenuItemSelectCallBackFunction(CallbackFunction: *mut libc::c_void) -> i32;
    pub fn dx_SetWindowMenu(MenuProc: *mut libc::c_void) -> i32;
    pub fn dx_SetDisplayMenuFlag(Flag: i32) -> i32;
    pub fn dx_GetDisplayMenuFlag() -> i32;
    pub fn dx_GetUseMenuFlag() -> i32;
    pub fn dx_SetAutoMenuDisplayFlag(Flag: i32) -> i32;
    pub fn dx_GetWinSockLastError() -> i32;
    pub fn dx_SetUseTSFFlag(UseFlag: i32) -> i32;
    pub fn dx_SetKeyExclusiveCooperativeLevelFlag(Flag: i32) -> i32;
    pub fn dx_SetKeyboardNotDirectInputFlag(Flag: i32) -> i32;
    pub fn dx_SetUseDirectInputFlag(UseFlag: i32) -> i32;
    pub fn dx_SetDirectInputMouseMode(Mode: i32) -> i32;
    pub fn dx_SetUseXInputFlag(Flag: i32) -> i32;
    pub fn dx_SetUseXboxControllerDirectInputFlag(Flag: i32) -> i32;
    pub fn dx_GetJoypadGUID(
        PadIndex: i32,
        GuidInstanceBuffer: *mut GUID,
        GuidProductBuffer: *mut GUID,
    ) -> i32;
    pub fn dx_GetJoypadName(
        InputType: i32,
        InstanceNameBuffer: *mut u8,
        ProductNameBuffer: *mut u8,
    ) -> i32;
    pub fn dx_ConvertKeyCodeToVirtualKey(KeyCode: i32) -> i32;
    pub fn dx_ConvertVirtualKeyToKeyCode(VirtualKey: i32) -> i32;
    pub fn dx_LoadGraphToResource(ResourceID: i32) -> i32;
    pub fn dx_LoadDivGraphToResource(
        ResourceID: i32,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        i32Array: *mut i32,
    ) -> i32;
    pub fn dx_LoadDivGraphFToResource(
        ResourceID: i32,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: f32,
        YSize: f32,
        i32Array: *mut i32,
    ) -> i32;
    pub fn dx_LoadGraphToResource_2(ResourceName: *const i8, ResourceType: *const i8) -> i32;
    pub fn dx_LoadGraphToResourceWithStrLen(
        ResourceName: *const i8,
        ResourceNameLength: usize,
        ResourceType: *const i8,
        ResourceTypeLength: usize,
    ) -> i32;
    pub fn dx_LoadDivGraphToResource_2(
        ResourceName: *const i8,
        ResourceType: *const i8,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        i32Array: *mut i32,
    ) -> i32;
    pub fn dx_LoadDivGraphToResourceWithStrLen(
        ResourceName: *const i8,
        ResourceNameLength: usize,
        ResourceType: *const i8,
        ResourceTypeLength: usize,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: i32,
        YSize: i32,
        i32Array: *mut i32,
    ) -> i32;
    pub fn dx_LoadDivGraphFToResource_2(
        ResourceName: *const i8,
        ResourceType: *const i8,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: f32,
        YSize: f32,
        i32Array: *mut i32,
    ) -> i32;
    pub fn dx_LoadDivGraphFToResourceWithStrLen(
        ResourceName: *const i8,
        ResourceNameLength: usize,
        ResourceType: *const i8,
        ResourceTypeLength: usize,
        AllNum: i32,
        XNum: i32,
        YNum: i32,
        XSize: f32,
        YSize: f32,
        i32Array: *mut i32,
    ) -> i32;
    pub fn dx_CreateGraphFromID3D11Texture2D(pID3D11Texture2D: *const libc::c_void) -> i32;
    pub fn dx_BltBackScreenToWindow(Window: HWND, ClientX: i32, ClientY: i32) -> i32;
    pub fn dx_BltRectBackScreenToWindow(
        Window: HWND,
        BackScreenRect: Rect,
        WindowClientRect: Rect,
    ) -> i32;
    pub fn dx_SetScreenFlipTargetWindow(TargetWindow: HWND, ScaleX: f64, ScaleY: f64) -> i32;
    pub fn dx_GetDesktopScreenGraph(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        GrHandle: i32,
        DestX: i32,
        DestY: i32,
    ) -> i32;
    pub fn dx_SetMultiThreadFlag(Flag: i32) -> i32;
    pub fn dx_SetUseDirectDrawDeviceIndex(Index: i32) -> i32;
    pub fn dx_SetAeroDisableFlag(Flag: i32) -> i32;
    pub fn dx_SetUseDirect3D9Ex(Flag: i32) -> i32;
    pub fn dx_SetUseDirect3D11(Flag: i32) -> i32;
    pub fn dx_SetUseDirect3D11MinFeatureLevel(Level: i32) -> i32;
    pub fn dx_SetUseDirect3D11WARPDriver(Flag: i32) -> i32;
    pub fn dx_SetUseDirect3DVersion(Version: i32) -> i32;
    pub fn dx_GetUseDirect3DVersion() -> i32;
    pub fn dx_GetUseDirect3D11FeatureLevel() -> i32;
    pub fn dx_SetUseDirect3D11AdapterIndex(Index: i32) -> i32;
    pub fn dx_SetUseDirectDrawFlag(Flag: i32) -> i32;
    pub fn dx_SetUseGDIFlag(Flag: i32) -> i32;
    pub fn dx_GetUseGDIFlag() -> i32;
    pub fn dx_SetDDrawUseGuid(Guid: *const GUID) -> i32;
    pub fn dx_GetDirectDrawDeviceDescription(Number: i32, StringBuffer: *mut u8) -> i32;
    pub fn dx_GetDirectDrawDeviceNum() -> i32;
    pub fn dx_SetDrawScreen_ID3D11RenderTargetView(
        pID3D11RenderTargetView: *const libc::c_void,
        pID3D11DepthStencilView: *const libc::c_void,
    ) -> i32;
    pub fn dx_RefreshDxLibDirect3DSetting() -> i32;
    pub fn dx_SetUseMediaFoundationFlag(Flag: i32) -> i32;
    pub fn dx_ColorKaiseki(PixelData: *const libc::c_void, ColorData: *mut ColorData) -> i32;
    pub fn dx_BmpBltToMask(
        Bmp: *const libc::c_void,
        BmpPointX: i32,
        BmpPointY: i32,
        MaskHandle: i32,
    ) -> i32;
    pub fn dx_AddFontFile(FontFilePath: *const i8) -> i32;
    pub fn dx_AddFontFileWithStrLen(FontFilePath: *const i8, FontFilePathLength: usize) -> i32;
    pub fn dx_AddFontFileFromMem(FontFileImage: *const libc::c_void, FontFileImageSize: i32)
        -> i32;
    pub fn dx_RemoveFontFile(FontHandle: i32) -> i32;
    pub fn dx_CreateFontDataFile(
        SaveFilePath: *const i8,
        FontName: *const i8,
        Size: i32,
        BitDepth: i32,
        Thick: i32,
        Italic: i32,
        CharSet: i32,
        SaveCharaList: *const i8,
    ) -> i32;
    pub fn dx_CreateFontDataFileWithStrLen(
        SaveFilePath: *const i8,
        SaveFilePathLength: usize,
        FontName: *const i8,
        FontNameLength: usize,
        Size: i32,
        BitDepth: i32,
        Thick: i32,
        Italic: i32,
        CharSet: i32,
        SaveCharaList: *const i8,
        SaveCharaListLength: usize,
    ) -> i32;
    pub fn dx_CreateDIBGraph(
        FileName: *const i8,
        ReverseFlag: i32,
        SrcColor: *mut ColorData,
    ) -> *const libc::c_void;
    pub fn dx_CreateDIBGraphWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        ReverseFlag: i32,
        SrcColor: *mut ColorData,
    ) -> *const libc::c_void;
    pub fn dx_CreateDIBGraphToMem(
        BmpInfo: *const BitMapInfo,
        GraphData: *const libc::c_void,
        ReverseFlag: i32,
        SrcColor: *mut ColorData,
    ) -> *const libc::c_void;
    pub fn dx_CreateDIBGraph_plus_Alpha(
        FileName: *const i8,
        RGBBmp: *mut libc::c_void,
        AlphaBmp: *mut *const libc::c_void,
        ReverseFlag: i32,
        SrcColor: *mut ColorData,
    ) -> i32;
    pub fn dx_CreateDIBGraph_plus_AlphaWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        RGBBmp: *mut *const libc::c_void,
        AlphaBmp: *mut *const libc::c_void,
        ReverseFlag: i32,
        SrcColor: *mut ColorData,
    ) -> i32;
    pub fn dx_CreateDIBGraphVer2(
        FileName: *const i8,
        MemImage: *const libc::c_void,
        MemImageSize: i32,
        ImageType: i32,
        ReverseFlag: i32,
        SrcColor: *mut ColorData,
    ) -> *const libc::c_void;
    pub fn dx_CreateDIBGraphVer2WithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        MemImage: *const libc::c_void,
        MemImageSize: i32,
        ImageType: i32,
        ReverseFlag: i32,
        SrcColor: *mut ColorData,
    ) -> *const libc::c_void;
    pub fn dx_CreateDIBGraphVer2_plus_Alpha(
        FileName: *const i8,
        MemImage: *const libc::c_void,
        MemImageSize: i32,
        AlphaImage: *const libc::c_void,
        AlphaImageSize: i32,
        ImageType: i32,
        RGBBmp: *mut *const libc::c_void,
        AlphaBmp: *mut *const libc::c_void,
        ReverseFlag: i32,
        SrcColor: *mut ColorData,
    ) -> i32;
    pub fn dx_CreateDIBGraphVer2_plus_AlphaWithStrLen(
        FileName: *const i8,
        FileNameLength: usize,
        MemImage: *const libc::c_void,
        MemImageSize: i32,
        AlphaImage: *const libc::c_void,
        AlphaImageSize: i32,
        ImageType: i32,
        RGBBmp: *mut *const libc::c_void,
        AlphaBmp: *mut *const libc::c_void,
        ReverseFlag: i32,
        SrcColor: *mut ColorData,
    ) -> i32;
    pub fn dx_ConvBitmapToGraphImage(
        BmpInfo: *const BitMapInfo,
        GraphData: *mut libc::c_void,
        GraphImage: *mut BaseImage,
        CopyFlag: i32,
    ) -> i32;
    pub fn dx_ConvGraphImageToBitmap(
        GraphImage: *const BaseImage,
        BmpInfo: *mut BitMapInfo,
        GraphData: *mut *mut libc::c_void,
        CopyFlag: i32,
        FullColorConv: i32,
    ) -> i32;
    pub fn dx_UpdateLayerdWindowForBaseImage(BaseImage: *const BaseImage) -> i32;
    pub fn dx_UpdateLayerdWindowForBaseImageRect(
        BaseImage: *const BaseImage,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> i32;
    pub fn dx_UpdateLayerdWindowForPremultipliedAlphaBaseImage(BaseImage: *const BaseImage) -> i32;
    pub fn dx_UpdateLayerdWindowForPremultipliedAlphaBaseImageRect(
        BaseImage: *const BaseImage,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> i32;
    pub fn dx_GetDesktopScreenBaseImage(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        BaseImage: *mut BaseImage,
        DestX: i32,
        DestY: i32,
    ) -> i32;
    pub fn dx_UpdateLayerdWindowForSoftImage(SIHandle: i32) -> i32;
    pub fn dx_UpdateLayerdWindowForSoftImageRect(
        SIHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> i32;
    pub fn dx_UpdateLayerdWindowForPremultipliedAlphaSoftImage(SIHandle: i32) -> i32;
    pub fn dx_UpdateLayerdWindowForPremultipliedAlphaSoftImageRect(
        SIHandle: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> i32;
    pub fn dx_GetDesktopScreenSoftImage(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        SIHandle: i32,
        DestX: i32,
        DestY: i32,
    ) -> i32;
    pub fn dx_LoadSoundMemByResource(
        ResourceName: *const i8,
        ResourceType: *const i8,
        BufferNum: i32,
    ) -> i32;
    pub fn dx_LoadSoundMemByResourceWithStrLen(
        ResourceName: *const i8,
        ResourceNameLength: usize,
        ResourceType: *const i8,
        ResourceTypeLength: usize,
        BufferNum: i32,
    ) -> i32;
    pub fn dx_SetUseSoftwareMixingSoundFlag(Flag: i32) -> i32;
    pub fn dx_SetEnableXAudioFlag(Flag: i32) -> i32;
    pub fn dx_SetEnableWASAPIFlag(
        Flag: i32,
        IsExclusive: i32,
        DevicePeriod: i32,
        SamplePerSec: i32,
    ) -> i32;
    pub fn dx_SetEnableASIOFlag(Flag: i32, BufferSize: i32, SamplePerSec: i32) -> i32;
    pub fn dx_LoadMusicMemByResource(ResourceName: *const i8, ResourceType: *const i8) -> i32;
    pub fn dx_LoadMusicMemByResourceWithStrLen(
        ResourceName: *const i8,
        ResourceNameLength: usize,
        ResourceType: *const i8,
        ResourceTypeLength: usize,
    ) -> i32;
    pub fn dx_PlayMusicByResource(
        ResourceName: *const i8,
        ResourceType: *const i8,
        PlayType: i32,
    ) -> i32;
    pub fn dx_PlayMusicByResourceWithStrLen(
        ResourceName: *const i8,
        ResourceNameLength: usize,
        ResourceType: *const i8,
        ResourceTypeLength: usize,
        PlayType: i32,
    ) -> i32;
}
