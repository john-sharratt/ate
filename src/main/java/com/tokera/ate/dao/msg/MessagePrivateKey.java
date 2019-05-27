// automatically generated by the FlatBuffers compiler, do not modify

package com.tokera.ate.dao.msg;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings({"unused", "return.type.incompatible"})
public final class MessagePrivateKey extends Table {
  public static MessagePrivateKey getRootAsMessagePrivateKey(ByteBuffer _bb) { return getRootAsMessagePrivateKey(_bb, new MessagePrivateKey()); }
  public static MessagePrivateKey getRootAsMessagePrivateKey(ByteBuffer _bb, MessagePrivateKey obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { bb_pos = _i; bb = _bb; }
  public MessagePrivateKey __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public @org.checkerframework.checker.nullness.qual.Nullable String privateKeyHash() { int o = __offset(4); return o != 0 ? __string(o + bb_pos) : null; }
  public @org.checkerframework.checker.nullness.qual.Nullable ByteBuffer privateKeyHashAsByteBuffer() { return __vector_as_bytebuffer(4, 1); }
  public @org.checkerframework.checker.nullness.qual.Nullable ByteBuffer privateKeyHashInByteBuffer(ByteBuffer _bb) { return __vector_in_bytebuffer(_bb, 4, 1); }
  public byte privateKey1(int j) { int o = __offset(6); return o != 0 ? bb.get(__vector(o) + j * 1) : 0; }
  public int privateKey1Length() { int o = __offset(6); return o != 0 ? __vector_len(o) : 0; }
  public @org.checkerframework.checker.nullness.qual.Nullable ByteBuffer privateKey1AsByteBuffer() { return __vector_as_bytebuffer(6, 1); }
  public @org.checkerframework.checker.nullness.qual.Nullable ByteBuffer privateKey1InByteBuffer(ByteBuffer _bb) { return __vector_in_bytebuffer(_bb, 6, 1); }
  public byte privateKey2(int j) { int o = __offset(8); return o != 0 ? bb.get(__vector(o) + j * 1) : 0; }
  public int privateKey2Length() { int o = __offset(8); return o != 0 ? __vector_len(o) : 0; }
  public @org.checkerframework.checker.nullness.qual.Nullable ByteBuffer privateKey2AsByteBuffer() { return __vector_as_bytebuffer(8, 1); }
  public @org.checkerframework.checker.nullness.qual.Nullable ByteBuffer privateKey2InByteBuffer(ByteBuffer _bb) { return __vector_in_bytebuffer(_bb, 8, 1); }
  public MessagePublicKey publicKey() { return publicKey(new MessagePublicKey()); }
  public MessagePublicKey publicKey(MessagePublicKey obj) { int o = __offset(10); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }

  public static int createMessagePrivateKey(FlatBufferBuilder builder,
      int privateKeyHashOffset,
      int privateKey1Offset,
      int privateKey2Offset,
      int publicKeyOffset) {
    builder.startObject(4);
    MessagePrivateKey.addPublicKey(builder, publicKeyOffset);
    MessagePrivateKey.addPrivateKey2(builder, privateKey2Offset);
    MessagePrivateKey.addPrivateKey1(builder, privateKey1Offset);
    MessagePrivateKey.addPrivateKeyHash(builder, privateKeyHashOffset);
    return MessagePrivateKey.endMessagePrivateKey(builder);
  }

  public static void startMessagePrivateKey(FlatBufferBuilder builder) { builder.startObject(4); }
  public static void addPrivateKeyHash(FlatBufferBuilder builder, int privateKeyHashOffset) { builder.addOffset(0, privateKeyHashOffset, 0); }
  public static void addPrivateKey1(FlatBufferBuilder builder, int privateKey1Offset) { builder.addOffset(1, privateKey1Offset, 0); }
  public static int createPrivateKey1Vector(FlatBufferBuilder builder, byte[] data) { builder.startVector(1, data.length, 1); for (int i = data.length - 1; i >= 0; i--) builder.addByte(data[i]); return builder.endVector(); }
  public static void startPrivateKey1Vector(FlatBufferBuilder builder, int numElems) { builder.startVector(1, numElems, 1); }
  public static void addPrivateKey2(FlatBufferBuilder builder, int privateKey2Offset) { builder.addOffset(2, privateKey2Offset, 0); }
  public static int createPrivateKey2Vector(FlatBufferBuilder builder, byte[] data) { builder.startVector(1, data.length, 1); for (int i = data.length - 1; i >= 0; i--) builder.addByte(data[i]); return builder.endVector(); }
  public static void startPrivateKey2Vector(FlatBufferBuilder builder, int numElems) { builder.startVector(1, numElems, 1); }
  public static void addPublicKey(FlatBufferBuilder builder, int publicKeyOffset) { builder.addOffset(3, publicKeyOffset, 0); }
  public static int endMessagePrivateKey(FlatBufferBuilder builder) {
    int o = builder.endObject();
    return o;
  }
}

