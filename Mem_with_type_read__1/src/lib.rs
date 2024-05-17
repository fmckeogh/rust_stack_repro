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
use AArch32_MemSingle_read__1::*;
use AArch32_UnalignedAccessFaults::*;
use AArch32_Abort::*;
use BigEndian::*;
use IsAligned__1::*;
use AArch32_MemSingle_read::*;
use ConstrainUnpredictable::*;
use AlignmentFault::*;
use reverse_endianness::*;
use common::*;
pub fn Mem_with_type_read__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    size: i64,
    accdesc: ProductType9878976b5bcce9c9,
    ispair: bool,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        gs_31128: bool,
        ga_24253: i64,
        gs_455879: Bits,
        ga_24242: i64,
        gs_455868: Bits,
        gs_31120: bool,
        gs_31122: bool,
        gs_31137: bool,
        gs_31123: bool,
        value_name: Bits,
        gs_31143: i64,
        gs_31121: bool,
        ga_24254: i64,
        i: i64,
        aligned: bool,
        cshadow_586: u32,
        halfsize: i64,
        address: u32,
        size: i64,
        accdesc: ProductType9878976b5bcce9c9,
        ispair: bool,
    }
    let fn_state = FunctionState {
        address,
        size,
        accdesc,
        ispair,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_0_0: const #1s : i
        let s_0_0: i128 = 1;
        // D s_0_1: read-var size:i64
        let s_0_1: i64 = fn_state.size;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: branch s_0_3 b39 b1
        if s_0_3 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_1_0: const #2s : i
        let s_1_0: i128 = 2;
        // D s_1_1: read-var size:i64
        let s_1_1: i64 = fn_state.size;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: cmp-eq s_1_2 s_1_0
        let s_1_3: bool = ((s_1_2) == (s_1_0));
        // N s_1_4: branch s_1_3 b38 b2
        if s_1_3 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_2_0: const #4s : i
        let s_2_0: i128 = 4;
        // D s_2_1: read-var size:i64
        let s_2_1: i64 = fn_state.size;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_0
        let s_2_3: bool = ((s_2_2) == (s_2_0));
        // N s_2_4: branch s_2_3 b37 b3
        if s_2_3 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var size:i64
        let s_3_1: i64 = fn_state.size;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // N s_3_4: branch s_3_3 b36 b4
        if s_3_3 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_4_0: const #16s : i
        let s_4_0: i128 = 16;
        // D s_4_1: read-var size:i64
        let s_4_1: i64 = fn_state.size;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // D s_4_4: write-var gs#31120 <= s_4_3
        fn_state.gs_31120 = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_5_0: read-var gs#31120:u8
        let s_5_0: bool = fn_state.gs_31120;
        // D s_5_1: write-var gs#31121 <= s_5_0
        fn_state.gs_31121 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_6_0: read-var gs#31121:u8
        let s_6_0: bool = fn_state.gs_31121;
        // D s_6_1: write-var gs#31122 <= s_6_0
        fn_state.gs_31122 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_7_0: read-var gs#31122:u8
        let s_7_0: bool = fn_state.gs_31122;
        // D s_7_1: write-var gs#31123 <= s_7_0
        fn_state.gs_31123 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var gs#31123:u8
        let s_8_0: bool = fn_state.gs_31123;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // C s_8_2: const #2s : i
        let s_8_2: i128 = 2;
        // D s_8_3: read-var size:i64
        let s_8_3: i64 = fn_state.size;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: div s_8_4 s_8_2
        let s_8_5: i128 = ((s_8_4) / (s_8_2));
        // D s_8_6: cast reint s_8_5 -> i64
        let s_8_6: i64 = (s_8_5 as i64);
        // D s_8_7: write-var halfsize <= s_8_6
        fn_state.halfsize = s_8_6;
        // D s_8_8: read-var ispair:u8
        let s_8_8: bool = fn_state.ispair;
        // N s_8_9: branch s_8_8 b35 b9
        if s_8_8 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_9_0: read-var size:i64
        let s_9_0: i64 = fn_state.size;
        // D s_9_1: write-var ga#24242 <= s_9_0
        fn_state.ga_24242 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_10_0: read-var ga#24242:i64
        let s_10_0: i64 = fn_state.ga_24242;
        // D s_10_1: read-var address:u32
        let s_10_1: u32 = fn_state.address;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 32u16);
        // D s_10_3: cast zx s_10_0 -> i
        let s_10_3: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_4: call IsAligned__1(s_10_2, s_10_3)
        let s_10_4: bool = IsAligned__1(state, tracer, s_10_2, s_10_3);
        // D s_10_5: write-var aligned <= s_10_4
        fn_state.aligned = s_10_4;
        // D s_10_6: read-var aligned:u8
        let s_10_6: bool = fn_state.aligned;
        // D s_10_7: not s_10_6
        let s_10_7: bool = !s_10_6;
        // N s_10_8: branch s_10_7 b34 b11
        if s_10_7 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#31128 <= s_11_0
        fn_state.gs_31128 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_12_0: read-var gs#31128:u8
        let s_12_0: bool = fn_state.gs_31128;
        // N s_12_1: branch s_12_0 b33 b13
        if s_12_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_14_0: read-var aligned:u8
        let s_14_0: bool = fn_state.aligned;
        // N s_14_1: branch s_14_0 b31 b15
        if s_14_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_15_0: const #1s : i
        let s_15_0: i128 = 1;
        // D s_15_1: read-var size:i64
        let s_15_1: i64 = fn_state.size;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: cmp-gt s_15_2 s_15_0
        let s_15_3: bool = ((s_15_2) > (s_15_0));
        // N s_15_4: assert s_15_3
        let s_15_4: () = assert!(s_15_3);
        // C s_15_5: const #1s : i64
        let s_15_5: i64 = 1;
        // D s_15_6: read-var address:u32
        let s_15_6: u32 = fn_state.address;
        // D s_15_7: read-var accdesc:struct
        let s_15_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_15_8: read-var aligned:u8
        let s_15_8: bool = fn_state.aligned;
        // D s_15_9: call AArch32_MemSingle_read(s_15_6, s_15_5, s_15_7, s_15_8)
        let s_15_9: Bits = AArch32_MemSingle_read(
            state,
            tracer,
            s_15_6,
            s_15_5,
            s_15_7,
            s_15_8,
        );
        // D s_15_10: write-var gs#455868 <= s_15_9
        fn_state.gs_455868 = s_15_9;
        // N s_15_11: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_16_0: read-var gs#455868:bv
        let s_16_0: Bits = fn_state.gs_455868;
        // D s_16_1: cast reint s_16_0 -> u8
        let s_16_1: u8 = (s_16_0.value() as u8);
        // C s_16_2: const #0s : i
        let s_16_2: i128 = 0;
        // D s_16_3: cast zx s_16_1 -> bv
        let s_16_3: Bits = Bits::new(s_16_1 as u128, 8u16);
        // D s_16_4: read-var value_name:bv
        let s_16_4: Bits = fn_state.value_name;
        // C s_16_5: const #7s : i
        let s_16_5: i128 = 7;
        // C s_16_6: const #1u : u64
        let s_16_6: u64 = 1;
        // C s_16_7: cast zx s_16_6 -> bv
        let s_16_7: Bits = Bits::new(s_16_6 as u128, 64u16);
        // C s_16_8: lsl s_16_7 s_16_5
        let s_16_8: Bits = s_16_7 << s_16_5;
        // C s_16_9: sub s_16_8 s_16_7
        let s_16_9: Bits = ((s_16_8) - (s_16_7));
        // D s_16_10: and s_16_3 s_16_9
        let s_16_10: Bits = ((s_16_3) & (s_16_9));
        // D s_16_11: lsl s_16_10 s_16_2
        let s_16_11: Bits = s_16_10 << s_16_2;
        // C s_16_12: lsl s_16_9 s_16_2
        let s_16_12: Bits = s_16_9 << s_16_2;
        // C s_16_13: cmpl s_16_12
        let s_16_13: Bits = !s_16_12;
        // D s_16_14: and s_16_4 s_16_13
        let s_16_14: Bits = ((s_16_4) & (s_16_13));
        // D s_16_15: or s_16_14 s_16_11
        let s_16_15: Bits = ((s_16_14) | (s_16_11));
        // D s_16_16: write-var value_name <= s_16_15
        fn_state.value_name = s_16_15;
        // C s_16_17: const #6u : u32
        let s_16_17: u32 = 6;
        // S s_16_18: call ConstrainUnpredictable(s_16_17)
        let s_16_18: u32 = ConstrainUnpredictable(state, tracer, s_16_17);
        // D s_16_19: write-var cshadow#586 <= s_16_18
        fn_state.cshadow_586 = s_16_18;
        // D s_16_20: read-var cshadow#586:u32
        let s_16_20: u32 = fn_state.cshadow_586;
        // C s_16_21: const #12u : u32
        let s_16_21: u32 = 12;
        // D s_16_22: cmp-eq s_16_20 s_16_21
        let s_16_22: bool = ((s_16_20) == (s_16_21));
        // N s_16_23: branch s_16_22 b30 b17
        if s_16_22 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_17_0: read-var cshadow#586:u32
        let s_17_0: u32 = fn_state.cshadow_586;
        // C s_17_1: const #0u : u32
        let s_17_1: u32 = 0;
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // D s_17_3: write-var gs#31137 <= s_17_2
        fn_state.gs_31137 = s_17_2;
        // N s_17_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_18_0: read-var gs#31137:u8
        let s_18_0: bool = fn_state.gs_31137;
        // N s_18_1: assert s_18_0
        let s_18_1: () = assert!(s_18_0);
        // D s_18_2: read-var cshadow#586:u32
        let s_18_2: u32 = fn_state.cshadow_586;
        // C s_18_3: const #0u : u32
        let s_18_3: u32 = 0;
        // D s_18_4: cmp-eq s_18_2 s_18_3
        let s_18_4: bool = ((s_18_2) == (s_18_3));
        // N s_18_5: branch s_18_4 b29 b19
        if s_18_4 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_20_0: const #1s : i64
        let s_20_0: i64 = 1;
        // C s_20_1: const #1s : i
        let s_20_1: i128 = 1;
        // D s_20_2: read-var size:i64
        let s_20_2: i64 = fn_state.size;
        // D s_20_3: cast zx s_20_2 -> i
        let s_20_3: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_4: sub s_20_3 s_20_1
        let s_20_4: i128 = ((s_20_3) - (s_20_1));
        // D s_20_5: cast reint s_20_4 -> i64
        let s_20_5: i64 = (s_20_4 as i64);
        // D s_20_6: write-var gs#31143 <= s_20_5
        fn_state.gs_31143 = s_20_5;
        // D s_20_7: write-var i <= s_20_0
        fn_state.i = s_20_0;
        // N s_20_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_21_0: read-var i:i64
        let s_21_0: i64 = fn_state.i;
        // D s_21_1: read-var gs#31143:i64
        let s_21_1: i64 = fn_state.gs_31143;
        // D s_21_2: cmp-gt s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) > (s_21_1));
        // N s_21_3: branch s_21_2 b24 b22
        if s_21_2 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_22_0: const #8s : i
        let s_22_0: i128 = 8;
        // D s_22_1: read-var i:i64
        let s_22_1: i64 = fn_state.i;
        // D s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (i128::try_from(s_22_1).unwrap());
        // D s_22_3: mul s_22_0 s_22_2
        let s_22_3: i128 = ((s_22_0) * (s_22_2));
        // D s_22_4: cast reint s_22_3 -> i64
        let s_22_4: i64 = (s_22_3 as i64);
        // C s_22_5: const #7s : i
        let s_22_5: i128 = 7;
        // D s_22_6: cast zx s_22_4 -> i
        let s_22_6: i128 = (i128::try_from(s_22_4).unwrap());
        // D s_22_7: add s_22_6 s_22_5
        let s_22_7: i128 = (s_22_6 + s_22_5);
        // D s_22_8: cast reint s_22_7 -> i64
        let s_22_8: i64 = (s_22_7 as i64);
        // D s_22_9: write-var ga#24253 <= s_22_8
        fn_state.ga_24253 = s_22_8;
        // C s_22_10: const #8s : i
        let s_22_10: i128 = 8;
        // D s_22_11: read-var i:i64
        let s_22_11: i64 = fn_state.i;
        // D s_22_12: cast zx s_22_11 -> i
        let s_22_12: i128 = (i128::try_from(s_22_11).unwrap());
        // D s_22_13: mul s_22_10 s_22_12
        let s_22_13: i128 = ((s_22_10) * (s_22_12));
        // D s_22_14: cast reint s_22_13 -> i64
        let s_22_14: i64 = (s_22_13 as i64);
        // D s_22_15: write-var ga#24254 <= s_22_14
        fn_state.ga_24254 = s_22_14;
        // D s_22_16: read-var address:u32
        let s_22_16: u32 = fn_state.address;
        // D s_22_17: cast zx s_22_16 -> bv
        let s_22_17: Bits = Bits::new(s_22_16 as u128, 32u16);
        // D s_22_18: read-var i:i64
        let s_22_18: i64 = fn_state.i;
        // D s_22_19: cast zx s_22_18 -> i
        let s_22_19: i128 = (i128::try_from(s_22_18).unwrap());
        // D s_22_20: cast cvt s_22_19 -> bv
        let s_22_20: Bits = Bits::new(s_22_19 as u128, 128);
        // D s_22_21: add s_22_17 s_22_20
        let s_22_21: Bits = (s_22_17 + s_22_20);
        // D s_22_22: cast reint s_22_21 -> u32
        let s_22_22: u32 = (s_22_21.value() as u32);
        // C s_22_23: const #1s : i64
        let s_22_23: i64 = 1;
        // D s_22_24: read-var accdesc:struct
        let s_22_24: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_22_25: read-var aligned:u8
        let s_22_25: bool = fn_state.aligned;
        // D s_22_26: call AArch32_MemSingle_read(s_22_22, s_22_23, s_22_24, s_22_25)
        let s_22_26: Bits = AArch32_MemSingle_read(
            state,
            tracer,
            s_22_22,
            s_22_23,
            s_22_24,
            s_22_25,
        );
        // D s_22_27: write-var gs#455879 <= s_22_26
        fn_state.gs_455879 = s_22_26;
        // N s_22_28: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_23_0: read-var gs#455879:bv
        let s_23_0: Bits = fn_state.gs_455879;
        // D s_23_1: cast reint s_23_0 -> u8
        let s_23_1: u8 = (s_23_0.value() as u8);
        // D s_23_2: read-var ga#24253:i64
        let s_23_2: i64 = fn_state.ga_24253;
        // D s_23_3: cast zx s_23_2 -> i
        let s_23_3: i128 = (i128::try_from(s_23_2).unwrap());
        // D s_23_4: read-var ga#24254:i64
        let s_23_4: i64 = fn_state.ga_24254;
        // D s_23_5: cast zx s_23_4 -> i
        let s_23_5: i128 = (i128::try_from(s_23_4).unwrap());
        // D s_23_6: cast zx s_23_1 -> bv
        let s_23_6: Bits = Bits::new(s_23_1 as u128, 8u16);
        // D s_23_7: read-var value_name:bv
        let s_23_7: Bits = fn_state.value_name;
        // D s_23_8: sub s_23_3 s_23_5
        let s_23_8: i128 = ((s_23_3) - (s_23_5));
        // C s_23_9: const #1u : u64
        let s_23_9: u64 = 1;
        // C s_23_10: cast zx s_23_9 -> bv
        let s_23_10: Bits = Bits::new(s_23_9 as u128, 64u16);
        // D s_23_11: lsl s_23_10 s_23_8
        let s_23_11: Bits = s_23_10 << s_23_8;
        // D s_23_12: sub s_23_11 s_23_10
        let s_23_12: Bits = ((s_23_11) - (s_23_10));
        // D s_23_13: and s_23_6 s_23_12
        let s_23_13: Bits = ((s_23_6) & (s_23_12));
        // D s_23_14: lsl s_23_13 s_23_5
        let s_23_14: Bits = s_23_13 << s_23_5;
        // D s_23_15: lsl s_23_12 s_23_5
        let s_23_15: Bits = s_23_12 << s_23_5;
        // D s_23_16: cmpl s_23_15
        let s_23_16: Bits = !s_23_15;
        // D s_23_17: and s_23_7 s_23_16
        let s_23_17: Bits = ((s_23_7) & (s_23_16));
        // D s_23_18: or s_23_17 s_23_14
        let s_23_18: Bits = ((s_23_17) | (s_23_14));
        // D s_23_19: write-var value_name <= s_23_18
        fn_state.value_name = s_23_18;
        // D s_23_20: read-var i:i64
        let s_23_20: i64 = fn_state.i;
        // C s_23_21: const #1s : i64
        let s_23_21: i64 = 1;
        // D s_23_22: add s_23_20 s_23_21
        let s_23_22: i64 = (s_23_20 + s_23_21);
        // D s_23_23: write-var i <= s_23_22
        fn_state.i = s_23_22;
        // N s_23_24: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_25_0: read-var accdesc.1:struct
        let s_25_0: u32 = fn_state.accdesc._1;
        // D s_25_1: call BigEndian(s_25_0)
        let s_25_1: bool = BigEndian(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b28 b26
        if s_25_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_27_0: read-var value_name:bv
        let s_27_0: Bits = fn_state.value_name;
        // N s_27_1: return s_27_0
        return s_27_0;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_28_0: read-var value_name:bv
        let s_28_0: Bits = fn_state.value_name;
        // D s_28_1: call reverse_endianness(s_28_0)
        let s_28_1: Bits = reverse_endianness(state, tracer, s_28_0);
        // D s_28_2: write-var value_name <= s_28_1
        fn_state.value_name = s_28_1;
        // N s_28_3: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var aligned <= s_29_0
        fn_state.aligned = s_29_0;
        // N s_29_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#31137 <= s_30_0
        fn_state.gs_31137 = s_30_0;
        // N s_30_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_31_0: read-var address:u32
        let s_31_0: u32 = fn_state.address;
        // D s_31_1: read-var size:i64
        let s_31_1: i64 = fn_state.size;
        // D s_31_2: read-var accdesc:struct
        let s_31_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_31_3: read-var aligned:u8
        let s_31_3: bool = fn_state.aligned;
        // D s_31_4: read-var ispair:u8
        let s_31_4: bool = fn_state.ispair;
        // D s_31_5: call AArch32_MemSingle_read__1(s_31_0, s_31_1, s_31_2, s_31_3, s_31_4)
        let s_31_5: Bits = AArch32_MemSingle_read__1(
            state,
            tracer,
            s_31_0,
            s_31_1,
            s_31_2,
            s_31_3,
            s_31_4,
        );
        // D s_31_6: write-var value_name <= s_31_5
        fn_state.value_name = s_31_5;
        // N s_31_7: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_32_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_33_0: read-var accdesc:struct
        let s_33_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_33_1: call AlignmentFault(s_33_0)
        let s_33_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_33_0);
        // D s_33_2: read-var address:u32
        let s_33_2: u32 = fn_state.address;
        // D s_33_3: call AArch32_Abort(s_33_2, s_33_1)
        let s_33_3: () = AArch32_Abort(state, tracer, s_33_2, s_33_1);
        // N s_33_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_34_0: read-var accdesc:struct
        let s_34_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_34_1: call AArch32_UnalignedAccessFaults(s_34_0)
        let s_34_1: bool = AArch32_UnalignedAccessFaults(state, tracer, s_34_0);
        // D s_34_2: write-var gs#31128 <= s_34_1
        fn_state.gs_31128 = s_34_1;
        // N s_34_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_35_0: read-var halfsize:i64
        let s_35_0: i64 = fn_state.halfsize;
        // D s_35_1: write-var ga#24242 <= s_35_0
        fn_state.ga_24242 = s_35_0;
        // N s_35_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#31120 <= s_36_0
        fn_state.gs_31120 = s_36_0;
        // N s_36_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#31121 <= s_37_0
        fn_state.gs_31121 = s_37_0;
        // N s_37_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#31122 <= s_38_0
        fn_state.gs_31122 = s_38_0;
        // N s_38_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#31123 <= s_39_0
        fn_state.gs_31123 = s_39_0;
        // N s_39_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
