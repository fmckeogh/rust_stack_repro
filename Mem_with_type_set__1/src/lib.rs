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
use AArch32_MemSingle_set::*;
use u__id::*;
use AArch32_Abort::*;
use AArch32_UnalignedAccessFaults::*;
use BigEndian::*;
use IsAligned__1::*;
use ConstrainUnpredictable::*;
use AArch32_MemSingle_set__1::*;
use reverse_endianness::*;
use AlignmentFault::*;
use common::*;
pub fn Mem_with_type_set__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    size: i128,
    accdesc: ProductType9878976b5bcce9c9,
    ispair: bool,
    value_in_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_31157: bool,
        gs_31165: bool,
        gs_31184: bool,
        cshadow_587: u32,
        value_name: Bits,
        gs_31171: i64,
        ga_24259: i128,
        halfsize: i128,
        gs_31199: bool,
        gs_31201: bool,
        gs_31180: bool,
        gs_31186: bool,
        gs_31182: bool,
        i: i64,
        aligned: bool,
        gs_31200: bool,
        gs_31202: bool,
        address: u32,
        size: i128,
        accdesc: ProductType9878976b5bcce9c9,
        ispair: bool,
        value_in_name: Bits,
    }
    let fn_state = FunctionState {
        address,
        size,
        accdesc,
        ispair,
        value_in_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #2s : i
        let s_0_0: i128 = 2;
        // D s_0_1: read-var size:i
        let s_0_1: i128 = fn_state.size;
        // D s_0_2: div s_0_1 s_0_0
        let s_0_2: i128 = ((s_0_1) / (s_0_0));
        // D s_0_3: write-var halfsize <= s_0_2
        fn_state.halfsize = s_0_2;
        // D s_0_4: read-var value_in_name:bv
        let s_0_4: Bits = fn_state.value_in_name;
        // D s_0_5: write-var value_name <= s_0_4
        fn_state.value_name = s_0_4;
        // D s_0_6: read-var ispair:u8
        let s_0_6: bool = fn_state.ispair;
        // N s_0_7: branch s_0_6 b49 b1
        if s_0_6 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:i
        let s_1_0: i128 = fn_state.size;
        // D s_1_1: write-var ga#24259 <= s_1_0
        fn_state.ga_24259 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#24259:i
        let s_2_0: i128 = fn_state.ga_24259;
        // D s_2_1: read-var address:u32
        let s_2_1: u32 = fn_state.address;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 32u16);
        // D s_2_3: call IsAligned__1(s_2_2, s_2_0)
        let s_2_3: bool = IsAligned__1(state, tracer, s_2_2, s_2_0);
        // D s_2_4: write-var aligned <= s_2_3
        fn_state.aligned = s_2_3;
        // D s_2_5: read-var aligned:u8
        let s_2_5: bool = fn_state.aligned;
        // D s_2_6: not s_2_5
        let s_2_6: bool = !s_2_5;
        // N s_2_7: branch s_2_6 b48 b3
        if s_2_6 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#31157 <= s_3_0
        fn_state.gs_31157 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#31157:u8
        let s_4_0: bool = fn_state.gs_31157;
        // N s_4_1: branch s_4_0 b47 b5
        if s_4_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var accdesc.1:struct
        let s_6_0: u32 = fn_state.accdesc._1;
        // D s_6_1: call BigEndian(s_6_0)
        let s_6_1: bool = BigEndian(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b34 b7
        if s_6_1 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var aligned:u8
        let s_8_0: bool = fn_state.aligned;
        // N s_8_1: branch s_8_0 b21 b9
        if s_8_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1s : i
        let s_9_0: i128 = 1;
        // D s_9_1: read-var size:i
        let s_9_1: i128 = fn_state.size;
        // D s_9_2: cmp-gt s_9_1 s_9_0
        let s_9_2: bool = ((s_9_1) > (s_9_0));
        // N s_9_3: assert s_9_2
        let s_9_3: () = assert!(s_9_2);
        // C s_9_4: const #0s : i
        let s_9_4: i128 = 0;
        // D s_9_5: read-var value_name:bv
        let s_9_5: Bits = fn_state.value_name;
        // C s_9_6: const #1s : i64
        let s_9_6: i64 = 1;
        // C s_9_7: cast zx s_9_6 -> i
        let s_9_7: i128 = (i128::try_from(s_9_6).unwrap());
        // C s_9_8: const #7s : i
        let s_9_8: i128 = 7;
        // C s_9_9: add s_9_8 s_9_7
        let s_9_9: i128 = (s_9_8 + s_9_7);
        // D s_9_10: bit-extract s_9_5 s_9_4 s_9_9
        let s_9_10: Bits = (Bits::new(
            ((s_9_5) >> (s_9_4)).value(),
            u16::try_from(s_9_9).unwrap(),
        ));
        // D s_9_11: cast reint s_9_10 -> u8
        let s_9_11: u8 = (s_9_10.value() as u8);
        // C s_9_12: const #1s : i64
        let s_9_12: i64 = 1;
        // D s_9_13: cast zx s_9_11 -> bv
        let s_9_13: Bits = Bits::new(s_9_11 as u128, 8u16);
        // D s_9_14: read-var address:u32
        let s_9_14: u32 = fn_state.address;
        // D s_9_15: read-var accdesc:struct
        let s_9_15: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_9_16: read-var aligned:u8
        let s_9_16: bool = fn_state.aligned;
        // D s_9_17: call AArch32_MemSingle_set(s_9_14, s_9_12, s_9_15, s_9_16, s_9_13)
        let s_9_17: () = AArch32_MemSingle_set(
            state,
            tracer,
            s_9_14,
            s_9_12,
            s_9_15,
            s_9_16,
            s_9_13,
        );
        // N s_9_18: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #6u : u32
        let s_10_0: u32 = 6;
        // S s_10_1: call ConstrainUnpredictable(s_10_0)
        let s_10_1: u32 = ConstrainUnpredictable(state, tracer, s_10_0);
        // D s_10_2: write-var cshadow#587 <= s_10_1
        fn_state.cshadow_587 = s_10_1;
        // D s_10_3: read-var cshadow#587:u32
        let s_10_3: u32 = fn_state.cshadow_587;
        // C s_10_4: const #12u : u32
        let s_10_4: u32 = 12;
        // D s_10_5: cmp-eq s_10_3 s_10_4
        let s_10_5: bool = ((s_10_3) == (s_10_4));
        // N s_10_6: branch s_10_5 b20 b11
        if s_10_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var cshadow#587:u32
        let s_11_0: u32 = fn_state.cshadow_587;
        // C s_11_1: const #0u : u32
        let s_11_1: u32 = 0;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: write-var gs#31165 <= s_11_2
        fn_state.gs_31165 = s_11_2;
        // N s_11_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#31165:u8
        let s_12_0: bool = fn_state.gs_31165;
        // N s_12_1: assert s_12_0
        let s_12_1: () = assert!(s_12_0);
        // D s_12_2: read-var cshadow#587:u32
        let s_12_2: u32 = fn_state.cshadow_587;
        // C s_12_3: const #0u : u32
        let s_12_3: u32 = 0;
        // D s_12_4: cmp-eq s_12_2 s_12_3
        let s_12_4: bool = ((s_12_2) == (s_12_3));
        // N s_12_5: branch s_12_4 b19 b13
        if s_12_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1s : i64
        let s_14_0: i64 = 1;
        // C s_14_1: const #1s : i
        let s_14_1: i128 = 1;
        // D s_14_2: read-var size:i
        let s_14_2: i128 = fn_state.size;
        // D s_14_3: sub s_14_2 s_14_1
        let s_14_3: i128 = ((s_14_2) - (s_14_1));
        // D s_14_4: cast reint s_14_3 -> i64
        let s_14_4: i64 = (s_14_3 as i64);
        // D s_14_5: write-var gs#31171 <= s_14_4
        fn_state.gs_31171 = s_14_4;
        // D s_14_6: write-var i <= s_14_0
        fn_state.i = s_14_0;
        // N s_14_7: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var i:i64
        let s_15_0: i64 = fn_state.i;
        // D s_15_1: read-var gs#31171:i64
        let s_15_1: i64 = fn_state.gs_31171;
        // D s_15_2: cmp-gt s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) > (s_15_1));
        // N s_15_3: branch s_15_2 b18 b16
        if s_15_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var address:u32
        let s_16_0: u32 = fn_state.address;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 32u16);
        // D s_16_2: read-var i:i64
        let s_16_2: i64 = fn_state.i;
        // D s_16_3: cast zx s_16_2 -> i
        let s_16_3: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_4: cast cvt s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 128);
        // D s_16_5: add s_16_1 s_16_4
        let s_16_5: Bits = (s_16_1 + s_16_4);
        // D s_16_6: cast reint s_16_5 -> u32
        let s_16_6: u32 = (s_16_5.value() as u32);
        // C s_16_7: const #8s : i
        let s_16_7: i128 = 8;
        // D s_16_8: read-var i:i64
        let s_16_8: i64 = fn_state.i;
        // D s_16_9: cast zx s_16_8 -> i
        let s_16_9: i128 = (i128::try_from(s_16_8).unwrap());
        // D s_16_10: mul s_16_7 s_16_9
        let s_16_10: i128 = ((s_16_7) * (s_16_9));
        // C s_16_11: const #8s : i
        let s_16_11: i128 = 8;
        // D s_16_12: read-var value_name:bv
        let s_16_12: Bits = fn_state.value_name;
        // D s_16_13: bit-extract s_16_12 s_16_10 s_16_11
        let s_16_13: Bits = (Bits::new(
            ((s_16_12) >> (s_16_10)).value(),
            u16::try_from(s_16_11).unwrap(),
        ));
        // D s_16_14: cast reint s_16_13 -> u8
        let s_16_14: u8 = (s_16_13.value() as u8);
        // C s_16_15: const #1s : i64
        let s_16_15: i64 = 1;
        // D s_16_16: cast zx s_16_14 -> bv
        let s_16_16: Bits = Bits::new(s_16_14 as u128, 8u16);
        // D s_16_17: read-var accdesc:struct
        let s_16_17: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_16_18: read-var aligned:u8
        let s_16_18: bool = fn_state.aligned;
        // D s_16_19: call AArch32_MemSingle_set(s_16_6, s_16_15, s_16_17, s_16_18, s_16_16)
        let s_16_19: () = AArch32_MemSingle_set(
            state,
            tracer,
            s_16_6,
            s_16_15,
            s_16_17,
            s_16_18,
            s_16_16,
        );
        // N s_16_20: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var i:i64
        let s_17_0: i64 = fn_state.i;
        // C s_17_1: const #1s : i64
        let s_17_1: i64 = 1;
        // D s_17_2: add s_17_0 s_17_1
        let s_17_2: i64 = (s_17_0 + s_17_1);
        // D s_17_3: write-var i <= s_17_2
        fn_state.i = s_17_2;
        // N s_17_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var aligned <= s_19_0
        fn_state.aligned = s_19_0;
        // N s_19_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#31165 <= s_20_0
        fn_state.gs_31165 = s_20_0;
        // N s_20_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var size:i
        let s_21_0: i128 = fn_state.size;
        // D s_21_1: call __id(s_21_0)
        let s_21_1: i128 = u__id(state, tracer, s_21_0);
        // C s_21_2: const #1s : i
        let s_21_2: i128 = 1;
        // D s_21_3: cmp-eq s_21_1 s_21_2
        let s_21_3: bool = ((s_21_1) == (s_21_2));
        // N s_21_4: branch s_21_3 b33 b22
        if s_21_3 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var size:i
        let s_22_0: i128 = fn_state.size;
        // D s_22_1: call __id(s_22_0)
        let s_22_1: i128 = u__id(state, tracer, s_22_0);
        // C s_22_2: const #2s : i
        let s_22_2: i128 = 2;
        // D s_22_3: cmp-eq s_22_1 s_22_2
        let s_22_3: bool = ((s_22_1) == (s_22_2));
        // D s_22_4: write-var gs#31180 <= s_22_3
        fn_state.gs_31180 = s_22_3;
        // N s_22_5: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#31180:u8
        let s_23_0: bool = fn_state.gs_31180;
        // N s_23_1: branch s_23_0 b32 b24
        if s_23_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var size:i
        let s_24_0: i128 = fn_state.size;
        // D s_24_1: call __id(s_24_0)
        let s_24_1: i128 = u__id(state, tracer, s_24_0);
        // C s_24_2: const #4s : i
        let s_24_2: i128 = 4;
        // D s_24_3: cmp-eq s_24_1 s_24_2
        let s_24_3: bool = ((s_24_1) == (s_24_2));
        // D s_24_4: write-var gs#31182 <= s_24_3
        fn_state.gs_31182 = s_24_3;
        // N s_24_5: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#31182:u8
        let s_25_0: bool = fn_state.gs_31182;
        // N s_25_1: branch s_25_0 b31 b26
        if s_25_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var size:i
        let s_26_0: i128 = fn_state.size;
        // D s_26_1: call __id(s_26_0)
        let s_26_1: i128 = u__id(state, tracer, s_26_0);
        // C s_26_2: const #8s : i
        let s_26_2: i128 = 8;
        // D s_26_3: cmp-eq s_26_1 s_26_2
        let s_26_3: bool = ((s_26_1) == (s_26_2));
        // D s_26_4: write-var gs#31184 <= s_26_3
        fn_state.gs_31184 = s_26_3;
        // N s_26_5: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#31184:u8
        let s_27_0: bool = fn_state.gs_31184;
        // N s_27_1: branch s_27_0 b30 b28
        if s_27_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var size:i
        let s_28_0: i128 = fn_state.size;
        // D s_28_1: call __id(s_28_0)
        let s_28_1: i128 = u__id(state, tracer, s_28_0);
        // C s_28_2: const #16s : i
        let s_28_2: i128 = 16;
        // D s_28_3: cmp-eq s_28_1 s_28_2
        let s_28_3: bool = ((s_28_1) == (s_28_2));
        // D s_28_4: write-var gs#31186 <= s_28_3
        fn_state.gs_31186 = s_28_3;
        // N s_28_5: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#31186:u8
        let s_29_0: bool = fn_state.gs_31186;
        // N s_29_1: assert s_29_0
        let s_29_1: () = assert!(s_29_0);
        // D s_29_2: read-var size:i
        let s_29_2: i128 = fn_state.size;
        // D s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // D s_29_4: read-var address:u32
        let s_29_4: u32 = fn_state.address;
        // D s_29_5: read-var accdesc:struct
        let s_29_5: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_29_6: read-var aligned:u8
        let s_29_6: bool = fn_state.aligned;
        // D s_29_7: read-var ispair:u8
        let s_29_7: bool = fn_state.ispair;
        // D s_29_8: read-var value_name:bv
        let s_29_8: Bits = fn_state.value_name;
        // D s_29_9: call AArch32_MemSingle_set__1(s_29_4, s_29_3, s_29_5, s_29_6, s_29_7, s_29_8)
        let s_29_9: () = AArch32_MemSingle_set__1(
            state,
            tracer,
            s_29_4,
            s_29_3,
            s_29_5,
            s_29_6,
            s_29_7,
            s_29_8,
        );
        // N s_29_10: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#31186 <= s_30_0
        fn_state.gs_31186 = s_30_0;
        // N s_30_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#31184 <= s_31_0
        fn_state.gs_31184 = s_31_0;
        // N s_31_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#31182 <= s_32_0
        fn_state.gs_31182 = s_32_0;
        // N s_32_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#31180 <= s_33_0
        fn_state.gs_31180 = s_33_0;
        // N s_33_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var size:i
        let s_34_0: i128 = fn_state.size;
        // D s_34_1: call __id(s_34_0)
        let s_34_1: i128 = u__id(state, tracer, s_34_0);
        // C s_34_2: const #8s : i
        let s_34_2: i128 = 8;
        // D s_34_3: mul s_34_1 s_34_2
        let s_34_3: i128 = ((s_34_1) * (s_34_2));
        // C s_34_4: const #8s : i
        let s_34_4: i128 = 8;
        // D s_34_5: cmp-eq s_34_3 s_34_4
        let s_34_5: bool = ((s_34_3) == (s_34_4));
        // N s_34_6: branch s_34_5 b46 b35
        if s_34_5 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var size:i
        let s_35_0: i128 = fn_state.size;
        // D s_35_1: call __id(s_35_0)
        let s_35_1: i128 = u__id(state, tracer, s_35_0);
        // C s_35_2: const #8s : i
        let s_35_2: i128 = 8;
        // D s_35_3: mul s_35_1 s_35_2
        let s_35_3: i128 = ((s_35_1) * (s_35_2));
        // C s_35_4: const #16s : i
        let s_35_4: i128 = 16;
        // D s_35_5: cmp-eq s_35_3 s_35_4
        let s_35_5: bool = ((s_35_3) == (s_35_4));
        // N s_35_6: branch s_35_5 b45 b36
        if s_35_5 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var size:i
        let s_36_0: i128 = fn_state.size;
        // D s_36_1: call __id(s_36_0)
        let s_36_1: i128 = u__id(state, tracer, s_36_0);
        // C s_36_2: const #8s : i
        let s_36_2: i128 = 8;
        // D s_36_3: mul s_36_1 s_36_2
        let s_36_3: i128 = ((s_36_1) * (s_36_2));
        // C s_36_4: const #32s : i
        let s_36_4: i128 = 32;
        // D s_36_5: cmp-eq s_36_3 s_36_4
        let s_36_5: bool = ((s_36_3) == (s_36_4));
        // N s_36_6: branch s_36_5 b44 b37
        if s_36_5 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var size:i
        let s_37_0: i128 = fn_state.size;
        // D s_37_1: call __id(s_37_0)
        let s_37_1: i128 = u__id(state, tracer, s_37_0);
        // C s_37_2: const #8s : i
        let s_37_2: i128 = 8;
        // D s_37_3: mul s_37_1 s_37_2
        let s_37_3: i128 = ((s_37_1) * (s_37_2));
        // C s_37_4: const #64s : i
        let s_37_4: i128 = 64;
        // D s_37_5: cmp-eq s_37_3 s_37_4
        let s_37_5: bool = ((s_37_3) == (s_37_4));
        // N s_37_6: branch s_37_5 b43 b38
        if s_37_5 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var size:i
        let s_38_0: i128 = fn_state.size;
        // D s_38_1: call __id(s_38_0)
        let s_38_1: i128 = u__id(state, tracer, s_38_0);
        // C s_38_2: const #8s : i
        let s_38_2: i128 = 8;
        // D s_38_3: mul s_38_1 s_38_2
        let s_38_3: i128 = ((s_38_1) * (s_38_2));
        // C s_38_4: const #128s : i
        let s_38_4: i128 = 128;
        // D s_38_5: cmp-eq s_38_3 s_38_4
        let s_38_5: bool = ((s_38_3) == (s_38_4));
        // D s_38_6: write-var gs#31199 <= s_38_5
        fn_state.gs_31199 = s_38_5;
        // N s_38_7: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#31199:u8
        let s_39_0: bool = fn_state.gs_31199;
        // D s_39_1: write-var gs#31200 <= s_39_0
        fn_state.gs_31200 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#31200:u8
        let s_40_0: bool = fn_state.gs_31200;
        // D s_40_1: write-var gs#31201 <= s_40_0
        fn_state.gs_31201 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#31201:u8
        let s_41_0: bool = fn_state.gs_31201;
        // D s_41_1: write-var gs#31202 <= s_41_0
        fn_state.gs_31202 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#31202:u8
        let s_42_0: bool = fn_state.gs_31202;
        // N s_42_1: assert s_42_0
        let s_42_1: () = assert!(s_42_0);
        // D s_42_2: read-var value_name:bv
        let s_42_2: Bits = fn_state.value_name;
        // D s_42_3: call reverse_endianness(s_42_2)
        let s_42_3: Bits = reverse_endianness(state, tracer, s_42_2);
        // D s_42_4: write-var value_name <= s_42_3
        fn_state.value_name = s_42_3;
        // N s_42_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#31199 <= s_43_0
        fn_state.gs_31199 = s_43_0;
        // N s_43_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#31200 <= s_44_0
        fn_state.gs_31200 = s_44_0;
        // N s_44_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#31201 <= s_45_0
        fn_state.gs_31201 = s_45_0;
        // N s_45_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#31202 <= s_46_0
        fn_state.gs_31202 = s_46_0;
        // N s_46_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var accdesc:struct
        let s_47_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_47_1: call AlignmentFault(s_47_0)
        let s_47_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_47_0);
        // D s_47_2: read-var address:u32
        let s_47_2: u32 = fn_state.address;
        // D s_47_3: call AArch32_Abort(s_47_2, s_47_1)
        let s_47_3: () = AArch32_Abort(state, tracer, s_47_2, s_47_1);
        // N s_47_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var accdesc:struct
        let s_48_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_48_1: call AArch32_UnalignedAccessFaults(s_48_0)
        let s_48_1: bool = AArch32_UnalignedAccessFaults(state, tracer, s_48_0);
        // D s_48_2: write-var gs#31157 <= s_48_1
        fn_state.gs_31157 = s_48_1;
        // N s_48_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var halfsize:i
        let s_49_0: i128 = fn_state.halfsize;
        // D s_49_1: write-var ga#24259 <= s_49_0
        fn_state.ga_24259 = s_49_0;
        // N s_49_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
