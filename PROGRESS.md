# API実装状況

- 〇: 実装済み
- -: 代替機能を使用/実装(Wait,Vectorなど)
- 無印: 未実装

|関数名|実装済みか|
|:---|:--:|
|使用必須関数||
|DxLib_Init|〇|
|DxLib_End|〇|
|ProcessMessage|〇|
|３Ｄ関係関数||
|[こちら](#3D関連)||
|Live2D関係関数||
|[こちら](#Live2D関連)||
|図形描画関数||
|DrawLine|〇|
|DrawLineAA|〇|
|DrawBox|〇|
|DrawBoxAA|〇|
|DrawCircle|〇|
|DrawCircleAA|〇|
|DrawOval|〇|
|DrawOvalAA|〇|
|DrawTriangle|〇|
|DrawTriangleAA|〇|
|DrawPixel|〇|
|GetPixel|〇|
|グラフィックデータ制御関数||
|LoadGraphScreen||
|LoadGraph|〇|
|LoadDivGraph||
|MakeGraph|〇|
|MakeScreen||
|SetCreateDrawValidGraphMultiSample||
|SetCreateGraphColorBitDepth||
|SetDrawValidFloatTypeGraphCreateFlag||
|SetCreateDrawValidGraphChannelNum||
|SetUsePremulAlphaConvertLoad||
|DrawGraph| - |
|DrawTurnGraph| - |
|DrawExtendGraph| - |
|DrawRotaGraph| - |
|DrawRotaGraph2| - |
|DrawRotaGraph3|〇|
|DrawModiGraph||
|DrawRectGraph||
|DerivationGraph||
|GetDrawScreenGraph||
|GetGraphSize||
|InitGraph||
|DeleteGraph|〇|
|SetDrawMode||
|SetDrawBlendMode|〇|
|SetDrawBright||
|SetTransColor||
|LoadBlendGraph||
|DrawBlendGraph||
|GraphFilter||
|GraphFilterBlt||
|GraphFilterRectBlt||
|GraphBlend||
|GraphBlendBlt||
|GraphBlendRectBlt||
|文字描画関係関数||
|DrawString|〇|
|DrawFormatString| - |
|GetDrawStringWidth||
|GetDrawFormatStringWidth||
|SetFontSize| - |
|SetFontThickness| - |
|ChangeFont| - |
|ChangeFontType| - |
|CreateFontToHandle|〇|
|LoadFontDataToHandle||
|DeleteFontToHandle|〇|
|SetFontCacheUsePremulAlphaFlag||
|DrawStringToHandle|〇|
|DrawFormatStringToHandle| - |
|GetDrawStringWidthToHandle||
|GetDrawFormatStringWidthToHandle||
|GetFontStateToHandle| - |
|InitFontToHandle| - |
|簡易画面出力関数||
|printfDx|〇|
|clsDx|〇|
|その他画面操作系関数||
|SetGraphMode|〇|
|SetFullScreenResolutionMode||
|SetFullScreenScalingMode||
|GetScreenState||
|SetDrawArea||
|ClearDrawScreen|〇|
|SetBackgroundColor|〇|
|GetColor|〇|
|SetDrawScreen|〇|
|ScreenFlip|〇|
|SetFullSceneAntiAliasingMode|〇|
|動画関係関数||
|PlayMovie||
|PlayMovieToGraph||
|PauseMovieToGraph||
|SeekMovieToGraph||
|TellMovieToGraph||
|GetMovieStateToGraph||
|マスク関係関数||
|CreateMaskScreen||
|DeleteMaskScreen||
|LoadMask||
|LoadDivMask||
|DrawMask||
|DrawFillMask||
|DeleteMask||
|InitMask||
|FillMaskScreen||
|SetUseMaskScreenFlag||
|MakeMask||
|GetMaskSize||
|SetDataToMask||
|DrawMaskToDirectData||
|DrawFillMaskToDirectData||
|入力関係の関数||
|ジョイパッド入力関連関数||
|GetJoypadNum||
|GetJoypadInputState||
|GetJoypadAnalogInput||
|GetJoypadDirectInputState||
|GetJoypadXInputState||
|SetJoypadDeadZone||
|StartJoypadVibration||
|StopJoypadVibration||
|マウス入力関連関数||
|SetMouseDispFlag|〇|
|GetMousePoint|〇|
|SetMousePoint|〇|
|GetMouseInput||
|GetMouseInputLog2||
|GetMouseWheelRotVol||
|タッチパネル入力関連関数||
|GetTouchInputNum||
|GetTouchInput||
|キーボード入力関連関数||
|CheckHitKeyAll|〇|
|CheckHitKey|〇|
|GetHitKeyStateAll|〇|
|半角文字入力関連関数||
|GetInputChar||
|GetInputCharWait||
|ClearInputCharBuf||
|日本語入力関連関数||
|KeyInputString||
|KeyInputSingleCharString||
|KeyInputNumber||
|SetKeyInputStringColor||
|MakeKeyInput||
|DeleteKeyInput||
|InitKeyInput||
|SetActiveKeyInput||
|CheckKeyInput||
|DrawKeyInputString||
|DrawKeyInputModeString||
|SetKeyInputString||
|SetKeyInputNumber||
|GetKeyInputString||
|GetKeyInputNumber||
|音利用関数||
|PlaySoundFile||
|CheckSoundFile||
|StopSoundFile||
|LoadSoundMem|〇|
|PlaySoundMem|〇|
|CheckSoundMem|〇|
|StopSoundMem|〇|
|DeleteSoundMem|〇|
|InitSoundMem| - |
|ChangePanSoundMem|〇|
|ChangeVolumeSoundMem|〇|
|ChangeNextPlayPanSoundMem||
|ChangeNextPlayVolumeSoundMem||
|SetFrequencySoundMem|〇|
|SetLoopPosSoundMem|〇|
|SetLoopSamplePosSoundMem|〇|
|SetCurrentPositionSoundMem||
|DuplicateSoundMem||
|SetCreateSoundPitchRate|〇|
|SetCreateSoundTimeStretchRate|〇|
|Set3DPositionSoundMem||
|Set3DRadiusSoundMem||
|Set3DVelocitySoundMem||
|SetNextPlay3DPositionSoundMem||
|SetNextPlay3DRadiusSoundMem||
|SetNextPlay3DVelocitySoundMem||
|Set3DReverbParamSoundMem||
|Set3DPresetReverbParamSoundMem||
|Get3DPresetReverbParamSoundMem||
|Set3DReverbParamSoundMemAll||
|Set3DPresetReverbParamSoundMemAll||
|SetCreate3DSoundFlag||
|SetEnableXAudioFlag||
|Set3DSoundOneMetre||
|Set3DSoundListenerPosAndFrontPos_UpVecY||
|Set3DSoundListenerPosAndFrontPosAndUpVec||
|Set3DSoundListenerVelocity||
|音楽再生関数||
|PlayMusic||
|CheckMusic||
|StopMusic||
|SetVolumeMusic||
|ウエイト関係の関数||
|WaitTimer| - |
|WaitVSync||
|WaitKey|〇|
|時間関係の関数||
|GetNowCount| - |
|GetNowHiPerformanceCount| - |
|GetDateTime| - |
|乱数取得関数||
|GetRand| - |
|SRand| - |
|ウインドウモード関係||
|ChangeWindowMode|〇|
|SetMainWindowText|〇|
|SetWindowIconID||
|SetWindowSizeChangeEnableFlag||
|SetWindowSizeExtendRate||
|通信関係||
|ConnectNetWork||
|CloseNetWork||
|PreparationListenNetWork||
|StopListenNetWork||
|NetWorkSend||
|GetNetWorkDataLength||
|GetNetWorkSendDataLength||
|NetWorkRecv||
|NetWorkRecvToPeek||
|GetNewAcceptNetWork||
|GetLostNetWork||
|GetNetWorkAcceptState||
|GetNetWorkIP||
|MakeUDPSocket||
|DeleteUDPSocket||
|NetWorkSendUDP||
|NetWorkRecvUDP||
|CheckNetWorkRecvUDP||
|ファイル読み込み関係||
|FileRead_open| - |
|FileRead_size| - |
|FileRead_close| - |
|FileRead_tell| - |
|FileRead_seek| - |
|FileRead_read| - |
|FileRead_eof| - |
|FileRead_gets| - |
|FileRead_getc| - |
|FileRead_scanf| - |
|ドット単位で画像にアクセスしたい関係||
|LoadSoftImage||
|LoadARGB8ColorSoftImage||
|LoadXRGB8ColorSoftImage||
|LoadSoftImageToMem||
|LoadARGB8ColorSoftImageToMem||
|LoadXRGB8ColorSoftImageToMem||
|MakeARGB8ColorSoftImage||
|MakeXRGB8ColorSoftImage||
|MakePAL8ColorSoftImage||
|DeleteSoftImage||
|InitSoftImage||
|GetSoftImageSize||
|FillSoftImage||
|SetPaletteSoftImage||
|GetPaletteSoftImage||
|DrawPixelPalCodeSoftImage||
|GetPixelPalCodeSoftImage||
|DrawPixelSoftImage||
|GetPixelSoftImage||
|BltSoftImage||
|DrawSoftImage||
|CreateGraphFromSoftImage||
|CreateDivGraphFromSoftImage||
|非同期読み込み関係||
|SetUseASyncLoadFlag||
|CheckHandleASyncLoad||
|GetASyncLoadNum||
|文字関係関数||
|SetUseCharCodeFormat||
|GetCharBytes||
|マイナー関数||
|SetAlwaysRunFlag|〇|
|SetOutApplicationLogValidFlag||
|SetUseDXArchiveFlag||
|SetDXArchiveExtension||
|SetDXArchiveKeyString||
|SetEmulation320x240||
|SetUse3DFlag||
|SetWaitVSyncFlag||
|SetUseDivGraphFlag||
|LoadPauseGraph||
|ScreenCopy||
|GetColorBitDepth||
|SaveDrawScreen||
|EnumFontName||
|DrawVString||
|DrawVStringToHandle||
|CreateGraphFromMem||
|ReCreateGraphFromMem||
|ReloadFileGraphAll||
|SetRestoreGraphCallback||
|SetCreateSoundDataType||
|LoadSoundMemByMemImage||
|SelectMidiMode||

# 3D関連

|関数名|実装済みか|
|:---|:--:|
|３Ｄ図形描画関係関数||
|DrawLine3D|〇|
|DrawTriangle3D|〇|
|DrawSphere3D|〇|
|DrawCapsule3D||〇
|DrawCone3D|〇|
|DrawBillboard3D|〇|
|DrawModiBillboard3D||
|DrawPolygon3D|〇|
|DrawPolygonIndexed3D|〇|
|SetMaterialUseVertDifColor||
|SetMaterialUseVertSpcColor||
|SetMaterialParam||
|SetUseZBuffer3D||
|SetWriteZBuffer3D||
|SetUseBackCulling||
|SetTextureAddressModeUV||
|SetFogEnable|〇|
|SetFogColor|〇|
|SetFogStartEnd|〇|
|GetColorF| - |
|GetColorU8| - |
|カメラ関数||
|SetCameraNearFar|〇|
|SetCameraPositionAndTarget_UpVecY|〇|
|SetCameraPositionAndTargetAndUpVec|〇|
|SetCameraPositionAndAngle|〇|
|SetCameraViewMatrix|〇|
|SetupCamera_Perspective|〇|
|SetupCamera_Ortho|〇|
|SetupCamera_ProjectionMatrix||
|SetCameraDotAspect|〇|
|ConvWorldPosToScreenPos|〇|
|ConvScreenPosToWorldPos|〇|
|SetCameraScreenCenter|〇|
|CheckCameraViewClip|〇|
|CheckCameraViewClip_Box|〇|
|GetCameraViewMatrix|〇|
|GetCameraProjectionMatrix|〇|
|ライト関数||
|SetUseLighting||
|SetGlobalAmbientLight|〇|
|標準ライト関数||
|ChangeLightTypeDir||
|ChangeLightTypePoint||
|ChangeLightTypeSpot||
|SetLightEnable||
|SetLightDifColor||
|SetLightSpcColor||
|SetLightAmbColor||
|SetLightDirection||
|SetLightPosition||
|SetLightRangeAtten||
|SetLightAngle||
|GetLightType||
|GetLightEnable||
|GetLightDifColor||
|GetLightSpcColor||
|GetLightAmbColor||
|GetLightDirection||
|GetLightPosition||
|GetLightRangeAtten||
|GetLightAngle||
|ライトハンドル関数||
|CreateDirLightHandle||
|CreatePointLightHandle||
|CreateSpotLightHandle||
|DeleteLightHandle||
|DeleteLightHandleAll||
|SetLightTypeHandle||
|SetLightEnableHandle||
|SetLightDifColorHandle||
|SetLightSpcColorHandle||
|SetLightAmbColorHandle||
|SetLightDirectionHandle||
|SetLightPositionHandle||
|SetLightRangeAttenHandle||
|SetLightAngleHandle||
|GetLightTypeHandle||
|GetLightEnableHandle||
|GetLightDifColorHandle||
|GetLightSpcColorHandle||
|GetLightAmbColorHandle||
|GetLightDirectionHandle||
|GetLightPositionHandle||
|GetLightRangeAttenHandle||
|GetLightAngleHandle||
|GetEnableLightHandleNum||
|GetEnableLightHandle||
|算術演算関数||
|VGet| - |
|VAdd| - |
|VSub| - |
|VDot| - |
|VCross| - |
|VScale| - |
|VSize| - |
|VSquareSize| - |
|VNorm| - |
|VTransform| - |
|VTransformSR| - |
|MGetIdent| - |
|MGetScale| - |
|MGetTranslate| - |
|MGetRotX| - |
|MGetRotY| - |
|MGetRotZ| - |
|MGetRotAxis| - |
|MGetRotVec2| - |
|MGetAxis1| - |
|MGetAxis2| - |
|MAdd| - |
|MMult| - |
|MScale| - |
|MTranspose| - |
|MInverse||
|衝突検出系関数||
|Segment_Segment_MinLength||
|Segment_Triangle_MinLength||
|Segment_Point_MinLength||
|HitCheck_Line_Triangle||
|３Ｄモデル関係の関数||
|モデルの読み込み・複製関係の関数||
|MV1LoadModel|〇|
|MV1DuplicateModel||
|MV1DeleteModel|〇|
|MV1SetLoadModelUsePhysicsMode|〇|
|MV1SetLoadModelPhysicsWorldGravity||
|モデル描画関数||
|MV1DrawModel|〇|
|MV1DrawFrame|〇|
|MV1DrawMesh|〇|
|MV1DrawTriangleList|〇|
|モデル描画設定関数||
|MV1SetUseOrigShader||
|MV1SetSemiTransDrawMode||
|モデル基本制御関数||
|MV1SetPosition|〇|
|MV1GetPosition|〇|
|MV1SetScale|〇|
|MV1GetScale|〇|
|MV1SetRotationXYZ|〇|
|MV1GetRotationXYZ|〇|
|MV1SetRotationZYAxis||
|MV1SetMatrix|〇|
|MV1GetMatrix|〇|
|MV1SetVisible|〇|
|MV1GetVisible|〇|
|MV1SetDifColorScale||
|MV1GetDifColorScale||
|MV1SetSpcColorScale||
|MV1GetSpcColorScale||
|MV1SetEmiColorScale||
|MV1GetEmiColorScale||
|MV1SetAmbColorScale||
|MV1GetAmbColorScale||
|MV1GetSemiTransState||
|MV1SetOpacityRate||
|MV1GetOpacityRate||
|MV1SetUseZBuffer||
|MV1SetWriteZBuffer||
|MV1SetUseVertDifColor||
|MV1SetUseVertSpcColor||
|MV1PhysicsCalculation||
|MV1PhysicsResetState||
|アニメーション関数||
|MV1AttachAnim||
|MV1DetachAnim||
|MV1SetAttachAnimTime||
|MV1GetAttachAnimTime||
|MV1GetAttachAnimTotalTime||
|MV1SetAttachAnimBlendRate||
|MV1GetAttachAnimBlendRate||
|MV1GetAttachAnim||
|MV1GetAttachAnimFrameLocalPosition||
|MV1GetAnimNum||
|MV1GetAnimName||
|MV1GetAnimIndex||
|MV1GetAnimTotalTime||
|マテリアル関数||
|MV1GetMaterialNum||
|MV1GetMaterialName||
|MV1SetMaterialDifColor||
|MV1GetMaterialDifColor||
|MV1SetMaterialSpcColor||
|MV1GetMaterialSpcColor||
|MV1SetMaterialEmiColor||
|MV1GetMaterialEmiColor||
|MV1SetMaterialAmbColor||
|MV1GetMaterialAmbColor||
|MV1SetMaterialSpcPower||
|MV1GetMaterialSpcPower||
|MV1GetMaterialDifMapTexture||
|MV1GetMaterialSpcMapTexture||
|MV1GetMaterialNormalMapTexture||
|MV1SetMaterialDrawBlendMode||
|MV1GetMaterialDrawBlendMode||
|MV1SetMaterialDrawBlendParam||
|MV1GetMaterialDrawBlendParam||
|MV1SetMaterialDrawAlphaTest||
|MV1SetMaterialDrawAlphaTestAll||
|MV1SetMaterialOutLineWidth||
|MV1GetMaterialOutLineWidth||
|MV1SetMaterialOutLineDotWidth||
|MV1GetMaterialOutLineDotWidth||
|MV1SetMaterialOutLineColor||
|MV1GetMaterialOutLineColor||
|テクスチャ関係||
|MV1GetTextureNum||
|MV1GetTextureName||
|MV1SetTextureGraphHandle||
|MV1GetTextureGraphHandle||
|MV1SetTextureAddressMode||
|MV1GetTextureAddressModeU||
|MV1GetTextureAddressModeV||
|MV1SetTextureSampleFilterMode||
|MV1GetTextureSampleFilterMode||
|フレーム関数||
|MV1GetFrameNum||
|MV1SearchFrame||
|MV1SearchFrameChild||
|MV1GetFrameName||
|MV1GetFrameParent||
|MV1GetFrameChildNum||
|MV1GetFrameChild||
|MV1GetFramePosition||
|MV1GetFrameLocalMatrix||
|MV1GetFrameLocalWorldMatrix||
|MV1SetFrameUserLocalMatrix||
|MV1ResetFrameUserLocalMatrix||
|MV1SetFrameVisible||
|MV1GetFrameVisible||
|MV1SetFrameDifColorScale||
|MV1GetFrameDifColorScale||
|MV1SetFrameSpcColorScale||
|MV1GetFrameSpcColorScale||
|MV1SetFrameEmiColorScale||
|MV1GetFrameEmiColorScale||
|MV1SetFrameAmbColorScale||
|MV1GetFrameAmbColorScale||
|MV1GetFrameSemiTransState||
|MV1SetFrameOpacityRate||
|MV1GetFrameOpacityRate||
|MV1SetFrameTextureAddressTransform||
|MV1ResetFrameTextureAddressTransform||
|MV1GetFrameTriangleNum||
|MV1GetFrameMeshNum||
|MV1GetFrameMesh||
|メッシュ関数||
|MV1GetMeshNum||
|MV1GetMeshMaterial||
|MV1GetMeshTriangleNum||
|MV1SetMeshVisible||
|MV1GetMeshVisible||
|MV1SetMeshDifColorScale||
|MV1GetMeshDifColorScale||
|MV1SetMeshSpcColorScale||
|MV1GetMeshSpcColorScale||
|MV1SetMeshEmiColorScale||
|MV1GetMeshEmiColorScale||
|MV1SetMeshAmbColorScale||
|MV1GetMeshAmbColorScale||
|MV1SetMeshOpacityRate||
|MV1GetMeshOpacityRate||
|MV1SetMeshDrawBlendMode||
|MV1GetMeshDrawBlendMode||
|MV1SetMeshDrawBlendParam||
|MV1GetMeshDrawBlendParam||
|MV1SetMeshBackCulling||
|MV1GetMeshBackCulling||
|MV1GetMeshMaxPosition||
|MV1GetMeshMinPosition||
|MV1GetMeshTListNum||
|MV1GetMeshTList||
|MV1GetMeshSemiTransState||
|MV1SetMeshUseVertDifColor||
|MV1GetMeshUseVertDifColor||
|MV1SetMeshUseVertSpcColor||
|MV1GetMeshUseVertSpcColor||
|シェイプ関数||
|MV1GetShapeNum||
|MV1SearchShape||
|MV1SetShapeRate||
|MV1GetShapeRate||
|トライアングルリスト関数||
|MV1GetTriangleListNum||
|MV1GetTriangleListVertexType||
|MV1GetTriangleListPolygonNum||
|MV1GetTriangleListVertexNum||
|コリジョン( 衝突判定 )関数||
|MV1SetupCollInfo||
|MV1TerminateCollInfo||
|MV1RefreshCollInfo||
|MV1CollCheck_Line||
|MV1CollCheck_Sphere||
|MV1CollCheck_Capsule||
|MV1CollCheck_GetResultPoly||
|MV1CollResultPolyDimTerminate||
|参照用メッシュ関数||
|MV1SetupReferenceMesh||
|MV1TerminateReferenceMesh||
|MV1RefreshReferenceMesh||
|MV1GetReferenceMesh||
|プログラマブルシェーダー関係関数||
|SetUseDirect3DVersion||
|GetUseDirect3DVersion||
|GetValidShaderVersion||
|GetMultiDrawScreenNum||
|LoadVertexShader||
|LoadPixelShader||
|DeleteShader||
|InitShader||
|SetVSConstF||
|SetVSConstFMtx||
|SetVSConstFArray||
|ResetVSConstF||
|SetPSConstF||
|SetPSConstFMtx||
|SetPSConstFArray||
|ResetPSConstF||
|SetRenderTargetToShader||
|SetUseTextureToShader||
|SetUseVertexShader||
|SetUsePixelShader||
|DrawPolygon2DToShader||
|DrawPolygon3DToShader||
|DrawPolygonIndexed2DToShader||
|DrawPolygonIndexed3DToShader||
|シャドウマップ関係関数||
|MakeShadowMap||
|DeleteShadowMap||
|SetShadowMapLightDirection||
|ShadowMap_DrawSetup||
|ShadowMap_DrawEnd||
|SetUseShadowMap||
|SetShadowMapDrawArea||
|SetShadowMapAdjustDepth||
|TestDrawShadowMap||

## Live2D関連

|関数名|実装済みか|
|:---|:--:|
|必ず使用する関数||
|Live2D_SetCubism4CoreDLLPath|〇|
|Live2D_RenderBegin|〇|
|Live2D_RenderEnd|〇|
|Live2D_LoadModel|〇|
|Live2D_DeleteModel|〇|
|Live2D_Model_Update|〇|
|Live2D_Model_Draw|〇|
|位置・拡大率・回転を設定する関数||
|Live2D_Model_SetTranslate|〇|
|Live2D_Model_SetExtendRate|〇|
|Live2D_Model_SetRotate||
|モーション関係の関数||
|Live2D_Model_StartMotion|〇|
|Live2D_Model_IsMotionFinished|〇|
|Live2D_Model_SetExpression|〇|
|パラメータ関係の関数||
|Live2D_Model_GetParameterValue|〇|
|Live2D_Model_SetParameterValue|〇|
|当たり判定関係の関数||
|Live2D_Model_HitTest|〇|
|モデルの情報取得系の関数||
|Live2D_Model_GetHitAreasCount|〇|
|Live2D_Model_GetHitAreaName|〇|
|Live2D_Model_GetPhysicsFileName||
|Live2D_Model_GetPoseFileName||
|Live2D_Model_GetExpressionCount||
|Live2D_Model_GetExpressionName||
|Live2D_Model_GetExpressionFileName||
|Live2D_Model_GetMotionGroupCount||
|Live2D_Model_GetMotionGroupName||
|Live2D_Model_GetMotionCount||
|Live2D_Model_GetMotionFileName||
|Live2D_Model_GetMotionSoundFileName||
|Live2D_Model_GetMotionFadeInTimeValue||
|Live2D_Model_GetMotionFadeOutTimeValue||
|Live2D_Model_GetUserDataFile||
|Live2D_Model_GetEyeBlinkParameterCount||
|Live2D_Model_GetEyeBlinkParameterId||
|Live2D_Model_GetLipSyncParameterCount||
|Live2D_Model_GetLipSyncParameterId||
