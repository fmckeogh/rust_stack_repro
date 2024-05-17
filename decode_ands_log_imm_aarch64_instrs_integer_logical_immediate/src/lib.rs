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
use DecodeBitMasks::*;
use execute_aarch64_instrs_integer_logical_immediate::*;
use common::*;
pub fn decode_ands_log_imm_aarch64_instrs_integer_logical_immediate<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    imms: u8,
    immr: u8,
    N: bool,
    opc: u8,
    sf: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        opshadow_1042: u32,
        ga_249400: i64,
        setflags: bool,
        gs_143390: bool,
        op: u32,
        n: i64,
        setflagsshadow_1043: bool,
        d: i64,
        ga_249405: ProductTypebc91b195b0b2a883,
        datasize: i64,
        Rd: u8,
        Rn: u8,
        imms: u8,
        immr: u8,
        N: bool,
        opc: u8,
        sf: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        imms,
        immr,
        N,
        opc,
        sf,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rd:u8
        let s_0_0: u8 = fn_state.Rd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var d <= s_0_3
        fn_state.d = s_0_3;
        // D s_0_5: read-var Rn:u8
        let s_0_5: u8 = fn_state.Rn;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var n <= s_0_8
        fn_state.n = s_0_8;
        // D s_0_10: read-var sf:u8
        let s_0_10: bool = fn_state.sf;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // C s_0_12: const #1u : u8
        let s_0_12: bool = true;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b16 b1
        if s_0_14 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: write-var ga#249400 <= s_1_0
        fn_state.ga_249400 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#249400:i64
        let s_2_0: i64 = fn_state.ga_249400;
        // D s_2_1: write-var datasize <= s_2_0
        fn_state.datasize = s_2_0;
        // D s_2_2: read-var opc:u8
        let s_2_2: u8 = fn_state.opc;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // C s_2_4: const #0u : u8
        let s_2_4: u8 = 0;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // D s_2_7: not s_2_6
        let s_2_7: bool = !s_2_6;
        // N s_2_8: branch s_2_7 b11 b3
        if s_2_7 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u32
        let s_3_0: u32 = 0;
        // D s_3_1: write-var op <= s_3_0
        fn_state.op = s_3_0;
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // D s_3_3: write-var setflags <= s_3_2
        fn_state.setflags = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var op:u32
        let s_4_0: u32 = fn_state.op;
        // D s_4_1: write-var opshadow#1042 <= s_4_0
        fn_state.opshadow_1042 = s_4_0;
        // D s_4_2: read-var setflags:u8
        let s_4_2: bool = fn_state.setflags;
        // D s_4_3: write-var setflagsshadow#1043 <= s_4_2
        fn_state.setflagsshadow_1043 = s_4_2;
        // D s_4_4: read-var sf:u8
        let s_4_4: bool = fn_state.sf;
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // C s_4_6: const #0u : u8
        let s_4_6: bool = false;
        // C s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 1u16);
        // D s_4_8: cmp-eq s_4_5 s_4_7
        let s_4_8: bool = ((s_4_5) == (s_4_7));
        // N s_4_9: branch s_4_8 b10 b5
        if s_4_8 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#143390 <= s_5_0
        fn_state.gs_143390 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#143390:u8
        let s_6_0: bool = fn_state.gs_143390;
        // N s_6_1: branch s_6_0 b9 b7
        if s_6_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var datasize:i64
        let s_7_0: i64 = fn_state.datasize;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var N:u8
        let s_7_4: bool = fn_state.N;
        // D s_7_5: read-var imms:u8
        let s_7_5: u8 = fn_state.imms;
        // D s_7_6: read-var immr:u8
        let s_7_6: u8 = fn_state.immr;
        // C s_7_7: const #1u : u8
        let s_7_7: bool = true;
        // D s_7_8: call DecodeBitMasks(s_7_4, s_7_5, s_7_6, s_7_7, s_7_3)
        let s_7_8: ProductTypebc91b195b0b2a883 = DecodeBitMasks(
            state,
            tracer,
            s_7_4,
            s_7_5,
            s_7_6,
            s_7_7,
            s_7_3,
        );
        // D s_7_9: write-var ga#249405 <= s_7_8
        fn_state.ga_249405 = s_7_8;
        // N s_7_10: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#249405.0:struct
        let s_8_0: Bits = fn_state.ga_249405._0;
        // D s_8_1: read-var datasize:i64
        let s_8_1: i64 = fn_state.datasize;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var d:i64
        let s_8_4: i64 = fn_state.d;
        // D s_8_5: read-var n:i64
        let s_8_5: i64 = fn_state.n;
        // D s_8_6: read-var opshadow#1042:u32
        let s_8_6: u32 = fn_state.opshadow_1042;
        // D s_8_7: read-var setflagsshadow#1043:u8
        let s_8_7: bool = fn_state.setflagsshadow_1043;
        // D s_8_8: call execute_aarch64_instrs_integer_logical_immediate(s_8_4, s_8_3, s_8_0, s_8_5, s_8_6, s_8_7)
        let s_8_8: () = execute_aarch64_instrs_integer_logical_immediate(
            state,
            tracer,
            s_8_4,
            s_8_3,
            s_8_0,
            s_8_5,
            s_8_6,
            s_8_7,
        );
        // N s_8_9: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var N:u8
        let s_10_0: bool = fn_state.N;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #0u : u8
        let s_10_2: bool = false;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-ne s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) != (s_10_3));
        // D s_10_5: write-var gs#143390 <= s_10_4
        fn_state.gs_143390 = s_10_4;
        // N s_10_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var opc:u8
        let s_11_0: u8 = fn_state.opc;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 2u16);
        // C s_11_2: const #1u : u8
        let s_11_2: u8 = 1;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 2u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: not s_11_4
        let s_11_5: bool = !s_11_4;
        // N s_11_6: branch s_11_5 b13 b12
        if s_11_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #2u : u32
        let s_12_0: u32 = 2;
        // D s_12_1: write-var op <= s_12_0
        fn_state.op = s_12_0;
        // C s_12_2: const #0u : u8
        let s_12_2: bool = false;
        // D s_12_3: write-var setflags <= s_12_2
        fn_state.setflags = s_12_2;
        // N s_12_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var opc:u8
        let s_13_0: u8 = fn_state.opc;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b15 b14
        if s_13_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u32
        let s_14_0: u32 = 1;
        // D s_14_1: write-var op <= s_14_0
        fn_state.op = s_14_0;
        // C s_14_2: const #0u : u8
        let s_14_2: bool = false;
        // D s_14_3: write-var setflags <= s_14_2
        fn_state.setflags = s_14_2;
        // N s_14_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u32
        let s_15_0: u32 = 0;
        // D s_15_1: write-var op <= s_15_0
        fn_state.op = s_15_0;
        // C s_15_2: const #1u : u8
        let s_15_2: bool = true;
        // D s_15_3: write-var setflags <= s_15_2
        fn_state.setflags = s_15_2;
        // N s_15_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #64s : i64
        let s_16_0: i64 = 64;
        // D s_16_1: write-var ga#249400 <= s_16_0
        fn_state.ga_249400 = s_16_0;
        // N s_16_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
