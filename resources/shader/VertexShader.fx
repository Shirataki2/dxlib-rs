// 頂点シェーダーの入力
struct VS_INPUT
{
    float4 Position             : POSITION ;    // 座標( VERTEX3DSHADER構造体の pos の値 )
    float3 Normal               : NORMAL0 ;     // 法線( VERTEX3DSHADER構造体の norm の値 )
    float4 DiffuseColor         : COLOR0 ;      // ディフューズカラー( VERTEX3DSHADER構造体の dif の値 )
    float4 SpecularColor        : COLOR1 ;      // スペキュラカラー( VERTEX3DSHADER構造体の spc の値 )
    float2 TextureCoord0        : TEXCOORD0 ;   // テクスチャ座標０( VERTEX3DSHADER構造体の u, v の値 )
    float2 TextureCoord1        : TEXCOORD1 ;   // テクスチャ座標１( VERTEX3DSHADER構造体の su, sv の値 )
} ;

// 頂点シェーダーの出力
struct VS_OUTPUT
{
    float4 ProjectionPosition   : POSITION ;    // 座標( 射影空間 )
    float4 DiffuseColor         : COLOR0 ;      // ディフューズカラー
    float2 TextureCoord0        : TEXCOORD0 ;   // テクスチャ座標
} ;


// C++ 側で設定する定数の定義
float4x4 cfViewMatrix       : register( c6 ) ;    // ワールド座標をビュー座標に変換する行列の転置行列
float4x4 cfProjectionMatrix : register( c2 ) ;    // ビュー座標を射影座標に変換する行列の転置行列

// 頂点座標に加算する値
float4 cfAddPosition : register( c0 ) ;


// main関数
VS_OUTPUT main( VS_INPUT VSInput )
{
    VS_OUTPUT VSOutput ;
    float4 lWorldPosition ;
    float4 lViewPosition ;

    // 入力の頂点座標にＣ＋＋プログラム側で設定した頂点座標を加算する
    lWorldPosition = VSInput.Position + cfAddPosition ;

    // 頂点座標をビュー空間の座標に変換する
    lViewPosition = mul( lWorldPosition, cfViewMatrix ) ;

    // ビュー空間の座標を射影空間の座標に変換する
    VSOutput.ProjectionPosition = mul( lViewPosition, cfProjectionMatrix ) ;

    // テクスチャ座標はそのまま代入
    VSOutput.TextureCoord0 = VSInput.TextureCoord0;

    // 頂点カラーはそのまま代入
    VSOutput.DiffuseColor = VSInput.DiffuseColor ;

    // 関数の戻り値がピクセルシェーダーに渡される
    return VSOutput ;
}
