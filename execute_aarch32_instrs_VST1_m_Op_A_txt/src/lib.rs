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
use neq_int::*;
use AlignmentEnforced::*;
use R_read::*;
use CheckAdvSIMDEnabled::*;
use MemU_set::*;
use AArch32_Abort::*;
use R_set::*;
use BigEndian::*;
use IsAligned__1::*;
use CreateAccDescASIMD::*;
use Elem_read::*;
use D_read::*;
use AlignmentFault::*;
use common::*;
pub fn execute_aarch32_instrs_VST1_m_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    alignment: i64,
    d: i64,
    ebytes: i64,
    elements: i128,
    m: i64,
    n: i64,
    register_index: bool,
    regs: i64,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        gs_320176: bool,
        e: i64,
        address: u32,
        data: u64,
        accdesc: ProductType9878976b5bcce9c9,
        gs_320165: i64,
        gs_320171: i64,
        alignment: i64,
        d: i64,
        ebytes: i64,
        elements: i128,
        m: i64,
        n: i64,
        register_index: bool,
        regs: i64,
        wback: bool,
    }
    let fn_state = FunctionState {
        alignment,
        d,
        ebytes,
        elements,
        m,
        n,
        register_index,
        regs,
        wback,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckAdvSIMDEnabled(s_0_0)
        let s_0_1: () = CheckAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call R_read(s_1_1)
        let s_1_2: u32 = R_read(state, tracer, s_1_1);
        // D s_1_3: write-var address <= s_1_2
        fn_state.address = s_1_2;
        // C s_1_4: const #1u : u32
        let s_1_4: u32 = 1;
        // C s_1_5: const #0u : u8
        let s_1_5: bool = false;
        // C s_1_6: const #0u : u8
        let s_1_6: bool = false;
        // S s_1_7: call CreateAccDescASIMD(s_1_4, s_1_5, s_1_6)
        let s_1_7: ProductType9878976b5bcce9c9 = CreateAccDescASIMD(
            state,
            tracer,
            s_1_4,
            s_1_5,
            s_1_6,
        );
        // D s_1_8: write-var accdesc <= s_1_7
        fn_state.accdesc = s_1_7;
        // D s_1_9: read-var address:u32
        let s_1_9: u32 = fn_state.address;
        // D s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 32u16);
        // D s_1_11: read-var alignment:i64
        let s_1_11: i64 = fn_state.alignment;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call IsAligned__1(s_1_10, s_1_12)
        let s_1_13: bool = IsAligned__1(state, tracer, s_1_10, s_1_12);
        // D s_1_14: not s_1_13
        let s_1_14: bool = !s_1_13;
        // N s_1_15: branch s_1_14 b27 b2
        if s_1_14 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var regs:i64
        let s_3_2: i64 = fn_state.regs;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#320165 <= s_3_5
        fn_state.gs_320165 = s_3_5;
        // D s_3_7: write-var r <= s_3_0
        fn_state.r = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var r:i64
        let s_4_0: i64 = fn_state.r;
        // D s_4_1: read-var gs#320165:i64
        let s_4_1: i64 = fn_state.gs_320165;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b22 b5
        if s_4_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // C s_5_1: const #1s : i
        let s_5_1: i128 = 1;
        // D s_5_2: read-var elements:i
        let s_5_2: i128 = fn_state.elements;
        // D s_5_3: sub s_5_2 s_5_1
        let s_5_3: i128 = ((s_5_2) - (s_5_1));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: write-var gs#320171 <= s_5_4
        fn_state.gs_320171 = s_5_4;
        // D s_5_6: write-var e <= s_5_0
        fn_state.e = s_5_0;
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#320171:i64
        let s_6_1: i64 = fn_state.gs_320171;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b21 b7
        if s_6_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #8s : i
        let s_7_0: i128 = 8;
        // D s_7_1: read-var ebytes:i64
        let s_7_1: i64 = fn_state.ebytes;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: call neq_int(s_7_2, s_7_0)
        let s_7_3: bool = neq_int(state, tracer, s_7_2, s_7_0);
        // N s_7_4: branch s_7_3 b20 b8
        if s_7_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var address:u32
        let s_8_0: u32 = fn_state.address;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 32u16);
        // D s_8_2: read-var ebytes:i64
        let s_8_2: i64 = fn_state.ebytes;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: call IsAligned__1(s_8_1, s_8_3)
        let s_8_4: bool = IsAligned__1(state, tracer, s_8_1, s_8_3);
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b19 b9
        if s_8_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#320176 <= s_9_0
        fn_state.gs_320176 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#320176:u8
        let s_10_0: bool = fn_state.gs_320176;
        // N s_10_1: branch s_10_0 b18 b11
        if s_10_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var d:i64
        let s_12_0: i64 = fn_state.d;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var r:i64
        let s_12_2: i64 = fn_state.r;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: add s_12_1 s_12_3
        let s_12_4: i128 = (s_12_1 + s_12_3);
        // D s_12_5: cast reint s_12_4 -> i64
        let s_12_5: i64 = (s_12_4 as i64);
        // D s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // D s_12_7: call D_read(s_12_6)
        let s_12_7: u64 = D_read(state, tracer, s_12_6);
        // C s_12_8: const #64s : i64
        let s_12_8: i64 = 64;
        // D s_12_9: cast zx s_12_7 -> bv
        let s_12_9: Bits = Bits::new(s_12_7 as u128, 64u16);
        // D s_12_10: read-var e:i64
        let s_12_10: i64 = fn_state.e;
        // D s_12_11: cast zx s_12_10 -> i
        let s_12_11: i128 = (i128::try_from(s_12_10).unwrap());
        // C s_12_12: cast zx s_12_8 -> i
        let s_12_12: i128 = (i128::try_from(s_12_8).unwrap());
        // D s_12_13: call Elem_read(s_12_9, s_12_11, s_12_12)
        let s_12_13: Bits = Elem_read(state, tracer, s_12_9, s_12_11, s_12_12);
        // D s_12_14: cast reint s_12_13 -> u64
        let s_12_14: u64 = (s_12_13.value() as u64);
        // D s_12_15: write-var data <= s_12_14
        fn_state.data = s_12_14;
        // C s_12_16: const #2u : u32
        let s_12_16: u32 = 2;
        // S s_12_17: call BigEndian(s_12_16)
        let s_12_17: bool = BigEndian(state, tracer, s_12_16);
        // N s_12_18: branch s_12_17 b16 b13
        if s_12_17 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var data:u64
        let s_13_1: u64 = fn_state.data;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 64u16);
        // C s_13_3: const #1s : i64
        let s_13_3: i64 = 1;
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #31s : i
        let s_13_5: i128 = 31;
        // C s_13_6: add s_13_5 s_13_4
        let s_13_6: i128 = (s_13_5 + s_13_4);
        // D s_13_7: bit-extract s_13_2 s_13_0 s_13_6
        let s_13_7: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_6).unwrap(),
        ));
        // D s_13_8: cast reint s_13_7 -> u32
        let s_13_8: u32 = (s_13_7.value() as u32);
        // C s_13_9: const #4s : i
        let s_13_9: i128 = 4;
        // D s_13_10: cast zx s_13_8 -> bv
        let s_13_10: Bits = Bits::new(s_13_8 as u128, 32u16);
        // D s_13_11: read-var address:u32
        let s_13_11: u32 = fn_state.address;
        // D s_13_12: call MemU_set(s_13_11, s_13_9, s_13_10)
        let s_13_12: () = MemU_set(state, tracer, s_13_11, s_13_9, s_13_10);
        // N s_13_13: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #4s : i
        let s_14_0: i128 = 4;
        // D s_14_1: read-var address:u32
        let s_14_1: u32 = fn_state.address;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 32u16);
        // C s_14_3: cast cvt s_14_0 -> bv
        let s_14_3: Bits = Bits::new(s_14_0 as u128, 128);
        // D s_14_4: add s_14_2 s_14_3
        let s_14_4: Bits = (s_14_2 + s_14_3);
        // D s_14_5: cast reint s_14_4 -> u32
        let s_14_5: u32 = (s_14_4.value() as u32);
        // C s_14_6: const #32s : i
        let s_14_6: i128 = 32;
        // D s_14_7: read-var data:u64
        let s_14_7: u64 = fn_state.data;
        // D s_14_8: cast zx s_14_7 -> bv
        let s_14_8: Bits = Bits::new(s_14_7 as u128, 64u16);
        // C s_14_9: const #1s : i64
        let s_14_9: i64 = 1;
        // C s_14_10: cast zx s_14_9 -> i
        let s_14_10: i128 = (i128::try_from(s_14_9).unwrap());
        // C s_14_11: const #31s : i
        let s_14_11: i128 = 31;
        // C s_14_12: add s_14_11 s_14_10
        let s_14_12: i128 = (s_14_11 + s_14_10);
        // D s_14_13: bit-extract s_14_8 s_14_6 s_14_12
        let s_14_13: Bits = (Bits::new(
            ((s_14_8) >> (s_14_6)).value(),
            u16::try_from(s_14_12).unwrap(),
        ));
        // D s_14_14: cast reint s_14_13 -> u32
        let s_14_14: u32 = (s_14_13.value() as u32);
        // C s_14_15: const #4s : i
        let s_14_15: i128 = 4;
        // D s_14_16: cast zx s_14_14 -> bv
        let s_14_16: Bits = Bits::new(s_14_14 as u128, 32u16);
        // D s_14_17: call MemU_set(s_14_5, s_14_15, s_14_16)
        let s_14_17: () = MemU_set(state, tracer, s_14_5, s_14_15, s_14_16);
        // N s_14_18: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var address:u32
        let s_15_0: u32 = fn_state.address;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 32u16);
        // D s_15_2: read-var ebytes:i64
        let s_15_2: i64 = fn_state.ebytes;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: cast cvt s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 128);
        // D s_15_5: add s_15_1 s_15_4
        let s_15_5: Bits = (s_15_1 + s_15_4);
        // D s_15_6: cast reint s_15_5 -> u32
        let s_15_6: u32 = (s_15_5.value() as u32);
        // D s_15_7: write-var address <= s_15_6
        fn_state.address = s_15_6;
        // D s_15_8: read-var e:i64
        let s_15_8: i64 = fn_state.e;
        // C s_15_9: const #1s : i64
        let s_15_9: i64 = 1;
        // D s_15_10: add s_15_8 s_15_9
        let s_15_10: i64 = (s_15_8 + s_15_9);
        // D s_15_11: write-var e <= s_15_10
        fn_state.e = s_15_10;
        // N s_15_12: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #32s : i
        let s_16_0: i128 = 32;
        // D s_16_1: read-var data:u64
        let s_16_1: u64 = fn_state.data;
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 64u16);
        // C s_16_3: const #1s : i64
        let s_16_3: i64 = 1;
        // C s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // C s_16_5: const #31s : i
        let s_16_5: i128 = 31;
        // C s_16_6: add s_16_5 s_16_4
        let s_16_6: i128 = (s_16_5 + s_16_4);
        // D s_16_7: bit-extract s_16_2 s_16_0 s_16_6
        let s_16_7: Bits = (Bits::new(
            ((s_16_2) >> (s_16_0)).value(),
            u16::try_from(s_16_6).unwrap(),
        ));
        // D s_16_8: cast reint s_16_7 -> u32
        let s_16_8: u32 = (s_16_7.value() as u32);
        // C s_16_9: const #4s : i
        let s_16_9: i128 = 4;
        // D s_16_10: cast zx s_16_8 -> bv
        let s_16_10: Bits = Bits::new(s_16_8 as u128, 32u16);
        // D s_16_11: read-var address:u32
        let s_16_11: u32 = fn_state.address;
        // D s_16_12: call MemU_set(s_16_11, s_16_9, s_16_10)
        let s_16_12: () = MemU_set(state, tracer, s_16_11, s_16_9, s_16_10);
        // N s_16_13: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #4s : i
        let s_17_0: i128 = 4;
        // D s_17_1: read-var address:u32
        let s_17_1: u32 = fn_state.address;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 32u16);
        // C s_17_3: cast cvt s_17_0 -> bv
        let s_17_3: Bits = Bits::new(s_17_0 as u128, 128);
        // D s_17_4: add s_17_2 s_17_3
        let s_17_4: Bits = (s_17_2 + s_17_3);
        // D s_17_5: cast reint s_17_4 -> u32
        let s_17_5: u32 = (s_17_4.value() as u32);
        // C s_17_6: const #0s : i
        let s_17_6: i128 = 0;
        // D s_17_7: read-var data:u64
        let s_17_7: u64 = fn_state.data;
        // D s_17_8: cast zx s_17_7 -> bv
        let s_17_8: Bits = Bits::new(s_17_7 as u128, 64u16);
        // C s_17_9: const #1s : i64
        let s_17_9: i64 = 1;
        // C s_17_10: cast zx s_17_9 -> i
        let s_17_10: i128 = (i128::try_from(s_17_9).unwrap());
        // C s_17_11: const #31s : i
        let s_17_11: i128 = 31;
        // C s_17_12: add s_17_11 s_17_10
        let s_17_12: i128 = (s_17_11 + s_17_10);
        // D s_17_13: bit-extract s_17_8 s_17_6 s_17_12
        let s_17_13: Bits = (Bits::new(
            ((s_17_8) >> (s_17_6)).value(),
            u16::try_from(s_17_12).unwrap(),
        ));
        // D s_17_14: cast reint s_17_13 -> u32
        let s_17_14: u32 = (s_17_13.value() as u32);
        // C s_17_15: const #4s : i
        let s_17_15: i128 = 4;
        // D s_17_16: cast zx s_17_14 -> bv
        let s_17_16: Bits = Bits::new(s_17_14 as u128, 32u16);
        // D s_17_17: call MemU_set(s_17_5, s_17_15, s_17_16)
        let s_17_17: () = MemU_set(state, tracer, s_17_5, s_17_15, s_17_16);
        // N s_17_18: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var accdesc:struct
        let s_18_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_18_1: call AlignmentFault(s_18_0)
        let s_18_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_18_0);
        // D s_18_2: read-var address:u32
        let s_18_2: u32 = fn_state.address;
        // D s_18_3: call AArch32_Abort(s_18_2, s_18_1)
        let s_18_3: () = AArch32_Abort(state, tracer, s_18_2, s_18_1);
        // N s_18_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call AlignmentEnforced(s_19_0)
        let s_19_1: bool = AlignmentEnforced(state, tracer, s_19_0);
        // D s_19_2: write-var gs#320176 <= s_19_1
        fn_state.gs_320176 = s_19_1;
        // N s_19_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var d:i64
        let s_20_0: i64 = fn_state.d;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: read-var r:i64
        let s_20_2: i64 = fn_state.r;
        // D s_20_3: cast zx s_20_2 -> i
        let s_20_3: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_4: add s_20_1 s_20_3
        let s_20_4: i128 = (s_20_1 + s_20_3);
        // D s_20_5: cast reint s_20_4 -> i64
        let s_20_5: i64 = (s_20_4 as i64);
        // D s_20_6: cast zx s_20_5 -> i
        let s_20_6: i128 = (i128::try_from(s_20_5).unwrap());
        // D s_20_7: call D_read(s_20_6)
        let s_20_7: u64 = D_read(state, tracer, s_20_6);
        // C s_20_8: const #8s : i
        let s_20_8: i128 = 8;
        // D s_20_9: read-var ebytes:i64
        let s_20_9: i64 = fn_state.ebytes;
        // D s_20_10: cast zx s_20_9 -> i
        let s_20_10: i128 = (i128::try_from(s_20_9).unwrap());
        // D s_20_11: mul s_20_8 s_20_10
        let s_20_11: i128 = ((s_20_8) * (s_20_10));
        // D s_20_12: cast reint s_20_11 -> i64
        let s_20_12: i64 = (s_20_11 as i64);
        // D s_20_13: cast zx s_20_12 -> i
        let s_20_13: i128 = (i128::try_from(s_20_12).unwrap());
        // D s_20_14: cast reint s_20_13 -> i64
        let s_20_14: i64 = (s_20_13 as i64);
        // D s_20_15: cast zx s_20_7 -> bv
        let s_20_15: Bits = Bits::new(s_20_7 as u128, 64u16);
        // D s_20_16: read-var e:i64
        let s_20_16: i64 = fn_state.e;
        // D s_20_17: cast zx s_20_16 -> i
        let s_20_17: i128 = (i128::try_from(s_20_16).unwrap());
        // D s_20_18: cast zx s_20_14 -> i
        let s_20_18: i128 = (i128::try_from(s_20_14).unwrap());
        // D s_20_19: call Elem_read(s_20_15, s_20_17, s_20_18)
        let s_20_19: Bits = Elem_read(state, tracer, s_20_15, s_20_17, s_20_18);
        // D s_20_20: read-var ebytes:i64
        let s_20_20: i64 = fn_state.ebytes;
        // D s_20_21: cast zx s_20_20 -> i
        let s_20_21: i128 = (i128::try_from(s_20_20).unwrap());
        // D s_20_22: read-var address:u32
        let s_20_22: u32 = fn_state.address;
        // D s_20_23: call MemU_set(s_20_22, s_20_21, s_20_19)
        let s_20_23: () = MemU_set(state, tracer, s_20_22, s_20_21, s_20_19);
        // N s_20_24: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var r:i64
        let s_21_0: i64 = fn_state.r;
        // C s_21_1: const #1s : i64
        let s_21_1: i64 = 1;
        // D s_21_2: add s_21_0 s_21_1
        let s_21_2: i64 = (s_21_0 + s_21_1);
        // D s_21_3: write-var r <= s_21_2
        fn_state.r = s_21_2;
        // N s_21_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var wback:u8
        let s_22_0: bool = fn_state.wback;
        // N s_22_1: branch s_22_0 b24 b23
        if s_22_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var register_index:u8
        let s_24_0: bool = fn_state.register_index;
        // N s_24_1: branch s_24_0 b26 b25
        if s_24_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var n:i64
        let s_25_0: i64 = fn_state.n;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: call R_read(s_25_1)
        let s_25_2: u32 = R_read(state, tracer, s_25_1);
        // C s_25_3: const #8s : i
        let s_25_3: i128 = 8;
        // D s_25_4: read-var regs:i64
        let s_25_4: i64 = fn_state.regs;
        // D s_25_5: cast zx s_25_4 -> i
        let s_25_5: i128 = (i128::try_from(s_25_4).unwrap());
        // D s_25_6: mul s_25_3 s_25_5
        let s_25_6: i128 = ((s_25_3) * (s_25_5));
        // D s_25_7: cast reint s_25_6 -> i64
        let s_25_7: i64 = (s_25_6 as i64);
        // D s_25_8: cast zx s_25_2 -> bv
        let s_25_8: Bits = Bits::new(s_25_2 as u128, 32u16);
        // D s_25_9: cast zx s_25_7 -> i
        let s_25_9: i128 = (i128::try_from(s_25_7).unwrap());
        // D s_25_10: cast cvt s_25_9 -> bv
        let s_25_10: Bits = Bits::new(s_25_9 as u128, 128);
        // D s_25_11: add s_25_8 s_25_10
        let s_25_11: Bits = (s_25_8 + s_25_10);
        // D s_25_12: cast reint s_25_11 -> u32
        let s_25_12: u32 = (s_25_11.value() as u32);
        // D s_25_13: read-var n:i64
        let s_25_13: i64 = fn_state.n;
        // D s_25_14: cast zx s_25_13 -> i
        let s_25_14: i128 = (i128::try_from(s_25_13).unwrap());
        // D s_25_15: call R_set(s_25_14, s_25_12)
        let s_25_15: () = R_set(state, tracer, s_25_14, s_25_12);
        // N s_25_16: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var n:i64
        let s_26_0: i64 = fn_state.n;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: call R_read(s_26_1)
        let s_26_2: u32 = R_read(state, tracer, s_26_1);
        // D s_26_3: read-var m:i64
        let s_26_3: i64 = fn_state.m;
        // D s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // D s_26_5: call R_read(s_26_4)
        let s_26_5: u32 = R_read(state, tracer, s_26_4);
        // D s_26_6: cast zx s_26_2 -> bv
        let s_26_6: Bits = Bits::new(s_26_2 as u128, 32u16);
        // D s_26_7: cast zx s_26_5 -> bv
        let s_26_7: Bits = Bits::new(s_26_5 as u128, 32u16);
        // D s_26_8: add s_26_6 s_26_7
        let s_26_8: Bits = (s_26_6 + s_26_7);
        // D s_26_9: cast reint s_26_8 -> u32
        let s_26_9: u32 = (s_26_8.value() as u32);
        // D s_26_10: read-var n:i64
        let s_26_10: i64 = fn_state.n;
        // D s_26_11: cast zx s_26_10 -> i
        let s_26_11: i128 = (i128::try_from(s_26_10).unwrap());
        // D s_26_12: call R_set(s_26_11, s_26_9)
        let s_26_12: () = R_set(state, tracer, s_26_11, s_26_9);
        // N s_26_13: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var accdesc:struct
        let s_27_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_27_1: call AlignmentFault(s_27_0)
        let s_27_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_27_0);
        // D s_27_2: read-var address:u32
        let s_27_2: u32 = fn_state.address;
        // D s_27_3: call AArch32_Abort(s_27_2, s_27_1)
        let s_27_3: () = AArch32_Abort(state, tracer, s_27_2, s_27_1);
        // N s_27_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
