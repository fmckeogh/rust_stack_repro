#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use X_set::*;
use u__UNKNOWN_bits::*;
use SP_read::*;
use BigEndian::*;
use CreateAccDescRCW::*;
use X_read::*;
use CheckSPAlignment::*;
use IsD128Enabled::*;
use MemAtomicRCW::*;
use common::*;
pub fn execute_aarch64_instrs_memory_rcw_ld_128_rcwclrp<T: Tracer>(
    state: &mut State,
    tracer: &T,
    acquire: bool,
    n: i64,
    op: u32,
    release: bool,
    soft: bool,
    t: i64,
    t2: i64,
    tagchecked: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        readdata: u128,
        address: u64,
        value2_name: u64,
        value1_name: u64,
        accdesc: ProductType9878976b5bcce9c9,
        newdata: u128,
        gs_708251: ProductType42c31f3d0ab6eb17,
        acquire: bool,
        n: i64,
        op: u32,
        release: bool,
        soft: bool,
        t: i64,
        t2: i64,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        acquire,
        n,
        op,
        release,
        soft,
        t,
        t2,
        tagchecked,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call IsD128Enabled(s_0_1)
        let s_0_2: bool = IsD128Enabled(state, tracer, s_0_1);
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b12 b1
        if s_0_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var op:u32
        let s_1_0: u32 = fn_state.op;
        // D s_1_1: read-var soft:u8
        let s_1_1: bool = fn_state.soft;
        // D s_1_2: read-var acquire:u8
        let s_1_2: bool = fn_state.acquire;
        // D s_1_3: read-var release:u8
        let s_1_3: bool = fn_state.release;
        // D s_1_4: read-var tagchecked:u8
        let s_1_4: bool = fn_state.tagchecked;
        // D s_1_5: call CreateAccDescRCW(s_1_0, s_1_1, s_1_2, s_1_3, s_1_4)
        let s_1_5: ProductType9878976b5bcce9c9 = CreateAccDescRCW(
            state,
            tracer,
            s_1_0,
            s_1_1,
            s_1_2,
            s_1_3,
            s_1_4,
        );
        // D s_1_6: write-var accdesc <= s_1_5
        fn_state.accdesc = s_1_5;
        // C s_1_7: const #31s : i
        let s_1_7: i128 = 31;
        // D s_1_8: read-var n:i64
        let s_1_8: i64 = fn_state.n;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: cmp-eq s_1_9 s_1_7
        let s_1_10: bool = ((s_1_9) == (s_1_7));
        // N s_1_11: branch s_1_10 b10 b2
        if s_1_10 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: write-var address <= s_2_4
        fn_state.address = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: read-var t:i64
        let s_3_1: i64 = fn_state.t;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: call X_read(s_3_2, s_3_0)
        let s_3_3: Bits = X_read(state, tracer, s_3_2, s_3_0);
        // D s_3_4: cast reint s_3_3 -> u64
        let s_3_4: u64 = (s_3_3.value() as u64);
        // D s_3_5: write-var value1_name <= s_3_4
        fn_state.value1_name = s_3_4;
        // C s_3_6: const #64s : i64
        let s_3_6: i64 = 64;
        // D s_3_7: read-var t2:i64
        let s_3_7: i64 = fn_state.t2;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: call X_read(s_3_8, s_3_6)
        let s_3_9: Bits = X_read(state, tracer, s_3_8, s_3_6);
        // D s_3_10: cast reint s_3_9 -> u64
        let s_3_10: u64 = (s_3_9.value() as u64);
        // D s_3_11: write-var value2_name <= s_3_10
        fn_state.value2_name = s_3_10;
        // D s_3_12: read-var accdesc.1:struct
        let s_3_12: u32 = fn_state.accdesc._1;
        // D s_3_13: call BigEndian(s_3_12)
        let s_3_13: bool = BigEndian(state, tracer, s_3_12);
        // N s_3_14: branch s_3_13 b9 b4
        if s_3_13 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var value2_name:u64
        let s_4_0: u64 = fn_state.value2_name;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 64u16);
        // D s_4_2: read-var value1_name:u64
        let s_4_2: u64 = fn_state.value1_name;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 64u16);
        // D s_4_4: cast reint s_4_1 -> u128
        let s_4_4: u128 = (s_4_1.value() as u128);
        // D s_4_5: size-of s_4_1
        let s_4_5: u16 = s_4_1.length();
        // D s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // D s_4_8: lsl s_4_4 s_4_7
        let s_4_8: u128 = s_4_4 << s_4_7;
        // D s_4_9: or s_4_8 s_4_6
        let s_4_9: u128 = ((s_4_8) | (s_4_6));
        // D s_4_10: add s_4_5 s_4_7
        let s_4_10: u16 = (s_4_5 + s_4_7);
        // D s_4_11: create-bits s_4_9 s_4_10
        let s_4_11: Bits = Bits::new(s_4_9, s_4_10);
        // D s_4_12: cast reint s_4_11 -> u128
        let s_4_12: u128 = (s_4_11.value() as u128);
        // D s_4_13: write-var newdata <= s_4_12
        fn_state.newdata = s_4_12;
        // N s_4_14: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #128s : i64
        let s_5_0: i64 = 128;
        // C s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // S s_5_2: call __UNKNOWN_bits(s_5_1)
        let s_5_2: Bits = u__UNKNOWN_bits(state, tracer, s_5_1);
        // S s_5_3: cast reint s_5_2 -> u128
        let s_5_3: u128 = (s_5_2.value() as u128);
        // S s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 128u16);
        // D s_5_5: read-var newdata:u128
        let s_5_5: u128 = fn_state.newdata;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 128u16);
        // D s_5_7: read-var address:u64
        let s_5_7: u64 = fn_state.address;
        // D s_5_8: read-var accdesc:struct
        let s_5_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_5_9: call MemAtomicRCW(s_5_7, s_5_4, s_5_6, s_5_8)
        let s_5_9: ProductType42c31f3d0ab6eb17 = MemAtomicRCW(
            state,
            tracer,
            s_5_7,
            s_5_4,
            s_5_6,
            s_5_8,
        );
        // D s_5_10: write-var gs#708251 <= s_5_9
        fn_state.gs_708251 = s_5_9;
        // N s_5_11: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#708251.0:struct
        let s_6_0: u8 = fn_state.gs_708251._0;
        // D s_6_1: read-var gs#708251.1:struct
        let s_6_1: Bits = fn_state.gs_708251._1;
        // D s_6_2: cast reint s_6_1 -> u128
        let s_6_2: u128 = (s_6_1.value() as u128);
        // D s_6_3: write-var readdata <= s_6_2
        fn_state.readdata = s_6_2;
        // C s_6_4: const #3s : i
        let s_6_4: i128 = 3;
        // D s_6_5: cast zx s_6_0 -> bv
        let s_6_5: Bits = Bits::new(s_6_0 as u128, 4u16);
        // C s_6_6: const #1s : i64
        let s_6_6: i64 = 1;
        // C s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // C s_6_8: const #0s : i
        let s_6_8: i128 = 0;
        // C s_6_9: add s_6_8 s_6_7
        let s_6_9: i128 = (s_6_8 + s_6_7);
        // D s_6_10: bit-extract s_6_5 s_6_4 s_6_9
        let s_6_10: Bits = (Bits::new(
            ((s_6_5) >> (s_6_4)).value(),
            u16::try_from(s_6_9).unwrap(),
        ));
        // D s_6_11: cast reint s_6_10 -> u8
        let s_6_11: bool = ((s_6_10.value()) != 0);
        // C s_6_12: const #16984u : u32
        let s_6_12: u32 = 16984;
        // N s_6_13: write-reg s_6_12 <= s_6_11
        let s_6_13: () = {
            state.write_register::<bool>(s_6_12 as isize, s_6_11);
            tracer.write_register(s_6_12 as isize, s_6_11);
        };
        // C s_6_14: const #2s : i
        let s_6_14: i128 = 2;
        // D s_6_15: cast zx s_6_0 -> bv
        let s_6_15: Bits = Bits::new(s_6_0 as u128, 4u16);
        // C s_6_16: const #1s : i64
        let s_6_16: i64 = 1;
        // C s_6_17: cast zx s_6_16 -> i
        let s_6_17: i128 = (i128::try_from(s_6_16).unwrap());
        // C s_6_18: const #0s : i
        let s_6_18: i128 = 0;
        // C s_6_19: add s_6_18 s_6_17
        let s_6_19: i128 = (s_6_18 + s_6_17);
        // D s_6_20: bit-extract s_6_15 s_6_14 s_6_19
        let s_6_20: Bits = (Bits::new(
            ((s_6_15) >> (s_6_14)).value(),
            u16::try_from(s_6_19).unwrap(),
        ));
        // D s_6_21: cast reint s_6_20 -> u8
        let s_6_21: bool = ((s_6_20.value()) != 0);
        // C s_6_22: const #16997u : u32
        let s_6_22: u32 = 16997;
        // N s_6_23: write-reg s_6_22 <= s_6_21
        let s_6_23: () = {
            state.write_register::<bool>(s_6_22 as isize, s_6_21);
            tracer.write_register(s_6_22 as isize, s_6_21);
        };
        // C s_6_24: const #1s : i
        let s_6_24: i128 = 1;
        // D s_6_25: cast zx s_6_0 -> bv
        let s_6_25: Bits = Bits::new(s_6_0 as u128, 4u16);
        // C s_6_26: const #1s : i64
        let s_6_26: i64 = 1;
        // C s_6_27: cast zx s_6_26 -> i
        let s_6_27: i128 = (i128::try_from(s_6_26).unwrap());
        // C s_6_28: const #0s : i
        let s_6_28: i128 = 0;
        // C s_6_29: add s_6_28 s_6_27
        let s_6_29: i128 = (s_6_28 + s_6_27);
        // D s_6_30: bit-extract s_6_25 s_6_24 s_6_29
        let s_6_30: Bits = (Bits::new(
            ((s_6_25) >> (s_6_24)).value(),
            u16::try_from(s_6_29).unwrap(),
        ));
        // D s_6_31: cast reint s_6_30 -> u8
        let s_6_31: bool = ((s_6_30.value()) != 0);
        // C s_6_32: const #16971u : u32
        let s_6_32: u32 = 16971;
        // N s_6_33: write-reg s_6_32 <= s_6_31
        let s_6_33: () = {
            state.write_register::<bool>(s_6_32 as isize, s_6_31);
            tracer.write_register(s_6_32 as isize, s_6_31);
        };
        // C s_6_34: const #0s : i
        let s_6_34: i128 = 0;
        // D s_6_35: cast zx s_6_0 -> bv
        let s_6_35: Bits = Bits::new(s_6_0 as u128, 4u16);
        // C s_6_36: const #1s : i64
        let s_6_36: i64 = 1;
        // C s_6_37: cast zx s_6_36 -> i
        let s_6_37: i128 = (i128::try_from(s_6_36).unwrap());
        // C s_6_38: const #0s : i
        let s_6_38: i128 = 0;
        // C s_6_39: add s_6_38 s_6_37
        let s_6_39: i128 = (s_6_38 + s_6_37);
        // D s_6_40: bit-extract s_6_35 s_6_34 s_6_39
        let s_6_40: Bits = (Bits::new(
            ((s_6_35) >> (s_6_34)).value(),
            u16::try_from(s_6_39).unwrap(),
        ));
        // D s_6_41: cast reint s_6_40 -> u8
        let s_6_41: bool = ((s_6_40.value()) != 0);
        // C s_6_42: const #16996u : u32
        let s_6_42: u32 = 16996;
        // N s_6_43: write-reg s_6_42 <= s_6_41
        let s_6_43: () = {
            state.write_register::<bool>(s_6_42 as isize, s_6_41);
            tracer.write_register(s_6_42 as isize, s_6_41);
        };
        // D s_6_44: read-var accdesc.1:struct
        let s_6_44: u32 = fn_state.accdesc._1;
        // D s_6_45: call BigEndian(s_6_44)
        let s_6_45: bool = BigEndian(state, tracer, s_6_44);
        // N s_6_46: branch s_6_45 b8 b7
        if s_6_45 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #0s : i
        let s_7_1: i128 = 0;
        // D s_7_2: read-var readdata:u128
        let s_7_2: u128 = fn_state.readdata;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 128u16);
        // C s_7_4: const #1s : i64
        let s_7_4: i64 = 1;
        // C s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // C s_7_6: const #63s : i
        let s_7_6: i128 = 63;
        // C s_7_7: add s_7_6 s_7_5
        let s_7_7: i128 = (s_7_6 + s_7_5);
        // D s_7_8: bit-extract s_7_3 s_7_1 s_7_7
        let s_7_8: Bits = (Bits::new(
            ((s_7_3) >> (s_7_1)).value(),
            u16::try_from(s_7_7).unwrap(),
        ));
        // D s_7_9: cast reint s_7_8 -> u64
        let s_7_9: u64 = (s_7_8.value() as u64);
        // D s_7_10: read-var t:i64
        let s_7_10: i64 = fn_state.t;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: cast zx s_7_9 -> bv
        let s_7_12: Bits = Bits::new(s_7_9 as u128, 64u16);
        // D s_7_13: call X_set(s_7_11, s_7_0, s_7_12)
        let s_7_13: () = X_set(state, tracer, s_7_11, s_7_0, s_7_12);
        // C s_7_14: const #64s : i64
        let s_7_14: i64 = 64;
        // C s_7_15: const #64s : i
        let s_7_15: i128 = 64;
        // D s_7_16: read-var readdata:u128
        let s_7_16: u128 = fn_state.readdata;
        // D s_7_17: cast zx s_7_16 -> bv
        let s_7_17: Bits = Bits::new(s_7_16 as u128, 128u16);
        // C s_7_18: const #1s : i64
        let s_7_18: i64 = 1;
        // C s_7_19: cast zx s_7_18 -> i
        let s_7_19: i128 = (i128::try_from(s_7_18).unwrap());
        // C s_7_20: const #63s : i
        let s_7_20: i128 = 63;
        // C s_7_21: add s_7_20 s_7_19
        let s_7_21: i128 = (s_7_20 + s_7_19);
        // D s_7_22: bit-extract s_7_17 s_7_15 s_7_21
        let s_7_22: Bits = (Bits::new(
            ((s_7_17) >> (s_7_15)).value(),
            u16::try_from(s_7_21).unwrap(),
        ));
        // D s_7_23: cast reint s_7_22 -> u64
        let s_7_23: u64 = (s_7_22.value() as u64);
        // D s_7_24: read-var t2:i64
        let s_7_24: i64 = fn_state.t2;
        // D s_7_25: cast zx s_7_24 -> i
        let s_7_25: i128 = (i128::try_from(s_7_24).unwrap());
        // D s_7_26: cast zx s_7_23 -> bv
        let s_7_26: Bits = Bits::new(s_7_23 as u128, 64u16);
        // D s_7_27: call X_set(s_7_25, s_7_14, s_7_26)
        let s_7_27: () = X_set(state, tracer, s_7_25, s_7_14, s_7_26);
        // N s_7_28: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // C s_8_1: const #64s : i
        let s_8_1: i128 = 64;
        // D s_8_2: read-var readdata:u128
        let s_8_2: u128 = fn_state.readdata;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 128u16);
        // C s_8_4: const #1s : i64
        let s_8_4: i64 = 1;
        // C s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // C s_8_6: const #63s : i
        let s_8_6: i128 = 63;
        // C s_8_7: add s_8_6 s_8_5
        let s_8_7: i128 = (s_8_6 + s_8_5);
        // D s_8_8: bit-extract s_8_3 s_8_1 s_8_7
        let s_8_8: Bits = (Bits::new(
            ((s_8_3) >> (s_8_1)).value(),
            u16::try_from(s_8_7).unwrap(),
        ));
        // D s_8_9: cast reint s_8_8 -> u64
        let s_8_9: u64 = (s_8_8.value() as u64);
        // D s_8_10: read-var t:i64
        let s_8_10: i64 = fn_state.t;
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: cast zx s_8_9 -> bv
        let s_8_12: Bits = Bits::new(s_8_9 as u128, 64u16);
        // D s_8_13: call X_set(s_8_11, s_8_0, s_8_12)
        let s_8_13: () = X_set(state, tracer, s_8_11, s_8_0, s_8_12);
        // C s_8_14: const #64s : i64
        let s_8_14: i64 = 64;
        // C s_8_15: const #0s : i
        let s_8_15: i128 = 0;
        // D s_8_16: read-var readdata:u128
        let s_8_16: u128 = fn_state.readdata;
        // D s_8_17: cast zx s_8_16 -> bv
        let s_8_17: Bits = Bits::new(s_8_16 as u128, 128u16);
        // C s_8_18: const #1s : i64
        let s_8_18: i64 = 1;
        // C s_8_19: cast zx s_8_18 -> i
        let s_8_19: i128 = (i128::try_from(s_8_18).unwrap());
        // C s_8_20: const #63s : i
        let s_8_20: i128 = 63;
        // C s_8_21: add s_8_20 s_8_19
        let s_8_21: i128 = (s_8_20 + s_8_19);
        // D s_8_22: bit-extract s_8_17 s_8_15 s_8_21
        let s_8_22: Bits = (Bits::new(
            ((s_8_17) >> (s_8_15)).value(),
            u16::try_from(s_8_21).unwrap(),
        ));
        // D s_8_23: cast reint s_8_22 -> u64
        let s_8_23: u64 = (s_8_22.value() as u64);
        // D s_8_24: read-var t2:i64
        let s_8_24: i64 = fn_state.t2;
        // D s_8_25: cast zx s_8_24 -> i
        let s_8_25: i128 = (i128::try_from(s_8_24).unwrap());
        // D s_8_26: cast zx s_8_23 -> bv
        let s_8_26: Bits = Bits::new(s_8_23 as u128, 64u16);
        // D s_8_27: call X_set(s_8_25, s_8_14, s_8_26)
        let s_8_27: () = X_set(state, tracer, s_8_25, s_8_14, s_8_26);
        // N s_8_28: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var value1_name:u64
        let s_9_0: u64 = fn_state.value1_name;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 64u16);
        // D s_9_2: read-var value2_name:u64
        let s_9_2: u64 = fn_state.value2_name;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 64u16);
        // D s_9_4: cast reint s_9_1 -> u128
        let s_9_4: u128 = (s_9_1.value() as u128);
        // D s_9_5: size-of s_9_1
        let s_9_5: u16 = s_9_1.length();
        // D s_9_6: cast reint s_9_3 -> u128
        let s_9_6: u128 = (s_9_3.value() as u128);
        // D s_9_7: size-of s_9_3
        let s_9_7: u16 = s_9_3.length();
        // D s_9_8: lsl s_9_4 s_9_7
        let s_9_8: u128 = s_9_4 << s_9_7;
        // D s_9_9: or s_9_8 s_9_6
        let s_9_9: u128 = ((s_9_8) | (s_9_6));
        // D s_9_10: add s_9_5 s_9_7
        let s_9_10: u16 = (s_9_5 + s_9_7);
        // D s_9_11: create-bits s_9_9 s_9_10
        let s_9_11: Bits = Bits::new(s_9_9, s_9_10);
        // D s_9_12: cast reint s_9_11 -> u128
        let s_9_12: u128 = (s_9_11.value() as u128);
        // D s_9_13: write-var newdata <= s_9_12
        fn_state.newdata = s_9_12;
        // N s_9_14: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call CheckSPAlignment(s_10_0)
        let s_10_1: () = CheckSPAlignment(state, tracer, s_10_0);
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call SP_read(s_11_0)
        let s_11_1: u64 = SP_read(state, tracer, s_11_0);
        // D s_11_2: write-var address <= s_11_1
        fn_state.address = s_11_1;
        // N s_11_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
}
