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
use decode_aarch32_instrs_STRB_i_A1enc_A_txt::*;
use decode_aarch32_instrs_LDRB_i_A1enc_A_txt::*;
use decode_aarch32_instrs_LDRT_A1enc_A_txt::*;
use decode_aarch32_instrs_LDRBT_A1enc_A_txt::*;
use decode_aarch32_instrs_STR_i_A1enc_A_txt::*;
use decode_aarch32_instrs_LDR_i_A1enc_A_txt::*;
use decode_aarch32_instrs_STRBT_A1enc_A_txt::*;
use decode_aarch32_instrs_LDRB_l_A1enc_A_txt::*;
use decode_aarch32_instrs_LDR_l_A1enc_A_txt::*;
use decode_aarch32_instrs_STRT_A1enc_A_txt::*;
use common::*;
pub fn u__DecodeA32_LoadStoreImmLit<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_408545: i128,
    gs_408546: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_408589: bool,
        gs_408615: bool,
        gs_408640: bool,
        gs_408722: bool,
        gs_408555: bool,
        gs_408749: bool,
        gs_408590: bool,
        gs_408774: bool,
        gs_408554: bool,
        gs_408671: bool,
        gs_408610: bool,
        u_33304: u32,
        gs_408666: bool,
        gs_408614: bool,
        u__opcode: u32,
        gs_408770: bool,
        merge_var: ProductType7b8639ca40b2f578,
        gs_408769: bool,
        gs_408560: bool,
        gs_408802: bool,
        u_33312: u32,
        gs_408585: bool,
        gs_408641: bool,
        gs_408744: bool,
        u_33335: u32,
        gs_408636: bool,
        gs_408801: bool,
        gs_408721: bool,
        u_33295: u32,
        u_33328: u32,
        gs_408665: bool,
        gs_408775: bool,
        gs_408717: bool,
        gs_408696: bool,
        gs_408584: bool,
        gs_408748: bool,
        gs_408797: bool,
        u_33344: u32,
        gs_408695: bool,
        gs_408691: bool,
        u_33288: u32,
        u_33280: u32,
        u_33319: u32,
        gs_408670: bool,
        gs_408716: bool,
        gs_408635: bool,
        gs_408559: bool,
        gs_408545: i128,
        gs_408546: u32,
    }
    let fn_state = FunctionState {
        gs_408545,
        gs_408546,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var gs#408545:i
        let s_0_0: i128 = fn_state.gs_408545;
        // D s_0_1: write-var merge#var.0 <= s_0_0
        fn_state.merge_var._0 = s_0_0;
        // D s_0_2: read-var gs#408546:u32
        let s_0_2: u32 = fn_state.gs_408546;
        // D s_0_3: write-var merge#var.1 <= s_0_2
        fn_state.merge_var._1 = s_0_2;
        // D s_0_4: read-var merge#var.1:struct
        let s_0_4: u32 = fn_state.merge_var._1;
        // D s_0_5: write-var __opcode <= s_0_4
        fn_state.u__opcode = s_0_4;
        // C s_0_6: const #25s : i
        let s_0_6: i128 = 25;
        // D s_0_7: read-var __opcode:u32
        let s_0_7: u32 = fn_state.u__opcode;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_9: const #1s : i64
        let s_0_9: i64 = 1;
        // C s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // C s_0_11: const #2s : i
        let s_0_11: i128 = 2;
        // C s_0_12: add s_0_11 s_0_10
        let s_0_12: i128 = (s_0_11 + s_0_10);
        // D s_0_13: bit-extract s_0_8 s_0_6 s_0_12
        let s_0_13: Bits = (Bits::new(
            ((s_0_8) >> (s_0_6)).value(),
            u16::try_from(s_0_12).unwrap(),
        ));
        // D s_0_14: cast reint s_0_13 -> u8
        let s_0_14: u8 = (s_0_13.value() as u8);
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 3u16);
        // C s_0_16: const #2u : u8
        let s_0_16: u8 = 2;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 3u16);
        // D s_0_18: cmp-eq s_0_15 s_0_17
        let s_0_18: bool = ((s_0_15) == (s_0_17));
        // N s_0_19: branch s_0_18 b125 b1
        if s_0_18 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#408555 <= s_1_0
        fn_state.gs_408555 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#408555:u8
        let s_2_0: bool = fn_state.gs_408555;
        // N s_2_1: branch s_2_0 b121 b3
        if s_2_0 {
            return block_121(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#408560 <= s_3_0
        fn_state.gs_408560 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#408560:u8
        let s_4_0: bool = fn_state.gs_408560;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // N s_4_2: branch s_4_1 b6 b5
        if s_4_1 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #2923s : i
        let s_5_0: i128 = 2923;
        // C s_5_1: const #14696u : u32
        let s_5_1: u32 = 14696;
        // N s_5_2: write-reg s_5_1 <= s_5_0
        let s_5_2: () = {
            state.write_register::<i128>(s_5_1 as isize, s_5_0);
            tracer.write_register(s_5_1 as isize, s_5_0);
        };
        // C s_5_3: const #28s : i
        let s_5_3: i128 = 28;
        // C s_5_4: const #4s : i
        let s_5_4: i128 = 4;
        // D s_5_5: read-var __opcode:u32
        let s_5_5: u32 = fn_state.u__opcode;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 32u16);
        // D s_5_7: bit-extract s_5_6 s_5_3 s_5_4
        let s_5_7: Bits = (Bits::new(
            ((s_5_6) >> (s_5_3)).value(),
            u16::try_from(s_5_4).unwrap(),
        ));
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: u8 = (s_5_7.value() as u8);
        // C s_5_9: const #24s : i
        let s_5_9: i128 = 24;
        // C s_5_10: const #1s : i
        let s_5_10: i128 = 1;
        // D s_5_11: read-var __opcode:u32
        let s_5_11: u32 = fn_state.u__opcode;
        // D s_5_12: cast zx s_5_11 -> bv
        let s_5_12: Bits = Bits::new(s_5_11 as u128, 32u16);
        // D s_5_13: bit-extract s_5_12 s_5_9 s_5_10
        let s_5_13: Bits = (Bits::new(
            ((s_5_12) >> (s_5_9)).value(),
            u16::try_from(s_5_10).unwrap(),
        ));
        // D s_5_14: cast reint s_5_13 -> u8
        let s_5_14: bool = ((s_5_13.value()) != 0);
        // C s_5_15: const #23s : i
        let s_5_15: i128 = 23;
        // C s_5_16: const #1s : i
        let s_5_16: i128 = 1;
        // D s_5_17: read-var __opcode:u32
        let s_5_17: u32 = fn_state.u__opcode;
        // D s_5_18: cast zx s_5_17 -> bv
        let s_5_18: Bits = Bits::new(s_5_17 as u128, 32u16);
        // D s_5_19: bit-extract s_5_18 s_5_15 s_5_16
        let s_5_19: Bits = (Bits::new(
            ((s_5_18) >> (s_5_15)).value(),
            u16::try_from(s_5_16).unwrap(),
        ));
        // D s_5_20: cast reint s_5_19 -> u8
        let s_5_20: bool = ((s_5_19.value()) != 0);
        // C s_5_21: const #21s : i
        let s_5_21: i128 = 21;
        // C s_5_22: const #1s : i
        let s_5_22: i128 = 1;
        // D s_5_23: read-var __opcode:u32
        let s_5_23: u32 = fn_state.u__opcode;
        // D s_5_24: cast zx s_5_23 -> bv
        let s_5_24: Bits = Bits::new(s_5_23 as u128, 32u16);
        // D s_5_25: bit-extract s_5_24 s_5_21 s_5_22
        let s_5_25: Bits = (Bits::new(
            ((s_5_24) >> (s_5_21)).value(),
            u16::try_from(s_5_22).unwrap(),
        ));
        // D s_5_26: cast reint s_5_25 -> u8
        let s_5_26: bool = ((s_5_25.value()) != 0);
        // C s_5_27: const #16s : i
        let s_5_27: i128 = 16;
        // C s_5_28: const #4s : i
        let s_5_28: i128 = 4;
        // D s_5_29: read-var __opcode:u32
        let s_5_29: u32 = fn_state.u__opcode;
        // D s_5_30: cast zx s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 32u16);
        // D s_5_31: bit-extract s_5_30 s_5_27 s_5_28
        let s_5_31: Bits = (Bits::new(
            ((s_5_30) >> (s_5_27)).value(),
            u16::try_from(s_5_28).unwrap(),
        ));
        // D s_5_32: cast reint s_5_31 -> u8
        let s_5_32: u8 = (s_5_31.value() as u8);
        // C s_5_33: const #12s : i
        let s_5_33: i128 = 12;
        // C s_5_34: const #4s : i
        let s_5_34: i128 = 4;
        // D s_5_35: read-var __opcode:u32
        let s_5_35: u32 = fn_state.u__opcode;
        // D s_5_36: cast zx s_5_35 -> bv
        let s_5_36: Bits = Bits::new(s_5_35 as u128, 32u16);
        // D s_5_37: bit-extract s_5_36 s_5_33 s_5_34
        let s_5_37: Bits = (Bits::new(
            ((s_5_36) >> (s_5_33)).value(),
            u16::try_from(s_5_34).unwrap(),
        ));
        // D s_5_38: cast reint s_5_37 -> u8
        let s_5_38: u8 = (s_5_37.value() as u8);
        // C s_5_39: const #0s : i
        let s_5_39: i128 = 0;
        // C s_5_40: const #12s : i
        let s_5_40: i128 = 12;
        // D s_5_41: read-var __opcode:u32
        let s_5_41: u32 = fn_state.u__opcode;
        // D s_5_42: cast zx s_5_41 -> bv
        let s_5_42: Bits = Bits::new(s_5_41 as u128, 32u16);
        // D s_5_43: bit-extract s_5_42 s_5_39 s_5_40
        let s_5_43: Bits = (Bits::new(
            ((s_5_42) >> (s_5_39)).value(),
            u16::try_from(s_5_40).unwrap(),
        ));
        // D s_5_44: cast reint s_5_43 -> u12
        let s_5_44: u16 = (s_5_43.value() as u16);
        // D s_5_45: call decode_aarch32_instrs_LDRB_i_A1enc_A_txt(s_5_8, s_5_14, s_5_20, s_5_26, s_5_32, s_5_38, s_5_44)
        let s_5_45: () = decode_aarch32_instrs_LDRB_i_A1enc_A_txt(
            state,
            tracer,
            s_5_8,
            s_5_14,
            s_5_20,
            s_5_26,
            s_5_32,
            s_5_38,
            s_5_44,
        );
        // N s_5_46: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var merge#var.1:struct
        let s_6_0: u32 = fn_state.merge_var._1;
        // D s_6_1: write-var u#33280 <= s_6_0
        fn_state.u_33280 = s_6_0;
        // C s_6_2: const #25s : i
        let s_6_2: i128 = 25;
        // D s_6_3: read-var u#33280:u32
        let s_6_3: u32 = fn_state.u_33280;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 32u16);
        // C s_6_5: const #1s : i64
        let s_6_5: i64 = 1;
        // C s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // C s_6_7: const #2s : i
        let s_6_7: i128 = 2;
        // C s_6_8: add s_6_7 s_6_6
        let s_6_8: i128 = (s_6_7 + s_6_6);
        // D s_6_9: bit-extract s_6_4 s_6_2 s_6_8
        let s_6_9: Bits = (Bits::new(
            ((s_6_4) >> (s_6_2)).value(),
            u16::try_from(s_6_8).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u8
        let s_6_10: u8 = (s_6_9.value() as u8);
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 3u16);
        // C s_6_12: const #2u : u8
        let s_6_12: u8 = 2;
        // C s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 3u16);
        // D s_6_14: cmp-eq s_6_11 s_6_13
        let s_6_14: bool = ((s_6_11) == (s_6_13));
        // N s_6_15: branch s_6_14 b117 b7
        if s_6_14 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#408585 <= s_7_0
        fn_state.gs_408585 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#408585:u8
        let s_8_0: bool = fn_state.gs_408585;
        // N s_8_1: branch s_8_0 b113 b9
        if s_8_0 {
            return block_113(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#408590 <= s_9_0
        fn_state.gs_408590 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#408590:u8
        let s_10_0: bool = fn_state.gs_408590;
        // D s_10_1: not s_10_0
        let s_10_1: bool = !s_10_0;
        // N s_10_2: branch s_10_1 b12 b11
        if s_10_1 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #2927s : i
        let s_11_0: i128 = 2927;
        // C s_11_1: const #14696u : u32
        let s_11_1: u32 = 14696;
        // N s_11_2: write-reg s_11_1 <= s_11_0
        let s_11_2: () = {
            state.write_register::<i128>(s_11_1 as isize, s_11_0);
            tracer.write_register(s_11_1 as isize, s_11_0);
        };
        // C s_11_3: const #28s : i
        let s_11_3: i128 = 28;
        // C s_11_4: const #4s : i
        let s_11_4: i128 = 4;
        // D s_11_5: read-var u#33280:u32
        let s_11_5: u32 = fn_state.u_33280;
        // D s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 32u16);
        // D s_11_7: bit-extract s_11_6 s_11_3 s_11_4
        let s_11_7: Bits = (Bits::new(
            ((s_11_6) >> (s_11_3)).value(),
            u16::try_from(s_11_4).unwrap(),
        ));
        // D s_11_8: cast reint s_11_7 -> u8
        let s_11_8: u8 = (s_11_7.value() as u8);
        // C s_11_9: const #24s : i
        let s_11_9: i128 = 24;
        // C s_11_10: const #1s : i
        let s_11_10: i128 = 1;
        // D s_11_11: read-var u#33280:u32
        let s_11_11: u32 = fn_state.u_33280;
        // D s_11_12: cast zx s_11_11 -> bv
        let s_11_12: Bits = Bits::new(s_11_11 as u128, 32u16);
        // D s_11_13: bit-extract s_11_12 s_11_9 s_11_10
        let s_11_13: Bits = (Bits::new(
            ((s_11_12) >> (s_11_9)).value(),
            u16::try_from(s_11_10).unwrap(),
        ));
        // D s_11_14: cast reint s_11_13 -> u8
        let s_11_14: bool = ((s_11_13.value()) != 0);
        // C s_11_15: const #23s : i
        let s_11_15: i128 = 23;
        // C s_11_16: const #1s : i
        let s_11_16: i128 = 1;
        // D s_11_17: read-var u#33280:u32
        let s_11_17: u32 = fn_state.u_33280;
        // D s_11_18: cast zx s_11_17 -> bv
        let s_11_18: Bits = Bits::new(s_11_17 as u128, 32u16);
        // D s_11_19: bit-extract s_11_18 s_11_15 s_11_16
        let s_11_19: Bits = (Bits::new(
            ((s_11_18) >> (s_11_15)).value(),
            u16::try_from(s_11_16).unwrap(),
        ));
        // D s_11_20: cast reint s_11_19 -> u8
        let s_11_20: bool = ((s_11_19.value()) != 0);
        // C s_11_21: const #21s : i
        let s_11_21: i128 = 21;
        // C s_11_22: const #1s : i
        let s_11_22: i128 = 1;
        // D s_11_23: read-var u#33280:u32
        let s_11_23: u32 = fn_state.u_33280;
        // D s_11_24: cast zx s_11_23 -> bv
        let s_11_24: Bits = Bits::new(s_11_23 as u128, 32u16);
        // D s_11_25: bit-extract s_11_24 s_11_21 s_11_22
        let s_11_25: Bits = (Bits::new(
            ((s_11_24) >> (s_11_21)).value(),
            u16::try_from(s_11_22).unwrap(),
        ));
        // D s_11_26: cast reint s_11_25 -> u8
        let s_11_26: bool = ((s_11_25.value()) != 0);
        // C s_11_27: const #12s : i
        let s_11_27: i128 = 12;
        // C s_11_28: const #4s : i
        let s_11_28: i128 = 4;
        // D s_11_29: read-var u#33280:u32
        let s_11_29: u32 = fn_state.u_33280;
        // D s_11_30: cast zx s_11_29 -> bv
        let s_11_30: Bits = Bits::new(s_11_29 as u128, 32u16);
        // D s_11_31: bit-extract s_11_30 s_11_27 s_11_28
        let s_11_31: Bits = (Bits::new(
            ((s_11_30) >> (s_11_27)).value(),
            u16::try_from(s_11_28).unwrap(),
        ));
        // D s_11_32: cast reint s_11_31 -> u8
        let s_11_32: u8 = (s_11_31.value() as u8);
        // C s_11_33: const #0s : i
        let s_11_33: i128 = 0;
        // C s_11_34: const #12s : i
        let s_11_34: i128 = 12;
        // D s_11_35: read-var u#33280:u32
        let s_11_35: u32 = fn_state.u_33280;
        // D s_11_36: cast zx s_11_35 -> bv
        let s_11_36: Bits = Bits::new(s_11_35 as u128, 32u16);
        // D s_11_37: bit-extract s_11_36 s_11_33 s_11_34
        let s_11_37: Bits = (Bits::new(
            ((s_11_36) >> (s_11_33)).value(),
            u16::try_from(s_11_34).unwrap(),
        ));
        // D s_11_38: cast reint s_11_37 -> u12
        let s_11_38: u16 = (s_11_37.value() as u16);
        // D s_11_39: call decode_aarch32_instrs_LDRB_l_A1enc_A_txt(s_11_8, s_11_14, s_11_20, s_11_26, s_11_32, s_11_38)
        let s_11_39: () = decode_aarch32_instrs_LDRB_l_A1enc_A_txt(
            state,
            tracer,
            s_11_8,
            s_11_14,
            s_11_20,
            s_11_26,
            s_11_32,
            s_11_38,
        );
        // N s_11_40: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var merge#var.1:struct
        let s_12_0: u32 = fn_state.merge_var._1;
        // D s_12_1: write-var u#33288 <= s_12_0
        fn_state.u_33288 = s_12_0;
        // C s_12_2: const #24s : i
        let s_12_2: i128 = 24;
        // D s_12_3: read-var u#33288:u32
        let s_12_3: u32 = fn_state.u_33288;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 32u16);
        // C s_12_5: const #1s : i64
        let s_12_5: i64 = 1;
        // C s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // C s_12_7: const #3s : i
        let s_12_7: i128 = 3;
        // C s_12_8: add s_12_7 s_12_6
        let s_12_8: i128 = (s_12_7 + s_12_6);
        // D s_12_9: bit-extract s_12_4 s_12_2 s_12_8
        let s_12_9: Bits = (Bits::new(
            ((s_12_4) >> (s_12_2)).value(),
            u16::try_from(s_12_8).unwrap(),
        ));
        // D s_12_10: cast reint s_12_9 -> u8
        let s_12_10: u8 = (s_12_9.value() as u8);
        // D s_12_11: cast zx s_12_10 -> bv
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 4u16);
        // C s_12_12: const #4u : u8
        let s_12_12: u8 = 4;
        // C s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 4u16);
        // D s_12_14: cmp-eq s_12_11 s_12_13
        let s_12_14: bool = ((s_12_11) == (s_12_13));
        // N s_12_15: branch s_12_14 b112 b13
        if s_12_14 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#408610 <= s_13_0
        fn_state.gs_408610 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#408610:u8
        let s_14_0: bool = fn_state.gs_408610;
        // N s_14_1: branch s_14_0 b108 b15
        if s_14_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#408615 <= s_15_0
        fn_state.gs_408615 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#408615:u8
        let s_16_0: bool = fn_state.gs_408615;
        // D s_16_1: not s_16_0
        let s_16_1: bool = !s_16_0;
        // N s_16_2: branch s_16_1 b18 b17
        if s_16_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #2932s : i
        let s_17_0: i128 = 2932;
        // C s_17_1: const #14696u : u32
        let s_17_1: u32 = 14696;
        // N s_17_2: write-reg s_17_1 <= s_17_0
        let s_17_2: () = {
            state.write_register::<i128>(s_17_1 as isize, s_17_0);
            tracer.write_register(s_17_1 as isize, s_17_0);
        };
        // C s_17_3: const #28s : i
        let s_17_3: i128 = 28;
        // C s_17_4: const #4s : i
        let s_17_4: i128 = 4;
        // D s_17_5: read-var u#33288:u32
        let s_17_5: u32 = fn_state.u_33288;
        // D s_17_6: cast zx s_17_5 -> bv
        let s_17_6: Bits = Bits::new(s_17_5 as u128, 32u16);
        // D s_17_7: bit-extract s_17_6 s_17_3 s_17_4
        let s_17_7: Bits = (Bits::new(
            ((s_17_6) >> (s_17_3)).value(),
            u16::try_from(s_17_4).unwrap(),
        ));
        // D s_17_8: cast reint s_17_7 -> u8
        let s_17_8: u8 = (s_17_7.value() as u8);
        // C s_17_9: const #23s : i
        let s_17_9: i128 = 23;
        // C s_17_10: const #1s : i
        let s_17_10: i128 = 1;
        // D s_17_11: read-var u#33288:u32
        let s_17_11: u32 = fn_state.u_33288;
        // D s_17_12: cast zx s_17_11 -> bv
        let s_17_12: Bits = Bits::new(s_17_11 as u128, 32u16);
        // D s_17_13: bit-extract s_17_12 s_17_9 s_17_10
        let s_17_13: Bits = (Bits::new(
            ((s_17_12) >> (s_17_9)).value(),
            u16::try_from(s_17_10).unwrap(),
        ));
        // D s_17_14: cast reint s_17_13 -> u8
        let s_17_14: bool = ((s_17_13.value()) != 0);
        // C s_17_15: const #16s : i
        let s_17_15: i128 = 16;
        // C s_17_16: const #4s : i
        let s_17_16: i128 = 4;
        // D s_17_17: read-var u#33288:u32
        let s_17_17: u32 = fn_state.u_33288;
        // D s_17_18: cast zx s_17_17 -> bv
        let s_17_18: Bits = Bits::new(s_17_17 as u128, 32u16);
        // D s_17_19: bit-extract s_17_18 s_17_15 s_17_16
        let s_17_19: Bits = (Bits::new(
            ((s_17_18) >> (s_17_15)).value(),
            u16::try_from(s_17_16).unwrap(),
        ));
        // D s_17_20: cast reint s_17_19 -> u8
        let s_17_20: u8 = (s_17_19.value() as u8);
        // C s_17_21: const #12s : i
        let s_17_21: i128 = 12;
        // C s_17_22: const #4s : i
        let s_17_22: i128 = 4;
        // D s_17_23: read-var u#33288:u32
        let s_17_23: u32 = fn_state.u_33288;
        // D s_17_24: cast zx s_17_23 -> bv
        let s_17_24: Bits = Bits::new(s_17_23 as u128, 32u16);
        // D s_17_25: bit-extract s_17_24 s_17_21 s_17_22
        let s_17_25: Bits = (Bits::new(
            ((s_17_24) >> (s_17_21)).value(),
            u16::try_from(s_17_22).unwrap(),
        ));
        // D s_17_26: cast reint s_17_25 -> u8
        let s_17_26: u8 = (s_17_25.value() as u8);
        // C s_17_27: const #0s : i
        let s_17_27: i128 = 0;
        // C s_17_28: const #12s : i
        let s_17_28: i128 = 12;
        // D s_17_29: read-var u#33288:u32
        let s_17_29: u32 = fn_state.u_33288;
        // D s_17_30: cast zx s_17_29 -> bv
        let s_17_30: Bits = Bits::new(s_17_29 as u128, 32u16);
        // D s_17_31: bit-extract s_17_30 s_17_27 s_17_28
        let s_17_31: Bits = (Bits::new(
            ((s_17_30) >> (s_17_27)).value(),
            u16::try_from(s_17_28).unwrap(),
        ));
        // D s_17_32: cast reint s_17_31 -> u12
        let s_17_32: u16 = (s_17_31.value() as u16);
        // D s_17_33: call decode_aarch32_instrs_LDRBT_A1enc_A_txt(s_17_8, s_17_14, s_17_20, s_17_26, s_17_32)
        let s_17_33: () = decode_aarch32_instrs_LDRBT_A1enc_A_txt(
            state,
            tracer,
            s_17_8,
            s_17_14,
            s_17_20,
            s_17_26,
            s_17_32,
        );
        // N s_17_34: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var merge#var.1:struct
        let s_18_0: u32 = fn_state.merge_var._1;
        // D s_18_1: write-var u#33295 <= s_18_0
        fn_state.u_33295 = s_18_0;
        // C s_18_2: const #25s : i
        let s_18_2: i128 = 25;
        // D s_18_3: read-var u#33295:u32
        let s_18_3: u32 = fn_state.u_33295;
        // D s_18_4: cast zx s_18_3 -> bv
        let s_18_4: Bits = Bits::new(s_18_3 as u128, 32u16);
        // C s_18_5: const #1s : i64
        let s_18_5: i64 = 1;
        // C s_18_6: cast zx s_18_5 -> i
        let s_18_6: i128 = (i128::try_from(s_18_5).unwrap());
        // C s_18_7: const #2s : i
        let s_18_7: i128 = 2;
        // C s_18_8: add s_18_7 s_18_6
        let s_18_8: i128 = (s_18_7 + s_18_6);
        // D s_18_9: bit-extract s_18_4 s_18_2 s_18_8
        let s_18_9: Bits = (Bits::new(
            ((s_18_4) >> (s_18_2)).value(),
            u16::try_from(s_18_8).unwrap(),
        ));
        // D s_18_10: cast reint s_18_9 -> u8
        let s_18_10: u8 = (s_18_9.value() as u8);
        // D s_18_11: cast zx s_18_10 -> bv
        let s_18_11: Bits = Bits::new(s_18_10 as u128, 3u16);
        // C s_18_12: const #2u : u8
        let s_18_12: u8 = 2;
        // C s_18_13: cast zx s_18_12 -> bv
        let s_18_13: Bits = Bits::new(s_18_12 as u128, 3u16);
        // D s_18_14: cmp-eq s_18_11 s_18_13
        let s_18_14: bool = ((s_18_11) == (s_18_13));
        // N s_18_15: branch s_18_14 b104 b19
        if s_18_14 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#408636 <= s_19_0
        fn_state.gs_408636 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#408636:u8
        let s_20_0: bool = fn_state.gs_408636;
        // N s_20_1: branch s_20_0 b100 b21
        if s_20_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#408641 <= s_21_0
        fn_state.gs_408641 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#408641:u8
        let s_22_0: bool = fn_state.gs_408641;
        // D s_22_1: not s_22_0
        let s_22_1: bool = !s_22_0;
        // N s_22_2: branch s_22_1 b24 b23
        if s_22_1 {
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
        // C s_23_0: const #2960s : i
        let s_23_0: i128 = 2960;
        // C s_23_1: const #14696u : u32
        let s_23_1: u32 = 14696;
        // N s_23_2: write-reg s_23_1 <= s_23_0
        let s_23_2: () = {
            state.write_register::<i128>(s_23_1 as isize, s_23_0);
            tracer.write_register(s_23_1 as isize, s_23_0);
        };
        // C s_23_3: const #28s : i
        let s_23_3: i128 = 28;
        // C s_23_4: const #4s : i
        let s_23_4: i128 = 4;
        // D s_23_5: read-var u#33295:u32
        let s_23_5: u32 = fn_state.u_33295;
        // D s_23_6: cast zx s_23_5 -> bv
        let s_23_6: Bits = Bits::new(s_23_5 as u128, 32u16);
        // D s_23_7: bit-extract s_23_6 s_23_3 s_23_4
        let s_23_7: Bits = (Bits::new(
            ((s_23_6) >> (s_23_3)).value(),
            u16::try_from(s_23_4).unwrap(),
        ));
        // D s_23_8: cast reint s_23_7 -> u8
        let s_23_8: u8 = (s_23_7.value() as u8);
        // C s_23_9: const #24s : i
        let s_23_9: i128 = 24;
        // C s_23_10: const #1s : i
        let s_23_10: i128 = 1;
        // D s_23_11: read-var u#33295:u32
        let s_23_11: u32 = fn_state.u_33295;
        // D s_23_12: cast zx s_23_11 -> bv
        let s_23_12: Bits = Bits::new(s_23_11 as u128, 32u16);
        // D s_23_13: bit-extract s_23_12 s_23_9 s_23_10
        let s_23_13: Bits = (Bits::new(
            ((s_23_12) >> (s_23_9)).value(),
            u16::try_from(s_23_10).unwrap(),
        ));
        // D s_23_14: cast reint s_23_13 -> u8
        let s_23_14: bool = ((s_23_13.value()) != 0);
        // C s_23_15: const #23s : i
        let s_23_15: i128 = 23;
        // C s_23_16: const #1s : i
        let s_23_16: i128 = 1;
        // D s_23_17: read-var u#33295:u32
        let s_23_17: u32 = fn_state.u_33295;
        // D s_23_18: cast zx s_23_17 -> bv
        let s_23_18: Bits = Bits::new(s_23_17 as u128, 32u16);
        // D s_23_19: bit-extract s_23_18 s_23_15 s_23_16
        let s_23_19: Bits = (Bits::new(
            ((s_23_18) >> (s_23_15)).value(),
            u16::try_from(s_23_16).unwrap(),
        ));
        // D s_23_20: cast reint s_23_19 -> u8
        let s_23_20: bool = ((s_23_19.value()) != 0);
        // C s_23_21: const #21s : i
        let s_23_21: i128 = 21;
        // C s_23_22: const #1s : i
        let s_23_22: i128 = 1;
        // D s_23_23: read-var u#33295:u32
        let s_23_23: u32 = fn_state.u_33295;
        // D s_23_24: cast zx s_23_23 -> bv
        let s_23_24: Bits = Bits::new(s_23_23 as u128, 32u16);
        // D s_23_25: bit-extract s_23_24 s_23_21 s_23_22
        let s_23_25: Bits = (Bits::new(
            ((s_23_24) >> (s_23_21)).value(),
            u16::try_from(s_23_22).unwrap(),
        ));
        // D s_23_26: cast reint s_23_25 -> u8
        let s_23_26: bool = ((s_23_25.value()) != 0);
        // C s_23_27: const #16s : i
        let s_23_27: i128 = 16;
        // C s_23_28: const #4s : i
        let s_23_28: i128 = 4;
        // D s_23_29: read-var u#33295:u32
        let s_23_29: u32 = fn_state.u_33295;
        // D s_23_30: cast zx s_23_29 -> bv
        let s_23_30: Bits = Bits::new(s_23_29 as u128, 32u16);
        // D s_23_31: bit-extract s_23_30 s_23_27 s_23_28
        let s_23_31: Bits = (Bits::new(
            ((s_23_30) >> (s_23_27)).value(),
            u16::try_from(s_23_28).unwrap(),
        ));
        // D s_23_32: cast reint s_23_31 -> u8
        let s_23_32: u8 = (s_23_31.value() as u8);
        // C s_23_33: const #12s : i
        let s_23_33: i128 = 12;
        // C s_23_34: const #4s : i
        let s_23_34: i128 = 4;
        // D s_23_35: read-var u#33295:u32
        let s_23_35: u32 = fn_state.u_33295;
        // D s_23_36: cast zx s_23_35 -> bv
        let s_23_36: Bits = Bits::new(s_23_35 as u128, 32u16);
        // D s_23_37: bit-extract s_23_36 s_23_33 s_23_34
        let s_23_37: Bits = (Bits::new(
            ((s_23_36) >> (s_23_33)).value(),
            u16::try_from(s_23_34).unwrap(),
        ));
        // D s_23_38: cast reint s_23_37 -> u8
        let s_23_38: u8 = (s_23_37.value() as u8);
        // C s_23_39: const #0s : i
        let s_23_39: i128 = 0;
        // C s_23_40: const #12s : i
        let s_23_40: i128 = 12;
        // D s_23_41: read-var u#33295:u32
        let s_23_41: u32 = fn_state.u_33295;
        // D s_23_42: cast zx s_23_41 -> bv
        let s_23_42: Bits = Bits::new(s_23_41 as u128, 32u16);
        // D s_23_43: bit-extract s_23_42 s_23_39 s_23_40
        let s_23_43: Bits = (Bits::new(
            ((s_23_42) >> (s_23_39)).value(),
            u16::try_from(s_23_40).unwrap(),
        ));
        // D s_23_44: cast reint s_23_43 -> u12
        let s_23_44: u16 = (s_23_43.value() as u16);
        // D s_23_45: call decode_aarch32_instrs_LDR_i_A1enc_A_txt(s_23_8, s_23_14, s_23_20, s_23_26, s_23_32, s_23_38, s_23_44)
        let s_23_45: () = decode_aarch32_instrs_LDR_i_A1enc_A_txt(
            state,
            tracer,
            s_23_8,
            s_23_14,
            s_23_20,
            s_23_26,
            s_23_32,
            s_23_38,
            s_23_44,
        );
        // N s_23_46: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var merge#var.1:struct
        let s_24_0: u32 = fn_state.merge_var._1;
        // D s_24_1: write-var u#33304 <= s_24_0
        fn_state.u_33304 = s_24_0;
        // C s_24_2: const #25s : i
        let s_24_2: i128 = 25;
        // D s_24_3: read-var u#33304:u32
        let s_24_3: u32 = fn_state.u_33304;
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 32u16);
        // C s_24_5: const #1s : i64
        let s_24_5: i64 = 1;
        // C s_24_6: cast zx s_24_5 -> i
        let s_24_6: i128 = (i128::try_from(s_24_5).unwrap());
        // C s_24_7: const #2s : i
        let s_24_7: i128 = 2;
        // C s_24_8: add s_24_7 s_24_6
        let s_24_8: i128 = (s_24_7 + s_24_6);
        // D s_24_9: bit-extract s_24_4 s_24_2 s_24_8
        let s_24_9: Bits = (Bits::new(
            ((s_24_4) >> (s_24_2)).value(),
            u16::try_from(s_24_8).unwrap(),
        ));
        // D s_24_10: cast reint s_24_9 -> u8
        let s_24_10: u8 = (s_24_9.value() as u8);
        // D s_24_11: cast zx s_24_10 -> bv
        let s_24_11: Bits = Bits::new(s_24_10 as u128, 3u16);
        // C s_24_12: const #2u : u8
        let s_24_12: u8 = 2;
        // C s_24_13: cast zx s_24_12 -> bv
        let s_24_13: Bits = Bits::new(s_24_12 as u128, 3u16);
        // D s_24_14: cmp-eq s_24_11 s_24_13
        let s_24_14: bool = ((s_24_11) == (s_24_13));
        // N s_24_15: branch s_24_14 b96 b25
        if s_24_14 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#408666 <= s_25_0
        fn_state.gs_408666 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#408666:u8
        let s_26_0: bool = fn_state.gs_408666;
        // N s_26_1: branch s_26_0 b92 b27
        if s_26_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#408671 <= s_27_0
        fn_state.gs_408671 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#408671:u8
        let s_28_0: bool = fn_state.gs_408671;
        // D s_28_1: not s_28_0
        let s_28_1: bool = !s_28_0;
        // N s_28_2: branch s_28_1 b30 b29
        if s_28_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #2965s : i
        let s_29_0: i128 = 2965;
        // C s_29_1: const #14696u : u32
        let s_29_1: u32 = 14696;
        // N s_29_2: write-reg s_29_1 <= s_29_0
        let s_29_2: () = {
            state.write_register::<i128>(s_29_1 as isize, s_29_0);
            tracer.write_register(s_29_1 as isize, s_29_0);
        };
        // C s_29_3: const #28s : i
        let s_29_3: i128 = 28;
        // C s_29_4: const #4s : i
        let s_29_4: i128 = 4;
        // D s_29_5: read-var u#33304:u32
        let s_29_5: u32 = fn_state.u_33304;
        // D s_29_6: cast zx s_29_5 -> bv
        let s_29_6: Bits = Bits::new(s_29_5 as u128, 32u16);
        // D s_29_7: bit-extract s_29_6 s_29_3 s_29_4
        let s_29_7: Bits = (Bits::new(
            ((s_29_6) >> (s_29_3)).value(),
            u16::try_from(s_29_4).unwrap(),
        ));
        // D s_29_8: cast reint s_29_7 -> u8
        let s_29_8: u8 = (s_29_7.value() as u8);
        // C s_29_9: const #24s : i
        let s_29_9: i128 = 24;
        // C s_29_10: const #1s : i
        let s_29_10: i128 = 1;
        // D s_29_11: read-var u#33304:u32
        let s_29_11: u32 = fn_state.u_33304;
        // D s_29_12: cast zx s_29_11 -> bv
        let s_29_12: Bits = Bits::new(s_29_11 as u128, 32u16);
        // D s_29_13: bit-extract s_29_12 s_29_9 s_29_10
        let s_29_13: Bits = (Bits::new(
            ((s_29_12) >> (s_29_9)).value(),
            u16::try_from(s_29_10).unwrap(),
        ));
        // D s_29_14: cast reint s_29_13 -> u8
        let s_29_14: bool = ((s_29_13.value()) != 0);
        // C s_29_15: const #23s : i
        let s_29_15: i128 = 23;
        // C s_29_16: const #1s : i
        let s_29_16: i128 = 1;
        // D s_29_17: read-var u#33304:u32
        let s_29_17: u32 = fn_state.u_33304;
        // D s_29_18: cast zx s_29_17 -> bv
        let s_29_18: Bits = Bits::new(s_29_17 as u128, 32u16);
        // D s_29_19: bit-extract s_29_18 s_29_15 s_29_16
        let s_29_19: Bits = (Bits::new(
            ((s_29_18) >> (s_29_15)).value(),
            u16::try_from(s_29_16).unwrap(),
        ));
        // D s_29_20: cast reint s_29_19 -> u8
        let s_29_20: bool = ((s_29_19.value()) != 0);
        // C s_29_21: const #21s : i
        let s_29_21: i128 = 21;
        // C s_29_22: const #1s : i
        let s_29_22: i128 = 1;
        // D s_29_23: read-var u#33304:u32
        let s_29_23: u32 = fn_state.u_33304;
        // D s_29_24: cast zx s_29_23 -> bv
        let s_29_24: Bits = Bits::new(s_29_23 as u128, 32u16);
        // D s_29_25: bit-extract s_29_24 s_29_21 s_29_22
        let s_29_25: Bits = (Bits::new(
            ((s_29_24) >> (s_29_21)).value(),
            u16::try_from(s_29_22).unwrap(),
        ));
        // D s_29_26: cast reint s_29_25 -> u8
        let s_29_26: bool = ((s_29_25.value()) != 0);
        // C s_29_27: const #12s : i
        let s_29_27: i128 = 12;
        // C s_29_28: const #4s : i
        let s_29_28: i128 = 4;
        // D s_29_29: read-var u#33304:u32
        let s_29_29: u32 = fn_state.u_33304;
        // D s_29_30: cast zx s_29_29 -> bv
        let s_29_30: Bits = Bits::new(s_29_29 as u128, 32u16);
        // D s_29_31: bit-extract s_29_30 s_29_27 s_29_28
        let s_29_31: Bits = (Bits::new(
            ((s_29_30) >> (s_29_27)).value(),
            u16::try_from(s_29_28).unwrap(),
        ));
        // D s_29_32: cast reint s_29_31 -> u8
        let s_29_32: u8 = (s_29_31.value() as u8);
        // C s_29_33: const #0s : i
        let s_29_33: i128 = 0;
        // C s_29_34: const #12s : i
        let s_29_34: i128 = 12;
        // D s_29_35: read-var u#33304:u32
        let s_29_35: u32 = fn_state.u_33304;
        // D s_29_36: cast zx s_29_35 -> bv
        let s_29_36: Bits = Bits::new(s_29_35 as u128, 32u16);
        // D s_29_37: bit-extract s_29_36 s_29_33 s_29_34
        let s_29_37: Bits = (Bits::new(
            ((s_29_36) >> (s_29_33)).value(),
            u16::try_from(s_29_34).unwrap(),
        ));
        // D s_29_38: cast reint s_29_37 -> u12
        let s_29_38: u16 = (s_29_37.value() as u16);
        // D s_29_39: call decode_aarch32_instrs_LDR_l_A1enc_A_txt(s_29_8, s_29_14, s_29_20, s_29_26, s_29_32, s_29_38)
        let s_29_39: () = decode_aarch32_instrs_LDR_l_A1enc_A_txt(
            state,
            tracer,
            s_29_8,
            s_29_14,
            s_29_20,
            s_29_26,
            s_29_32,
            s_29_38,
        );
        // N s_29_40: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var merge#var.1:struct
        let s_30_0: u32 = fn_state.merge_var._1;
        // D s_30_1: write-var u#33312 <= s_30_0
        fn_state.u_33312 = s_30_0;
        // C s_30_2: const #24s : i
        let s_30_2: i128 = 24;
        // D s_30_3: read-var u#33312:u32
        let s_30_3: u32 = fn_state.u_33312;
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 32u16);
        // C s_30_5: const #1s : i64
        let s_30_5: i64 = 1;
        // C s_30_6: cast zx s_30_5 -> i
        let s_30_6: i128 = (i128::try_from(s_30_5).unwrap());
        // C s_30_7: const #3s : i
        let s_30_7: i128 = 3;
        // C s_30_8: add s_30_7 s_30_6
        let s_30_8: i128 = (s_30_7 + s_30_6);
        // D s_30_9: bit-extract s_30_4 s_30_2 s_30_8
        let s_30_9: Bits = (Bits::new(
            ((s_30_4) >> (s_30_2)).value(),
            u16::try_from(s_30_8).unwrap(),
        ));
        // D s_30_10: cast reint s_30_9 -> u8
        let s_30_10: u8 = (s_30_9.value() as u8);
        // D s_30_11: cast zx s_30_10 -> bv
        let s_30_11: Bits = Bits::new(s_30_10 as u128, 4u16);
        // C s_30_12: const #4u : u8
        let s_30_12: u8 = 4;
        // C s_30_13: cast zx s_30_12 -> bv
        let s_30_13: Bits = Bits::new(s_30_12 as u128, 4u16);
        // D s_30_14: cmp-eq s_30_11 s_30_13
        let s_30_14: bool = ((s_30_11) == (s_30_13));
        // N s_30_15: branch s_30_14 b91 b31
        if s_30_14 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#408691 <= s_31_0
        fn_state.gs_408691 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#408691:u8
        let s_32_0: bool = fn_state.gs_408691;
        // N s_32_1: branch s_32_0 b87 b33
        if s_32_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#408696 <= s_33_0
        fn_state.gs_408696 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#408696:u8
        let s_34_0: bool = fn_state.gs_408696;
        // D s_34_1: not s_34_0
        let s_34_1: bool = !s_34_0;
        // N s_34_2: branch s_34_1 b36 b35
        if s_34_1 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #2993s : i
        let s_35_0: i128 = 2993;
        // C s_35_1: const #14696u : u32
        let s_35_1: u32 = 14696;
        // N s_35_2: write-reg s_35_1 <= s_35_0
        let s_35_2: () = {
            state.write_register::<i128>(s_35_1 as isize, s_35_0);
            tracer.write_register(s_35_1 as isize, s_35_0);
        };
        // C s_35_3: const #28s : i
        let s_35_3: i128 = 28;
        // C s_35_4: const #4s : i
        let s_35_4: i128 = 4;
        // D s_35_5: read-var u#33312:u32
        let s_35_5: u32 = fn_state.u_33312;
        // D s_35_6: cast zx s_35_5 -> bv
        let s_35_6: Bits = Bits::new(s_35_5 as u128, 32u16);
        // D s_35_7: bit-extract s_35_6 s_35_3 s_35_4
        let s_35_7: Bits = (Bits::new(
            ((s_35_6) >> (s_35_3)).value(),
            u16::try_from(s_35_4).unwrap(),
        ));
        // D s_35_8: cast reint s_35_7 -> u8
        let s_35_8: u8 = (s_35_7.value() as u8);
        // C s_35_9: const #23s : i
        let s_35_9: i128 = 23;
        // C s_35_10: const #1s : i
        let s_35_10: i128 = 1;
        // D s_35_11: read-var u#33312:u32
        let s_35_11: u32 = fn_state.u_33312;
        // D s_35_12: cast zx s_35_11 -> bv
        let s_35_12: Bits = Bits::new(s_35_11 as u128, 32u16);
        // D s_35_13: bit-extract s_35_12 s_35_9 s_35_10
        let s_35_13: Bits = (Bits::new(
            ((s_35_12) >> (s_35_9)).value(),
            u16::try_from(s_35_10).unwrap(),
        ));
        // D s_35_14: cast reint s_35_13 -> u8
        let s_35_14: bool = ((s_35_13.value()) != 0);
        // C s_35_15: const #16s : i
        let s_35_15: i128 = 16;
        // C s_35_16: const #4s : i
        let s_35_16: i128 = 4;
        // D s_35_17: read-var u#33312:u32
        let s_35_17: u32 = fn_state.u_33312;
        // D s_35_18: cast zx s_35_17 -> bv
        let s_35_18: Bits = Bits::new(s_35_17 as u128, 32u16);
        // D s_35_19: bit-extract s_35_18 s_35_15 s_35_16
        let s_35_19: Bits = (Bits::new(
            ((s_35_18) >> (s_35_15)).value(),
            u16::try_from(s_35_16).unwrap(),
        ));
        // D s_35_20: cast reint s_35_19 -> u8
        let s_35_20: u8 = (s_35_19.value() as u8);
        // C s_35_21: const #12s : i
        let s_35_21: i128 = 12;
        // C s_35_22: const #4s : i
        let s_35_22: i128 = 4;
        // D s_35_23: read-var u#33312:u32
        let s_35_23: u32 = fn_state.u_33312;
        // D s_35_24: cast zx s_35_23 -> bv
        let s_35_24: Bits = Bits::new(s_35_23 as u128, 32u16);
        // D s_35_25: bit-extract s_35_24 s_35_21 s_35_22
        let s_35_25: Bits = (Bits::new(
            ((s_35_24) >> (s_35_21)).value(),
            u16::try_from(s_35_22).unwrap(),
        ));
        // D s_35_26: cast reint s_35_25 -> u8
        let s_35_26: u8 = (s_35_25.value() as u8);
        // C s_35_27: const #0s : i
        let s_35_27: i128 = 0;
        // C s_35_28: const #12s : i
        let s_35_28: i128 = 12;
        // D s_35_29: read-var u#33312:u32
        let s_35_29: u32 = fn_state.u_33312;
        // D s_35_30: cast zx s_35_29 -> bv
        let s_35_30: Bits = Bits::new(s_35_29 as u128, 32u16);
        // D s_35_31: bit-extract s_35_30 s_35_27 s_35_28
        let s_35_31: Bits = (Bits::new(
            ((s_35_30) >> (s_35_27)).value(),
            u16::try_from(s_35_28).unwrap(),
        ));
        // D s_35_32: cast reint s_35_31 -> u12
        let s_35_32: u16 = (s_35_31.value() as u16);
        // D s_35_33: call decode_aarch32_instrs_LDRT_A1enc_A_txt(s_35_8, s_35_14, s_35_20, s_35_26, s_35_32)
        let s_35_33: () = decode_aarch32_instrs_LDRT_A1enc_A_txt(
            state,
            tracer,
            s_35_8,
            s_35_14,
            s_35_20,
            s_35_26,
            s_35_32,
        );
        // N s_35_34: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var merge#var.1:struct
        let s_36_0: u32 = fn_state.merge_var._1;
        // D s_36_1: write-var u#33319 <= s_36_0
        fn_state.u_33319 = s_36_0;
        // C s_36_2: const #25s : i
        let s_36_2: i128 = 25;
        // D s_36_3: read-var u#33319:u32
        let s_36_3: u32 = fn_state.u_33319;
        // D s_36_4: cast zx s_36_3 -> bv
        let s_36_4: Bits = Bits::new(s_36_3 as u128, 32u16);
        // C s_36_5: const #1s : i64
        let s_36_5: i64 = 1;
        // C s_36_6: cast zx s_36_5 -> i
        let s_36_6: i128 = (i128::try_from(s_36_5).unwrap());
        // C s_36_7: const #2s : i
        let s_36_7: i128 = 2;
        // C s_36_8: add s_36_7 s_36_6
        let s_36_8: i128 = (s_36_7 + s_36_6);
        // D s_36_9: bit-extract s_36_4 s_36_2 s_36_8
        let s_36_9: Bits = (Bits::new(
            ((s_36_4) >> (s_36_2)).value(),
            u16::try_from(s_36_8).unwrap(),
        ));
        // D s_36_10: cast reint s_36_9 -> u8
        let s_36_10: u8 = (s_36_9.value() as u8);
        // D s_36_11: cast zx s_36_10 -> bv
        let s_36_11: Bits = Bits::new(s_36_10 as u128, 3u16);
        // C s_36_12: const #2u : u8
        let s_36_12: u8 = 2;
        // C s_36_13: cast zx s_36_12 -> bv
        let s_36_13: Bits = Bits::new(s_36_12 as u128, 3u16);
        // D s_36_14: cmp-eq s_36_11 s_36_13
        let s_36_14: bool = ((s_36_11) == (s_36_13));
        // N s_36_15: branch s_36_14 b83 b37
        if s_36_14 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#408717 <= s_37_0
        fn_state.gs_408717 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#408717:u8
        let s_38_0: bool = fn_state.gs_408717;
        // N s_38_1: branch s_38_0 b79 b39
        if s_38_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#408722 <= s_39_0
        fn_state.gs_408722 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#408722:u8
        let s_40_0: bool = fn_state.gs_408722;
        // D s_40_1: not s_40_0
        let s_40_1: bool = !s_40_0;
        // N s_40_2: branch s_40_1 b42 b41
        if s_40_1 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #3193s : i
        let s_41_0: i128 = 3193;
        // C s_41_1: const #14696u : u32
        let s_41_1: u32 = 14696;
        // N s_41_2: write-reg s_41_1 <= s_41_0
        let s_41_2: () = {
            state.write_register::<i128>(s_41_1 as isize, s_41_0);
            tracer.write_register(s_41_1 as isize, s_41_0);
        };
        // C s_41_3: const #28s : i
        let s_41_3: i128 = 28;
        // C s_41_4: const #4s : i
        let s_41_4: i128 = 4;
        // D s_41_5: read-var u#33319:u32
        let s_41_5: u32 = fn_state.u_33319;
        // D s_41_6: cast zx s_41_5 -> bv
        let s_41_6: Bits = Bits::new(s_41_5 as u128, 32u16);
        // D s_41_7: bit-extract s_41_6 s_41_3 s_41_4
        let s_41_7: Bits = (Bits::new(
            ((s_41_6) >> (s_41_3)).value(),
            u16::try_from(s_41_4).unwrap(),
        ));
        // D s_41_8: cast reint s_41_7 -> u8
        let s_41_8: u8 = (s_41_7.value() as u8);
        // C s_41_9: const #24s : i
        let s_41_9: i128 = 24;
        // C s_41_10: const #1s : i
        let s_41_10: i128 = 1;
        // D s_41_11: read-var u#33319:u32
        let s_41_11: u32 = fn_state.u_33319;
        // D s_41_12: cast zx s_41_11 -> bv
        let s_41_12: Bits = Bits::new(s_41_11 as u128, 32u16);
        // D s_41_13: bit-extract s_41_12 s_41_9 s_41_10
        let s_41_13: Bits = (Bits::new(
            ((s_41_12) >> (s_41_9)).value(),
            u16::try_from(s_41_10).unwrap(),
        ));
        // D s_41_14: cast reint s_41_13 -> u8
        let s_41_14: bool = ((s_41_13.value()) != 0);
        // C s_41_15: const #23s : i
        let s_41_15: i128 = 23;
        // C s_41_16: const #1s : i
        let s_41_16: i128 = 1;
        // D s_41_17: read-var u#33319:u32
        let s_41_17: u32 = fn_state.u_33319;
        // D s_41_18: cast zx s_41_17 -> bv
        let s_41_18: Bits = Bits::new(s_41_17 as u128, 32u16);
        // D s_41_19: bit-extract s_41_18 s_41_15 s_41_16
        let s_41_19: Bits = (Bits::new(
            ((s_41_18) >> (s_41_15)).value(),
            u16::try_from(s_41_16).unwrap(),
        ));
        // D s_41_20: cast reint s_41_19 -> u8
        let s_41_20: bool = ((s_41_19.value()) != 0);
        // C s_41_21: const #21s : i
        let s_41_21: i128 = 21;
        // C s_41_22: const #1s : i
        let s_41_22: i128 = 1;
        // D s_41_23: read-var u#33319:u32
        let s_41_23: u32 = fn_state.u_33319;
        // D s_41_24: cast zx s_41_23 -> bv
        let s_41_24: Bits = Bits::new(s_41_23 as u128, 32u16);
        // D s_41_25: bit-extract s_41_24 s_41_21 s_41_22
        let s_41_25: Bits = (Bits::new(
            ((s_41_24) >> (s_41_21)).value(),
            u16::try_from(s_41_22).unwrap(),
        ));
        // D s_41_26: cast reint s_41_25 -> u8
        let s_41_26: bool = ((s_41_25.value()) != 0);
        // C s_41_27: const #16s : i
        let s_41_27: i128 = 16;
        // C s_41_28: const #4s : i
        let s_41_28: i128 = 4;
        // D s_41_29: read-var u#33319:u32
        let s_41_29: u32 = fn_state.u_33319;
        // D s_41_30: cast zx s_41_29 -> bv
        let s_41_30: Bits = Bits::new(s_41_29 as u128, 32u16);
        // D s_41_31: bit-extract s_41_30 s_41_27 s_41_28
        let s_41_31: Bits = (Bits::new(
            ((s_41_30) >> (s_41_27)).value(),
            u16::try_from(s_41_28).unwrap(),
        ));
        // D s_41_32: cast reint s_41_31 -> u8
        let s_41_32: u8 = (s_41_31.value() as u8);
        // C s_41_33: const #12s : i
        let s_41_33: i128 = 12;
        // C s_41_34: const #4s : i
        let s_41_34: i128 = 4;
        // D s_41_35: read-var u#33319:u32
        let s_41_35: u32 = fn_state.u_33319;
        // D s_41_36: cast zx s_41_35 -> bv
        let s_41_36: Bits = Bits::new(s_41_35 as u128, 32u16);
        // D s_41_37: bit-extract s_41_36 s_41_33 s_41_34
        let s_41_37: Bits = (Bits::new(
            ((s_41_36) >> (s_41_33)).value(),
            u16::try_from(s_41_34).unwrap(),
        ));
        // D s_41_38: cast reint s_41_37 -> u8
        let s_41_38: u8 = (s_41_37.value() as u8);
        // C s_41_39: const #0s : i
        let s_41_39: i128 = 0;
        // C s_41_40: const #12s : i
        let s_41_40: i128 = 12;
        // D s_41_41: read-var u#33319:u32
        let s_41_41: u32 = fn_state.u_33319;
        // D s_41_42: cast zx s_41_41 -> bv
        let s_41_42: Bits = Bits::new(s_41_41 as u128, 32u16);
        // D s_41_43: bit-extract s_41_42 s_41_39 s_41_40
        let s_41_43: Bits = (Bits::new(
            ((s_41_42) >> (s_41_39)).value(),
            u16::try_from(s_41_40).unwrap(),
        ));
        // D s_41_44: cast reint s_41_43 -> u12
        let s_41_44: u16 = (s_41_43.value() as u16);
        // D s_41_45: call decode_aarch32_instrs_STRB_i_A1enc_A_txt(s_41_8, s_41_14, s_41_20, s_41_26, s_41_32, s_41_38, s_41_44)
        let s_41_45: () = decode_aarch32_instrs_STRB_i_A1enc_A_txt(
            state,
            tracer,
            s_41_8,
            s_41_14,
            s_41_20,
            s_41_26,
            s_41_32,
            s_41_38,
            s_41_44,
        );
        // N s_41_46: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var merge#var.1:struct
        let s_42_0: u32 = fn_state.merge_var._1;
        // D s_42_1: write-var u#33328 <= s_42_0
        fn_state.u_33328 = s_42_0;
        // C s_42_2: const #24s : i
        let s_42_2: i128 = 24;
        // D s_42_3: read-var u#33328:u32
        let s_42_3: u32 = fn_state.u_33328;
        // D s_42_4: cast zx s_42_3 -> bv
        let s_42_4: Bits = Bits::new(s_42_3 as u128, 32u16);
        // C s_42_5: const #1s : i64
        let s_42_5: i64 = 1;
        // C s_42_6: cast zx s_42_5 -> i
        let s_42_6: i128 = (i128::try_from(s_42_5).unwrap());
        // C s_42_7: const #3s : i
        let s_42_7: i128 = 3;
        // C s_42_8: add s_42_7 s_42_6
        let s_42_8: i128 = (s_42_7 + s_42_6);
        // D s_42_9: bit-extract s_42_4 s_42_2 s_42_8
        let s_42_9: Bits = (Bits::new(
            ((s_42_4) >> (s_42_2)).value(),
            u16::try_from(s_42_8).unwrap(),
        ));
        // D s_42_10: cast reint s_42_9 -> u8
        let s_42_10: u8 = (s_42_9.value() as u8);
        // D s_42_11: cast zx s_42_10 -> bv
        let s_42_11: Bits = Bits::new(s_42_10 as u128, 4u16);
        // C s_42_12: const #4u : u8
        let s_42_12: u8 = 4;
        // C s_42_13: cast zx s_42_12 -> bv
        let s_42_13: Bits = Bits::new(s_42_12 as u128, 4u16);
        // D s_42_14: cmp-eq s_42_11 s_42_13
        let s_42_14: bool = ((s_42_11) == (s_42_13));
        // N s_42_15: branch s_42_14 b78 b43
        if s_42_14 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#408744 <= s_43_0
        fn_state.gs_408744 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#408744:u8
        let s_44_0: bool = fn_state.gs_408744;
        // N s_44_1: branch s_44_0 b74 b45
        if s_44_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#408749 <= s_45_0
        fn_state.gs_408749 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#408749:u8
        let s_46_0: bool = fn_state.gs_408749;
        // D s_46_1: not s_46_0
        let s_46_1: bool = !s_46_0;
        // N s_46_2: branch s_46_1 b48 b47
        if s_46_1 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #3200s : i
        let s_47_0: i128 = 3200;
        // C s_47_1: const #14696u : u32
        let s_47_1: u32 = 14696;
        // N s_47_2: write-reg s_47_1 <= s_47_0
        let s_47_2: () = {
            state.write_register::<i128>(s_47_1 as isize, s_47_0);
            tracer.write_register(s_47_1 as isize, s_47_0);
        };
        // C s_47_3: const #28s : i
        let s_47_3: i128 = 28;
        // C s_47_4: const #4s : i
        let s_47_4: i128 = 4;
        // D s_47_5: read-var u#33328:u32
        let s_47_5: u32 = fn_state.u_33328;
        // D s_47_6: cast zx s_47_5 -> bv
        let s_47_6: Bits = Bits::new(s_47_5 as u128, 32u16);
        // D s_47_7: bit-extract s_47_6 s_47_3 s_47_4
        let s_47_7: Bits = (Bits::new(
            ((s_47_6) >> (s_47_3)).value(),
            u16::try_from(s_47_4).unwrap(),
        ));
        // D s_47_8: cast reint s_47_7 -> u8
        let s_47_8: u8 = (s_47_7.value() as u8);
        // C s_47_9: const #23s : i
        let s_47_9: i128 = 23;
        // C s_47_10: const #1s : i
        let s_47_10: i128 = 1;
        // D s_47_11: read-var u#33328:u32
        let s_47_11: u32 = fn_state.u_33328;
        // D s_47_12: cast zx s_47_11 -> bv
        let s_47_12: Bits = Bits::new(s_47_11 as u128, 32u16);
        // D s_47_13: bit-extract s_47_12 s_47_9 s_47_10
        let s_47_13: Bits = (Bits::new(
            ((s_47_12) >> (s_47_9)).value(),
            u16::try_from(s_47_10).unwrap(),
        ));
        // D s_47_14: cast reint s_47_13 -> u8
        let s_47_14: bool = ((s_47_13.value()) != 0);
        // C s_47_15: const #16s : i
        let s_47_15: i128 = 16;
        // C s_47_16: const #4s : i
        let s_47_16: i128 = 4;
        // D s_47_17: read-var u#33328:u32
        let s_47_17: u32 = fn_state.u_33328;
        // D s_47_18: cast zx s_47_17 -> bv
        let s_47_18: Bits = Bits::new(s_47_17 as u128, 32u16);
        // D s_47_19: bit-extract s_47_18 s_47_15 s_47_16
        let s_47_19: Bits = (Bits::new(
            ((s_47_18) >> (s_47_15)).value(),
            u16::try_from(s_47_16).unwrap(),
        ));
        // D s_47_20: cast reint s_47_19 -> u8
        let s_47_20: u8 = (s_47_19.value() as u8);
        // C s_47_21: const #12s : i
        let s_47_21: i128 = 12;
        // C s_47_22: const #4s : i
        let s_47_22: i128 = 4;
        // D s_47_23: read-var u#33328:u32
        let s_47_23: u32 = fn_state.u_33328;
        // D s_47_24: cast zx s_47_23 -> bv
        let s_47_24: Bits = Bits::new(s_47_23 as u128, 32u16);
        // D s_47_25: bit-extract s_47_24 s_47_21 s_47_22
        let s_47_25: Bits = (Bits::new(
            ((s_47_24) >> (s_47_21)).value(),
            u16::try_from(s_47_22).unwrap(),
        ));
        // D s_47_26: cast reint s_47_25 -> u8
        let s_47_26: u8 = (s_47_25.value() as u8);
        // C s_47_27: const #0s : i
        let s_47_27: i128 = 0;
        // C s_47_28: const #12s : i
        let s_47_28: i128 = 12;
        // D s_47_29: read-var u#33328:u32
        let s_47_29: u32 = fn_state.u_33328;
        // D s_47_30: cast zx s_47_29 -> bv
        let s_47_30: Bits = Bits::new(s_47_29 as u128, 32u16);
        // D s_47_31: bit-extract s_47_30 s_47_27 s_47_28
        let s_47_31: Bits = (Bits::new(
            ((s_47_30) >> (s_47_27)).value(),
            u16::try_from(s_47_28).unwrap(),
        ));
        // D s_47_32: cast reint s_47_31 -> u12
        let s_47_32: u16 = (s_47_31.value() as u16);
        // D s_47_33: call decode_aarch32_instrs_STRBT_A1enc_A_txt(s_47_8, s_47_14, s_47_20, s_47_26, s_47_32)
        let s_47_33: () = decode_aarch32_instrs_STRBT_A1enc_A_txt(
            state,
            tracer,
            s_47_8,
            s_47_14,
            s_47_20,
            s_47_26,
            s_47_32,
        );
        // N s_47_34: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var merge#var.1:struct
        let s_48_0: u32 = fn_state.merge_var._1;
        // D s_48_1: write-var u#33335 <= s_48_0
        fn_state.u_33335 = s_48_0;
        // C s_48_2: const #25s : i
        let s_48_2: i128 = 25;
        // D s_48_3: read-var u#33335:u32
        let s_48_3: u32 = fn_state.u_33335;
        // D s_48_4: cast zx s_48_3 -> bv
        let s_48_4: Bits = Bits::new(s_48_3 as u128, 32u16);
        // C s_48_5: const #1s : i64
        let s_48_5: i64 = 1;
        // C s_48_6: cast zx s_48_5 -> i
        let s_48_6: i128 = (i128::try_from(s_48_5).unwrap());
        // C s_48_7: const #2s : i
        let s_48_7: i128 = 2;
        // C s_48_8: add s_48_7 s_48_6
        let s_48_8: i128 = (s_48_7 + s_48_6);
        // D s_48_9: bit-extract s_48_4 s_48_2 s_48_8
        let s_48_9: Bits = (Bits::new(
            ((s_48_4) >> (s_48_2)).value(),
            u16::try_from(s_48_8).unwrap(),
        ));
        // D s_48_10: cast reint s_48_9 -> u8
        let s_48_10: u8 = (s_48_9.value() as u8);
        // D s_48_11: cast zx s_48_10 -> bv
        let s_48_11: Bits = Bits::new(s_48_10 as u128, 3u16);
        // C s_48_12: const #2u : u8
        let s_48_12: u8 = 2;
        // C s_48_13: cast zx s_48_12 -> bv
        let s_48_13: Bits = Bits::new(s_48_12 as u128, 3u16);
        // D s_48_14: cmp-eq s_48_11 s_48_13
        let s_48_14: bool = ((s_48_11) == (s_48_13));
        // N s_48_15: branch s_48_14 b70 b49
        if s_48_14 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#408770 <= s_49_0
        fn_state.gs_408770 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#408770:u8
        let s_50_0: bool = fn_state.gs_408770;
        // N s_50_1: branch s_50_0 b66 b51
        if s_50_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#408775 <= s_51_0
        fn_state.gs_408775 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#408775:u8
        let s_52_0: bool = fn_state.gs_408775;
        // D s_52_1: not s_52_0
        let s_52_1: bool = !s_52_0;
        // N s_52_2: branch s_52_1 b54 b53
        if s_52_1 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #3224s : i
        let s_53_0: i128 = 3224;
        // C s_53_1: const #14696u : u32
        let s_53_1: u32 = 14696;
        // N s_53_2: write-reg s_53_1 <= s_53_0
        let s_53_2: () = {
            state.write_register::<i128>(s_53_1 as isize, s_53_0);
            tracer.write_register(s_53_1 as isize, s_53_0);
        };
        // C s_53_3: const #28s : i
        let s_53_3: i128 = 28;
        // C s_53_4: const #4s : i
        let s_53_4: i128 = 4;
        // D s_53_5: read-var u#33335:u32
        let s_53_5: u32 = fn_state.u_33335;
        // D s_53_6: cast zx s_53_5 -> bv
        let s_53_6: Bits = Bits::new(s_53_5 as u128, 32u16);
        // D s_53_7: bit-extract s_53_6 s_53_3 s_53_4
        let s_53_7: Bits = (Bits::new(
            ((s_53_6) >> (s_53_3)).value(),
            u16::try_from(s_53_4).unwrap(),
        ));
        // D s_53_8: cast reint s_53_7 -> u8
        let s_53_8: u8 = (s_53_7.value() as u8);
        // C s_53_9: const #24s : i
        let s_53_9: i128 = 24;
        // C s_53_10: const #1s : i
        let s_53_10: i128 = 1;
        // D s_53_11: read-var u#33335:u32
        let s_53_11: u32 = fn_state.u_33335;
        // D s_53_12: cast zx s_53_11 -> bv
        let s_53_12: Bits = Bits::new(s_53_11 as u128, 32u16);
        // D s_53_13: bit-extract s_53_12 s_53_9 s_53_10
        let s_53_13: Bits = (Bits::new(
            ((s_53_12) >> (s_53_9)).value(),
            u16::try_from(s_53_10).unwrap(),
        ));
        // D s_53_14: cast reint s_53_13 -> u8
        let s_53_14: bool = ((s_53_13.value()) != 0);
        // C s_53_15: const #23s : i
        let s_53_15: i128 = 23;
        // C s_53_16: const #1s : i
        let s_53_16: i128 = 1;
        // D s_53_17: read-var u#33335:u32
        let s_53_17: u32 = fn_state.u_33335;
        // D s_53_18: cast zx s_53_17 -> bv
        let s_53_18: Bits = Bits::new(s_53_17 as u128, 32u16);
        // D s_53_19: bit-extract s_53_18 s_53_15 s_53_16
        let s_53_19: Bits = (Bits::new(
            ((s_53_18) >> (s_53_15)).value(),
            u16::try_from(s_53_16).unwrap(),
        ));
        // D s_53_20: cast reint s_53_19 -> u8
        let s_53_20: bool = ((s_53_19.value()) != 0);
        // C s_53_21: const #21s : i
        let s_53_21: i128 = 21;
        // C s_53_22: const #1s : i
        let s_53_22: i128 = 1;
        // D s_53_23: read-var u#33335:u32
        let s_53_23: u32 = fn_state.u_33335;
        // D s_53_24: cast zx s_53_23 -> bv
        let s_53_24: Bits = Bits::new(s_53_23 as u128, 32u16);
        // D s_53_25: bit-extract s_53_24 s_53_21 s_53_22
        let s_53_25: Bits = (Bits::new(
            ((s_53_24) >> (s_53_21)).value(),
            u16::try_from(s_53_22).unwrap(),
        ));
        // D s_53_26: cast reint s_53_25 -> u8
        let s_53_26: bool = ((s_53_25.value()) != 0);
        // C s_53_27: const #16s : i
        let s_53_27: i128 = 16;
        // C s_53_28: const #4s : i
        let s_53_28: i128 = 4;
        // D s_53_29: read-var u#33335:u32
        let s_53_29: u32 = fn_state.u_33335;
        // D s_53_30: cast zx s_53_29 -> bv
        let s_53_30: Bits = Bits::new(s_53_29 as u128, 32u16);
        // D s_53_31: bit-extract s_53_30 s_53_27 s_53_28
        let s_53_31: Bits = (Bits::new(
            ((s_53_30) >> (s_53_27)).value(),
            u16::try_from(s_53_28).unwrap(),
        ));
        // D s_53_32: cast reint s_53_31 -> u8
        let s_53_32: u8 = (s_53_31.value() as u8);
        // C s_53_33: const #12s : i
        let s_53_33: i128 = 12;
        // C s_53_34: const #4s : i
        let s_53_34: i128 = 4;
        // D s_53_35: read-var u#33335:u32
        let s_53_35: u32 = fn_state.u_33335;
        // D s_53_36: cast zx s_53_35 -> bv
        let s_53_36: Bits = Bits::new(s_53_35 as u128, 32u16);
        // D s_53_37: bit-extract s_53_36 s_53_33 s_53_34
        let s_53_37: Bits = (Bits::new(
            ((s_53_36) >> (s_53_33)).value(),
            u16::try_from(s_53_34).unwrap(),
        ));
        // D s_53_38: cast reint s_53_37 -> u8
        let s_53_38: u8 = (s_53_37.value() as u8);
        // C s_53_39: const #0s : i
        let s_53_39: i128 = 0;
        // C s_53_40: const #12s : i
        let s_53_40: i128 = 12;
        // D s_53_41: read-var u#33335:u32
        let s_53_41: u32 = fn_state.u_33335;
        // D s_53_42: cast zx s_53_41 -> bv
        let s_53_42: Bits = Bits::new(s_53_41 as u128, 32u16);
        // D s_53_43: bit-extract s_53_42 s_53_39 s_53_40
        let s_53_43: Bits = (Bits::new(
            ((s_53_42) >> (s_53_39)).value(),
            u16::try_from(s_53_40).unwrap(),
        ));
        // D s_53_44: cast reint s_53_43 -> u12
        let s_53_44: u16 = (s_53_43.value() as u16);
        // D s_53_45: call decode_aarch32_instrs_STR_i_A1enc_A_txt(s_53_8, s_53_14, s_53_20, s_53_26, s_53_32, s_53_38, s_53_44)
        let s_53_45: () = decode_aarch32_instrs_STR_i_A1enc_A_txt(
            state,
            tracer,
            s_53_8,
            s_53_14,
            s_53_20,
            s_53_26,
            s_53_32,
            s_53_38,
            s_53_44,
        );
        // N s_53_46: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var merge#var.1:struct
        let s_54_0: u32 = fn_state.merge_var._1;
        // D s_54_1: write-var u#33344 <= s_54_0
        fn_state.u_33344 = s_54_0;
        // C s_54_2: const #24s : i
        let s_54_2: i128 = 24;
        // D s_54_3: read-var u#33344:u32
        let s_54_3: u32 = fn_state.u_33344;
        // D s_54_4: cast zx s_54_3 -> bv
        let s_54_4: Bits = Bits::new(s_54_3 as u128, 32u16);
        // C s_54_5: const #1s : i64
        let s_54_5: i64 = 1;
        // C s_54_6: cast zx s_54_5 -> i
        let s_54_6: i128 = (i128::try_from(s_54_5).unwrap());
        // C s_54_7: const #3s : i
        let s_54_7: i128 = 3;
        // C s_54_8: add s_54_7 s_54_6
        let s_54_8: i128 = (s_54_7 + s_54_6);
        // D s_54_9: bit-extract s_54_4 s_54_2 s_54_8
        let s_54_9: Bits = (Bits::new(
            ((s_54_4) >> (s_54_2)).value(),
            u16::try_from(s_54_8).unwrap(),
        ));
        // D s_54_10: cast reint s_54_9 -> u8
        let s_54_10: u8 = (s_54_9.value() as u8);
        // D s_54_11: cast zx s_54_10 -> bv
        let s_54_11: Bits = Bits::new(s_54_10 as u128, 4u16);
        // C s_54_12: const #4u : u8
        let s_54_12: u8 = 4;
        // C s_54_13: cast zx s_54_12 -> bv
        let s_54_13: Bits = Bits::new(s_54_12 as u128, 4u16);
        // D s_54_14: cmp-eq s_54_11 s_54_13
        let s_54_14: bool = ((s_54_11) == (s_54_13));
        // N s_54_15: branch s_54_14 b65 b55
        if s_54_14 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#408797 <= s_55_0
        fn_state.gs_408797 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#408797:u8
        let s_56_0: bool = fn_state.gs_408797;
        // N s_56_1: branch s_56_0 b61 b57
        if s_56_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#408802 <= s_57_0
        fn_state.gs_408802 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#408802:u8
        let s_58_0: bool = fn_state.gs_408802;
        // D s_58_1: not s_58_0
        let s_58_1: bool = !s_58_0;
        // N s_58_2: branch s_58_1 b60 b59
        if s_58_1 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #3232s : i
        let s_59_0: i128 = 3232;
        // C s_59_1: const #14696u : u32
        let s_59_1: u32 = 14696;
        // N s_59_2: write-reg s_59_1 <= s_59_0
        let s_59_2: () = {
            state.write_register::<i128>(s_59_1 as isize, s_59_0);
            tracer.write_register(s_59_1 as isize, s_59_0);
        };
        // C s_59_3: const #28s : i
        let s_59_3: i128 = 28;
        // C s_59_4: const #4s : i
        let s_59_4: i128 = 4;
        // D s_59_5: read-var u#33344:u32
        let s_59_5: u32 = fn_state.u_33344;
        // D s_59_6: cast zx s_59_5 -> bv
        let s_59_6: Bits = Bits::new(s_59_5 as u128, 32u16);
        // D s_59_7: bit-extract s_59_6 s_59_3 s_59_4
        let s_59_7: Bits = (Bits::new(
            ((s_59_6) >> (s_59_3)).value(),
            u16::try_from(s_59_4).unwrap(),
        ));
        // D s_59_8: cast reint s_59_7 -> u8
        let s_59_8: u8 = (s_59_7.value() as u8);
        // C s_59_9: const #23s : i
        let s_59_9: i128 = 23;
        // C s_59_10: const #1s : i
        let s_59_10: i128 = 1;
        // D s_59_11: read-var u#33344:u32
        let s_59_11: u32 = fn_state.u_33344;
        // D s_59_12: cast zx s_59_11 -> bv
        let s_59_12: Bits = Bits::new(s_59_11 as u128, 32u16);
        // D s_59_13: bit-extract s_59_12 s_59_9 s_59_10
        let s_59_13: Bits = (Bits::new(
            ((s_59_12) >> (s_59_9)).value(),
            u16::try_from(s_59_10).unwrap(),
        ));
        // D s_59_14: cast reint s_59_13 -> u8
        let s_59_14: bool = ((s_59_13.value()) != 0);
        // C s_59_15: const #16s : i
        let s_59_15: i128 = 16;
        // C s_59_16: const #4s : i
        let s_59_16: i128 = 4;
        // D s_59_17: read-var u#33344:u32
        let s_59_17: u32 = fn_state.u_33344;
        // D s_59_18: cast zx s_59_17 -> bv
        let s_59_18: Bits = Bits::new(s_59_17 as u128, 32u16);
        // D s_59_19: bit-extract s_59_18 s_59_15 s_59_16
        let s_59_19: Bits = (Bits::new(
            ((s_59_18) >> (s_59_15)).value(),
            u16::try_from(s_59_16).unwrap(),
        ));
        // D s_59_20: cast reint s_59_19 -> u8
        let s_59_20: u8 = (s_59_19.value() as u8);
        // C s_59_21: const #12s : i
        let s_59_21: i128 = 12;
        // C s_59_22: const #4s : i
        let s_59_22: i128 = 4;
        // D s_59_23: read-var u#33344:u32
        let s_59_23: u32 = fn_state.u_33344;
        // D s_59_24: cast zx s_59_23 -> bv
        let s_59_24: Bits = Bits::new(s_59_23 as u128, 32u16);
        // D s_59_25: bit-extract s_59_24 s_59_21 s_59_22
        let s_59_25: Bits = (Bits::new(
            ((s_59_24) >> (s_59_21)).value(),
            u16::try_from(s_59_22).unwrap(),
        ));
        // D s_59_26: cast reint s_59_25 -> u8
        let s_59_26: u8 = (s_59_25.value() as u8);
        // C s_59_27: const #0s : i
        let s_59_27: i128 = 0;
        // C s_59_28: const #12s : i
        let s_59_28: i128 = 12;
        // D s_59_29: read-var u#33344:u32
        let s_59_29: u32 = fn_state.u_33344;
        // D s_59_30: cast zx s_59_29 -> bv
        let s_59_30: Bits = Bits::new(s_59_29 as u128, 32u16);
        // D s_59_31: bit-extract s_59_30 s_59_27 s_59_28
        let s_59_31: Bits = (Bits::new(
            ((s_59_30) >> (s_59_27)).value(),
            u16::try_from(s_59_28).unwrap(),
        ));
        // D s_59_32: cast reint s_59_31 -> u12
        let s_59_32: u16 = (s_59_31.value() as u16);
        // D s_59_33: call decode_aarch32_instrs_STRT_A1enc_A_txt(s_59_8, s_59_14, s_59_20, s_59_26, s_59_32)
        let s_59_33: () = decode_aarch32_instrs_STRT_A1enc_A_txt(
            state,
            tracer,
            s_59_8,
            s_59_14,
            s_59_20,
            s_59_26,
            s_59_32,
        );
        // N s_59_34: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_60_0: panic
        panic!("{:?}", ());
        // N s_60_1: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #28s : i
        let s_61_0: i128 = 28;
        // C s_61_1: const #4s : i
        let s_61_1: i128 = 4;
        // D s_61_2: read-var u#33344:u32
        let s_61_2: u32 = fn_state.u_33344;
        // D s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 32u16);
        // D s_61_4: bit-extract s_61_3 s_61_0 s_61_1
        let s_61_4: Bits = (Bits::new(
            ((s_61_3) >> (s_61_0)).value(),
            u16::try_from(s_61_1).unwrap(),
        ));
        // D s_61_5: cast reint s_61_4 -> u8
        let s_61_5: u8 = (s_61_4.value() as u8);
        // D s_61_6: cast zx s_61_5 -> bv
        let s_61_6: Bits = Bits::new(s_61_5 as u128, 4u16);
        // C s_61_7: const #15u : u8
        let s_61_7: u8 = 15;
        // C s_61_8: cast zx s_61_7 -> bv
        let s_61_8: Bits = Bits::new(s_61_7 as u128, 4u16);
        // D s_61_9: cmp-ne s_61_6 s_61_8
        let s_61_9: bool = ((s_61_6) != (s_61_8));
        // N s_61_10: branch s_61_9 b64 b62
        if s_61_9 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#408801 <= s_62_0
        fn_state.gs_408801 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#408801:u8
        let s_63_0: bool = fn_state.gs_408801;
        // D s_63_1: write-var gs#408802 <= s_63_0
        fn_state.gs_408802 = s_63_0;
        // N s_63_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #3232s : i
        let s_64_0: i128 = 3232;
        // C s_64_1: const #14696u : u32
        let s_64_1: u32 = 14696;
        // D s_64_2: read-reg s_64_1:i
        let s_64_2: i128 = {
            let value = state.read_register::<i128>(s_64_1 as isize);
            tracer.read_register(s_64_1 as isize, value);
            value
        };
        // D s_64_3: cmp-lt s_64_2 s_64_0
        let s_64_3: bool = ((s_64_2) < (s_64_0));
        // D s_64_4: write-var gs#408801 <= s_64_3
        fn_state.gs_408801 = s_64_3;
        // N s_64_5: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #20s : i
        let s_65_0: i128 = 20;
        // D s_65_1: read-var u#33344:u32
        let s_65_1: u32 = fn_state.u_33344;
        // D s_65_2: cast zx s_65_1 -> bv
        let s_65_2: Bits = Bits::new(s_65_1 as u128, 32u16);
        // C s_65_3: const #1s : i64
        let s_65_3: i64 = 1;
        // C s_65_4: cast zx s_65_3 -> i
        let s_65_4: i128 = (i128::try_from(s_65_3).unwrap());
        // C s_65_5: const #2s : i
        let s_65_5: i128 = 2;
        // C s_65_6: add s_65_5 s_65_4
        let s_65_6: i128 = (s_65_5 + s_65_4);
        // D s_65_7: bit-extract s_65_2 s_65_0 s_65_6
        let s_65_7: Bits = (Bits::new(
            ((s_65_2) >> (s_65_0)).value(),
            u16::try_from(s_65_6).unwrap(),
        ));
        // D s_65_8: cast reint s_65_7 -> u8
        let s_65_8: u8 = (s_65_7.value() as u8);
        // D s_65_9: cast zx s_65_8 -> bv
        let s_65_9: Bits = Bits::new(s_65_8 as u128, 3u16);
        // C s_65_10: const #2u : u8
        let s_65_10: u8 = 2;
        // C s_65_11: cast zx s_65_10 -> bv
        let s_65_11: Bits = Bits::new(s_65_10 as u128, 3u16);
        // D s_65_12: cmp-eq s_65_9 s_65_11
        let s_65_12: bool = ((s_65_9) == (s_65_11));
        // D s_65_13: write-var gs#408797 <= s_65_12
        fn_state.gs_408797 = s_65_12;
        // N s_65_14: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #28s : i
        let s_66_0: i128 = 28;
        // C s_66_1: const #4s : i
        let s_66_1: i128 = 4;
        // D s_66_2: read-var u#33335:u32
        let s_66_2: u32 = fn_state.u_33335;
        // D s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 32u16);
        // D s_66_4: bit-extract s_66_3 s_66_0 s_66_1
        let s_66_4: Bits = (Bits::new(
            ((s_66_3) >> (s_66_0)).value(),
            u16::try_from(s_66_1).unwrap(),
        ));
        // D s_66_5: cast reint s_66_4 -> u8
        let s_66_5: u8 = (s_66_4.value() as u8);
        // D s_66_6: cast zx s_66_5 -> bv
        let s_66_6: Bits = Bits::new(s_66_5 as u128, 4u16);
        // C s_66_7: const #15u : u8
        let s_66_7: u8 = 15;
        // C s_66_8: cast zx s_66_7 -> bv
        let s_66_8: Bits = Bits::new(s_66_7 as u128, 4u16);
        // D s_66_9: cmp-ne s_66_6 s_66_8
        let s_66_9: bool = ((s_66_6) != (s_66_8));
        // N s_66_10: branch s_66_9 b69 b67
        if s_66_9 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#408774 <= s_67_0
        fn_state.gs_408774 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#408774:u8
        let s_68_0: bool = fn_state.gs_408774;
        // D s_68_1: write-var gs#408775 <= s_68_0
        fn_state.gs_408775 = s_68_0;
        // N s_68_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #3224s : i
        let s_69_0: i128 = 3224;
        // C s_69_1: const #14696u : u32
        let s_69_1: u32 = 14696;
        // D s_69_2: read-reg s_69_1:i
        let s_69_2: i128 = {
            let value = state.read_register::<i128>(s_69_1 as isize);
            tracer.read_register(s_69_1 as isize, value);
            value
        };
        // D s_69_3: cmp-lt s_69_2 s_69_0
        let s_69_3: bool = ((s_69_2) < (s_69_0));
        // D s_69_4: write-var gs#408774 <= s_69_3
        fn_state.gs_408774 = s_69_3;
        // N s_69_5: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #22s : i
        let s_70_0: i128 = 22;
        // D s_70_1: read-var u#33335:u32
        let s_70_1: u32 = fn_state.u_33335;
        // D s_70_2: cast zx s_70_1 -> bv
        let s_70_2: Bits = Bits::new(s_70_1 as u128, 32u16);
        // C s_70_3: const #1s : i64
        let s_70_3: i64 = 1;
        // C s_70_4: cast zx s_70_3 -> i
        let s_70_4: i128 = (i128::try_from(s_70_3).unwrap());
        // C s_70_5: const #0s : i
        let s_70_5: i128 = 0;
        // C s_70_6: add s_70_5 s_70_4
        let s_70_6: i128 = (s_70_5 + s_70_4);
        // D s_70_7: bit-extract s_70_2 s_70_0 s_70_6
        let s_70_7: Bits = (Bits::new(
            ((s_70_2) >> (s_70_0)).value(),
            u16::try_from(s_70_6).unwrap(),
        ));
        // D s_70_8: cast reint s_70_7 -> u8
        let s_70_8: bool = ((s_70_7.value()) != 0);
        // D s_70_9: cast zx s_70_8 -> bv
        let s_70_9: Bits = Bits::new(s_70_8 as u128, 1u16);
        // C s_70_10: const #0u : u8
        let s_70_10: bool = false;
        // C s_70_11: cast zx s_70_10 -> bv
        let s_70_11: Bits = Bits::new(s_70_10 as u128, 1u16);
        // D s_70_12: cmp-eq s_70_9 s_70_11
        let s_70_12: bool = ((s_70_9) == (s_70_11));
        // N s_70_13: branch s_70_12 b73 b71
        if s_70_12 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#408769 <= s_71_0
        fn_state.gs_408769 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#408769:u8
        let s_72_0: bool = fn_state.gs_408769;
        // D s_72_1: write-var gs#408770 <= s_72_0
        fn_state.gs_408770 = s_72_0;
        // N s_72_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #20s : i
        let s_73_0: i128 = 20;
        // D s_73_1: read-var u#33335:u32
        let s_73_1: u32 = fn_state.u_33335;
        // D s_73_2: cast zx s_73_1 -> bv
        let s_73_2: Bits = Bits::new(s_73_1 as u128, 32u16);
        // C s_73_3: const #1s : i64
        let s_73_3: i64 = 1;
        // C s_73_4: cast zx s_73_3 -> i
        let s_73_4: i128 = (i128::try_from(s_73_3).unwrap());
        // C s_73_5: const #0s : i
        let s_73_5: i128 = 0;
        // C s_73_6: add s_73_5 s_73_4
        let s_73_6: i128 = (s_73_5 + s_73_4);
        // D s_73_7: bit-extract s_73_2 s_73_0 s_73_6
        let s_73_7: Bits = (Bits::new(
            ((s_73_2) >> (s_73_0)).value(),
            u16::try_from(s_73_6).unwrap(),
        ));
        // D s_73_8: cast reint s_73_7 -> u8
        let s_73_8: bool = ((s_73_7.value()) != 0);
        // D s_73_9: cast zx s_73_8 -> bv
        let s_73_9: Bits = Bits::new(s_73_8 as u128, 1u16);
        // C s_73_10: const #0u : u8
        let s_73_10: bool = false;
        // C s_73_11: cast zx s_73_10 -> bv
        let s_73_11: Bits = Bits::new(s_73_10 as u128, 1u16);
        // D s_73_12: cmp-eq s_73_9 s_73_11
        let s_73_12: bool = ((s_73_9) == (s_73_11));
        // D s_73_13: write-var gs#408769 <= s_73_12
        fn_state.gs_408769 = s_73_12;
        // N s_73_14: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #28s : i
        let s_74_0: i128 = 28;
        // C s_74_1: const #4s : i
        let s_74_1: i128 = 4;
        // D s_74_2: read-var u#33328:u32
        let s_74_2: u32 = fn_state.u_33328;
        // D s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 32u16);
        // D s_74_4: bit-extract s_74_3 s_74_0 s_74_1
        let s_74_4: Bits = (Bits::new(
            ((s_74_3) >> (s_74_0)).value(),
            u16::try_from(s_74_1).unwrap(),
        ));
        // D s_74_5: cast reint s_74_4 -> u8
        let s_74_5: u8 = (s_74_4.value() as u8);
        // D s_74_6: cast zx s_74_5 -> bv
        let s_74_6: Bits = Bits::new(s_74_5 as u128, 4u16);
        // C s_74_7: const #15u : u8
        let s_74_7: u8 = 15;
        // C s_74_8: cast zx s_74_7 -> bv
        let s_74_8: Bits = Bits::new(s_74_7 as u128, 4u16);
        // D s_74_9: cmp-ne s_74_6 s_74_8
        let s_74_9: bool = ((s_74_6) != (s_74_8));
        // N s_74_10: branch s_74_9 b77 b75
        if s_74_9 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var gs#408748 <= s_75_0
        fn_state.gs_408748 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#408748:u8
        let s_76_0: bool = fn_state.gs_408748;
        // D s_76_1: write-var gs#408749 <= s_76_0
        fn_state.gs_408749 = s_76_0;
        // N s_76_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #3200s : i
        let s_77_0: i128 = 3200;
        // C s_77_1: const #14696u : u32
        let s_77_1: u32 = 14696;
        // D s_77_2: read-reg s_77_1:i
        let s_77_2: i128 = {
            let value = state.read_register::<i128>(s_77_1 as isize);
            tracer.read_register(s_77_1 as isize, value);
            value
        };
        // D s_77_3: cmp-lt s_77_2 s_77_0
        let s_77_3: bool = ((s_77_2) < (s_77_0));
        // D s_77_4: write-var gs#408748 <= s_77_3
        fn_state.gs_408748 = s_77_3;
        // N s_77_5: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #20s : i
        let s_78_0: i128 = 20;
        // D s_78_1: read-var u#33328:u32
        let s_78_1: u32 = fn_state.u_33328;
        // D s_78_2: cast zx s_78_1 -> bv
        let s_78_2: Bits = Bits::new(s_78_1 as u128, 32u16);
        // C s_78_3: const #1s : i64
        let s_78_3: i64 = 1;
        // C s_78_4: cast zx s_78_3 -> i
        let s_78_4: i128 = (i128::try_from(s_78_3).unwrap());
        // C s_78_5: const #2s : i
        let s_78_5: i128 = 2;
        // C s_78_6: add s_78_5 s_78_4
        let s_78_6: i128 = (s_78_5 + s_78_4);
        // D s_78_7: bit-extract s_78_2 s_78_0 s_78_6
        let s_78_7: Bits = (Bits::new(
            ((s_78_2) >> (s_78_0)).value(),
            u16::try_from(s_78_6).unwrap(),
        ));
        // D s_78_8: cast reint s_78_7 -> u8
        let s_78_8: u8 = (s_78_7.value() as u8);
        // D s_78_9: cast zx s_78_8 -> bv
        let s_78_9: Bits = Bits::new(s_78_8 as u128, 3u16);
        // C s_78_10: const #6u : u8
        let s_78_10: u8 = 6;
        // C s_78_11: cast zx s_78_10 -> bv
        let s_78_11: Bits = Bits::new(s_78_10 as u128, 3u16);
        // D s_78_12: cmp-eq s_78_9 s_78_11
        let s_78_12: bool = ((s_78_9) == (s_78_11));
        // D s_78_13: write-var gs#408744 <= s_78_12
        fn_state.gs_408744 = s_78_12;
        // N s_78_14: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #28s : i
        let s_79_0: i128 = 28;
        // C s_79_1: const #4s : i
        let s_79_1: i128 = 4;
        // D s_79_2: read-var u#33319:u32
        let s_79_2: u32 = fn_state.u_33319;
        // D s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 32u16);
        // D s_79_4: bit-extract s_79_3 s_79_0 s_79_1
        let s_79_4: Bits = (Bits::new(
            ((s_79_3) >> (s_79_0)).value(),
            u16::try_from(s_79_1).unwrap(),
        ));
        // D s_79_5: cast reint s_79_4 -> u8
        let s_79_5: u8 = (s_79_4.value() as u8);
        // D s_79_6: cast zx s_79_5 -> bv
        let s_79_6: Bits = Bits::new(s_79_5 as u128, 4u16);
        // C s_79_7: const #15u : u8
        let s_79_7: u8 = 15;
        // C s_79_8: cast zx s_79_7 -> bv
        let s_79_8: Bits = Bits::new(s_79_7 as u128, 4u16);
        // D s_79_9: cmp-ne s_79_6 s_79_8
        let s_79_9: bool = ((s_79_6) != (s_79_8));
        // N s_79_10: branch s_79_9 b82 b80
        if s_79_9 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var gs#408721 <= s_80_0
        fn_state.gs_408721 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#408721:u8
        let s_81_0: bool = fn_state.gs_408721;
        // D s_81_1: write-var gs#408722 <= s_81_0
        fn_state.gs_408722 = s_81_0;
        // N s_81_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #3193s : i
        let s_82_0: i128 = 3193;
        // C s_82_1: const #14696u : u32
        let s_82_1: u32 = 14696;
        // D s_82_2: read-reg s_82_1:i
        let s_82_2: i128 = {
            let value = state.read_register::<i128>(s_82_1 as isize);
            tracer.read_register(s_82_1 as isize, value);
            value
        };
        // D s_82_3: cmp-lt s_82_2 s_82_0
        let s_82_3: bool = ((s_82_2) < (s_82_0));
        // D s_82_4: write-var gs#408721 <= s_82_3
        fn_state.gs_408721 = s_82_3;
        // N s_82_5: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #22s : i
        let s_83_0: i128 = 22;
        // D s_83_1: read-var u#33319:u32
        let s_83_1: u32 = fn_state.u_33319;
        // D s_83_2: cast zx s_83_1 -> bv
        let s_83_2: Bits = Bits::new(s_83_1 as u128, 32u16);
        // C s_83_3: const #1s : i64
        let s_83_3: i64 = 1;
        // C s_83_4: cast zx s_83_3 -> i
        let s_83_4: i128 = (i128::try_from(s_83_3).unwrap());
        // C s_83_5: const #0s : i
        let s_83_5: i128 = 0;
        // C s_83_6: add s_83_5 s_83_4
        let s_83_6: i128 = (s_83_5 + s_83_4);
        // D s_83_7: bit-extract s_83_2 s_83_0 s_83_6
        let s_83_7: Bits = (Bits::new(
            ((s_83_2) >> (s_83_0)).value(),
            u16::try_from(s_83_6).unwrap(),
        ));
        // D s_83_8: cast reint s_83_7 -> u8
        let s_83_8: bool = ((s_83_7.value()) != 0);
        // D s_83_9: cast zx s_83_8 -> bv
        let s_83_9: Bits = Bits::new(s_83_8 as u128, 1u16);
        // C s_83_10: const #1u : u8
        let s_83_10: bool = true;
        // C s_83_11: cast zx s_83_10 -> bv
        let s_83_11: Bits = Bits::new(s_83_10 as u128, 1u16);
        // D s_83_12: cmp-eq s_83_9 s_83_11
        let s_83_12: bool = ((s_83_9) == (s_83_11));
        // N s_83_13: branch s_83_12 b86 b84
        if s_83_12 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #0u : u8
        let s_84_0: bool = false;
        // D s_84_1: write-var gs#408716 <= s_84_0
        fn_state.gs_408716 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#408716:u8
        let s_85_0: bool = fn_state.gs_408716;
        // D s_85_1: write-var gs#408717 <= s_85_0
        fn_state.gs_408717 = s_85_0;
        // N s_85_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #20s : i
        let s_86_0: i128 = 20;
        // D s_86_1: read-var u#33319:u32
        let s_86_1: u32 = fn_state.u_33319;
        // D s_86_2: cast zx s_86_1 -> bv
        let s_86_2: Bits = Bits::new(s_86_1 as u128, 32u16);
        // C s_86_3: const #1s : i64
        let s_86_3: i64 = 1;
        // C s_86_4: cast zx s_86_3 -> i
        let s_86_4: i128 = (i128::try_from(s_86_3).unwrap());
        // C s_86_5: const #0s : i
        let s_86_5: i128 = 0;
        // C s_86_6: add s_86_5 s_86_4
        let s_86_6: i128 = (s_86_5 + s_86_4);
        // D s_86_7: bit-extract s_86_2 s_86_0 s_86_6
        let s_86_7: Bits = (Bits::new(
            ((s_86_2) >> (s_86_0)).value(),
            u16::try_from(s_86_6).unwrap(),
        ));
        // D s_86_8: cast reint s_86_7 -> u8
        let s_86_8: bool = ((s_86_7.value()) != 0);
        // D s_86_9: cast zx s_86_8 -> bv
        let s_86_9: Bits = Bits::new(s_86_8 as u128, 1u16);
        // C s_86_10: const #0u : u8
        let s_86_10: bool = false;
        // C s_86_11: cast zx s_86_10 -> bv
        let s_86_11: Bits = Bits::new(s_86_10 as u128, 1u16);
        // D s_86_12: cmp-eq s_86_9 s_86_11
        let s_86_12: bool = ((s_86_9) == (s_86_11));
        // D s_86_13: write-var gs#408716 <= s_86_12
        fn_state.gs_408716 = s_86_12;
        // N s_86_14: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #28s : i
        let s_87_0: i128 = 28;
        // C s_87_1: const #4s : i
        let s_87_1: i128 = 4;
        // D s_87_2: read-var u#33312:u32
        let s_87_2: u32 = fn_state.u_33312;
        // D s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 32u16);
        // D s_87_4: bit-extract s_87_3 s_87_0 s_87_1
        let s_87_4: Bits = (Bits::new(
            ((s_87_3) >> (s_87_0)).value(),
            u16::try_from(s_87_1).unwrap(),
        ));
        // D s_87_5: cast reint s_87_4 -> u8
        let s_87_5: u8 = (s_87_4.value() as u8);
        // D s_87_6: cast zx s_87_5 -> bv
        let s_87_6: Bits = Bits::new(s_87_5 as u128, 4u16);
        // C s_87_7: const #15u : u8
        let s_87_7: u8 = 15;
        // C s_87_8: cast zx s_87_7 -> bv
        let s_87_8: Bits = Bits::new(s_87_7 as u128, 4u16);
        // D s_87_9: cmp-ne s_87_6 s_87_8
        let s_87_9: bool = ((s_87_6) != (s_87_8));
        // N s_87_10: branch s_87_9 b90 b88
        if s_87_9 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#408695 <= s_88_0
        fn_state.gs_408695 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#408695:u8
        let s_89_0: bool = fn_state.gs_408695;
        // D s_89_1: write-var gs#408696 <= s_89_0
        fn_state.gs_408696 = s_89_0;
        // N s_89_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #2993s : i
        let s_90_0: i128 = 2993;
        // C s_90_1: const #14696u : u32
        let s_90_1: u32 = 14696;
        // D s_90_2: read-reg s_90_1:i
        let s_90_2: i128 = {
            let value = state.read_register::<i128>(s_90_1 as isize);
            tracer.read_register(s_90_1 as isize, value);
            value
        };
        // D s_90_3: cmp-lt s_90_2 s_90_0
        let s_90_3: bool = ((s_90_2) < (s_90_0));
        // D s_90_4: write-var gs#408695 <= s_90_3
        fn_state.gs_408695 = s_90_3;
        // N s_90_5: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #20s : i
        let s_91_0: i128 = 20;
        // D s_91_1: read-var u#33312:u32
        let s_91_1: u32 = fn_state.u_33312;
        // D s_91_2: cast zx s_91_1 -> bv
        let s_91_2: Bits = Bits::new(s_91_1 as u128, 32u16);
        // C s_91_3: const #1s : i64
        let s_91_3: i64 = 1;
        // C s_91_4: cast zx s_91_3 -> i
        let s_91_4: i128 = (i128::try_from(s_91_3).unwrap());
        // C s_91_5: const #2s : i
        let s_91_5: i128 = 2;
        // C s_91_6: add s_91_5 s_91_4
        let s_91_6: i128 = (s_91_5 + s_91_4);
        // D s_91_7: bit-extract s_91_2 s_91_0 s_91_6
        let s_91_7: Bits = (Bits::new(
            ((s_91_2) >> (s_91_0)).value(),
            u16::try_from(s_91_6).unwrap(),
        ));
        // D s_91_8: cast reint s_91_7 -> u8
        let s_91_8: u8 = (s_91_7.value() as u8);
        // D s_91_9: cast zx s_91_8 -> bv
        let s_91_9: Bits = Bits::new(s_91_8 as u128, 3u16);
        // C s_91_10: const #3u : u8
        let s_91_10: u8 = 3;
        // C s_91_11: cast zx s_91_10 -> bv
        let s_91_11: Bits = Bits::new(s_91_10 as u128, 3u16);
        // D s_91_12: cmp-eq s_91_9 s_91_11
        let s_91_12: bool = ((s_91_9) == (s_91_11));
        // D s_91_13: write-var gs#408691 <= s_91_12
        fn_state.gs_408691 = s_91_12;
        // N s_91_14: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #28s : i
        let s_92_0: i128 = 28;
        // C s_92_1: const #4s : i
        let s_92_1: i128 = 4;
        // D s_92_2: read-var u#33304:u32
        let s_92_2: u32 = fn_state.u_33304;
        // D s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 32u16);
        // D s_92_4: bit-extract s_92_3 s_92_0 s_92_1
        let s_92_4: Bits = (Bits::new(
            ((s_92_3) >> (s_92_0)).value(),
            u16::try_from(s_92_1).unwrap(),
        ));
        // D s_92_5: cast reint s_92_4 -> u8
        let s_92_5: u8 = (s_92_4.value() as u8);
        // D s_92_6: cast zx s_92_5 -> bv
        let s_92_6: Bits = Bits::new(s_92_5 as u128, 4u16);
        // C s_92_7: const #15u : u8
        let s_92_7: u8 = 15;
        // C s_92_8: cast zx s_92_7 -> bv
        let s_92_8: Bits = Bits::new(s_92_7 as u128, 4u16);
        // D s_92_9: cmp-ne s_92_6 s_92_8
        let s_92_9: bool = ((s_92_6) != (s_92_8));
        // N s_92_10: branch s_92_9 b95 b93
        if s_92_9 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #0u : u8
        let s_93_0: bool = false;
        // D s_93_1: write-var gs#408670 <= s_93_0
        fn_state.gs_408670 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#408670:u8
        let s_94_0: bool = fn_state.gs_408670;
        // D s_94_1: write-var gs#408671 <= s_94_0
        fn_state.gs_408671 = s_94_0;
        // N s_94_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #2965s : i
        let s_95_0: i128 = 2965;
        // C s_95_1: const #14696u : u32
        let s_95_1: u32 = 14696;
        // D s_95_2: read-reg s_95_1:i
        let s_95_2: i128 = {
            let value = state.read_register::<i128>(s_95_1 as isize);
            tracer.read_register(s_95_1 as isize, value);
            value
        };
        // D s_95_3: cmp-lt s_95_2 s_95_0
        let s_95_3: bool = ((s_95_2) < (s_95_0));
        // D s_95_4: write-var gs#408670 <= s_95_3
        fn_state.gs_408670 = s_95_3;
        // N s_95_5: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #22s : i
        let s_96_0: i128 = 22;
        // D s_96_1: read-var u#33304:u32
        let s_96_1: u32 = fn_state.u_33304;
        // D s_96_2: cast zx s_96_1 -> bv
        let s_96_2: Bits = Bits::new(s_96_1 as u128, 32u16);
        // C s_96_3: const #1s : i64
        let s_96_3: i64 = 1;
        // C s_96_4: cast zx s_96_3 -> i
        let s_96_4: i128 = (i128::try_from(s_96_3).unwrap());
        // C s_96_5: const #0s : i
        let s_96_5: i128 = 0;
        // C s_96_6: add s_96_5 s_96_4
        let s_96_6: i128 = (s_96_5 + s_96_4);
        // D s_96_7: bit-extract s_96_2 s_96_0 s_96_6
        let s_96_7: Bits = (Bits::new(
            ((s_96_2) >> (s_96_0)).value(),
            u16::try_from(s_96_6).unwrap(),
        ));
        // D s_96_8: cast reint s_96_7 -> u8
        let s_96_8: bool = ((s_96_7.value()) != 0);
        // D s_96_9: cast zx s_96_8 -> bv
        let s_96_9: Bits = Bits::new(s_96_8 as u128, 1u16);
        // C s_96_10: const #0u : u8
        let s_96_10: bool = false;
        // C s_96_11: cast zx s_96_10 -> bv
        let s_96_11: Bits = Bits::new(s_96_10 as u128, 1u16);
        // D s_96_12: cmp-eq s_96_9 s_96_11
        let s_96_12: bool = ((s_96_9) == (s_96_11));
        // N s_96_13: branch s_96_12 b99 b97
        if s_96_12 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // D s_97_1: write-var gs#408665 <= s_97_0
        fn_state.gs_408665 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#408665:u8
        let s_98_0: bool = fn_state.gs_408665;
        // D s_98_1: write-var gs#408666 <= s_98_0
        fn_state.gs_408666 = s_98_0;
        // N s_98_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #16s : i
        let s_99_0: i128 = 16;
        // D s_99_1: read-var u#33304:u32
        let s_99_1: u32 = fn_state.u_33304;
        // D s_99_2: cast zx s_99_1 -> bv
        let s_99_2: Bits = Bits::new(s_99_1 as u128, 32u16);
        // C s_99_3: const #1s : i64
        let s_99_3: i64 = 1;
        // C s_99_4: cast zx s_99_3 -> i
        let s_99_4: i128 = (i128::try_from(s_99_3).unwrap());
        // C s_99_5: const #4s : i
        let s_99_5: i128 = 4;
        // C s_99_6: add s_99_5 s_99_4
        let s_99_6: i128 = (s_99_5 + s_99_4);
        // D s_99_7: bit-extract s_99_2 s_99_0 s_99_6
        let s_99_7: Bits = (Bits::new(
            ((s_99_2) >> (s_99_0)).value(),
            u16::try_from(s_99_6).unwrap(),
        ));
        // D s_99_8: cast reint s_99_7 -> u8
        let s_99_8: u8 = (s_99_7.value() as u8);
        // D s_99_9: cast zx s_99_8 -> bv
        let s_99_9: Bits = Bits::new(s_99_8 as u128, 5u16);
        // C s_99_10: const #31u : u8
        let s_99_10: u8 = 31;
        // C s_99_11: cast zx s_99_10 -> bv
        let s_99_11: Bits = Bits::new(s_99_10 as u128, 5u16);
        // D s_99_12: cmp-eq s_99_9 s_99_11
        let s_99_12: bool = ((s_99_9) == (s_99_11));
        // D s_99_13: write-var gs#408665 <= s_99_12
        fn_state.gs_408665 = s_99_12;
        // N s_99_14: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #28s : i
        let s_100_0: i128 = 28;
        // C s_100_1: const #4s : i
        let s_100_1: i128 = 4;
        // D s_100_2: read-var u#33295:u32
        let s_100_2: u32 = fn_state.u_33295;
        // D s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 32u16);
        // D s_100_4: bit-extract s_100_3 s_100_0 s_100_1
        let s_100_4: Bits = (Bits::new(
            ((s_100_3) >> (s_100_0)).value(),
            u16::try_from(s_100_1).unwrap(),
        ));
        // D s_100_5: cast reint s_100_4 -> u8
        let s_100_5: u8 = (s_100_4.value() as u8);
        // D s_100_6: cast zx s_100_5 -> bv
        let s_100_6: Bits = Bits::new(s_100_5 as u128, 4u16);
        // C s_100_7: const #15u : u8
        let s_100_7: u8 = 15;
        // C s_100_8: cast zx s_100_7 -> bv
        let s_100_8: Bits = Bits::new(s_100_7 as u128, 4u16);
        // D s_100_9: cmp-ne s_100_6 s_100_8
        let s_100_9: bool = ((s_100_6) != (s_100_8));
        // N s_100_10: branch s_100_9 b103 b101
        if s_100_9 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #0u : u8
        let s_101_0: bool = false;
        // D s_101_1: write-var gs#408640 <= s_101_0
        fn_state.gs_408640 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#408640:u8
        let s_102_0: bool = fn_state.gs_408640;
        // D s_102_1: write-var gs#408641 <= s_102_0
        fn_state.gs_408641 = s_102_0;
        // N s_102_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #2960s : i
        let s_103_0: i128 = 2960;
        // C s_103_1: const #14696u : u32
        let s_103_1: u32 = 14696;
        // D s_103_2: read-reg s_103_1:i
        let s_103_2: i128 = {
            let value = state.read_register::<i128>(s_103_1 as isize);
            tracer.read_register(s_103_1 as isize, value);
            value
        };
        // D s_103_3: cmp-lt s_103_2 s_103_0
        let s_103_3: bool = ((s_103_2) < (s_103_0));
        // D s_103_4: write-var gs#408640 <= s_103_3
        fn_state.gs_408640 = s_103_3;
        // N s_103_5: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #22s : i
        let s_104_0: i128 = 22;
        // D s_104_1: read-var u#33295:u32
        let s_104_1: u32 = fn_state.u_33295;
        // D s_104_2: cast zx s_104_1 -> bv
        let s_104_2: Bits = Bits::new(s_104_1 as u128, 32u16);
        // C s_104_3: const #1s : i64
        let s_104_3: i64 = 1;
        // C s_104_4: cast zx s_104_3 -> i
        let s_104_4: i128 = (i128::try_from(s_104_3).unwrap());
        // C s_104_5: const #0s : i
        let s_104_5: i128 = 0;
        // C s_104_6: add s_104_5 s_104_4
        let s_104_6: i128 = (s_104_5 + s_104_4);
        // D s_104_7: bit-extract s_104_2 s_104_0 s_104_6
        let s_104_7: Bits = (Bits::new(
            ((s_104_2) >> (s_104_0)).value(),
            u16::try_from(s_104_6).unwrap(),
        ));
        // D s_104_8: cast reint s_104_7 -> u8
        let s_104_8: bool = ((s_104_7.value()) != 0);
        // D s_104_9: cast zx s_104_8 -> bv
        let s_104_9: Bits = Bits::new(s_104_8 as u128, 1u16);
        // C s_104_10: const #0u : u8
        let s_104_10: bool = false;
        // C s_104_11: cast zx s_104_10 -> bv
        let s_104_11: Bits = Bits::new(s_104_10 as u128, 1u16);
        // D s_104_12: cmp-eq s_104_9 s_104_11
        let s_104_12: bool = ((s_104_9) == (s_104_11));
        // N s_104_13: branch s_104_12 b107 b105
        if s_104_12 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #0u : u8
        let s_105_0: bool = false;
        // D s_105_1: write-var gs#408635 <= s_105_0
        fn_state.gs_408635 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#408635:u8
        let s_106_0: bool = fn_state.gs_408635;
        // D s_106_1: write-var gs#408636 <= s_106_0
        fn_state.gs_408636 = s_106_0;
        // N s_106_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #20s : i
        let s_107_0: i128 = 20;
        // D s_107_1: read-var u#33295:u32
        let s_107_1: u32 = fn_state.u_33295;
        // D s_107_2: cast zx s_107_1 -> bv
        let s_107_2: Bits = Bits::new(s_107_1 as u128, 32u16);
        // C s_107_3: const #1s : i64
        let s_107_3: i64 = 1;
        // C s_107_4: cast zx s_107_3 -> i
        let s_107_4: i128 = (i128::try_from(s_107_3).unwrap());
        // C s_107_5: const #0s : i
        let s_107_5: i128 = 0;
        // C s_107_6: add s_107_5 s_107_4
        let s_107_6: i128 = (s_107_5 + s_107_4);
        // D s_107_7: bit-extract s_107_2 s_107_0 s_107_6
        let s_107_7: Bits = (Bits::new(
            ((s_107_2) >> (s_107_0)).value(),
            u16::try_from(s_107_6).unwrap(),
        ));
        // D s_107_8: cast reint s_107_7 -> u8
        let s_107_8: bool = ((s_107_7.value()) != 0);
        // D s_107_9: cast zx s_107_8 -> bv
        let s_107_9: Bits = Bits::new(s_107_8 as u128, 1u16);
        // C s_107_10: const #1u : u8
        let s_107_10: bool = true;
        // C s_107_11: cast zx s_107_10 -> bv
        let s_107_11: Bits = Bits::new(s_107_10 as u128, 1u16);
        // D s_107_12: cmp-eq s_107_9 s_107_11
        let s_107_12: bool = ((s_107_9) == (s_107_11));
        // D s_107_13: write-var gs#408635 <= s_107_12
        fn_state.gs_408635 = s_107_12;
        // N s_107_14: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #28s : i
        let s_108_0: i128 = 28;
        // C s_108_1: const #4s : i
        let s_108_1: i128 = 4;
        // D s_108_2: read-var u#33288:u32
        let s_108_2: u32 = fn_state.u_33288;
        // D s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 32u16);
        // D s_108_4: bit-extract s_108_3 s_108_0 s_108_1
        let s_108_4: Bits = (Bits::new(
            ((s_108_3) >> (s_108_0)).value(),
            u16::try_from(s_108_1).unwrap(),
        ));
        // D s_108_5: cast reint s_108_4 -> u8
        let s_108_5: u8 = (s_108_4.value() as u8);
        // D s_108_6: cast zx s_108_5 -> bv
        let s_108_6: Bits = Bits::new(s_108_5 as u128, 4u16);
        // C s_108_7: const #15u : u8
        let s_108_7: u8 = 15;
        // C s_108_8: cast zx s_108_7 -> bv
        let s_108_8: Bits = Bits::new(s_108_7 as u128, 4u16);
        // D s_108_9: cmp-ne s_108_6 s_108_8
        let s_108_9: bool = ((s_108_6) != (s_108_8));
        // N s_108_10: branch s_108_9 b111 b109
        if s_108_9 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #0u : u8
        let s_109_0: bool = false;
        // D s_109_1: write-var gs#408614 <= s_109_0
        fn_state.gs_408614 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#408614:u8
        let s_110_0: bool = fn_state.gs_408614;
        // D s_110_1: write-var gs#408615 <= s_110_0
        fn_state.gs_408615 = s_110_0;
        // N s_110_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #2932s : i
        let s_111_0: i128 = 2932;
        // C s_111_1: const #14696u : u32
        let s_111_1: u32 = 14696;
        // D s_111_2: read-reg s_111_1:i
        let s_111_2: i128 = {
            let value = state.read_register::<i128>(s_111_1 as isize);
            tracer.read_register(s_111_1 as isize, value);
            value
        };
        // D s_111_3: cmp-lt s_111_2 s_111_0
        let s_111_3: bool = ((s_111_2) < (s_111_0));
        // D s_111_4: write-var gs#408614 <= s_111_3
        fn_state.gs_408614 = s_111_3;
        // N s_111_5: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #20s : i
        let s_112_0: i128 = 20;
        // D s_112_1: read-var u#33288:u32
        let s_112_1: u32 = fn_state.u_33288;
        // D s_112_2: cast zx s_112_1 -> bv
        let s_112_2: Bits = Bits::new(s_112_1 as u128, 32u16);
        // C s_112_3: const #1s : i64
        let s_112_3: i64 = 1;
        // C s_112_4: cast zx s_112_3 -> i
        let s_112_4: i128 = (i128::try_from(s_112_3).unwrap());
        // C s_112_5: const #2s : i
        let s_112_5: i128 = 2;
        // C s_112_6: add s_112_5 s_112_4
        let s_112_6: i128 = (s_112_5 + s_112_4);
        // D s_112_7: bit-extract s_112_2 s_112_0 s_112_6
        let s_112_7: Bits = (Bits::new(
            ((s_112_2) >> (s_112_0)).value(),
            u16::try_from(s_112_6).unwrap(),
        ));
        // D s_112_8: cast reint s_112_7 -> u8
        let s_112_8: u8 = (s_112_7.value() as u8);
        // D s_112_9: cast zx s_112_8 -> bv
        let s_112_9: Bits = Bits::new(s_112_8 as u128, 3u16);
        // C s_112_10: const #7u : u8
        let s_112_10: u8 = 7;
        // C s_112_11: cast zx s_112_10 -> bv
        let s_112_11: Bits = Bits::new(s_112_10 as u128, 3u16);
        // D s_112_12: cmp-eq s_112_9 s_112_11
        let s_112_12: bool = ((s_112_9) == (s_112_11));
        // D s_112_13: write-var gs#408610 <= s_112_12
        fn_state.gs_408610 = s_112_12;
        // N s_112_14: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #28s : i
        let s_113_0: i128 = 28;
        // C s_113_1: const #4s : i
        let s_113_1: i128 = 4;
        // D s_113_2: read-var u#33280:u32
        let s_113_2: u32 = fn_state.u_33280;
        // D s_113_3: cast zx s_113_2 -> bv
        let s_113_3: Bits = Bits::new(s_113_2 as u128, 32u16);
        // D s_113_4: bit-extract s_113_3 s_113_0 s_113_1
        let s_113_4: Bits = (Bits::new(
            ((s_113_3) >> (s_113_0)).value(),
            u16::try_from(s_113_1).unwrap(),
        ));
        // D s_113_5: cast reint s_113_4 -> u8
        let s_113_5: u8 = (s_113_4.value() as u8);
        // D s_113_6: cast zx s_113_5 -> bv
        let s_113_6: Bits = Bits::new(s_113_5 as u128, 4u16);
        // C s_113_7: const #15u : u8
        let s_113_7: u8 = 15;
        // C s_113_8: cast zx s_113_7 -> bv
        let s_113_8: Bits = Bits::new(s_113_7 as u128, 4u16);
        // D s_113_9: cmp-ne s_113_6 s_113_8
        let s_113_9: bool = ((s_113_6) != (s_113_8));
        // N s_113_10: branch s_113_9 b116 b114
        if s_113_9 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #0u : u8
        let s_114_0: bool = false;
        // D s_114_1: write-var gs#408589 <= s_114_0
        fn_state.gs_408589 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#408589:u8
        let s_115_0: bool = fn_state.gs_408589;
        // D s_115_1: write-var gs#408590 <= s_115_0
        fn_state.gs_408590 = s_115_0;
        // N s_115_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #2927s : i
        let s_116_0: i128 = 2927;
        // C s_116_1: const #14696u : u32
        let s_116_1: u32 = 14696;
        // D s_116_2: read-reg s_116_1:i
        let s_116_2: i128 = {
            let value = state.read_register::<i128>(s_116_1 as isize);
            tracer.read_register(s_116_1 as isize, value);
            value
        };
        // D s_116_3: cmp-lt s_116_2 s_116_0
        let s_116_3: bool = ((s_116_2) < (s_116_0));
        // D s_116_4: write-var gs#408589 <= s_116_3
        fn_state.gs_408589 = s_116_3;
        // N s_116_5: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #22s : i
        let s_117_0: i128 = 22;
        // D s_117_1: read-var u#33280:u32
        let s_117_1: u32 = fn_state.u_33280;
        // D s_117_2: cast zx s_117_1 -> bv
        let s_117_2: Bits = Bits::new(s_117_1 as u128, 32u16);
        // C s_117_3: const #1s : i64
        let s_117_3: i64 = 1;
        // C s_117_4: cast zx s_117_3 -> i
        let s_117_4: i128 = (i128::try_from(s_117_3).unwrap());
        // C s_117_5: const #0s : i
        let s_117_5: i128 = 0;
        // C s_117_6: add s_117_5 s_117_4
        let s_117_6: i128 = (s_117_5 + s_117_4);
        // D s_117_7: bit-extract s_117_2 s_117_0 s_117_6
        let s_117_7: Bits = (Bits::new(
            ((s_117_2) >> (s_117_0)).value(),
            u16::try_from(s_117_6).unwrap(),
        ));
        // D s_117_8: cast reint s_117_7 -> u8
        let s_117_8: bool = ((s_117_7.value()) != 0);
        // D s_117_9: cast zx s_117_8 -> bv
        let s_117_9: Bits = Bits::new(s_117_8 as u128, 1u16);
        // C s_117_10: const #1u : u8
        let s_117_10: bool = true;
        // C s_117_11: cast zx s_117_10 -> bv
        let s_117_11: Bits = Bits::new(s_117_10 as u128, 1u16);
        // D s_117_12: cmp-eq s_117_9 s_117_11
        let s_117_12: bool = ((s_117_9) == (s_117_11));
        // N s_117_13: branch s_117_12 b120 b118
        if s_117_12 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #0u : u8
        let s_118_0: bool = false;
        // D s_118_1: write-var gs#408584 <= s_118_0
        fn_state.gs_408584 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#408584:u8
        let s_119_0: bool = fn_state.gs_408584;
        // D s_119_1: write-var gs#408585 <= s_119_0
        fn_state.gs_408585 = s_119_0;
        // N s_119_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #16s : i
        let s_120_0: i128 = 16;
        // D s_120_1: read-var u#33280:u32
        let s_120_1: u32 = fn_state.u_33280;
        // D s_120_2: cast zx s_120_1 -> bv
        let s_120_2: Bits = Bits::new(s_120_1 as u128, 32u16);
        // C s_120_3: const #1s : i64
        let s_120_3: i64 = 1;
        // C s_120_4: cast zx s_120_3 -> i
        let s_120_4: i128 = (i128::try_from(s_120_3).unwrap());
        // C s_120_5: const #4s : i
        let s_120_5: i128 = 4;
        // C s_120_6: add s_120_5 s_120_4
        let s_120_6: i128 = (s_120_5 + s_120_4);
        // D s_120_7: bit-extract s_120_2 s_120_0 s_120_6
        let s_120_7: Bits = (Bits::new(
            ((s_120_2) >> (s_120_0)).value(),
            u16::try_from(s_120_6).unwrap(),
        ));
        // D s_120_8: cast reint s_120_7 -> u8
        let s_120_8: u8 = (s_120_7.value() as u8);
        // D s_120_9: cast zx s_120_8 -> bv
        let s_120_9: Bits = Bits::new(s_120_8 as u128, 5u16);
        // C s_120_10: const #31u : u8
        let s_120_10: u8 = 31;
        // C s_120_11: cast zx s_120_10 -> bv
        let s_120_11: Bits = Bits::new(s_120_10 as u128, 5u16);
        // D s_120_12: cmp-eq s_120_9 s_120_11
        let s_120_12: bool = ((s_120_9) == (s_120_11));
        // D s_120_13: write-var gs#408584 <= s_120_12
        fn_state.gs_408584 = s_120_12;
        // N s_120_14: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #28s : i
        let s_121_0: i128 = 28;
        // C s_121_1: const #4s : i
        let s_121_1: i128 = 4;
        // D s_121_2: read-var __opcode:u32
        let s_121_2: u32 = fn_state.u__opcode;
        // D s_121_3: cast zx s_121_2 -> bv
        let s_121_3: Bits = Bits::new(s_121_2 as u128, 32u16);
        // D s_121_4: bit-extract s_121_3 s_121_0 s_121_1
        let s_121_4: Bits = (Bits::new(
            ((s_121_3) >> (s_121_0)).value(),
            u16::try_from(s_121_1).unwrap(),
        ));
        // D s_121_5: cast reint s_121_4 -> u8
        let s_121_5: u8 = (s_121_4.value() as u8);
        // D s_121_6: cast zx s_121_5 -> bv
        let s_121_6: Bits = Bits::new(s_121_5 as u128, 4u16);
        // C s_121_7: const #15u : u8
        let s_121_7: u8 = 15;
        // C s_121_8: cast zx s_121_7 -> bv
        let s_121_8: Bits = Bits::new(s_121_7 as u128, 4u16);
        // D s_121_9: cmp-ne s_121_6 s_121_8
        let s_121_9: bool = ((s_121_6) != (s_121_8));
        // N s_121_10: branch s_121_9 b124 b122
        if s_121_9 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #0u : u8
        let s_122_0: bool = false;
        // D s_122_1: write-var gs#408559 <= s_122_0
        fn_state.gs_408559 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#408559:u8
        let s_123_0: bool = fn_state.gs_408559;
        // D s_123_1: write-var gs#408560 <= s_123_0
        fn_state.gs_408560 = s_123_0;
        // N s_123_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #2923s : i
        let s_124_0: i128 = 2923;
        // C s_124_1: const #14696u : u32
        let s_124_1: u32 = 14696;
        // D s_124_2: read-reg s_124_1:i
        let s_124_2: i128 = {
            let value = state.read_register::<i128>(s_124_1 as isize);
            tracer.read_register(s_124_1 as isize, value);
            value
        };
        // D s_124_3: cmp-lt s_124_2 s_124_0
        let s_124_3: bool = ((s_124_2) < (s_124_0));
        // D s_124_4: write-var gs#408559 <= s_124_3
        fn_state.gs_408559 = s_124_3;
        // N s_124_5: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #22s : i
        let s_125_0: i128 = 22;
        // D s_125_1: read-var __opcode:u32
        let s_125_1: u32 = fn_state.u__opcode;
        // D s_125_2: cast zx s_125_1 -> bv
        let s_125_2: Bits = Bits::new(s_125_1 as u128, 32u16);
        // C s_125_3: const #1s : i64
        let s_125_3: i64 = 1;
        // C s_125_4: cast zx s_125_3 -> i
        let s_125_4: i128 = (i128::try_from(s_125_3).unwrap());
        // C s_125_5: const #0s : i
        let s_125_5: i128 = 0;
        // C s_125_6: add s_125_5 s_125_4
        let s_125_6: i128 = (s_125_5 + s_125_4);
        // D s_125_7: bit-extract s_125_2 s_125_0 s_125_6
        let s_125_7: Bits = (Bits::new(
            ((s_125_2) >> (s_125_0)).value(),
            u16::try_from(s_125_6).unwrap(),
        ));
        // D s_125_8: cast reint s_125_7 -> u8
        let s_125_8: bool = ((s_125_7.value()) != 0);
        // D s_125_9: cast zx s_125_8 -> bv
        let s_125_9: Bits = Bits::new(s_125_8 as u128, 1u16);
        // C s_125_10: const #1u : u8
        let s_125_10: bool = true;
        // C s_125_11: cast zx s_125_10 -> bv
        let s_125_11: Bits = Bits::new(s_125_10 as u128, 1u16);
        // D s_125_12: cmp-eq s_125_9 s_125_11
        let s_125_12: bool = ((s_125_9) == (s_125_11));
        // N s_125_13: branch s_125_12 b128 b126
        if s_125_12 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #0u : u8
        let s_126_0: bool = false;
        // D s_126_1: write-var gs#408554 <= s_126_0
        fn_state.gs_408554 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#408554:u8
        let s_127_0: bool = fn_state.gs_408554;
        // D s_127_1: write-var gs#408555 <= s_127_0
        fn_state.gs_408555 = s_127_0;
        // N s_127_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #20s : i
        let s_128_0: i128 = 20;
        // D s_128_1: read-var __opcode:u32
        let s_128_1: u32 = fn_state.u__opcode;
        // D s_128_2: cast zx s_128_1 -> bv
        let s_128_2: Bits = Bits::new(s_128_1 as u128, 32u16);
        // C s_128_3: const #1s : i64
        let s_128_3: i64 = 1;
        // C s_128_4: cast zx s_128_3 -> i
        let s_128_4: i128 = (i128::try_from(s_128_3).unwrap());
        // C s_128_5: const #0s : i
        let s_128_5: i128 = 0;
        // C s_128_6: add s_128_5 s_128_4
        let s_128_6: i128 = (s_128_5 + s_128_4);
        // D s_128_7: bit-extract s_128_2 s_128_0 s_128_6
        let s_128_7: Bits = (Bits::new(
            ((s_128_2) >> (s_128_0)).value(),
            u16::try_from(s_128_6).unwrap(),
        ));
        // D s_128_8: cast reint s_128_7 -> u8
        let s_128_8: bool = ((s_128_7.value()) != 0);
        // D s_128_9: cast zx s_128_8 -> bv
        let s_128_9: Bits = Bits::new(s_128_8 as u128, 1u16);
        // C s_128_10: const #1u : u8
        let s_128_10: bool = true;
        // C s_128_11: cast zx s_128_10 -> bv
        let s_128_11: Bits = Bits::new(s_128_10 as u128, 1u16);
        // D s_128_12: cmp-eq s_128_9 s_128_11
        let s_128_12: bool = ((s_128_9) == (s_128_11));
        // D s_128_13: write-var gs#408554 <= s_128_12
        fn_state.gs_408554 = s_128_12;
        // N s_128_14: jump b127
        return block_127(state, tracer, fn_state);
    }
}
