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
use HavePACExt::*;
use neq_int::*;
use execute_aarch64_instrs_memory_single_general_immediate_signed_pac::*;
use common::*;
pub fn decode_ldra_aarch64_instrs_memory_single_general_immediate_signed_pac<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    W: bool,
    imm9: u16,
    S: bool,
    M: bool,
    size: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        offset: u64,
        wback: bool,
        use_key_a: bool,
        n: i64,
        gs_164044: bool,
        gs_164036: bool,
        Rt: u8,
        Rn: u8,
        W: bool,
        imm9: u16,
        S: bool,
        M: bool,
        size: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        W,
        imm9,
        S,
        M,
        size,
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
        // S s_0_1: call HavePACExt(s_0_0)
        let s_0_1: bool = HavePACExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b8 b1
        if s_0_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:u8
        let s_1_0: u8 = fn_state.size;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #3u : u8
        let s_1_2: u8 = 3;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cmp-ne s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) != (s_1_3));
        // D s_1_5: write-var gs#164036 <= s_1_4
        fn_state.gs_164036 = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#164036:u8
        let s_2_0: bool = fn_state.gs_164036;
        // N s_2_1: branch s_2_0 b7 b3
        if s_2_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rt:u8
        let s_3_0: u8 = fn_state.Rt;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 5u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: write-var t <= s_3_3
        fn_state.t = s_3_3;
        // D s_3_5: read-var Rn:u8
        let s_3_5: u8 = fn_state.Rn;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 5u16);
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (s_3_6.value() as i128);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: write-var n <= s_3_8
        fn_state.n = s_3_8;
        // D s_3_10: read-var W:u8
        let s_3_10: bool = fn_state.W;
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 1u16);
        // C s_3_12: const #1u : u8
        let s_3_12: bool = true;
        // C s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 1u16);
        // D s_3_14: cmp-eq s_3_11 s_3_13
        let s_3_14: bool = ((s_3_11) == (s_3_13));
        // D s_3_15: write-var wback <= s_3_14
        fn_state.wback = s_3_14;
        // D s_3_16: read-var M:u8
        let s_3_16: bool = fn_state.M;
        // D s_3_17: cast zx s_3_16 -> bv
        let s_3_17: Bits = Bits::new(s_3_16 as u128, 1u16);
        // C s_3_18: const #0u : u8
        let s_3_18: bool = false;
        // C s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 1u16);
        // D s_3_20: cmp-eq s_3_17 s_3_19
        let s_3_20: bool = ((s_3_17) == (s_3_19));
        // D s_3_21: write-var use_key_a <= s_3_20
        fn_state.use_key_a = s_3_20;
        // D s_3_22: read-var S:u8
        let s_3_22: bool = fn_state.S;
        // D s_3_23: cast zx s_3_22 -> bv
        let s_3_23: Bits = Bits::new(s_3_22 as u128, 1u16);
        // D s_3_24: read-var imm9:u9
        let s_3_24: u16 = fn_state.imm9;
        // D s_3_25: cast zx s_3_24 -> bv
        let s_3_25: Bits = Bits::new(s_3_24 as u128, 9u16);
        // D s_3_26: cast reint s_3_23 -> u128
        let s_3_26: u128 = (s_3_23.value() as u128);
        // D s_3_27: size-of s_3_23
        let s_3_27: u16 = s_3_23.length();
        // D s_3_28: cast reint s_3_25 -> u128
        let s_3_28: u128 = (s_3_25.value() as u128);
        // D s_3_29: size-of s_3_25
        let s_3_29: u16 = s_3_25.length();
        // D s_3_30: lsl s_3_26 s_3_29
        let s_3_30: u128 = s_3_26 << s_3_29;
        // D s_3_31: or s_3_30 s_3_28
        let s_3_31: u128 = ((s_3_30) | (s_3_28));
        // D s_3_32: add s_3_27 s_3_29
        let s_3_32: u16 = (s_3_27 + s_3_29);
        // D s_3_33: create-bits s_3_31 s_3_32
        let s_3_33: Bits = Bits::new(s_3_31, s_3_32);
        // D s_3_34: cast reint s_3_33 -> u10
        let s_3_34: u16 = (s_3_33.value() as u16);
        // C s_3_35: const #3s : i64
        let s_3_35: i64 = 3;
        // C s_3_36: const #64s : i
        let s_3_36: i128 = 64;
        // D s_3_37: cast zx s_3_34 -> bv
        let s_3_37: Bits = Bits::new(s_3_34 as u128, 10u16);
        // D s_3_38: bits-cast sx s_3_37 -> bv length s_3_36
        let s_3_38: Bits = s_3_37.sign_extend(s_3_36);
        // D s_3_39: cast reint s_3_38 -> u64
        let s_3_39: u64 = (s_3_38.value() as u64);
        // D s_3_40: cast zx s_3_39 -> bv
        let s_3_40: Bits = Bits::new(s_3_39 as u128, 64u16);
        // C s_3_41: cast zx s_3_35 -> i
        let s_3_41: i128 = (i128::try_from(s_3_35).unwrap());
        // D s_3_42: lsl s_3_40 s_3_41
        let s_3_42: Bits = s_3_40 << s_3_41;
        // D s_3_43: cast reint s_3_42 -> u64
        let s_3_43: u64 = (s_3_42.value() as u64);
        // D s_3_44: write-var offset <= s_3_43
        fn_state.offset = s_3_43;
        // D s_3_45: read-var wback:u8
        let s_3_45: bool = fn_state.wback;
        // N s_3_46: branch s_3_45 b6 b4
        if s_3_45 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #31s : i
        let s_4_0: i128 = 31;
        // D s_4_1: read-var n:i64
        let s_4_1: i64 = fn_state.n;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: call neq_int(s_4_2, s_4_0)
        let s_4_3: bool = neq_int(state, tracer, s_4_2, s_4_0);
        // D s_4_4: write-var gs#164044 <= s_4_3
        fn_state.gs_164044 = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#164044:u8
        let s_5_0: bool = fn_state.gs_164044;
        // C s_5_1: const #0u : u32
        let s_5_1: u32 = 0;
        // D s_5_2: read-var n:i64
        let s_5_2: i64 = fn_state.n;
        // C s_5_3: const #0u : u8
        let s_5_3: bool = false;
        // D s_5_4: read-var offset:u64
        let s_5_4: u64 = fn_state.offset;
        // D s_5_5: read-var t:i64
        let s_5_5: i64 = fn_state.t;
        // D s_5_6: read-var use_key_a:u8
        let s_5_6: bool = fn_state.use_key_a;
        // D s_5_7: read-var wback:u8
        let s_5_7: bool = fn_state.wback;
        // D s_5_8: call execute_aarch64_instrs_memory_single_general_immediate_signed_pac(s_5_1, s_5_2, s_5_3, s_5_4, s_5_5, s_5_0, s_5_6, s_5_7)
        let s_5_8: () = execute_aarch64_instrs_memory_single_general_immediate_signed_pac(
            state,
            tracer,
            s_5_1,
            s_5_2,
            s_5_3,
            s_5_4,
            s_5_5,
            s_5_0,
            s_5_6,
            s_5_7,
        );
        // N s_5_9: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var gs#164044 <= s_6_0
        fn_state.gs_164044 = s_6_0;
        // N s_6_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#164036 <= s_8_0
        fn_state.gs_164036 = s_8_0;
        // N s_8_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
