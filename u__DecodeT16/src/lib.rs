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
use decode_aarch32_instrs_ADD_r_T1enc_A_txt::*;
use decode_aarch32_instrs_LDRH_r_T1enc_A_txt::*;
use decode_aarch32_instrs_LDRH_i_T1enc_A_txt::*;
use decode_aarch32_instrs_CMN_r_T1enc_A_txt::*;
use decode_aarch32_instrs_SUB_r_T1enc_A_txt::*;
use decode_aarch32_instrs_ADD_SP_r_T1enc_A_txt::*;
use decode_aarch32_instrs_PUSH_T1enc_A_txt::*;
use decode_aarch32_instrs_MOV_r_T1enc_A_txt::*;
use decode_aarch32_instrs_ADD_SP_r_T2enc_A_txt::*;
use decode_aarch32_instrs_CMP_i_T1enc_A_txt::*;
use decode_aarch32_instrs_EOR_r_T1enc_A_txt::*;
use decode_aarch32_instrs_ORR_r_T1enc_A_txt::*;
use decode_aarch32_instrs_SUB_i_T2enc_A_txt::*;
use decode_aarch32_instrs_SETPAN_T1enc_A_txt::*;
use decode_aarch32_instrs_LSR_r_T1enc_A_txt::*;
use decode_aarch32_instrs_WFI_T1enc_A_txt::*;
use decode_aarch32_instrs_ADR_T1enc_A_txt::*;
use decode_aarch32_instrs_SBC_r_T1enc_A_txt::*;
use decode_aarch32_instrs_YIELD_T1enc_A_txt::*;
use decode_aarch32_instrs_SXTH_T1enc_A_txt::*;
use decode_aarch32_instrs_STM_T1enc_A_txt::*;
use decode_aarch32_instrs_MOV_rr_T1enc_A_txt::*;
use decode_aarch32_instrs_SEVL_T1enc_A_txt::*;
use decode_aarch32_instrs_LDRSH_r_T1enc_A_txt::*;
use decode_aarch32_instrs_CMP_r_T2enc_A_txt::*;
use decode_aarch32_instrs_REVSH_T1enc_A_txt::*;
use decode_aarch32_instrs_LSL_r_T1enc_A_txt::*;
use decode_aarch32_instrs_ADD_i_T1enc_A_txt::*;
use decode_aarch32_instrs_CBNZ_T1enc_A_txt::*;
use decode_aarch32_instrs_LDRSB_r_T1enc_A_txt::*;
use decode_aarch32_instrs_NOP_T1enc_A_txt::*;
use decode_aarch32_instrs_REV16_T1enc_A_txt::*;
use decode_aarch32_instrs_ROR_r_T1enc_A_txt::*;
use decode_aarch32_instrs_SUB_i_T1enc_A_txt::*;
use decode_aarch32_instrs_STR_i_T2enc_A_txt::*;
use decode_aarch32_instrs_REV_T1enc_A_txt::*;
use decode_aarch32_instrs_SUB_SP_i_T1enc_A_txt::*;
use decode_aarch32_instrs_ADD_r_T2enc_A_txt::*;
use decode_aarch32_instrs_B_T2enc_A_txt::*;
use decode_aarch32_instrs_MUL_T1enc_A_txt::*;
use decode_aarch32_instrs_ADD_SP_i_T2enc_A_txt::*;
use decode_aarch32_instrs_RSB_i_T1enc_A_txt::*;
use decode_aarch32_instrs_STRH_r_T1enc_A_txt::*;
use decode_aarch32_instrs_UXTH_T1enc_A_txt::*;
use decode_aarch32_instrs_LSL_i_T1enc_A_txt::*;
use decode_aarch32_instrs_BX_T1enc_A_txt::*;
use decode_aarch32_instrs_WFE_T1enc_A_txt::*;
use decode_aarch32_instrs_HLT_T1enc_A_txt::*;
use decode_aarch32_instrs_LDRB_i_T1enc_A_txt::*;
use decode_aarch32_instrs_ADC_r_T1enc_A_txt::*;
use decode_aarch32_instrs_STR_i_T1enc_A_txt::*;
use decode_aarch32_instrs_UXTB_T1enc_A_txt::*;
use decode_aarch32_instrs_ADD_i_T2enc_A_txt::*;
use decode_aarch32_instrs_SEV_T1enc_A_txt::*;
use decode_aarch32_instrs_STRB_r_T1enc_A_txt::*;
use decode_aarch32_instrs_LDM_T1enc_A_txt::*;
use decode_aarch32_instrs_SETEND_T1enc_A_txt::*;
use decode_aarch32_instrs_LDR_l_T1enc_A_txt::*;
use decode_aarch32_instrs_STRH_i_T1enc_A_txt::*;
use decode_aarch32_instrs_MVN_r_T1enc_A_txt::*;
use decode_aarch32_instrs_AND_r_T1enc_A_txt::*;
use decode_aarch32_instrs_LDR_i_T1enc_A_txt::*;
use decode_aarch32_instrs_B_T1enc_A_txt::*;
use decode_aarch32_instrs_IT_T1enc_A_txt::*;
use decode_aarch32_instrs_LDR_r_T1enc_A_txt::*;
use decode_aarch32_instrs_MOV_r_T2enc_A_txt::*;
use decode_aarch32_instrs_LDR_i_T2enc_A_txt::*;
use decode_aarch32_instrs_LSR_i_T1enc_A_txt::*;
use decode_aarch32_instrs_SVC_T1enc_A_txt::*;
use decode_aarch32_instrs_TST_r_T1enc_A_txt::*;
use decode_aarch32_instrs_BIC_r_T1enc_A_txt::*;
use decode_aarch32_instrs_LDRB_r_T1enc_A_txt::*;
use decode_aarch32_instrs_UDF_T1enc_A_txt::*;
use decode_aarch32_instrs_POP_T1enc_A_txt::*;
use decode_aarch32_instrs_ASR_i_T1enc_A_txt::*;
use decode_aarch32_instrs_BLX_r_T1enc_A_txt::*;
use decode_aarch32_instrs_ASR_r_T1enc_A_txt::*;
use decode_aarch32_instrs_BKPT_T1enc_A_txt::*;
use decode_aarch32_instrs_CPS_T1enc_AS_txt::*;
use decode_aarch32_instrs_ADD_SP_i_T1enc_A_txt::*;
use decode_aarch32_instrs_SXTB_T1enc_A_txt::*;
use decode_aarch32_instrs_STR_r_T1enc_A_txt::*;
use decode_aarch32_instrs_STRB_i_T1enc_A_txt::*;
use decode_aarch32_instrs_MOV_i_T1enc_A_txt::*;
use decode_aarch32_instrs_CMP_r_T1enc_A_txt::*;
use common::*;
pub fn u__DecodeT16<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_427891: i128,
    gs_427892: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u_38422: u16,
        gs_428783: bool,
        u_38331: u16,
        gs_427995: bool,
        u_38404: u16,
        u_38244: u16,
        gs_428873: bool,
        gs_428094: bool,
        u_38501: u16,
        u_38283: u16,
        u_38451: u16,
        u_38237: u16,
        gs_428187: bool,
        gs_428455: bool,
        gs_428114: bool,
        u_38250: u16,
        u_38258: u16,
        gs_428475: bool,
        u_38463: u16,
        gs_428803: bool,
        gs_428165: bool,
        gs_428875: bool,
        gs_428884: bool,
        u_38412: u16,
        u_38224: u16,
        gs_428823: bool,
        gs_428743: bool,
        u_38348: u16,
        u_38311: u16,
        gs_428890: bool,
        u_38360: u16,
        u_38293: u16,
        gs_428333: bool,
        gs_428320: bool,
        gs_428643: bool,
        u_38228: u16,
        gs_428015: bool,
        u_38470: u16,
        gs_428211: bool,
        gs_428606: bool,
        u_38201: u16,
        gs_428396: bool,
        gs_427897: bool,
        u_38279: u16,
        u_38340: u16,
        gs_428233: bool,
        gs_428706: bool,
        u_38316: u16,
        u_38485: u16,
        u_38205: u16,
        u_38298: u16,
        gs_428515: bool,
        u_38497: u16,
        gs_428719: bool,
        u_38303: u16,
        gs_428084: bool,
        u_38455: u16,
        gs_428693: bool,
        u_38262: u16,
        gs_428039: bool,
        gs_428116: bool,
        gs_428843: bool,
        gs_428064: bool,
        u_38396: u16,
        u_38356: u16,
        u_38209: u16,
        gs_428865: bool,
        gs_428105: bool,
        gs_428137: bool,
        u_38193: u16,
        gs_428570: bool,
        gs_428593: bool,
        gs_428004: bool,
        gs_428548: bool,
        gs_428887: bool,
        u_38213: u16,
        u_38481: u16,
        gs_428176: bool,
        gs_428772: bool,
        gs_428504: bool,
        gs_428139: bool,
        u_38392: u16,
        u_38474: u16,
        gs_427908: bool,
        u_38364: u16,
        u_38276: u16,
        u_38245: u8,
        gs_428761: bool,
        gs_428682: bool,
        gs_428617: bool,
        gs_427981: bool,
        gs_428814: bool,
        u_38466: u16,
        u_38288: u16,
        gs_428053: bool,
        gs_427921: bool,
        u_38417: u16,
        u_38437: u16,
        u_38266: u16,
        gs_428466: bool,
        gs_427979: bool,
        gs_428346: bool,
        gs_428559: bool,
        u_38441: u16,
        gs_428136: bool,
        u_38374: u16,
        gs_428418: bool,
        gs_428431: bool,
        u_38427: u16,
        u_38478: u16,
        gs_428841: bool,
        gs_428222: bool,
        gs_427932: bool,
        gs_428656: bool,
        E: bool,
        u_38247: u16,
        u_38235: u16,
        u_38384: u16,
        gs_428298: bool,
        gs_427945: bool,
        gs_428309: bool,
        imm1: bool,
        gs_428026: bool,
        gs_428073: bool,
        gs_428442: bool,
        gs_428752: bool,
        gs_428096: bool,
        gs_428200: bool,
        gs_428858: bool,
        u_38408: u16,
        u_38191: u16,
        u_38232: u16,
        gs_428259: bool,
        gs_428482: bool,
        gs_428582: bool,
        gs_428372: bool,
        gs_428596: bool,
        gs_428108: bool,
        u_38241: u16,
        gs_428383: bool,
        u_38370: u16,
        gs_428833: bool,
        u_38321: u16,
        gs_428359: bool,
        gs_428599: bool,
        gs_428285: bool,
        gs_428246: bool,
        u_38460: u16,
        u_38377: u16,
        u_38211: u16,
        u_38248: u8,
        u_38495: u16,
        u_38352: u16,
        u_38432: u16,
        gs_428669: bool,
        gs_427958: bool,
        u_38380: u16,
        u_38400: u16,
        gs_428052: bool,
        gs_428272: bool,
        gs_428526: bool,
        u_38216: u16,
        u_38274: u16,
        gs_428493: bool,
        gs_428125: bool,
        u__opcode: u16,
        u_38270: u16,
        merge_var: ProductTypeb265ba323b463df,
        gs_427969: bool,
        u_38254: u16,
        u_38446: u16,
        gs_428537: bool,
        gs_428730: bool,
        u_38344: u16,
        u_38326: u16,
        gs_428630: bool,
        gs_428794: bool,
        gs_427993: bool,
        u_38196: u16,
        gs_428828: bool,
        gs_428407: bool,
        u_38335: u16,
        gs_428128: bool,
        gs_428154: bool,
        u_38220: u16,
        gs_428584: bool,
        u_38388: u16,
        u_38307: u16,
        gs_427891: i128,
        gs_427892: u16,
    }
    let fn_state = FunctionState {
        gs_427891,
        gs_427892,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var gs#427891:i
        let s_0_0: i128 = fn_state.gs_427891;
        // D s_0_1: write-var merge#var.0 <= s_0_0
        fn_state.merge_var._0 = s_0_0;
        // D s_0_2: read-var gs#427892:u16
        let s_0_2: u16 = fn_state.gs_427892;
        // D s_0_3: write-var merge#var.1 <= s_0_2
        fn_state.merge_var._1 = s_0_2;
        // D s_0_4: read-var merge#var.1:struct
        let s_0_4: u16 = fn_state.merge_var._1;
        // D s_0_5: write-var __opcode <= s_0_4
        fn_state.u__opcode = s_0_4;
        // C s_0_6: const #6s : i
        let s_0_6: i128 = 6;
        // D s_0_7: read-var __opcode:u16
        let s_0_7: u16 = fn_state.u__opcode;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 16u16);
        // C s_0_9: const #1s : i64
        let s_0_9: i64 = 1;
        // C s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // C s_0_11: const #9s : i
        let s_0_11: i128 = 9;
        // C s_0_12: add s_0_11 s_0_10
        let s_0_12: i128 = (s_0_11 + s_0_10);
        // D s_0_13: bit-extract s_0_8 s_0_6 s_0_12
        let s_0_13: Bits = (Bits::new(
            ((s_0_8) >> (s_0_6)).value(),
            u16::try_from(s_0_12).unwrap(),
        ));
        // D s_0_14: cast reint s_0_13 -> u10
        let s_0_14: u16 = (s_0_13.value() as u16);
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 10u16);
        // C s_0_16: const #261u : u10
        let s_0_16: u16 = 261;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 10u16);
        // D s_0_18: cmp-eq s_0_15 s_0_17
        let s_0_18: bool = ((s_0_15) == (s_0_17));
        // N s_0_19: branch s_0_18 b490 b1
        if s_0_18 {
            return block_490(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#427897 <= s_1_0
        fn_state.gs_427897 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#427897:u8
        let s_2_0: bool = fn_state.gs_427897;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b4 b3
        if s_2_1 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #2816s : i
        let s_3_0: i128 = 2816;
        // C s_3_1: const #14696u : u32
        let s_3_1: u32 = 14696;
        // N s_3_2: write-reg s_3_1 <= s_3_0
        let s_3_2: () = {
            state.write_register::<i128>(s_3_1 as isize, s_3_0);
            tracer.write_register(s_3_1 as isize, s_3_0);
        };
        // C s_3_3: const #3s : i
        let s_3_3: i128 = 3;
        // C s_3_4: const #3s : i
        let s_3_4: i128 = 3;
        // D s_3_5: read-var __opcode:u16
        let s_3_5: u16 = fn_state.u__opcode;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 16u16);
        // D s_3_7: bit-extract s_3_6 s_3_3 s_3_4
        let s_3_7: Bits = (Bits::new(
            ((s_3_6) >> (s_3_3)).value(),
            u16::try_from(s_3_4).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: u8 = (s_3_7.value() as u8);
        // C s_3_9: const #0s : i
        let s_3_9: i128 = 0;
        // C s_3_10: const #3s : i
        let s_3_10: i128 = 3;
        // D s_3_11: read-var __opcode:u16
        let s_3_11: u16 = fn_state.u__opcode;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 16u16);
        // D s_3_13: bit-extract s_3_12 s_3_9 s_3_10
        let s_3_13: Bits = (Bits::new(
            ((s_3_12) >> (s_3_9)).value(),
            u16::try_from(s_3_10).unwrap(),
        ));
        // D s_3_14: cast reint s_3_13 -> u8
        let s_3_14: u8 = (s_3_13.value() as u8);
        // D s_3_15: call decode_aarch32_instrs_ADC_r_T1enc_A_txt(s_3_8, s_3_14)
        let s_3_15: () = decode_aarch32_instrs_ADC_r_T1enc_A_txt(
            state,
            tracer,
            s_3_8,
            s_3_14,
        );
        // N s_3_16: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var merge#var.1:struct
        let s_4_0: u16 = fn_state.merge_var._1;
        // D s_4_1: write-var u#38191 <= s_4_0
        fn_state.u_38191 = s_4_0;
        // C s_4_2: const #9s : i
        let s_4_2: i128 = 9;
        // D s_4_3: read-var u#38191:u16
        let s_4_3: u16 = fn_state.u_38191;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 16u16);
        // C s_4_5: const #1s : i64
        let s_4_5: i64 = 1;
        // C s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // C s_4_7: const #6s : i
        let s_4_7: i128 = 6;
        // C s_4_8: add s_4_7 s_4_6
        let s_4_8: i128 = (s_4_7 + s_4_6);
        // D s_4_9: bit-extract s_4_4 s_4_2 s_4_8
        let s_4_9: Bits = (Bits::new(
            ((s_4_4) >> (s_4_2)).value(),
            u16::try_from(s_4_8).unwrap(),
        ));
        // D s_4_10: cast reint s_4_9 -> u8
        let s_4_10: u8 = (s_4_9.value() as u8);
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 7u16);
        // C s_4_12: const #14u : u8
        let s_4_12: u8 = 14;
        // C s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 7u16);
        // D s_4_14: cmp-eq s_4_11 s_4_13
        let s_4_14: bool = ((s_4_11) == (s_4_13));
        // N s_4_15: branch s_4_14 b489 b5
        if s_4_14 {
            return block_489(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#427908 <= s_5_0
        fn_state.gs_427908 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#427908:u8
        let s_6_0: bool = fn_state.gs_427908;
        // D s_6_1: not s_6_0
        let s_6_1: bool = !s_6_0;
        // N s_6_2: branch s_6_1 b8 b7
        if s_6_1 {
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
        // C s_7_0: const #2820s : i
        let s_7_0: i128 = 2820;
        // C s_7_1: const #14696u : u32
        let s_7_1: u32 = 14696;
        // N s_7_2: write-reg s_7_1 <= s_7_0
        let s_7_2: () = {
            state.write_register::<i128>(s_7_1 as isize, s_7_0);
            tracer.write_register(s_7_1 as isize, s_7_0);
        };
        // C s_7_3: const #6s : i
        let s_7_3: i128 = 6;
        // C s_7_4: const #3s : i
        let s_7_4: i128 = 3;
        // D s_7_5: read-var u#38191:u16
        let s_7_5: u16 = fn_state.u_38191;
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 16u16);
        // D s_7_7: bit-extract s_7_6 s_7_3 s_7_4
        let s_7_7: Bits = (Bits::new(
            ((s_7_6) >> (s_7_3)).value(),
            u16::try_from(s_7_4).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u8
        let s_7_8: u8 = (s_7_7.value() as u8);
        // C s_7_9: const #3s : i
        let s_7_9: i128 = 3;
        // C s_7_10: const #3s : i
        let s_7_10: i128 = 3;
        // D s_7_11: read-var u#38191:u16
        let s_7_11: u16 = fn_state.u_38191;
        // D s_7_12: cast zx s_7_11 -> bv
        let s_7_12: Bits = Bits::new(s_7_11 as u128, 16u16);
        // D s_7_13: bit-extract s_7_12 s_7_9 s_7_10
        let s_7_13: Bits = (Bits::new(
            ((s_7_12) >> (s_7_9)).value(),
            u16::try_from(s_7_10).unwrap(),
        ));
        // D s_7_14: cast reint s_7_13 -> u8
        let s_7_14: u8 = (s_7_13.value() as u8);
        // C s_7_15: const #0s : i
        let s_7_15: i128 = 0;
        // C s_7_16: const #3s : i
        let s_7_16: i128 = 3;
        // D s_7_17: read-var u#38191:u16
        let s_7_17: u16 = fn_state.u_38191;
        // D s_7_18: cast zx s_7_17 -> bv
        let s_7_18: Bits = Bits::new(s_7_17 as u128, 16u16);
        // D s_7_19: bit-extract s_7_18 s_7_15 s_7_16
        let s_7_19: Bits = (Bits::new(
            ((s_7_18) >> (s_7_15)).value(),
            u16::try_from(s_7_16).unwrap(),
        ));
        // D s_7_20: cast reint s_7_19 -> u8
        let s_7_20: u8 = (s_7_19.value() as u8);
        // D s_7_21: call decode_aarch32_instrs_ADD_i_T1enc_A_txt(s_7_8, s_7_14, s_7_20)
        let s_7_21: () = decode_aarch32_instrs_ADD_i_T1enc_A_txt(
            state,
            tracer,
            s_7_8,
            s_7_14,
            s_7_20,
        );
        // N s_7_22: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var merge#var.1:struct
        let s_8_0: u16 = fn_state.merge_var._1;
        // D s_8_1: write-var u#38193 <= s_8_0
        fn_state.u_38193 = s_8_0;
        // C s_8_2: const #11s : i
        let s_8_2: i128 = 11;
        // D s_8_3: read-var u#38193:u16
        let s_8_3: u16 = fn_state.u_38193;
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 16u16);
        // C s_8_5: const #1s : i64
        let s_8_5: i64 = 1;
        // C s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // C s_8_7: const #4s : i
        let s_8_7: i128 = 4;
        // C s_8_8: add s_8_7 s_8_6
        let s_8_8: i128 = (s_8_7 + s_8_6);
        // D s_8_9: bit-extract s_8_4 s_8_2 s_8_8
        let s_8_9: Bits = (Bits::new(
            ((s_8_4) >> (s_8_2)).value(),
            u16::try_from(s_8_8).unwrap(),
        ));
        // D s_8_10: cast reint s_8_9 -> u8
        let s_8_10: u8 = (s_8_9.value() as u8);
        // D s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 5u16);
        // C s_8_12: const #6u : u8
        let s_8_12: u8 = 6;
        // C s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 5u16);
        // D s_8_14: cmp-eq s_8_11 s_8_13
        let s_8_14: bool = ((s_8_11) == (s_8_13));
        // N s_8_15: branch s_8_14 b488 b9
        if s_8_14 {
            return block_488(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#427921 <= s_9_0
        fn_state.gs_427921 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#427921:u8
        let s_10_0: bool = fn_state.gs_427921;
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
        // C s_11_0: const #2821s : i
        let s_11_0: i128 = 2821;
        // C s_11_1: const #14696u : u32
        let s_11_1: u32 = 14696;
        // N s_11_2: write-reg s_11_1 <= s_11_0
        let s_11_2: () = {
            state.write_register::<i128>(s_11_1 as isize, s_11_0);
            tracer.write_register(s_11_1 as isize, s_11_0);
        };
        // C s_11_3: const #8s : i
        let s_11_3: i128 = 8;
        // C s_11_4: const #3s : i
        let s_11_4: i128 = 3;
        // D s_11_5: read-var u#38193:u16
        let s_11_5: u16 = fn_state.u_38193;
        // D s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 16u16);
        // D s_11_7: bit-extract s_11_6 s_11_3 s_11_4
        let s_11_7: Bits = (Bits::new(
            ((s_11_6) >> (s_11_3)).value(),
            u16::try_from(s_11_4).unwrap(),
        ));
        // D s_11_8: cast reint s_11_7 -> u8
        let s_11_8: u8 = (s_11_7.value() as u8);
        // C s_11_9: const #0s : i
        let s_11_9: i128 = 0;
        // C s_11_10: const #8s : i
        let s_11_10: i128 = 8;
        // D s_11_11: read-var u#38193:u16
        let s_11_11: u16 = fn_state.u_38193;
        // D s_11_12: cast zx s_11_11 -> bv
        let s_11_12: Bits = Bits::new(s_11_11 as u128, 16u16);
        // D s_11_13: bit-extract s_11_12 s_11_9 s_11_10
        let s_11_13: Bits = (Bits::new(
            ((s_11_12) >> (s_11_9)).value(),
            u16::try_from(s_11_10).unwrap(),
        ));
        // D s_11_14: cast reint s_11_13 -> u8
        let s_11_14: u8 = (s_11_13.value() as u8);
        // D s_11_15: call decode_aarch32_instrs_ADD_i_T2enc_A_txt(s_11_8, s_11_14)
        let s_11_15: () = decode_aarch32_instrs_ADD_i_T2enc_A_txt(
            state,
            tracer,
            s_11_8,
            s_11_14,
        );
        // N s_11_16: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var merge#var.1:struct
        let s_12_0: u16 = fn_state.merge_var._1;
        // D s_12_1: write-var u#38196 <= s_12_0
        fn_state.u_38196 = s_12_0;
        // C s_12_2: const #9s : i
        let s_12_2: i128 = 9;
        // D s_12_3: read-var u#38196:u16
        let s_12_3: u16 = fn_state.u_38196;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 16u16);
        // C s_12_5: const #1s : i64
        let s_12_5: i64 = 1;
        // C s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // C s_12_7: const #6s : i
        let s_12_7: i128 = 6;
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
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 7u16);
        // C s_12_12: const #12u : u8
        let s_12_12: u8 = 12;
        // C s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 7u16);
        // D s_12_14: cmp-eq s_12_11 s_12_13
        let s_12_14: bool = ((s_12_11) == (s_12_13));
        // N s_12_15: branch s_12_14 b487 b13
        if s_12_14 {
            return block_487(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#427932 <= s_13_0
        fn_state.gs_427932 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#427932:u8
        let s_14_0: bool = fn_state.gs_427932;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // N s_14_2: branch s_14_1 b16 b15
        if s_14_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #2825s : i
        let s_15_0: i128 = 2825;
        // C s_15_1: const #14696u : u32
        let s_15_1: u32 = 14696;
        // N s_15_2: write-reg s_15_1 <= s_15_0
        let s_15_2: () = {
            state.write_register::<i128>(s_15_1 as isize, s_15_0);
            tracer.write_register(s_15_1 as isize, s_15_0);
        };
        // C s_15_3: const #6s : i
        let s_15_3: i128 = 6;
        // C s_15_4: const #3s : i
        let s_15_4: i128 = 3;
        // D s_15_5: read-var u#38196:u16
        let s_15_5: u16 = fn_state.u_38196;
        // D s_15_6: cast zx s_15_5 -> bv
        let s_15_6: Bits = Bits::new(s_15_5 as u128, 16u16);
        // D s_15_7: bit-extract s_15_6 s_15_3 s_15_4
        let s_15_7: Bits = (Bits::new(
            ((s_15_6) >> (s_15_3)).value(),
            u16::try_from(s_15_4).unwrap(),
        ));
        // D s_15_8: cast reint s_15_7 -> u8
        let s_15_8: u8 = (s_15_7.value() as u8);
        // C s_15_9: const #3s : i
        let s_15_9: i128 = 3;
        // C s_15_10: const #3s : i
        let s_15_10: i128 = 3;
        // D s_15_11: read-var u#38196:u16
        let s_15_11: u16 = fn_state.u_38196;
        // D s_15_12: cast zx s_15_11 -> bv
        let s_15_12: Bits = Bits::new(s_15_11 as u128, 16u16);
        // D s_15_13: bit-extract s_15_12 s_15_9 s_15_10
        let s_15_13: Bits = (Bits::new(
            ((s_15_12) >> (s_15_9)).value(),
            u16::try_from(s_15_10).unwrap(),
        ));
        // D s_15_14: cast reint s_15_13 -> u8
        let s_15_14: u8 = (s_15_13.value() as u8);
        // C s_15_15: const #0s : i
        let s_15_15: i128 = 0;
        // C s_15_16: const #3s : i
        let s_15_16: i128 = 3;
        // D s_15_17: read-var u#38196:u16
        let s_15_17: u16 = fn_state.u_38196;
        // D s_15_18: cast zx s_15_17 -> bv
        let s_15_18: Bits = Bits::new(s_15_17 as u128, 16u16);
        // D s_15_19: bit-extract s_15_18 s_15_15 s_15_16
        let s_15_19: Bits = (Bits::new(
            ((s_15_18) >> (s_15_15)).value(),
            u16::try_from(s_15_16).unwrap(),
        ));
        // D s_15_20: cast reint s_15_19 -> u8
        let s_15_20: u8 = (s_15_19.value() as u8);
        // D s_15_21: call decode_aarch32_instrs_ADD_r_T1enc_A_txt(s_15_8, s_15_14, s_15_20)
        let s_15_21: () = decode_aarch32_instrs_ADD_r_T1enc_A_txt(
            state,
            tracer,
            s_15_8,
            s_15_14,
            s_15_20,
        );
        // N s_15_22: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var merge#var.1:struct
        let s_16_0: u16 = fn_state.merge_var._1;
        // D s_16_1: write-var u#38201 <= s_16_0
        fn_state.u_38201 = s_16_0;
        // C s_16_2: const #8s : i
        let s_16_2: i128 = 8;
        // D s_16_3: read-var u#38201:u16
        let s_16_3: u16 = fn_state.u_38201;
        // D s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 16u16);
        // C s_16_5: const #1s : i64
        let s_16_5: i64 = 1;
        // C s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // C s_16_7: const #7s : i
        let s_16_7: i128 = 7;
        // C s_16_8: add s_16_7 s_16_6
        let s_16_8: i128 = (s_16_7 + s_16_6);
        // D s_16_9: bit-extract s_16_4 s_16_2 s_16_8
        let s_16_9: Bits = (Bits::new(
            ((s_16_4) >> (s_16_2)).value(),
            u16::try_from(s_16_8).unwrap(),
        ));
        // D s_16_10: cast reint s_16_9 -> u8
        let s_16_10: u8 = (s_16_9.value() as u8);
        // D s_16_11: cast zx s_16_10 -> bv
        let s_16_11: Bits = Bits::new(s_16_10 as u128, 8u16);
        // C s_16_12: const #68u : u8
        let s_16_12: u8 = 68;
        // C s_16_13: cast zx s_16_12 -> bv
        let s_16_13: Bits = Bits::new(s_16_12 as u128, 8u16);
        // D s_16_14: cmp-eq s_16_11 s_16_13
        let s_16_14: bool = ((s_16_11) == (s_16_13));
        // N s_16_15: branch s_16_14 b486 b17
        if s_16_14 {
            return block_486(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#427945 <= s_17_0
        fn_state.gs_427945 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#427945:u8
        let s_18_0: bool = fn_state.gs_427945;
        // D s_18_1: not s_18_0
        let s_18_1: bool = !s_18_0;
        // N s_18_2: branch s_18_1 b20 b19
        if s_18_1 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #2826s : i
        let s_19_0: i128 = 2826;
        // C s_19_1: const #14696u : u32
        let s_19_1: u32 = 14696;
        // N s_19_2: write-reg s_19_1 <= s_19_0
        let s_19_2: () = {
            state.write_register::<i128>(s_19_1 as isize, s_19_0);
            tracer.write_register(s_19_1 as isize, s_19_0);
        };
        // C s_19_3: const #7s : i
        let s_19_3: i128 = 7;
        // C s_19_4: const #1s : i
        let s_19_4: i128 = 1;
        // D s_19_5: read-var u#38201:u16
        let s_19_5: u16 = fn_state.u_38201;
        // D s_19_6: cast zx s_19_5 -> bv
        let s_19_6: Bits = Bits::new(s_19_5 as u128, 16u16);
        // D s_19_7: bit-extract s_19_6 s_19_3 s_19_4
        let s_19_7: Bits = (Bits::new(
            ((s_19_6) >> (s_19_3)).value(),
            u16::try_from(s_19_4).unwrap(),
        ));
        // D s_19_8: cast reint s_19_7 -> u8
        let s_19_8: bool = ((s_19_7.value()) != 0);
        // C s_19_9: const #3s : i
        let s_19_9: i128 = 3;
        // C s_19_10: const #4s : i
        let s_19_10: i128 = 4;
        // D s_19_11: read-var u#38201:u16
        let s_19_11: u16 = fn_state.u_38201;
        // D s_19_12: cast zx s_19_11 -> bv
        let s_19_12: Bits = Bits::new(s_19_11 as u128, 16u16);
        // D s_19_13: bit-extract s_19_12 s_19_9 s_19_10
        let s_19_13: Bits = (Bits::new(
            ((s_19_12) >> (s_19_9)).value(),
            u16::try_from(s_19_10).unwrap(),
        ));
        // D s_19_14: cast reint s_19_13 -> u8
        let s_19_14: u8 = (s_19_13.value() as u8);
        // C s_19_15: const #0s : i
        let s_19_15: i128 = 0;
        // C s_19_16: const #3s : i
        let s_19_16: i128 = 3;
        // D s_19_17: read-var u#38201:u16
        let s_19_17: u16 = fn_state.u_38201;
        // D s_19_18: cast zx s_19_17 -> bv
        let s_19_18: Bits = Bits::new(s_19_17 as u128, 16u16);
        // D s_19_19: bit-extract s_19_18 s_19_15 s_19_16
        let s_19_19: Bits = (Bits::new(
            ((s_19_18) >> (s_19_15)).value(),
            u16::try_from(s_19_16).unwrap(),
        ));
        // D s_19_20: cast reint s_19_19 -> u8
        let s_19_20: u8 = (s_19_19.value() as u8);
        // D s_19_21: call decode_aarch32_instrs_ADD_r_T2enc_A_txt(s_19_8, s_19_14, s_19_20)
        let s_19_21: () = decode_aarch32_instrs_ADD_r_T2enc_A_txt(
            state,
            tracer,
            s_19_8,
            s_19_14,
            s_19_20,
        );
        // N s_19_22: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var merge#var.1:struct
        let s_20_0: u16 = fn_state.merge_var._1;
        // D s_20_1: write-var u#38205 <= s_20_0
        fn_state.u_38205 = s_20_0;
        // C s_20_2: const #11s : i
        let s_20_2: i128 = 11;
        // D s_20_3: read-var u#38205:u16
        let s_20_3: u16 = fn_state.u_38205;
        // D s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 16u16);
        // C s_20_5: const #1s : i64
        let s_20_5: i64 = 1;
        // C s_20_6: cast zx s_20_5 -> i
        let s_20_6: i128 = (i128::try_from(s_20_5).unwrap());
        // C s_20_7: const #4s : i
        let s_20_7: i128 = 4;
        // C s_20_8: add s_20_7 s_20_6
        let s_20_8: i128 = (s_20_7 + s_20_6);
        // D s_20_9: bit-extract s_20_4 s_20_2 s_20_8
        let s_20_9: Bits = (Bits::new(
            ((s_20_4) >> (s_20_2)).value(),
            u16::try_from(s_20_8).unwrap(),
        ));
        // D s_20_10: cast reint s_20_9 -> u8
        let s_20_10: u8 = (s_20_9.value() as u8);
        // D s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 5u16);
        // C s_20_12: const #21u : u8
        let s_20_12: u8 = 21;
        // C s_20_13: cast zx s_20_12 -> bv
        let s_20_13: Bits = Bits::new(s_20_12 as u128, 5u16);
        // D s_20_14: cmp-eq s_20_11 s_20_13
        let s_20_14: bool = ((s_20_11) == (s_20_13));
        // N s_20_15: branch s_20_14 b485 b21
        if s_20_14 {
            return block_485(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#427958 <= s_21_0
        fn_state.gs_427958 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#427958:u8
        let s_22_0: bool = fn_state.gs_427958;
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
        // C s_23_0: const #2830s : i
        let s_23_0: i128 = 2830;
        // C s_23_1: const #14696u : u32
        let s_23_1: u32 = 14696;
        // N s_23_2: write-reg s_23_1 <= s_23_0
        let s_23_2: () = {
            state.write_register::<i128>(s_23_1 as isize, s_23_0);
            tracer.write_register(s_23_1 as isize, s_23_0);
        };
        // C s_23_3: const #8s : i
        let s_23_3: i128 = 8;
        // C s_23_4: const #3s : i
        let s_23_4: i128 = 3;
        // D s_23_5: read-var u#38205:u16
        let s_23_5: u16 = fn_state.u_38205;
        // D s_23_6: cast zx s_23_5 -> bv
        let s_23_6: Bits = Bits::new(s_23_5 as u128, 16u16);
        // D s_23_7: bit-extract s_23_6 s_23_3 s_23_4
        let s_23_7: Bits = (Bits::new(
            ((s_23_6) >> (s_23_3)).value(),
            u16::try_from(s_23_4).unwrap(),
        ));
        // D s_23_8: cast reint s_23_7 -> u8
        let s_23_8: u8 = (s_23_7.value() as u8);
        // C s_23_9: const #0s : i
        let s_23_9: i128 = 0;
        // C s_23_10: const #8s : i
        let s_23_10: i128 = 8;
        // D s_23_11: read-var u#38205:u16
        let s_23_11: u16 = fn_state.u_38205;
        // D s_23_12: cast zx s_23_11 -> bv
        let s_23_12: Bits = Bits::new(s_23_11 as u128, 16u16);
        // D s_23_13: bit-extract s_23_12 s_23_9 s_23_10
        let s_23_13: Bits = (Bits::new(
            ((s_23_12) >> (s_23_9)).value(),
            u16::try_from(s_23_10).unwrap(),
        ));
        // D s_23_14: cast reint s_23_13 -> u8
        let s_23_14: u8 = (s_23_13.value() as u8);
        // D s_23_15: call decode_aarch32_instrs_ADD_SP_i_T1enc_A_txt(s_23_8, s_23_14)
        let s_23_15: () = decode_aarch32_instrs_ADD_SP_i_T1enc_A_txt(
            state,
            tracer,
            s_23_8,
            s_23_14,
        );
        // N s_23_16: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var merge#var.1:struct
        let s_24_0: u16 = fn_state.merge_var._1;
        // D s_24_1: write-var u#38209 <= s_24_0
        fn_state.u_38209 = s_24_0;
        // C s_24_2: const #7s : i
        let s_24_2: i128 = 7;
        // D s_24_3: read-var u#38209:u16
        let s_24_3: u16 = fn_state.u_38209;
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 16u16);
        // C s_24_5: const #1s : i64
        let s_24_5: i64 = 1;
        // C s_24_6: cast zx s_24_5 -> i
        let s_24_6: i128 = (i128::try_from(s_24_5).unwrap());
        // C s_24_7: const #8s : i
        let s_24_7: i128 = 8;
        // C s_24_8: add s_24_7 s_24_6
        let s_24_8: i128 = (s_24_7 + s_24_6);
        // D s_24_9: bit-extract s_24_4 s_24_2 s_24_8
        let s_24_9: Bits = (Bits::new(
            ((s_24_4) >> (s_24_2)).value(),
            u16::try_from(s_24_8).unwrap(),
        ));
        // D s_24_10: cast reint s_24_9 -> u9
        let s_24_10: u16 = (s_24_9.value() as u16);
        // D s_24_11: cast zx s_24_10 -> bv
        let s_24_11: Bits = Bits::new(s_24_10 as u128, 9u16);
        // C s_24_12: const #352u : u9
        let s_24_12: u16 = 352;
        // C s_24_13: cast zx s_24_12 -> bv
        let s_24_13: Bits = Bits::new(s_24_12 as u128, 9u16);
        // D s_24_14: cmp-eq s_24_11 s_24_13
        let s_24_14: bool = ((s_24_11) == (s_24_13));
        // N s_24_15: branch s_24_14 b484 b25
        if s_24_14 {
            return block_484(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#427969 <= s_25_0
        fn_state.gs_427969 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#427969:u8
        let s_26_0: bool = fn_state.gs_427969;
        // D s_26_1: not s_26_0
        let s_26_1: bool = !s_26_0;
        // N s_26_2: branch s_26_1 b28 b27
        if s_26_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #2831s : i
        let s_27_0: i128 = 2831;
        // C s_27_1: const #14696u : u32
        let s_27_1: u32 = 14696;
        // N s_27_2: write-reg s_27_1 <= s_27_0
        let s_27_2: () = {
            state.write_register::<i128>(s_27_1 as isize, s_27_0);
            tracer.write_register(s_27_1 as isize, s_27_0);
        };
        // C s_27_3: const #0s : i
        let s_27_3: i128 = 0;
        // C s_27_4: const #7s : i
        let s_27_4: i128 = 7;
        // D s_27_5: read-var u#38209:u16
        let s_27_5: u16 = fn_state.u_38209;
        // D s_27_6: cast zx s_27_5 -> bv
        let s_27_6: Bits = Bits::new(s_27_5 as u128, 16u16);
        // D s_27_7: bit-extract s_27_6 s_27_3 s_27_4
        let s_27_7: Bits = (Bits::new(
            ((s_27_6) >> (s_27_3)).value(),
            u16::try_from(s_27_4).unwrap(),
        ));
        // D s_27_8: cast reint s_27_7 -> u8
        let s_27_8: u8 = (s_27_7.value() as u8);
        // D s_27_9: call decode_aarch32_instrs_ADD_SP_i_T2enc_A_txt(s_27_8)
        let s_27_9: () = decode_aarch32_instrs_ADD_SP_i_T2enc_A_txt(
            state,
            tracer,
            s_27_8,
        );
        // N s_27_10: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var merge#var.1:struct
        let s_28_0: u16 = fn_state.merge_var._1;
        // D s_28_1: write-var u#38211 <= s_28_0
        fn_state.u_38211 = s_28_0;
        // C s_28_2: const #8s : i
        let s_28_2: i128 = 8;
        // D s_28_3: read-var u#38211:u16
        let s_28_3: u16 = fn_state.u_38211;
        // D s_28_4: cast zx s_28_3 -> bv
        let s_28_4: Bits = Bits::new(s_28_3 as u128, 16u16);
        // C s_28_5: const #1s : i64
        let s_28_5: i64 = 1;
        // C s_28_6: cast zx s_28_5 -> i
        let s_28_6: i128 = (i128::try_from(s_28_5).unwrap());
        // C s_28_7: const #7s : i
        let s_28_7: i128 = 7;
        // C s_28_8: add s_28_7 s_28_6
        let s_28_8: i128 = (s_28_7 + s_28_6);
        // D s_28_9: bit-extract s_28_4 s_28_2 s_28_8
        let s_28_9: Bits = (Bits::new(
            ((s_28_4) >> (s_28_2)).value(),
            u16::try_from(s_28_8).unwrap(),
        ));
        // D s_28_10: cast reint s_28_9 -> u8
        let s_28_10: u8 = (s_28_9.value() as u8);
        // D s_28_11: cast zx s_28_10 -> bv
        let s_28_11: Bits = Bits::new(s_28_10 as u128, 8u16);
        // C s_28_12: const #68u : u8
        let s_28_12: u8 = 68;
        // C s_28_13: cast zx s_28_12 -> bv
        let s_28_13: Bits = Bits::new(s_28_12 as u128, 8u16);
        // D s_28_14: cmp-eq s_28_11 s_28_13
        let s_28_14: bool = ((s_28_11) == (s_28_13));
        // N s_28_15: branch s_28_14 b483 b29
        if s_28_14 {
            return block_483(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#427979 <= s_29_0
        fn_state.gs_427979 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#427979:u8
        let s_30_0: bool = fn_state.gs_427979;
        // N s_30_1: branch s_30_0 b482 b31
        if s_30_0 {
            return block_482(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#427981 <= s_31_0
        fn_state.gs_427981 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#427981:u8
        let s_32_0: bool = fn_state.gs_427981;
        // D s_32_1: not s_32_0
        let s_32_1: bool = !s_32_0;
        // N s_32_2: branch s_32_1 b34 b33
        if s_32_1 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #2835s : i
        let s_33_0: i128 = 2835;
        // C s_33_1: const #14696u : u32
        let s_33_1: u32 = 14696;
        // N s_33_2: write-reg s_33_1 <= s_33_0
        let s_33_2: () = {
            state.write_register::<i128>(s_33_1 as isize, s_33_0);
            tracer.write_register(s_33_1 as isize, s_33_0);
        };
        // C s_33_3: const #7s : i
        let s_33_3: i128 = 7;
        // C s_33_4: const #1s : i
        let s_33_4: i128 = 1;
        // D s_33_5: read-var u#38211:u16
        let s_33_5: u16 = fn_state.u_38211;
        // D s_33_6: cast zx s_33_5 -> bv
        let s_33_6: Bits = Bits::new(s_33_5 as u128, 16u16);
        // D s_33_7: bit-extract s_33_6 s_33_3 s_33_4
        let s_33_7: Bits = (Bits::new(
            ((s_33_6) >> (s_33_3)).value(),
            u16::try_from(s_33_4).unwrap(),
        ));
        // D s_33_8: cast reint s_33_7 -> u8
        let s_33_8: bool = ((s_33_7.value()) != 0);
        // C s_33_9: const #0s : i
        let s_33_9: i128 = 0;
        // C s_33_10: const #3s : i
        let s_33_10: i128 = 3;
        // D s_33_11: read-var u#38211:u16
        let s_33_11: u16 = fn_state.u_38211;
        // D s_33_12: cast zx s_33_11 -> bv
        let s_33_12: Bits = Bits::new(s_33_11 as u128, 16u16);
        // D s_33_13: bit-extract s_33_12 s_33_9 s_33_10
        let s_33_13: Bits = (Bits::new(
            ((s_33_12) >> (s_33_9)).value(),
            u16::try_from(s_33_10).unwrap(),
        ));
        // D s_33_14: cast reint s_33_13 -> u8
        let s_33_14: u8 = (s_33_13.value() as u8);
        // D s_33_15: call decode_aarch32_instrs_ADD_SP_r_T1enc_A_txt(s_33_8, s_33_14)
        let s_33_15: () = decode_aarch32_instrs_ADD_SP_r_T1enc_A_txt(
            state,
            tracer,
            s_33_8,
            s_33_14,
        );
        // N s_33_16: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var merge#var.1:struct
        let s_34_0: u16 = fn_state.merge_var._1;
        // D s_34_1: write-var u#38213 <= s_34_0
        fn_state.u_38213 = s_34_0;
        // C s_34_2: const #7s : i
        let s_34_2: i128 = 7;
        // D s_34_3: read-var u#38213:u16
        let s_34_3: u16 = fn_state.u_38213;
        // D s_34_4: cast zx s_34_3 -> bv
        let s_34_4: Bits = Bits::new(s_34_3 as u128, 16u16);
        // C s_34_5: const #1s : i64
        let s_34_5: i64 = 1;
        // C s_34_6: cast zx s_34_5 -> i
        let s_34_6: i128 = (i128::try_from(s_34_5).unwrap());
        // C s_34_7: const #8s : i
        let s_34_7: i128 = 8;
        // C s_34_8: add s_34_7 s_34_6
        let s_34_8: i128 = (s_34_7 + s_34_6);
        // D s_34_9: bit-extract s_34_4 s_34_2 s_34_8
        let s_34_9: Bits = (Bits::new(
            ((s_34_4) >> (s_34_2)).value(),
            u16::try_from(s_34_8).unwrap(),
        ));
        // D s_34_10: cast reint s_34_9 -> u9
        let s_34_10: u16 = (s_34_9.value() as u16);
        // D s_34_11: cast zx s_34_10 -> bv
        let s_34_11: Bits = Bits::new(s_34_10 as u128, 9u16);
        // C s_34_12: const #137u : u9
        let s_34_12: u16 = 137;
        // C s_34_13: cast zx s_34_12 -> bv
        let s_34_13: Bits = Bits::new(s_34_12 as u128, 9u16);
        // D s_34_14: cmp-eq s_34_11 s_34_13
        let s_34_14: bool = ((s_34_11) == (s_34_13));
        // N s_34_15: branch s_34_14 b481 b35
        if s_34_14 {
            return block_481(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#427993 <= s_35_0
        fn_state.gs_427993 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#427993:u8
        let s_36_0: bool = fn_state.gs_427993;
        // N s_36_1: branch s_36_0 b480 b37
        if s_36_0 {
            return block_480(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#427995 <= s_37_0
        fn_state.gs_427995 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#427995:u8
        let s_38_0: bool = fn_state.gs_427995;
        // D s_38_1: not s_38_0
        let s_38_1: bool = !s_38_0;
        // N s_38_2: branch s_38_1 b40 b39
        if s_38_1 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #2836s : i
        let s_39_0: i128 = 2836;
        // C s_39_1: const #14696u : u32
        let s_39_1: u32 = 14696;
        // N s_39_2: write-reg s_39_1 <= s_39_0
        let s_39_2: () = {
            state.write_register::<i128>(s_39_1 as isize, s_39_0);
            tracer.write_register(s_39_1 as isize, s_39_0);
        };
        // C s_39_3: const #3s : i
        let s_39_3: i128 = 3;
        // C s_39_4: const #4s : i
        let s_39_4: i128 = 4;
        // D s_39_5: read-var u#38213:u16
        let s_39_5: u16 = fn_state.u_38213;
        // D s_39_6: cast zx s_39_5 -> bv
        let s_39_6: Bits = Bits::new(s_39_5 as u128, 16u16);
        // D s_39_7: bit-extract s_39_6 s_39_3 s_39_4
        let s_39_7: Bits = (Bits::new(
            ((s_39_6) >> (s_39_3)).value(),
            u16::try_from(s_39_4).unwrap(),
        ));
        // D s_39_8: cast reint s_39_7 -> u8
        let s_39_8: u8 = (s_39_7.value() as u8);
        // D s_39_9: call decode_aarch32_instrs_ADD_SP_r_T2enc_A_txt(s_39_8)
        let s_39_9: () = decode_aarch32_instrs_ADD_SP_r_T2enc_A_txt(
            state,
            tracer,
            s_39_8,
        );
        // N s_39_10: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var merge#var.1:struct
        let s_40_0: u16 = fn_state.merge_var._1;
        // D s_40_1: write-var u#38216 <= s_40_0
        fn_state.u_38216 = s_40_0;
        // C s_40_2: const #11s : i
        let s_40_2: i128 = 11;
        // D s_40_3: read-var u#38216:u16
        let s_40_3: u16 = fn_state.u_38216;
        // D s_40_4: cast zx s_40_3 -> bv
        let s_40_4: Bits = Bits::new(s_40_3 as u128, 16u16);
        // C s_40_5: const #1s : i64
        let s_40_5: i64 = 1;
        // C s_40_6: cast zx s_40_5 -> i
        let s_40_6: i128 = (i128::try_from(s_40_5).unwrap());
        // C s_40_7: const #4s : i
        let s_40_7: i128 = 4;
        // C s_40_8: add s_40_7 s_40_6
        let s_40_8: i128 = (s_40_7 + s_40_6);
        // D s_40_9: bit-extract s_40_4 s_40_2 s_40_8
        let s_40_9: Bits = (Bits::new(
            ((s_40_4) >> (s_40_2)).value(),
            u16::try_from(s_40_8).unwrap(),
        ));
        // D s_40_10: cast reint s_40_9 -> u8
        let s_40_10: u8 = (s_40_9.value() as u8);
        // D s_40_11: cast zx s_40_10 -> bv
        let s_40_11: Bits = Bits::new(s_40_10 as u128, 5u16);
        // C s_40_12: const #20u : u8
        let s_40_12: u8 = 20;
        // C s_40_13: cast zx s_40_12 -> bv
        let s_40_13: Bits = Bits::new(s_40_12 as u128, 5u16);
        // D s_40_14: cmp-eq s_40_11 s_40_13
        let s_40_14: bool = ((s_40_11) == (s_40_13));
        // N s_40_15: branch s_40_14 b479 b41
        if s_40_14 {
            return block_479(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#428004 <= s_41_0
        fn_state.gs_428004 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#428004:u8
        let s_42_0: bool = fn_state.gs_428004;
        // D s_42_1: not s_42_0
        let s_42_1: bool = !s_42_0;
        // N s_42_2: branch s_42_1 b44 b43
        if s_42_1 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #2840s : i
        let s_43_0: i128 = 2840;
        // C s_43_1: const #14696u : u32
        let s_43_1: u32 = 14696;
        // N s_43_2: write-reg s_43_1 <= s_43_0
        let s_43_2: () = {
            state.write_register::<i128>(s_43_1 as isize, s_43_0);
            tracer.write_register(s_43_1 as isize, s_43_0);
        };
        // C s_43_3: const #8s : i
        let s_43_3: i128 = 8;
        // C s_43_4: const #3s : i
        let s_43_4: i128 = 3;
        // D s_43_5: read-var u#38216:u16
        let s_43_5: u16 = fn_state.u_38216;
        // D s_43_6: cast zx s_43_5 -> bv
        let s_43_6: Bits = Bits::new(s_43_5 as u128, 16u16);
        // D s_43_7: bit-extract s_43_6 s_43_3 s_43_4
        let s_43_7: Bits = (Bits::new(
            ((s_43_6) >> (s_43_3)).value(),
            u16::try_from(s_43_4).unwrap(),
        ));
        // D s_43_8: cast reint s_43_7 -> u8
        let s_43_8: u8 = (s_43_7.value() as u8);
        // C s_43_9: const #0s : i
        let s_43_9: i128 = 0;
        // C s_43_10: const #8s : i
        let s_43_10: i128 = 8;
        // D s_43_11: read-var u#38216:u16
        let s_43_11: u16 = fn_state.u_38216;
        // D s_43_12: cast zx s_43_11 -> bv
        let s_43_12: Bits = Bits::new(s_43_11 as u128, 16u16);
        // D s_43_13: bit-extract s_43_12 s_43_9 s_43_10
        let s_43_13: Bits = (Bits::new(
            ((s_43_12) >> (s_43_9)).value(),
            u16::try_from(s_43_10).unwrap(),
        ));
        // D s_43_14: cast reint s_43_13 -> u8
        let s_43_14: u8 = (s_43_13.value() as u8);
        // D s_43_15: call decode_aarch32_instrs_ADR_T1enc_A_txt(s_43_8, s_43_14)
        let s_43_15: () = decode_aarch32_instrs_ADR_T1enc_A_txt(
            state,
            tracer,
            s_43_8,
            s_43_14,
        );
        // N s_43_16: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var merge#var.1:struct
        let s_44_0: u16 = fn_state.merge_var._1;
        // D s_44_1: write-var u#38220 <= s_44_0
        fn_state.u_38220 = s_44_0;
        // C s_44_2: const #6s : i
        let s_44_2: i128 = 6;
        // D s_44_3: read-var u#38220:u16
        let s_44_3: u16 = fn_state.u_38220;
        // D s_44_4: cast zx s_44_3 -> bv
        let s_44_4: Bits = Bits::new(s_44_3 as u128, 16u16);
        // C s_44_5: const #1s : i64
        let s_44_5: i64 = 1;
        // C s_44_6: cast zx s_44_5 -> i
        let s_44_6: i128 = (i128::try_from(s_44_5).unwrap());
        // C s_44_7: const #9s : i
        let s_44_7: i128 = 9;
        // C s_44_8: add s_44_7 s_44_6
        let s_44_8: i128 = (s_44_7 + s_44_6);
        // D s_44_9: bit-extract s_44_4 s_44_2 s_44_8
        let s_44_9: Bits = (Bits::new(
            ((s_44_4) >> (s_44_2)).value(),
            u16::try_from(s_44_8).unwrap(),
        ));
        // D s_44_10: cast reint s_44_9 -> u10
        let s_44_10: u16 = (s_44_9.value() as u16);
        // D s_44_11: cast zx s_44_10 -> bv
        let s_44_11: Bits = Bits::new(s_44_10 as u128, 10u16);
        // C s_44_12: const #256u : u10
        let s_44_12: u16 = 256;
        // C s_44_13: cast zx s_44_12 -> bv
        let s_44_13: Bits = Bits::new(s_44_12 as u128, 10u16);
        // D s_44_14: cmp-eq s_44_11 s_44_13
        let s_44_14: bool = ((s_44_11) == (s_44_13));
        // N s_44_15: branch s_44_14 b478 b45
        if s_44_14 {
            return block_478(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#428015 <= s_45_0
        fn_state.gs_428015 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#428015:u8
        let s_46_0: bool = fn_state.gs_428015;
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
        // C s_47_0: const #2846s : i
        let s_47_0: i128 = 2846;
        // C s_47_1: const #14696u : u32
        let s_47_1: u32 = 14696;
        // N s_47_2: write-reg s_47_1 <= s_47_0
        let s_47_2: () = {
            state.write_register::<i128>(s_47_1 as isize, s_47_0);
            tracer.write_register(s_47_1 as isize, s_47_0);
        };
        // C s_47_3: const #3s : i
        let s_47_3: i128 = 3;
        // C s_47_4: const #3s : i
        let s_47_4: i128 = 3;
        // D s_47_5: read-var u#38220:u16
        let s_47_5: u16 = fn_state.u_38220;
        // D s_47_6: cast zx s_47_5 -> bv
        let s_47_6: Bits = Bits::new(s_47_5 as u128, 16u16);
        // D s_47_7: bit-extract s_47_6 s_47_3 s_47_4
        let s_47_7: Bits = (Bits::new(
            ((s_47_6) >> (s_47_3)).value(),
            u16::try_from(s_47_4).unwrap(),
        ));
        // D s_47_8: cast reint s_47_7 -> u8
        let s_47_8: u8 = (s_47_7.value() as u8);
        // C s_47_9: const #0s : i
        let s_47_9: i128 = 0;
        // C s_47_10: const #3s : i
        let s_47_10: i128 = 3;
        // D s_47_11: read-var u#38220:u16
        let s_47_11: u16 = fn_state.u_38220;
        // D s_47_12: cast zx s_47_11 -> bv
        let s_47_12: Bits = Bits::new(s_47_11 as u128, 16u16);
        // D s_47_13: bit-extract s_47_12 s_47_9 s_47_10
        let s_47_13: Bits = (Bits::new(
            ((s_47_12) >> (s_47_9)).value(),
            u16::try_from(s_47_10).unwrap(),
        ));
        // D s_47_14: cast reint s_47_13 -> u8
        let s_47_14: u8 = (s_47_13.value() as u8);
        // D s_47_15: call decode_aarch32_instrs_AND_r_T1enc_A_txt(s_47_8, s_47_14)
        let s_47_15: () = decode_aarch32_instrs_AND_r_T1enc_A_txt(
            state,
            tracer,
            s_47_8,
            s_47_14,
        );
        // N s_47_16: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var merge#var.1:struct
        let s_48_0: u16 = fn_state.merge_var._1;
        // D s_48_1: write-var u#38224 <= s_48_0
        fn_state.u_38224 = s_48_0;
        // C s_48_2: const #11s : i
        let s_48_2: i128 = 11;
        // D s_48_3: read-var u#38224:u16
        let s_48_3: u16 = fn_state.u_38224;
        // D s_48_4: cast zx s_48_3 -> bv
        let s_48_4: Bits = Bits::new(s_48_3 as u128, 16u16);
        // C s_48_5: const #1s : i64
        let s_48_5: i64 = 1;
        // C s_48_6: cast zx s_48_5 -> i
        let s_48_6: i128 = (i128::try_from(s_48_5).unwrap());
        // C s_48_7: const #4s : i
        let s_48_7: i128 = 4;
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
        let s_48_11: Bits = Bits::new(s_48_10 as u128, 5u16);
        // C s_48_12: const #2u : u8
        let s_48_12: u8 = 2;
        // C s_48_13: cast zx s_48_12 -> bv
        let s_48_13: Bits = Bits::new(s_48_12 as u128, 5u16);
        // D s_48_14: cmp-eq s_48_11 s_48_13
        let s_48_14: bool = ((s_48_11) == (s_48_13));
        // N s_48_15: branch s_48_14 b477 b49
        if s_48_14 {
            return block_477(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#428026 <= s_49_0
        fn_state.gs_428026 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#428026:u8
        let s_50_0: bool = fn_state.gs_428026;
        // D s_50_1: not s_50_0
        let s_50_1: bool = !s_50_0;
        // N s_50_2: branch s_50_1 b52 b51
        if s_50_1 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #2849s : i
        let s_51_0: i128 = 2849;
        // C s_51_1: const #14696u : u32
        let s_51_1: u32 = 14696;
        // N s_51_2: write-reg s_51_1 <= s_51_0
        let s_51_2: () = {
            state.write_register::<i128>(s_51_1 as isize, s_51_0);
            tracer.write_register(s_51_1 as isize, s_51_0);
        };
        // C s_51_3: const #6s : i
        let s_51_3: i128 = 6;
        // C s_51_4: const #5s : i
        let s_51_4: i128 = 5;
        // D s_51_5: read-var u#38224:u16
        let s_51_5: u16 = fn_state.u_38224;
        // D s_51_6: cast zx s_51_5 -> bv
        let s_51_6: Bits = Bits::new(s_51_5 as u128, 16u16);
        // D s_51_7: bit-extract s_51_6 s_51_3 s_51_4
        let s_51_7: Bits = (Bits::new(
            ((s_51_6) >> (s_51_3)).value(),
            u16::try_from(s_51_4).unwrap(),
        ));
        // D s_51_8: cast reint s_51_7 -> u8
        let s_51_8: u8 = (s_51_7.value() as u8);
        // C s_51_9: const #3s : i
        let s_51_9: i128 = 3;
        // C s_51_10: const #3s : i
        let s_51_10: i128 = 3;
        // D s_51_11: read-var u#38224:u16
        let s_51_11: u16 = fn_state.u_38224;
        // D s_51_12: cast zx s_51_11 -> bv
        let s_51_12: Bits = Bits::new(s_51_11 as u128, 16u16);
        // D s_51_13: bit-extract s_51_12 s_51_9 s_51_10
        let s_51_13: Bits = (Bits::new(
            ((s_51_12) >> (s_51_9)).value(),
            u16::try_from(s_51_10).unwrap(),
        ));
        // D s_51_14: cast reint s_51_13 -> u8
        let s_51_14: u8 = (s_51_13.value() as u8);
        // C s_51_15: const #0s : i
        let s_51_15: i128 = 0;
        // C s_51_16: const #3s : i
        let s_51_16: i128 = 3;
        // D s_51_17: read-var u#38224:u16
        let s_51_17: u16 = fn_state.u_38224;
        // D s_51_18: cast zx s_51_17 -> bv
        let s_51_18: Bits = Bits::new(s_51_17 as u128, 16u16);
        // D s_51_19: bit-extract s_51_18 s_51_15 s_51_16
        let s_51_19: Bits = (Bits::new(
            ((s_51_18) >> (s_51_15)).value(),
            u16::try_from(s_51_16).unwrap(),
        ));
        // D s_51_20: cast reint s_51_19 -> u8
        let s_51_20: u8 = (s_51_19.value() as u8);
        // D s_51_21: call decode_aarch32_instrs_ASR_i_T1enc_A_txt(s_51_8, s_51_14, s_51_20)
        let s_51_21: () = decode_aarch32_instrs_ASR_i_T1enc_A_txt(
            state,
            tracer,
            s_51_8,
            s_51_14,
            s_51_20,
        );
        // N s_51_22: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var merge#var.1:struct
        let s_52_0: u16 = fn_state.merge_var._1;
        // D s_52_1: write-var u#38228 <= s_52_0
        fn_state.u_38228 = s_52_0;
        // C s_52_2: const #6s : i
        let s_52_2: i128 = 6;
        // D s_52_3: read-var u#38228:u16
        let s_52_3: u16 = fn_state.u_38228;
        // D s_52_4: cast zx s_52_3 -> bv
        let s_52_4: Bits = Bits::new(s_52_3 as u128, 16u16);
        // C s_52_5: const #1s : i64
        let s_52_5: i64 = 1;
        // C s_52_6: cast zx s_52_5 -> i
        let s_52_6: i128 = (i128::try_from(s_52_5).unwrap());
        // C s_52_7: const #9s : i
        let s_52_7: i128 = 9;
        // C s_52_8: add s_52_7 s_52_6
        let s_52_8: i128 = (s_52_7 + s_52_6);
        // D s_52_9: bit-extract s_52_4 s_52_2 s_52_8
        let s_52_9: Bits = (Bits::new(
            ((s_52_4) >> (s_52_2)).value(),
            u16::try_from(s_52_8).unwrap(),
        ));
        // D s_52_10: cast reint s_52_9 -> u10
        let s_52_10: u16 = (s_52_9.value() as u16);
        // D s_52_11: cast zx s_52_10 -> bv
        let s_52_11: Bits = Bits::new(s_52_10 as u128, 10u16);
        // C s_52_12: const #260u : u10
        let s_52_12: u16 = 260;
        // C s_52_13: cast zx s_52_12 -> bv
        let s_52_13: Bits = Bits::new(s_52_12 as u128, 10u16);
        // D s_52_14: cmp-eq s_52_11 s_52_13
        let s_52_14: bool = ((s_52_11) == (s_52_13));
        // N s_52_15: branch s_52_14 b476 b53
        if s_52_14 {
            return block_476(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#428039 <= s_53_0
        fn_state.gs_428039 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#428039:u8
        let s_54_0: bool = fn_state.gs_428039;
        // D s_54_1: not s_54_0
        let s_54_1: bool = !s_54_0;
        // N s_54_2: branch s_54_1 b56 b55
        if s_54_1 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #2850s : i
        let s_55_0: i128 = 2850;
        // C s_55_1: const #14696u : u32
        let s_55_1: u32 = 14696;
        // N s_55_2: write-reg s_55_1 <= s_55_0
        let s_55_2: () = {
            state.write_register::<i128>(s_55_1 as isize, s_55_0);
            tracer.write_register(s_55_1 as isize, s_55_0);
        };
        // C s_55_3: const #3s : i
        let s_55_3: i128 = 3;
        // C s_55_4: const #3s : i
        let s_55_4: i128 = 3;
        // D s_55_5: read-var u#38228:u16
        let s_55_5: u16 = fn_state.u_38228;
        // D s_55_6: cast zx s_55_5 -> bv
        let s_55_6: Bits = Bits::new(s_55_5 as u128, 16u16);
        // D s_55_7: bit-extract s_55_6 s_55_3 s_55_4
        let s_55_7: Bits = (Bits::new(
            ((s_55_6) >> (s_55_3)).value(),
            u16::try_from(s_55_4).unwrap(),
        ));
        // D s_55_8: cast reint s_55_7 -> u8
        let s_55_8: u8 = (s_55_7.value() as u8);
        // C s_55_9: const #0s : i
        let s_55_9: i128 = 0;
        // C s_55_10: const #3s : i
        let s_55_10: i128 = 3;
        // D s_55_11: read-var u#38228:u16
        let s_55_11: u16 = fn_state.u_38228;
        // D s_55_12: cast zx s_55_11 -> bv
        let s_55_12: Bits = Bits::new(s_55_11 as u128, 16u16);
        // D s_55_13: bit-extract s_55_12 s_55_9 s_55_10
        let s_55_13: Bits = (Bits::new(
            ((s_55_12) >> (s_55_9)).value(),
            u16::try_from(s_55_10).unwrap(),
        ));
        // D s_55_14: cast reint s_55_13 -> u8
        let s_55_14: u8 = (s_55_13.value() as u8);
        // D s_55_15: call decode_aarch32_instrs_ASR_r_T1enc_A_txt(s_55_8, s_55_14)
        let s_55_15: () = decode_aarch32_instrs_ASR_r_T1enc_A_txt(
            state,
            tracer,
            s_55_8,
            s_55_14,
        );
        // N s_55_16: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var merge#var.1:struct
        let s_56_0: u16 = fn_state.merge_var._1;
        // D s_56_1: write-var u#38232 <= s_56_0
        fn_state.u_38232 = s_56_0;
        // C s_56_2: const #12s : i
        let s_56_2: i128 = 12;
        // D s_56_3: read-var u#38232:u16
        let s_56_3: u16 = fn_state.u_38232;
        // D s_56_4: cast zx s_56_3 -> bv
        let s_56_4: Bits = Bits::new(s_56_3 as u128, 16u16);
        // C s_56_5: const #1s : i64
        let s_56_5: i64 = 1;
        // C s_56_6: cast zx s_56_5 -> i
        let s_56_6: i128 = (i128::try_from(s_56_5).unwrap());
        // C s_56_7: const #3s : i
        let s_56_7: i128 = 3;
        // C s_56_8: add s_56_7 s_56_6
        let s_56_8: i128 = (s_56_7 + s_56_6);
        // D s_56_9: bit-extract s_56_4 s_56_2 s_56_8
        let s_56_9: Bits = (Bits::new(
            ((s_56_4) >> (s_56_2)).value(),
            u16::try_from(s_56_8).unwrap(),
        ));
        // D s_56_10: cast reint s_56_9 -> u8
        let s_56_10: u8 = (s_56_9.value() as u8);
        // D s_56_11: cast zx s_56_10 -> bv
        let s_56_11: Bits = Bits::new(s_56_10 as u128, 4u16);
        // C s_56_12: const #13u : u8
        let s_56_12: u8 = 13;
        // C s_56_13: cast zx s_56_12 -> bv
        let s_56_13: Bits = Bits::new(s_56_12 as u128, 4u16);
        // D s_56_14: cmp-eq s_56_11 s_56_13
        let s_56_14: bool = ((s_56_11) == (s_56_13));
        // N s_56_15: branch s_56_14 b472 b57
        if s_56_14 {
            return block_472(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#428053 <= s_57_0
        fn_state.gs_428053 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#428053:u8
        let s_58_0: bool = fn_state.gs_428053;
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
        // C s_59_0: const #2852s : i
        let s_59_0: i128 = 2852;
        // C s_59_1: const #14696u : u32
        let s_59_1: u32 = 14696;
        // N s_59_2: write-reg s_59_1 <= s_59_0
        let s_59_2: () = {
            state.write_register::<i128>(s_59_1 as isize, s_59_0);
            tracer.write_register(s_59_1 as isize, s_59_0);
        };
        // C s_59_3: const #8s : i
        let s_59_3: i128 = 8;
        // C s_59_4: const #4s : i
        let s_59_4: i128 = 4;
        // D s_59_5: read-var u#38232:u16
        let s_59_5: u16 = fn_state.u_38232;
        // D s_59_6: cast zx s_59_5 -> bv
        let s_59_6: Bits = Bits::new(s_59_5 as u128, 16u16);
        // D s_59_7: bit-extract s_59_6 s_59_3 s_59_4
        let s_59_7: Bits = (Bits::new(
            ((s_59_6) >> (s_59_3)).value(),
            u16::try_from(s_59_4).unwrap(),
        ));
        // D s_59_8: cast reint s_59_7 -> u8
        let s_59_8: u8 = (s_59_7.value() as u8);
        // C s_59_9: const #0s : i
        let s_59_9: i128 = 0;
        // C s_59_10: const #8s : i
        let s_59_10: i128 = 8;
        // D s_59_11: read-var u#38232:u16
        let s_59_11: u16 = fn_state.u_38232;
        // D s_59_12: cast zx s_59_11 -> bv
        let s_59_12: Bits = Bits::new(s_59_11 as u128, 16u16);
        // D s_59_13: bit-extract s_59_12 s_59_9 s_59_10
        let s_59_13: Bits = (Bits::new(
            ((s_59_12) >> (s_59_9)).value(),
            u16::try_from(s_59_10).unwrap(),
        ));
        // D s_59_14: cast reint s_59_13 -> u8
        let s_59_14: u8 = (s_59_13.value() as u8);
        // D s_59_15: call decode_aarch32_instrs_B_T1enc_A_txt(s_59_8, s_59_14)
        let s_59_15: () = decode_aarch32_instrs_B_T1enc_A_txt(
            state,
            tracer,
            s_59_8,
            s_59_14,
        );
        // N s_59_16: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var merge#var.1:struct
        let s_60_0: u16 = fn_state.merge_var._1;
        // D s_60_1: write-var u#38235 <= s_60_0
        fn_state.u_38235 = s_60_0;
        // C s_60_2: const #11s : i
        let s_60_2: i128 = 11;
        // D s_60_3: read-var u#38235:u16
        let s_60_3: u16 = fn_state.u_38235;
        // D s_60_4: cast zx s_60_3 -> bv
        let s_60_4: Bits = Bits::new(s_60_3 as u128, 16u16);
        // C s_60_5: const #1s : i64
        let s_60_5: i64 = 1;
        // C s_60_6: cast zx s_60_5 -> i
        let s_60_6: i128 = (i128::try_from(s_60_5).unwrap());
        // C s_60_7: const #4s : i
        let s_60_7: i128 = 4;
        // C s_60_8: add s_60_7 s_60_6
        let s_60_8: i128 = (s_60_7 + s_60_6);
        // D s_60_9: bit-extract s_60_4 s_60_2 s_60_8
        let s_60_9: Bits = (Bits::new(
            ((s_60_4) >> (s_60_2)).value(),
            u16::try_from(s_60_8).unwrap(),
        ));
        // D s_60_10: cast reint s_60_9 -> u8
        let s_60_10: u8 = (s_60_9.value() as u8);
        // D s_60_11: cast zx s_60_10 -> bv
        let s_60_11: Bits = Bits::new(s_60_10 as u128, 5u16);
        // C s_60_12: const #28u : u8
        let s_60_12: u8 = 28;
        // C s_60_13: cast zx s_60_12 -> bv
        let s_60_13: Bits = Bits::new(s_60_12 as u128, 5u16);
        // D s_60_14: cmp-eq s_60_11 s_60_13
        let s_60_14: bool = ((s_60_11) == (s_60_13));
        // N s_60_15: branch s_60_14 b471 b61
        if s_60_14 {
            return block_471(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#428064 <= s_61_0
        fn_state.gs_428064 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#428064:u8
        let s_62_0: bool = fn_state.gs_428064;
        // D s_62_1: not s_62_0
        let s_62_1: bool = !s_62_0;
        // N s_62_2: branch s_62_1 b64 b63
        if s_62_1 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #2853s : i
        let s_63_0: i128 = 2853;
        // C s_63_1: const #14696u : u32
        let s_63_1: u32 = 14696;
        // N s_63_2: write-reg s_63_1 <= s_63_0
        let s_63_2: () = {
            state.write_register::<i128>(s_63_1 as isize, s_63_0);
            tracer.write_register(s_63_1 as isize, s_63_0);
        };
        // C s_63_3: const #0s : i
        let s_63_3: i128 = 0;
        // C s_63_4: const #11s : i
        let s_63_4: i128 = 11;
        // D s_63_5: read-var u#38235:u16
        let s_63_5: u16 = fn_state.u_38235;
        // D s_63_6: cast zx s_63_5 -> bv
        let s_63_6: Bits = Bits::new(s_63_5 as u128, 16u16);
        // D s_63_7: bit-extract s_63_6 s_63_3 s_63_4
        let s_63_7: Bits = (Bits::new(
            ((s_63_6) >> (s_63_3)).value(),
            u16::try_from(s_63_4).unwrap(),
        ));
        // D s_63_8: cast reint s_63_7 -> u11
        let s_63_8: u16 = (s_63_7.value() as u16);
        // D s_63_9: call decode_aarch32_instrs_B_T2enc_A_txt(s_63_8)
        let s_63_9: () = decode_aarch32_instrs_B_T2enc_A_txt(state, tracer, s_63_8);
        // N s_63_10: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var merge#var.1:struct
        let s_64_0: u16 = fn_state.merge_var._1;
        // D s_64_1: write-var u#38237 <= s_64_0
        fn_state.u_38237 = s_64_0;
        // C s_64_2: const #6s : i
        let s_64_2: i128 = 6;
        // D s_64_3: read-var u#38237:u16
        let s_64_3: u16 = fn_state.u_38237;
        // D s_64_4: cast zx s_64_3 -> bv
        let s_64_4: Bits = Bits::new(s_64_3 as u128, 16u16);
        // C s_64_5: const #1s : i64
        let s_64_5: i64 = 1;
        // C s_64_6: cast zx s_64_5 -> i
        let s_64_6: i128 = (i128::try_from(s_64_5).unwrap());
        // C s_64_7: const #9s : i
        let s_64_7: i128 = 9;
        // C s_64_8: add s_64_7 s_64_6
        let s_64_8: i128 = (s_64_7 + s_64_6);
        // D s_64_9: bit-extract s_64_4 s_64_2 s_64_8
        let s_64_9: Bits = (Bits::new(
            ((s_64_4) >> (s_64_2)).value(),
            u16::try_from(s_64_8).unwrap(),
        ));
        // D s_64_10: cast reint s_64_9 -> u10
        let s_64_10: u16 = (s_64_9.value() as u16);
        // D s_64_11: cast zx s_64_10 -> bv
        let s_64_11: Bits = Bits::new(s_64_10 as u128, 10u16);
        // C s_64_12: const #270u : u10
        let s_64_12: u16 = 270;
        // C s_64_13: cast zx s_64_12 -> bv
        let s_64_13: Bits = Bits::new(s_64_12 as u128, 10u16);
        // D s_64_14: cmp-eq s_64_11 s_64_13
        let s_64_14: bool = ((s_64_11) == (s_64_13));
        // N s_64_15: branch s_64_14 b470 b65
        if s_64_14 {
            return block_470(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#428073 <= s_65_0
        fn_state.gs_428073 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#428073:u8
        let s_66_0: bool = fn_state.gs_428073;
        // D s_66_1: not s_66_0
        let s_66_1: bool = !s_66_0;
        // N s_66_2: branch s_66_1 b68 b67
        if s_66_1 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #2863s : i
        let s_67_0: i128 = 2863;
        // C s_67_1: const #14696u : u32
        let s_67_1: u32 = 14696;
        // N s_67_2: write-reg s_67_1 <= s_67_0
        let s_67_2: () = {
            state.write_register::<i128>(s_67_1 as isize, s_67_0);
            tracer.write_register(s_67_1 as isize, s_67_0);
        };
        // C s_67_3: const #3s : i
        let s_67_3: i128 = 3;
        // C s_67_4: const #3s : i
        let s_67_4: i128 = 3;
        // D s_67_5: read-var u#38237:u16
        let s_67_5: u16 = fn_state.u_38237;
        // D s_67_6: cast zx s_67_5 -> bv
        let s_67_6: Bits = Bits::new(s_67_5 as u128, 16u16);
        // D s_67_7: bit-extract s_67_6 s_67_3 s_67_4
        let s_67_7: Bits = (Bits::new(
            ((s_67_6) >> (s_67_3)).value(),
            u16::try_from(s_67_4).unwrap(),
        ));
        // D s_67_8: cast reint s_67_7 -> u8
        let s_67_8: u8 = (s_67_7.value() as u8);
        // C s_67_9: const #0s : i
        let s_67_9: i128 = 0;
        // C s_67_10: const #3s : i
        let s_67_10: i128 = 3;
        // D s_67_11: read-var u#38237:u16
        let s_67_11: u16 = fn_state.u_38237;
        // D s_67_12: cast zx s_67_11 -> bv
        let s_67_12: Bits = Bits::new(s_67_11 as u128, 16u16);
        // D s_67_13: bit-extract s_67_12 s_67_9 s_67_10
        let s_67_13: Bits = (Bits::new(
            ((s_67_12) >> (s_67_9)).value(),
            u16::try_from(s_67_10).unwrap(),
        ));
        // D s_67_14: cast reint s_67_13 -> u8
        let s_67_14: u8 = (s_67_13.value() as u8);
        // D s_67_15: call decode_aarch32_instrs_BIC_r_T1enc_A_txt(s_67_8, s_67_14)
        let s_67_15: () = decode_aarch32_instrs_BIC_r_T1enc_A_txt(
            state,
            tracer,
            s_67_8,
            s_67_14,
        );
        // N s_67_16: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var merge#var.1:struct
        let s_68_0: u16 = fn_state.merge_var._1;
        // D s_68_1: write-var u#38241 <= s_68_0
        fn_state.u_38241 = s_68_0;
        // C s_68_2: const #8s : i
        let s_68_2: i128 = 8;
        // D s_68_3: read-var u#38241:u16
        let s_68_3: u16 = fn_state.u_38241;
        // D s_68_4: cast zx s_68_3 -> bv
        let s_68_4: Bits = Bits::new(s_68_3 as u128, 16u16);
        // C s_68_5: const #1s : i64
        let s_68_5: i64 = 1;
        // C s_68_6: cast zx s_68_5 -> i
        let s_68_6: i128 = (i128::try_from(s_68_5).unwrap());
        // C s_68_7: const #7s : i
        let s_68_7: i128 = 7;
        // C s_68_8: add s_68_7 s_68_6
        let s_68_8: i128 = (s_68_7 + s_68_6);
        // D s_68_9: bit-extract s_68_4 s_68_2 s_68_8
        let s_68_9: Bits = (Bits::new(
            ((s_68_4) >> (s_68_2)).value(),
            u16::try_from(s_68_8).unwrap(),
        ));
        // D s_68_10: cast reint s_68_9 -> u8
        let s_68_10: u8 = (s_68_9.value() as u8);
        // D s_68_11: cast zx s_68_10 -> bv
        let s_68_11: Bits = Bits::new(s_68_10 as u128, 8u16);
        // C s_68_12: const #190u : u8
        let s_68_12: u8 = 190;
        // C s_68_13: cast zx s_68_12 -> bv
        let s_68_13: Bits = Bits::new(s_68_12 as u128, 8u16);
        // D s_68_14: cmp-eq s_68_11 s_68_13
        let s_68_14: bool = ((s_68_11) == (s_68_13));
        // N s_68_15: branch s_68_14 b469 b69
        if s_68_14 {
            return block_469(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#428084 <= s_69_0
        fn_state.gs_428084 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#428084:u8
        let s_70_0: bool = fn_state.gs_428084;
        // D s_70_1: not s_70_0
        let s_70_1: bool = !s_70_0;
        // N s_70_2: branch s_70_1 b72 b71
        if s_70_1 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #2867s : i
        let s_71_0: i128 = 2867;
        // C s_71_1: const #14696u : u32
        let s_71_1: u32 = 14696;
        // N s_71_2: write-reg s_71_1 <= s_71_0
        let s_71_2: () = {
            state.write_register::<i128>(s_71_1 as isize, s_71_0);
            tracer.write_register(s_71_1 as isize, s_71_0);
        };
        // C s_71_3: const #0s : i
        let s_71_3: i128 = 0;
        // C s_71_4: const #8s : i
        let s_71_4: i128 = 8;
        // D s_71_5: read-var u#38241:u16
        let s_71_5: u16 = fn_state.u_38241;
        // D s_71_6: cast zx s_71_5 -> bv
        let s_71_6: Bits = Bits::new(s_71_5 as u128, 16u16);
        // D s_71_7: bit-extract s_71_6 s_71_3 s_71_4
        let s_71_7: Bits = (Bits::new(
            ((s_71_6) >> (s_71_3)).value(),
            u16::try_from(s_71_4).unwrap(),
        ));
        // D s_71_8: cast reint s_71_7 -> u8
        let s_71_8: u8 = (s_71_7.value() as u8);
        // D s_71_9: call decode_aarch32_instrs_BKPT_T1enc_A_txt(s_71_8)
        let s_71_9: () = decode_aarch32_instrs_BKPT_T1enc_A_txt(state, tracer, s_71_8);
        // N s_71_10: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var merge#var.1:struct
        let s_72_0: u16 = fn_state.merge_var._1;
        // D s_72_1: write-var u#38244 <= s_72_0
        fn_state.u_38244 = s_72_0;
        // C s_72_2: const #7s : i
        let s_72_2: i128 = 7;
        // D s_72_3: read-var u#38244:u16
        let s_72_3: u16 = fn_state.u_38244;
        // D s_72_4: cast zx s_72_3 -> bv
        let s_72_4: Bits = Bits::new(s_72_3 as u128, 16u16);
        // C s_72_5: const #1s : i64
        let s_72_5: i64 = 1;
        // C s_72_6: cast zx s_72_5 -> i
        let s_72_6: i128 = (i128::try_from(s_72_5).unwrap());
        // C s_72_7: const #8s : i
        let s_72_7: i128 = 8;
        // C s_72_8: add s_72_7 s_72_6
        let s_72_8: i128 = (s_72_7 + s_72_6);
        // D s_72_9: bit-extract s_72_4 s_72_2 s_72_8
        let s_72_9: Bits = (Bits::new(
            ((s_72_4) >> (s_72_2)).value(),
            u16::try_from(s_72_8).unwrap(),
        ));
        // D s_72_10: cast reint s_72_9 -> u9
        let s_72_10: u16 = (s_72_9.value() as u16);
        // D s_72_11: cast zx s_72_10 -> bv
        let s_72_11: Bits = Bits::new(s_72_10 as u128, 9u16);
        // C s_72_12: const #143u : u9
        let s_72_12: u16 = 143;
        // C s_72_13: cast zx s_72_12 -> bv
        let s_72_13: Bits = Bits::new(s_72_12 as u128, 9u16);
        // D s_72_14: cmp-eq s_72_11 s_72_13
        let s_72_14: bool = ((s_72_11) == (s_72_13));
        // N s_72_15: branch s_72_14 b468 b73
        if s_72_14 {
            return block_468(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#428094 <= s_73_0
        fn_state.gs_428094 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#428094:u8
        let s_74_0: bool = fn_state.gs_428094;
        // N s_74_1: branch s_74_0 b467 b75
        if s_74_0 {
            return block_467(state, tracer, fn_state);
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
        // D s_75_1: write-var gs#428096 <= s_75_0
        fn_state.gs_428096 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#428096:u8
        let s_76_0: bool = fn_state.gs_428096;
        // D s_76_1: not s_76_0
        let s_76_1: bool = !s_76_0;
        // N s_76_2: branch s_76_1 b86 b77
        if s_76_1 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #2873s : i
        let s_77_0: i128 = 2873;
        // C s_77_1: const #14696u : u32
        let s_77_1: u32 = 14696;
        // N s_77_2: write-reg s_77_1 <= s_77_0
        let s_77_2: () = {
            state.write_register::<i128>(s_77_1 as isize, s_77_0);
            tracer.write_register(s_77_1 as isize, s_77_0);
        };
        // C s_77_3: const #3s : i
        let s_77_3: i128 = 3;
        // C s_77_4: const #4s : i
        let s_77_4: i128 = 4;
        // D s_77_5: read-var u#38244:u16
        let s_77_5: u16 = fn_state.u_38244;
        // D s_77_6: cast zx s_77_5 -> bv
        let s_77_6: Bits = Bits::new(s_77_5 as u128, 16u16);
        // D s_77_7: bit-extract s_77_6 s_77_3 s_77_4
        let s_77_7: Bits = (Bits::new(
            ((s_77_6) >> (s_77_3)).value(),
            u16::try_from(s_77_4).unwrap(),
        ));
        // D s_77_8: cast reint s_77_7 -> u8
        let s_77_8: u8 = (s_77_7.value() as u8);
        // D s_77_9: write-var u#38245 <= s_77_8
        fn_state.u_38245 = s_77_8;
        // C s_77_10: const #0s : i
        let s_77_10: i128 = 0;
        // D s_77_11: read-var u#38244:u16
        let s_77_11: u16 = fn_state.u_38244;
        // D s_77_12: cast zx s_77_11 -> bv
        let s_77_12: Bits = Bits::new(s_77_11 as u128, 16u16);
        // C s_77_13: const #1u : u64
        let s_77_13: u64 = 1;
        // D s_77_14: bit-extract s_77_12 s_77_10 s_77_13
        let s_77_14: Bits = (Bits::new(
            ((s_77_12) >> (s_77_10)).value(),
            u16::try_from(s_77_13).unwrap(),
        ));
        // D s_77_15: cast reint s_77_14 -> u8
        let s_77_15: bool = ((s_77_14.value()) != 0);
        // C s_77_16: const #0s : i
        let s_77_16: i128 = 0;
        // C s_77_17: const #0u : u64
        let s_77_17: u64 = 0;
        // D s_77_18: cast zx s_77_15 -> u64
        let s_77_18: u64 = (s_77_15 as u64);
        // C s_77_19: const #1u : u64
        let s_77_19: u64 = 1;
        // D s_77_20: and s_77_18 s_77_19
        let s_77_20: u64 = ((s_77_18) & (s_77_19));
        // D s_77_21: cmp-eq s_77_20 s_77_19
        let s_77_21: bool = ((s_77_20) == (s_77_19));
        // D s_77_22: lsl s_77_18 s_77_16
        let s_77_22: u64 = s_77_18 << s_77_16;
        // D s_77_23: or s_77_17 s_77_22
        let s_77_23: u64 = ((s_77_17) | (s_77_22));
        // D s_77_24: cmpl s_77_22
        let s_77_24: u64 = !s_77_22;
        // D s_77_25: and s_77_17 s_77_24
        let s_77_25: u64 = ((s_77_17) & (s_77_24));
        // D s_77_26: select s_77_21 s_77_23 s_77_25
        let s_77_26: u64 = if s_77_21 { s_77_23 } else { s_77_25 };
        // D s_77_27: cast trunc s_77_26 -> u8
        let s_77_27: bool = ((s_77_26) != 0);
        // D s_77_28: cast zx s_77_27 -> bv
        let s_77_28: Bits = Bits::new(s_77_27 as u128, 1u16);
        // C s_77_29: const #0u : u8
        let s_77_29: bool = false;
        // C s_77_30: cast zx s_77_29 -> bv
        let s_77_30: Bits = Bits::new(s_77_29 as u128, 1u16);
        // D s_77_31: cmp-ne s_77_28 s_77_30
        let s_77_31: bool = ((s_77_28) != (s_77_30));
        // N s_77_32: branch s_77_31 b85 b78
        if s_77_31 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #1s : i
        let s_78_0: i128 = 1;
        // D s_78_1: read-var u#38244:u16
        let s_78_1: u16 = fn_state.u_38244;
        // D s_78_2: cast zx s_78_1 -> bv
        let s_78_2: Bits = Bits::new(s_78_1 as u128, 16u16);
        // C s_78_3: const #1u : u64
        let s_78_3: u64 = 1;
        // D s_78_4: bit-extract s_78_2 s_78_0 s_78_3
        let s_78_4: Bits = (Bits::new(
            ((s_78_2) >> (s_78_0)).value(),
            u16::try_from(s_78_3).unwrap(),
        ));
        // D s_78_5: cast reint s_78_4 -> u8
        let s_78_5: bool = ((s_78_4.value()) != 0);
        // C s_78_6: const #0s : i
        let s_78_6: i128 = 0;
        // C s_78_7: const #0u : u64
        let s_78_7: u64 = 0;
        // D s_78_8: cast zx s_78_5 -> u64
        let s_78_8: u64 = (s_78_5 as u64);
        // C s_78_9: const #1u : u64
        let s_78_9: u64 = 1;
        // D s_78_10: and s_78_8 s_78_9
        let s_78_10: u64 = ((s_78_8) & (s_78_9));
        // D s_78_11: cmp-eq s_78_10 s_78_9
        let s_78_11: bool = ((s_78_10) == (s_78_9));
        // D s_78_12: lsl s_78_8 s_78_6
        let s_78_12: u64 = s_78_8 << s_78_6;
        // D s_78_13: or s_78_7 s_78_12
        let s_78_13: u64 = ((s_78_7) | (s_78_12));
        // D s_78_14: cmpl s_78_12
        let s_78_14: u64 = !s_78_12;
        // D s_78_15: and s_78_7 s_78_14
        let s_78_15: u64 = ((s_78_7) & (s_78_14));
        // D s_78_16: select s_78_11 s_78_13 s_78_15
        let s_78_16: u64 = if s_78_11 { s_78_13 } else { s_78_15 };
        // D s_78_17: cast trunc s_78_16 -> u8
        let s_78_17: bool = ((s_78_16) != 0);
        // D s_78_18: cast zx s_78_17 -> bv
        let s_78_18: Bits = Bits::new(s_78_17 as u128, 1u16);
        // C s_78_19: const #0u : u8
        let s_78_19: bool = false;
        // C s_78_20: cast zx s_78_19 -> bv
        let s_78_20: Bits = Bits::new(s_78_19 as u128, 1u16);
        // D s_78_21: cmp-ne s_78_18 s_78_20
        let s_78_21: bool = ((s_78_18) != (s_78_20));
        // D s_78_22: write-var gs#428105 <= s_78_21
        fn_state.gs_428105 = s_78_21;
        // N s_78_23: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#428105:u8
        let s_79_0: bool = fn_state.gs_428105;
        // N s_79_1: branch s_79_0 b84 b80
        if s_79_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #2s : i
        let s_80_0: i128 = 2;
        // D s_80_1: read-var u#38244:u16
        let s_80_1: u16 = fn_state.u_38244;
        // D s_80_2: cast zx s_80_1 -> bv
        let s_80_2: Bits = Bits::new(s_80_1 as u128, 16u16);
        // C s_80_3: const #1u : u64
        let s_80_3: u64 = 1;
        // D s_80_4: bit-extract s_80_2 s_80_0 s_80_3
        let s_80_4: Bits = (Bits::new(
            ((s_80_2) >> (s_80_0)).value(),
            u16::try_from(s_80_3).unwrap(),
        ));
        // D s_80_5: cast reint s_80_4 -> u8
        let s_80_5: bool = ((s_80_4.value()) != 0);
        // C s_80_6: const #0s : i
        let s_80_6: i128 = 0;
        // C s_80_7: const #0u : u64
        let s_80_7: u64 = 0;
        // D s_80_8: cast zx s_80_5 -> u64
        let s_80_8: u64 = (s_80_5 as u64);
        // C s_80_9: const #1u : u64
        let s_80_9: u64 = 1;
        // D s_80_10: and s_80_8 s_80_9
        let s_80_10: u64 = ((s_80_8) & (s_80_9));
        // D s_80_11: cmp-eq s_80_10 s_80_9
        let s_80_11: bool = ((s_80_10) == (s_80_9));
        // D s_80_12: lsl s_80_8 s_80_6
        let s_80_12: u64 = s_80_8 << s_80_6;
        // D s_80_13: or s_80_7 s_80_12
        let s_80_13: u64 = ((s_80_7) | (s_80_12));
        // D s_80_14: cmpl s_80_12
        let s_80_14: u64 = !s_80_12;
        // D s_80_15: and s_80_7 s_80_14
        let s_80_15: u64 = ((s_80_7) & (s_80_14));
        // D s_80_16: select s_80_11 s_80_13 s_80_15
        let s_80_16: u64 = if s_80_11 { s_80_13 } else { s_80_15 };
        // D s_80_17: cast trunc s_80_16 -> u8
        let s_80_17: bool = ((s_80_16) != 0);
        // D s_80_18: cast zx s_80_17 -> bv
        let s_80_18: Bits = Bits::new(s_80_17 as u128, 1u16);
        // C s_80_19: const #0u : u8
        let s_80_19: bool = false;
        // C s_80_20: cast zx s_80_19 -> bv
        let s_80_20: Bits = Bits::new(s_80_19 as u128, 1u16);
        // D s_80_21: cmp-ne s_80_18 s_80_20
        let s_80_21: bool = ((s_80_18) != (s_80_20));
        // D s_80_22: write-var gs#428108 <= s_80_21
        fn_state.gs_428108 = s_80_21;
        // N s_80_23: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#428108:u8
        let s_81_0: bool = fn_state.gs_428108;
        // N s_81_1: branch s_81_0 b83 b82
        if s_81_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var u#38245:u8
        let s_82_0: u8 = fn_state.u_38245;
        // D s_82_1: call decode_aarch32_instrs_BLX_r_T1enc_A_txt(s_82_0)
        let s_82_1: () = decode_aarch32_instrs_BLX_r_T1enc_A_txt(state, tracer, s_82_0);
        // N s_82_2: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_83_0: panic
        panic!("{:?}", ());
        // N s_83_1: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #1u : u8
        let s_84_0: bool = true;
        // D s_84_1: write-var gs#428108 <= s_84_0
        fn_state.gs_428108 = s_84_0;
        // N s_84_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #1u : u8
        let s_85_0: bool = true;
        // D s_85_1: write-var gs#428105 <= s_85_0
        fn_state.gs_428105 = s_85_0;
        // N s_85_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var merge#var.1:struct
        let s_86_0: u16 = fn_state.merge_var._1;
        // D s_86_1: write-var u#38247 <= s_86_0
        fn_state.u_38247 = s_86_0;
        // C s_86_2: const #7s : i
        let s_86_2: i128 = 7;
        // D s_86_3: read-var u#38247:u16
        let s_86_3: u16 = fn_state.u_38247;
        // D s_86_4: cast zx s_86_3 -> bv
        let s_86_4: Bits = Bits::new(s_86_3 as u128, 16u16);
        // C s_86_5: const #1s : i64
        let s_86_5: i64 = 1;
        // C s_86_6: cast zx s_86_5 -> i
        let s_86_6: i128 = (i128::try_from(s_86_5).unwrap());
        // C s_86_7: const #8s : i
        let s_86_7: i128 = 8;
        // C s_86_8: add s_86_7 s_86_6
        let s_86_8: i128 = (s_86_7 + s_86_6);
        // D s_86_9: bit-extract s_86_4 s_86_2 s_86_8
        let s_86_9: Bits = (Bits::new(
            ((s_86_4) >> (s_86_2)).value(),
            u16::try_from(s_86_8).unwrap(),
        ));
        // D s_86_10: cast reint s_86_9 -> u9
        let s_86_10: u16 = (s_86_9.value() as u16);
        // D s_86_11: cast zx s_86_10 -> bv
        let s_86_11: Bits = Bits::new(s_86_10 as u128, 9u16);
        // C s_86_12: const #142u : u9
        let s_86_12: u16 = 142;
        // C s_86_13: cast zx s_86_12 -> bv
        let s_86_13: Bits = Bits::new(s_86_12 as u128, 9u16);
        // D s_86_14: cmp-eq s_86_11 s_86_13
        let s_86_14: bool = ((s_86_11) == (s_86_13));
        // N s_86_15: branch s_86_14 b466 b87
        if s_86_14 {
            return block_466(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#428114 <= s_87_0
        fn_state.gs_428114 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#428114:u8
        let s_88_0: bool = fn_state.gs_428114;
        // N s_88_1: branch s_88_0 b465 b89
        if s_88_0 {
            return block_465(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var gs#428116 <= s_89_0
        fn_state.gs_428116 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#428116:u8
        let s_90_0: bool = fn_state.gs_428116;
        // D s_90_1: not s_90_0
        let s_90_1: bool = !s_90_0;
        // N s_90_2: branch s_90_1 b100 b91
        if s_90_1 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #2875s : i
        let s_91_0: i128 = 2875;
        // C s_91_1: const #14696u : u32
        let s_91_1: u32 = 14696;
        // N s_91_2: write-reg s_91_1 <= s_91_0
        let s_91_2: () = {
            state.write_register::<i128>(s_91_1 as isize, s_91_0);
            tracer.write_register(s_91_1 as isize, s_91_0);
        };
        // C s_91_3: const #3s : i
        let s_91_3: i128 = 3;
        // C s_91_4: const #4s : i
        let s_91_4: i128 = 4;
        // D s_91_5: read-var u#38247:u16
        let s_91_5: u16 = fn_state.u_38247;
        // D s_91_6: cast zx s_91_5 -> bv
        let s_91_6: Bits = Bits::new(s_91_5 as u128, 16u16);
        // D s_91_7: bit-extract s_91_6 s_91_3 s_91_4
        let s_91_7: Bits = (Bits::new(
            ((s_91_6) >> (s_91_3)).value(),
            u16::try_from(s_91_4).unwrap(),
        ));
        // D s_91_8: cast reint s_91_7 -> u8
        let s_91_8: u8 = (s_91_7.value() as u8);
        // D s_91_9: write-var u#38248 <= s_91_8
        fn_state.u_38248 = s_91_8;
        // C s_91_10: const #0s : i
        let s_91_10: i128 = 0;
        // D s_91_11: read-var u#38247:u16
        let s_91_11: u16 = fn_state.u_38247;
        // D s_91_12: cast zx s_91_11 -> bv
        let s_91_12: Bits = Bits::new(s_91_11 as u128, 16u16);
        // C s_91_13: const #1u : u64
        let s_91_13: u64 = 1;
        // D s_91_14: bit-extract s_91_12 s_91_10 s_91_13
        let s_91_14: Bits = (Bits::new(
            ((s_91_12) >> (s_91_10)).value(),
            u16::try_from(s_91_13).unwrap(),
        ));
        // D s_91_15: cast reint s_91_14 -> u8
        let s_91_15: bool = ((s_91_14.value()) != 0);
        // C s_91_16: const #0s : i
        let s_91_16: i128 = 0;
        // C s_91_17: const #0u : u64
        let s_91_17: u64 = 0;
        // D s_91_18: cast zx s_91_15 -> u64
        let s_91_18: u64 = (s_91_15 as u64);
        // C s_91_19: const #1u : u64
        let s_91_19: u64 = 1;
        // D s_91_20: and s_91_18 s_91_19
        let s_91_20: u64 = ((s_91_18) & (s_91_19));
        // D s_91_21: cmp-eq s_91_20 s_91_19
        let s_91_21: bool = ((s_91_20) == (s_91_19));
        // D s_91_22: lsl s_91_18 s_91_16
        let s_91_22: u64 = s_91_18 << s_91_16;
        // D s_91_23: or s_91_17 s_91_22
        let s_91_23: u64 = ((s_91_17) | (s_91_22));
        // D s_91_24: cmpl s_91_22
        let s_91_24: u64 = !s_91_22;
        // D s_91_25: and s_91_17 s_91_24
        let s_91_25: u64 = ((s_91_17) & (s_91_24));
        // D s_91_26: select s_91_21 s_91_23 s_91_25
        let s_91_26: u64 = if s_91_21 { s_91_23 } else { s_91_25 };
        // D s_91_27: cast trunc s_91_26 -> u8
        let s_91_27: bool = ((s_91_26) != 0);
        // D s_91_28: cast zx s_91_27 -> bv
        let s_91_28: Bits = Bits::new(s_91_27 as u128, 1u16);
        // C s_91_29: const #0u : u8
        let s_91_29: bool = false;
        // C s_91_30: cast zx s_91_29 -> bv
        let s_91_30: Bits = Bits::new(s_91_29 as u128, 1u16);
        // D s_91_31: cmp-ne s_91_28 s_91_30
        let s_91_31: bool = ((s_91_28) != (s_91_30));
        // N s_91_32: branch s_91_31 b99 b92
        if s_91_31 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #1s : i
        let s_92_0: i128 = 1;
        // D s_92_1: read-var u#38247:u16
        let s_92_1: u16 = fn_state.u_38247;
        // D s_92_2: cast zx s_92_1 -> bv
        let s_92_2: Bits = Bits::new(s_92_1 as u128, 16u16);
        // C s_92_3: const #1u : u64
        let s_92_3: u64 = 1;
        // D s_92_4: bit-extract s_92_2 s_92_0 s_92_3
        let s_92_4: Bits = (Bits::new(
            ((s_92_2) >> (s_92_0)).value(),
            u16::try_from(s_92_3).unwrap(),
        ));
        // D s_92_5: cast reint s_92_4 -> u8
        let s_92_5: bool = ((s_92_4.value()) != 0);
        // C s_92_6: const #0s : i
        let s_92_6: i128 = 0;
        // C s_92_7: const #0u : u64
        let s_92_7: u64 = 0;
        // D s_92_8: cast zx s_92_5 -> u64
        let s_92_8: u64 = (s_92_5 as u64);
        // C s_92_9: const #1u : u64
        let s_92_9: u64 = 1;
        // D s_92_10: and s_92_8 s_92_9
        let s_92_10: u64 = ((s_92_8) & (s_92_9));
        // D s_92_11: cmp-eq s_92_10 s_92_9
        let s_92_11: bool = ((s_92_10) == (s_92_9));
        // D s_92_12: lsl s_92_8 s_92_6
        let s_92_12: u64 = s_92_8 << s_92_6;
        // D s_92_13: or s_92_7 s_92_12
        let s_92_13: u64 = ((s_92_7) | (s_92_12));
        // D s_92_14: cmpl s_92_12
        let s_92_14: u64 = !s_92_12;
        // D s_92_15: and s_92_7 s_92_14
        let s_92_15: u64 = ((s_92_7) & (s_92_14));
        // D s_92_16: select s_92_11 s_92_13 s_92_15
        let s_92_16: u64 = if s_92_11 { s_92_13 } else { s_92_15 };
        // D s_92_17: cast trunc s_92_16 -> u8
        let s_92_17: bool = ((s_92_16) != 0);
        // D s_92_18: cast zx s_92_17 -> bv
        let s_92_18: Bits = Bits::new(s_92_17 as u128, 1u16);
        // C s_92_19: const #0u : u8
        let s_92_19: bool = false;
        // C s_92_20: cast zx s_92_19 -> bv
        let s_92_20: Bits = Bits::new(s_92_19 as u128, 1u16);
        // D s_92_21: cmp-ne s_92_18 s_92_20
        let s_92_21: bool = ((s_92_18) != (s_92_20));
        // D s_92_22: write-var gs#428125 <= s_92_21
        fn_state.gs_428125 = s_92_21;
        // N s_92_23: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#428125:u8
        let s_93_0: bool = fn_state.gs_428125;
        // N s_93_1: branch s_93_0 b98 b94
        if s_93_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #2s : i
        let s_94_0: i128 = 2;
        // D s_94_1: read-var u#38247:u16
        let s_94_1: u16 = fn_state.u_38247;
        // D s_94_2: cast zx s_94_1 -> bv
        let s_94_2: Bits = Bits::new(s_94_1 as u128, 16u16);
        // C s_94_3: const #1u : u64
        let s_94_3: u64 = 1;
        // D s_94_4: bit-extract s_94_2 s_94_0 s_94_3
        let s_94_4: Bits = (Bits::new(
            ((s_94_2) >> (s_94_0)).value(),
            u16::try_from(s_94_3).unwrap(),
        ));
        // D s_94_5: cast reint s_94_4 -> u8
        let s_94_5: bool = ((s_94_4.value()) != 0);
        // C s_94_6: const #0s : i
        let s_94_6: i128 = 0;
        // C s_94_7: const #0u : u64
        let s_94_7: u64 = 0;
        // D s_94_8: cast zx s_94_5 -> u64
        let s_94_8: u64 = (s_94_5 as u64);
        // C s_94_9: const #1u : u64
        let s_94_9: u64 = 1;
        // D s_94_10: and s_94_8 s_94_9
        let s_94_10: u64 = ((s_94_8) & (s_94_9));
        // D s_94_11: cmp-eq s_94_10 s_94_9
        let s_94_11: bool = ((s_94_10) == (s_94_9));
        // D s_94_12: lsl s_94_8 s_94_6
        let s_94_12: u64 = s_94_8 << s_94_6;
        // D s_94_13: or s_94_7 s_94_12
        let s_94_13: u64 = ((s_94_7) | (s_94_12));
        // D s_94_14: cmpl s_94_12
        let s_94_14: u64 = !s_94_12;
        // D s_94_15: and s_94_7 s_94_14
        let s_94_15: u64 = ((s_94_7) & (s_94_14));
        // D s_94_16: select s_94_11 s_94_13 s_94_15
        let s_94_16: u64 = if s_94_11 { s_94_13 } else { s_94_15 };
        // D s_94_17: cast trunc s_94_16 -> u8
        let s_94_17: bool = ((s_94_16) != 0);
        // D s_94_18: cast zx s_94_17 -> bv
        let s_94_18: Bits = Bits::new(s_94_17 as u128, 1u16);
        // C s_94_19: const #0u : u8
        let s_94_19: bool = false;
        // C s_94_20: cast zx s_94_19 -> bv
        let s_94_20: Bits = Bits::new(s_94_19 as u128, 1u16);
        // D s_94_21: cmp-ne s_94_18 s_94_20
        let s_94_21: bool = ((s_94_18) != (s_94_20));
        // D s_94_22: write-var gs#428128 <= s_94_21
        fn_state.gs_428128 = s_94_21;
        // N s_94_23: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#428128:u8
        let s_95_0: bool = fn_state.gs_428128;
        // N s_95_1: branch s_95_0 b97 b96
        if s_95_0 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var u#38248:u8
        let s_96_0: u8 = fn_state.u_38248;
        // D s_96_1: call decode_aarch32_instrs_BX_T1enc_A_txt(s_96_0)
        let s_96_1: () = decode_aarch32_instrs_BX_T1enc_A_txt(state, tracer, s_96_0);
        // N s_96_2: return
        return;
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_97_0: panic
        panic!("{:?}", ());
        // N s_97_1: return
        return;
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #1u : u8
        let s_98_0: bool = true;
        // D s_98_1: write-var gs#428128 <= s_98_0
        fn_state.gs_428128 = s_98_0;
        // N s_98_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #1u : u8
        let s_99_0: bool = true;
        // D s_99_1: write-var gs#428125 <= s_99_0
        fn_state.gs_428125 = s_99_0;
        // N s_99_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var merge#var.1:struct
        let s_100_0: u16 = fn_state.merge_var._1;
        // D s_100_1: write-var u#38250 <= s_100_0
        fn_state.u_38250 = s_100_0;
        // C s_100_2: const #12s : i
        let s_100_2: i128 = 12;
        // D s_100_3: read-var u#38250:u16
        let s_100_3: u16 = fn_state.u_38250;
        // D s_100_4: cast zx s_100_3 -> bv
        let s_100_4: Bits = Bits::new(s_100_3 as u128, 16u16);
        // C s_100_5: const #1s : i64
        let s_100_5: i64 = 1;
        // C s_100_6: cast zx s_100_5 -> i
        let s_100_6: i128 = (i128::try_from(s_100_5).unwrap());
        // C s_100_7: const #3s : i
        let s_100_7: i128 = 3;
        // C s_100_8: add s_100_7 s_100_6
        let s_100_8: i128 = (s_100_7 + s_100_6);
        // D s_100_9: bit-extract s_100_4 s_100_2 s_100_8
        let s_100_9: Bits = (Bits::new(
            ((s_100_4) >> (s_100_2)).value(),
            u16::try_from(s_100_8).unwrap(),
        ));
        // D s_100_10: cast reint s_100_9 -> u8
        let s_100_10: u8 = (s_100_9.value() as u8);
        // D s_100_11: cast zx s_100_10 -> bv
        let s_100_11: Bits = Bits::new(s_100_10 as u128, 4u16);
        // C s_100_12: const #11u : u8
        let s_100_12: u8 = 11;
        // C s_100_13: cast zx s_100_12 -> bv
        let s_100_13: Bits = Bits::new(s_100_12 as u128, 4u16);
        // D s_100_14: cmp-eq s_100_11 s_100_13
        let s_100_14: bool = ((s_100_11) == (s_100_13));
        // N s_100_15: branch s_100_14 b461 b101
        if s_100_14 {
            return block_461(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#428137 <= s_101_0
        fn_state.gs_428137 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#428137:u8
        let s_102_0: bool = fn_state.gs_428137;
        // N s_102_1: branch s_102_0 b460 b103
        if s_102_0 {
            return block_460(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#428139 <= s_103_0
        fn_state.gs_428139 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#428139:u8
        let s_104_0: bool = fn_state.gs_428139;
        // D s_104_1: not s_104_0
        let s_104_1: bool = !s_104_0;
        // N s_104_2: branch s_104_1 b106 b105
        if s_104_1 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #2878s : i
        let s_105_0: i128 = 2878;
        // C s_105_1: const #14696u : u32
        let s_105_1: u32 = 14696;
        // N s_105_2: write-reg s_105_1 <= s_105_0
        let s_105_2: () = {
            state.write_register::<i128>(s_105_1 as isize, s_105_0);
            tracer.write_register(s_105_1 as isize, s_105_0);
        };
        // C s_105_3: const #11s : i
        let s_105_3: i128 = 11;
        // C s_105_4: const #1s : i
        let s_105_4: i128 = 1;
        // D s_105_5: read-var u#38250:u16
        let s_105_5: u16 = fn_state.u_38250;
        // D s_105_6: cast zx s_105_5 -> bv
        let s_105_6: Bits = Bits::new(s_105_5 as u128, 16u16);
        // D s_105_7: bit-extract s_105_6 s_105_3 s_105_4
        let s_105_7: Bits = (Bits::new(
            ((s_105_6) >> (s_105_3)).value(),
            u16::try_from(s_105_4).unwrap(),
        ));
        // D s_105_8: cast reint s_105_7 -> u8
        let s_105_8: bool = ((s_105_7.value()) != 0);
        // C s_105_9: const #9s : i
        let s_105_9: i128 = 9;
        // C s_105_10: const #1s : i
        let s_105_10: i128 = 1;
        // D s_105_11: read-var u#38250:u16
        let s_105_11: u16 = fn_state.u_38250;
        // D s_105_12: cast zx s_105_11 -> bv
        let s_105_12: Bits = Bits::new(s_105_11 as u128, 16u16);
        // D s_105_13: bit-extract s_105_12 s_105_9 s_105_10
        let s_105_13: Bits = (Bits::new(
            ((s_105_12) >> (s_105_9)).value(),
            u16::try_from(s_105_10).unwrap(),
        ));
        // D s_105_14: cast reint s_105_13 -> u8
        let s_105_14: bool = ((s_105_13.value()) != 0);
        // C s_105_15: const #3s : i
        let s_105_15: i128 = 3;
        // C s_105_16: const #5s : i
        let s_105_16: i128 = 5;
        // D s_105_17: read-var u#38250:u16
        let s_105_17: u16 = fn_state.u_38250;
        // D s_105_18: cast zx s_105_17 -> bv
        let s_105_18: Bits = Bits::new(s_105_17 as u128, 16u16);
        // D s_105_19: bit-extract s_105_18 s_105_15 s_105_16
        let s_105_19: Bits = (Bits::new(
            ((s_105_18) >> (s_105_15)).value(),
            u16::try_from(s_105_16).unwrap(),
        ));
        // D s_105_20: cast reint s_105_19 -> u8
        let s_105_20: u8 = (s_105_19.value() as u8);
        // C s_105_21: const #0s : i
        let s_105_21: i128 = 0;
        // C s_105_22: const #3s : i
        let s_105_22: i128 = 3;
        // D s_105_23: read-var u#38250:u16
        let s_105_23: u16 = fn_state.u_38250;
        // D s_105_24: cast zx s_105_23 -> bv
        let s_105_24: Bits = Bits::new(s_105_23 as u128, 16u16);
        // D s_105_25: bit-extract s_105_24 s_105_21 s_105_22
        let s_105_25: Bits = (Bits::new(
            ((s_105_24) >> (s_105_21)).value(),
            u16::try_from(s_105_22).unwrap(),
        ));
        // D s_105_26: cast reint s_105_25 -> u8
        let s_105_26: u8 = (s_105_25.value() as u8);
        // D s_105_27: call decode_aarch32_instrs_CBNZ_T1enc_A_txt(s_105_8, s_105_14, s_105_20, s_105_26)
        let s_105_27: () = decode_aarch32_instrs_CBNZ_T1enc_A_txt(
            state,
            tracer,
            s_105_8,
            s_105_14,
            s_105_20,
            s_105_26,
        );
        // N s_105_28: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var merge#var.1:struct
        let s_106_0: u16 = fn_state.merge_var._1;
        // D s_106_1: write-var u#38254 <= s_106_0
        fn_state.u_38254 = s_106_0;
        // C s_106_2: const #6s : i
        let s_106_2: i128 = 6;
        // D s_106_3: read-var u#38254:u16
        let s_106_3: u16 = fn_state.u_38254;
        // D s_106_4: cast zx s_106_3 -> bv
        let s_106_4: Bits = Bits::new(s_106_3 as u128, 16u16);
        // C s_106_5: const #1s : i64
        let s_106_5: i64 = 1;
        // C s_106_6: cast zx s_106_5 -> i
        let s_106_6: i128 = (i128::try_from(s_106_5).unwrap());
        // C s_106_7: const #9s : i
        let s_106_7: i128 = 9;
        // C s_106_8: add s_106_7 s_106_6
        let s_106_8: i128 = (s_106_7 + s_106_6);
        // D s_106_9: bit-extract s_106_4 s_106_2 s_106_8
        let s_106_9: Bits = (Bits::new(
            ((s_106_4) >> (s_106_2)).value(),
            u16::try_from(s_106_8).unwrap(),
        ));
        // D s_106_10: cast reint s_106_9 -> u10
        let s_106_10: u16 = (s_106_9.value() as u16);
        // D s_106_11: cast zx s_106_10 -> bv
        let s_106_11: Bits = Bits::new(s_106_10 as u128, 10u16);
        // C s_106_12: const #267u : u10
        let s_106_12: u16 = 267;
        // C s_106_13: cast zx s_106_12 -> bv
        let s_106_13: Bits = Bits::new(s_106_12 as u128, 10u16);
        // D s_106_14: cmp-eq s_106_11 s_106_13
        let s_106_14: bool = ((s_106_11) == (s_106_13));
        // N s_106_15: branch s_106_14 b459 b107
        if s_106_14 {
            return block_459(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // D s_107_1: write-var gs#428154 <= s_107_0
        fn_state.gs_428154 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#428154:u8
        let s_108_0: bool = fn_state.gs_428154;
        // D s_108_1: not s_108_0
        let s_108_1: bool = !s_108_0;
        // N s_108_2: branch s_108_1 b110 b109
        if s_108_1 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #2886s : i
        let s_109_0: i128 = 2886;
        // C s_109_1: const #14696u : u32
        let s_109_1: u32 = 14696;
        // N s_109_2: write-reg s_109_1 <= s_109_0
        let s_109_2: () = {
            state.write_register::<i128>(s_109_1 as isize, s_109_0);
            tracer.write_register(s_109_1 as isize, s_109_0);
        };
        // C s_109_3: const #3s : i
        let s_109_3: i128 = 3;
        // C s_109_4: const #3s : i
        let s_109_4: i128 = 3;
        // D s_109_5: read-var u#38254:u16
        let s_109_5: u16 = fn_state.u_38254;
        // D s_109_6: cast zx s_109_5 -> bv
        let s_109_6: Bits = Bits::new(s_109_5 as u128, 16u16);
        // D s_109_7: bit-extract s_109_6 s_109_3 s_109_4
        let s_109_7: Bits = (Bits::new(
            ((s_109_6) >> (s_109_3)).value(),
            u16::try_from(s_109_4).unwrap(),
        ));
        // D s_109_8: cast reint s_109_7 -> u8
        let s_109_8: u8 = (s_109_7.value() as u8);
        // C s_109_9: const #0s : i
        let s_109_9: i128 = 0;
        // C s_109_10: const #3s : i
        let s_109_10: i128 = 3;
        // D s_109_11: read-var u#38254:u16
        let s_109_11: u16 = fn_state.u_38254;
        // D s_109_12: cast zx s_109_11 -> bv
        let s_109_12: Bits = Bits::new(s_109_11 as u128, 16u16);
        // D s_109_13: bit-extract s_109_12 s_109_9 s_109_10
        let s_109_13: Bits = (Bits::new(
            ((s_109_12) >> (s_109_9)).value(),
            u16::try_from(s_109_10).unwrap(),
        ));
        // D s_109_14: cast reint s_109_13 -> u8
        let s_109_14: u8 = (s_109_13.value() as u8);
        // D s_109_15: call decode_aarch32_instrs_CMN_r_T1enc_A_txt(s_109_8, s_109_14)
        let s_109_15: () = decode_aarch32_instrs_CMN_r_T1enc_A_txt(
            state,
            tracer,
            s_109_8,
            s_109_14,
        );
        // N s_109_16: return
        return;
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var merge#var.1:struct
        let s_110_0: u16 = fn_state.merge_var._1;
        // D s_110_1: write-var u#38258 <= s_110_0
        fn_state.u_38258 = s_110_0;
        // C s_110_2: const #11s : i
        let s_110_2: i128 = 11;
        // D s_110_3: read-var u#38258:u16
        let s_110_3: u16 = fn_state.u_38258;
        // D s_110_4: cast zx s_110_3 -> bv
        let s_110_4: Bits = Bits::new(s_110_3 as u128, 16u16);
        // C s_110_5: const #1s : i64
        let s_110_5: i64 = 1;
        // C s_110_6: cast zx s_110_5 -> i
        let s_110_6: i128 = (i128::try_from(s_110_5).unwrap());
        // C s_110_7: const #4s : i
        let s_110_7: i128 = 4;
        // C s_110_8: add s_110_7 s_110_6
        let s_110_8: i128 = (s_110_7 + s_110_6);
        // D s_110_9: bit-extract s_110_4 s_110_2 s_110_8
        let s_110_9: Bits = (Bits::new(
            ((s_110_4) >> (s_110_2)).value(),
            u16::try_from(s_110_8).unwrap(),
        ));
        // D s_110_10: cast reint s_110_9 -> u8
        let s_110_10: u8 = (s_110_9.value() as u8);
        // D s_110_11: cast zx s_110_10 -> bv
        let s_110_11: Bits = Bits::new(s_110_10 as u128, 5u16);
        // C s_110_12: const #5u : u8
        let s_110_12: u8 = 5;
        // C s_110_13: cast zx s_110_12 -> bv
        let s_110_13: Bits = Bits::new(s_110_12 as u128, 5u16);
        // D s_110_14: cmp-eq s_110_11 s_110_13
        let s_110_14: bool = ((s_110_11) == (s_110_13));
        // N s_110_15: branch s_110_14 b458 b111
        if s_110_14 {
            return block_458(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #0u : u8
        let s_111_0: bool = false;
        // D s_111_1: write-var gs#428165 <= s_111_0
        fn_state.gs_428165 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#428165:u8
        let s_112_0: bool = fn_state.gs_428165;
        // D s_112_1: not s_112_0
        let s_112_1: bool = !s_112_0;
        // N s_112_2: branch s_112_1 b114 b113
        if s_112_1 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #2890s : i
        let s_113_0: i128 = 2890;
        // C s_113_1: const #14696u : u32
        let s_113_1: u32 = 14696;
        // N s_113_2: write-reg s_113_1 <= s_113_0
        let s_113_2: () = {
            state.write_register::<i128>(s_113_1 as isize, s_113_0);
            tracer.write_register(s_113_1 as isize, s_113_0);
        };
        // C s_113_3: const #8s : i
        let s_113_3: i128 = 8;
        // C s_113_4: const #3s : i
        let s_113_4: i128 = 3;
        // D s_113_5: read-var u#38258:u16
        let s_113_5: u16 = fn_state.u_38258;
        // D s_113_6: cast zx s_113_5 -> bv
        let s_113_6: Bits = Bits::new(s_113_5 as u128, 16u16);
        // D s_113_7: bit-extract s_113_6 s_113_3 s_113_4
        let s_113_7: Bits = (Bits::new(
            ((s_113_6) >> (s_113_3)).value(),
            u16::try_from(s_113_4).unwrap(),
        ));
        // D s_113_8: cast reint s_113_7 -> u8
        let s_113_8: u8 = (s_113_7.value() as u8);
        // C s_113_9: const #0s : i
        let s_113_9: i128 = 0;
        // C s_113_10: const #8s : i
        let s_113_10: i128 = 8;
        // D s_113_11: read-var u#38258:u16
        let s_113_11: u16 = fn_state.u_38258;
        // D s_113_12: cast zx s_113_11 -> bv
        let s_113_12: Bits = Bits::new(s_113_11 as u128, 16u16);
        // D s_113_13: bit-extract s_113_12 s_113_9 s_113_10
        let s_113_13: Bits = (Bits::new(
            ((s_113_12) >> (s_113_9)).value(),
            u16::try_from(s_113_10).unwrap(),
        ));
        // D s_113_14: cast reint s_113_13 -> u8
        let s_113_14: u8 = (s_113_13.value() as u8);
        // D s_113_15: call decode_aarch32_instrs_CMP_i_T1enc_A_txt(s_113_8, s_113_14)
        let s_113_15: () = decode_aarch32_instrs_CMP_i_T1enc_A_txt(
            state,
            tracer,
            s_113_8,
            s_113_14,
        );
        // N s_113_16: return
        return;
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var merge#var.1:struct
        let s_114_0: u16 = fn_state.merge_var._1;
        // D s_114_1: write-var u#38262 <= s_114_0
        fn_state.u_38262 = s_114_0;
        // C s_114_2: const #6s : i
        let s_114_2: i128 = 6;
        // D s_114_3: read-var u#38262:u16
        let s_114_3: u16 = fn_state.u_38262;
        // D s_114_4: cast zx s_114_3 -> bv
        let s_114_4: Bits = Bits::new(s_114_3 as u128, 16u16);
        // C s_114_5: const #1s : i64
        let s_114_5: i64 = 1;
        // C s_114_6: cast zx s_114_5 -> i
        let s_114_6: i128 = (i128::try_from(s_114_5).unwrap());
        // C s_114_7: const #9s : i
        let s_114_7: i128 = 9;
        // C s_114_8: add s_114_7 s_114_6
        let s_114_8: i128 = (s_114_7 + s_114_6);
        // D s_114_9: bit-extract s_114_4 s_114_2 s_114_8
        let s_114_9: Bits = (Bits::new(
            ((s_114_4) >> (s_114_2)).value(),
            u16::try_from(s_114_8).unwrap(),
        ));
        // D s_114_10: cast reint s_114_9 -> u10
        let s_114_10: u16 = (s_114_9.value() as u16);
        // D s_114_11: cast zx s_114_10 -> bv
        let s_114_11: Bits = Bits::new(s_114_10 as u128, 10u16);
        // C s_114_12: const #266u : u10
        let s_114_12: u16 = 266;
        // C s_114_13: cast zx s_114_12 -> bv
        let s_114_13: Bits = Bits::new(s_114_12 as u128, 10u16);
        // D s_114_14: cmp-eq s_114_11 s_114_13
        let s_114_14: bool = ((s_114_11) == (s_114_13));
        // N s_114_15: branch s_114_14 b457 b115
        if s_114_14 {
            return block_457(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #0u : u8
        let s_115_0: bool = false;
        // D s_115_1: write-var gs#428176 <= s_115_0
        fn_state.gs_428176 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#428176:u8
        let s_116_0: bool = fn_state.gs_428176;
        // D s_116_1: not s_116_0
        let s_116_1: bool = !s_116_0;
        // N s_116_2: branch s_116_1 b118 b117
        if s_116_1 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #2893s : i
        let s_117_0: i128 = 2893;
        // C s_117_1: const #14696u : u32
        let s_117_1: u32 = 14696;
        // N s_117_2: write-reg s_117_1 <= s_117_0
        let s_117_2: () = {
            state.write_register::<i128>(s_117_1 as isize, s_117_0);
            tracer.write_register(s_117_1 as isize, s_117_0);
        };
        // C s_117_3: const #3s : i
        let s_117_3: i128 = 3;
        // C s_117_4: const #3s : i
        let s_117_4: i128 = 3;
        // D s_117_5: read-var u#38262:u16
        let s_117_5: u16 = fn_state.u_38262;
        // D s_117_6: cast zx s_117_5 -> bv
        let s_117_6: Bits = Bits::new(s_117_5 as u128, 16u16);
        // D s_117_7: bit-extract s_117_6 s_117_3 s_117_4
        let s_117_7: Bits = (Bits::new(
            ((s_117_6) >> (s_117_3)).value(),
            u16::try_from(s_117_4).unwrap(),
        ));
        // D s_117_8: cast reint s_117_7 -> u8
        let s_117_8: u8 = (s_117_7.value() as u8);
        // C s_117_9: const #0s : i
        let s_117_9: i128 = 0;
        // C s_117_10: const #3s : i
        let s_117_10: i128 = 3;
        // D s_117_11: read-var u#38262:u16
        let s_117_11: u16 = fn_state.u_38262;
        // D s_117_12: cast zx s_117_11 -> bv
        let s_117_12: Bits = Bits::new(s_117_11 as u128, 16u16);
        // D s_117_13: bit-extract s_117_12 s_117_9 s_117_10
        let s_117_13: Bits = (Bits::new(
            ((s_117_12) >> (s_117_9)).value(),
            u16::try_from(s_117_10).unwrap(),
        ));
        // D s_117_14: cast reint s_117_13 -> u8
        let s_117_14: u8 = (s_117_13.value() as u8);
        // D s_117_15: call decode_aarch32_instrs_CMP_r_T1enc_A_txt(s_117_8, s_117_14)
        let s_117_15: () = decode_aarch32_instrs_CMP_r_T1enc_A_txt(
            state,
            tracer,
            s_117_8,
            s_117_14,
        );
        // N s_117_16: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var merge#var.1:struct
        let s_118_0: u16 = fn_state.merge_var._1;
        // D s_118_1: write-var u#38266 <= s_118_0
        fn_state.u_38266 = s_118_0;
        // C s_118_2: const #8s : i
        let s_118_2: i128 = 8;
        // D s_118_3: read-var u#38266:u16
        let s_118_3: u16 = fn_state.u_38266;
        // D s_118_4: cast zx s_118_3 -> bv
        let s_118_4: Bits = Bits::new(s_118_3 as u128, 16u16);
        // C s_118_5: const #1s : i64
        let s_118_5: i64 = 1;
        // C s_118_6: cast zx s_118_5 -> i
        let s_118_6: i128 = (i128::try_from(s_118_5).unwrap());
        // C s_118_7: const #7s : i
        let s_118_7: i128 = 7;
        // C s_118_8: add s_118_7 s_118_6
        let s_118_8: i128 = (s_118_7 + s_118_6);
        // D s_118_9: bit-extract s_118_4 s_118_2 s_118_8
        let s_118_9: Bits = (Bits::new(
            ((s_118_4) >> (s_118_2)).value(),
            u16::try_from(s_118_8).unwrap(),
        ));
        // D s_118_10: cast reint s_118_9 -> u8
        let s_118_10: u8 = (s_118_9.value() as u8);
        // D s_118_11: cast zx s_118_10 -> bv
        let s_118_11: Bits = Bits::new(s_118_10 as u128, 8u16);
        // C s_118_12: const #69u : u8
        let s_118_12: u8 = 69;
        // C s_118_13: cast zx s_118_12 -> bv
        let s_118_13: Bits = Bits::new(s_118_12 as u128, 8u16);
        // D s_118_14: cmp-eq s_118_11 s_118_13
        let s_118_14: bool = ((s_118_11) == (s_118_13));
        // N s_118_15: branch s_118_14 b456 b119
        if s_118_14 {
            return block_456(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#428187 <= s_119_0
        fn_state.gs_428187 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#428187:u8
        let s_120_0: bool = fn_state.gs_428187;
        // D s_120_1: not s_120_0
        let s_120_1: bool = !s_120_0;
        // N s_120_2: branch s_120_1 b122 b121
        if s_120_1 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #2894s : i
        let s_121_0: i128 = 2894;
        // C s_121_1: const #14696u : u32
        let s_121_1: u32 = 14696;
        // N s_121_2: write-reg s_121_1 <= s_121_0
        let s_121_2: () = {
            state.write_register::<i128>(s_121_1 as isize, s_121_0);
            tracer.write_register(s_121_1 as isize, s_121_0);
        };
        // C s_121_3: const #7s : i
        let s_121_3: i128 = 7;
        // C s_121_4: const #1s : i
        let s_121_4: i128 = 1;
        // D s_121_5: read-var u#38266:u16
        let s_121_5: u16 = fn_state.u_38266;
        // D s_121_6: cast zx s_121_5 -> bv
        let s_121_6: Bits = Bits::new(s_121_5 as u128, 16u16);
        // D s_121_7: bit-extract s_121_6 s_121_3 s_121_4
        let s_121_7: Bits = (Bits::new(
            ((s_121_6) >> (s_121_3)).value(),
            u16::try_from(s_121_4).unwrap(),
        ));
        // D s_121_8: cast reint s_121_7 -> u8
        let s_121_8: bool = ((s_121_7.value()) != 0);
        // C s_121_9: const #3s : i
        let s_121_9: i128 = 3;
        // C s_121_10: const #4s : i
        let s_121_10: i128 = 4;
        // D s_121_11: read-var u#38266:u16
        let s_121_11: u16 = fn_state.u_38266;
        // D s_121_12: cast zx s_121_11 -> bv
        let s_121_12: Bits = Bits::new(s_121_11 as u128, 16u16);
        // D s_121_13: bit-extract s_121_12 s_121_9 s_121_10
        let s_121_13: Bits = (Bits::new(
            ((s_121_12) >> (s_121_9)).value(),
            u16::try_from(s_121_10).unwrap(),
        ));
        // D s_121_14: cast reint s_121_13 -> u8
        let s_121_14: u8 = (s_121_13.value() as u8);
        // C s_121_15: const #0s : i
        let s_121_15: i128 = 0;
        // C s_121_16: const #3s : i
        let s_121_16: i128 = 3;
        // D s_121_17: read-var u#38266:u16
        let s_121_17: u16 = fn_state.u_38266;
        // D s_121_18: cast zx s_121_17 -> bv
        let s_121_18: Bits = Bits::new(s_121_17 as u128, 16u16);
        // D s_121_19: bit-extract s_121_18 s_121_15 s_121_16
        let s_121_19: Bits = (Bits::new(
            ((s_121_18) >> (s_121_15)).value(),
            u16::try_from(s_121_16).unwrap(),
        ));
        // D s_121_20: cast reint s_121_19 -> u8
        let s_121_20: u8 = (s_121_19.value() as u8);
        // D s_121_21: call decode_aarch32_instrs_CMP_r_T2enc_A_txt(s_121_8, s_121_14, s_121_20)
        let s_121_21: () = decode_aarch32_instrs_CMP_r_T2enc_A_txt(
            state,
            tracer,
            s_121_8,
            s_121_14,
            s_121_20,
        );
        // N s_121_22: return
        return;
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var merge#var.1:struct
        let s_122_0: u16 = fn_state.merge_var._1;
        // D s_122_1: write-var u#38270 <= s_122_0
        fn_state.u_38270 = s_122_0;
        // C s_122_2: const #6s : i
        let s_122_2: i128 = 6;
        // D s_122_3: read-var u#38270:u16
        let s_122_3: u16 = fn_state.u_38270;
        // D s_122_4: cast zx s_122_3 -> bv
        let s_122_4: Bits = Bits::new(s_122_3 as u128, 16u16);
        // C s_122_5: const #1s : i64
        let s_122_5: i64 = 1;
        // C s_122_6: cast zx s_122_5 -> i
        let s_122_6: i128 = (i128::try_from(s_122_5).unwrap());
        // C s_122_7: const #9s : i
        let s_122_7: i128 = 9;
        // C s_122_8: add s_122_7 s_122_6
        let s_122_8: i128 = (s_122_7 + s_122_6);
        // D s_122_9: bit-extract s_122_4 s_122_2 s_122_8
        let s_122_9: Bits = (Bits::new(
            ((s_122_4) >> (s_122_2)).value(),
            u16::try_from(s_122_8).unwrap(),
        ));
        // D s_122_10: cast reint s_122_9 -> u10
        let s_122_10: u16 = (s_122_9.value() as u16);
        // D s_122_11: cast zx s_122_10 -> bv
        let s_122_11: Bits = Bits::new(s_122_10 as u128, 10u16);
        // C s_122_12: const #257u : u10
        let s_122_12: u16 = 257;
        // C s_122_13: cast zx s_122_12 -> bv
        let s_122_13: Bits = Bits::new(s_122_12 as u128, 10u16);
        // D s_122_14: cmp-eq s_122_11 s_122_13
        let s_122_14: bool = ((s_122_11) == (s_122_13));
        // N s_122_15: branch s_122_14 b455 b123
        if s_122_14 {
            return block_455(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #0u : u8
        let s_123_0: bool = false;
        // D s_123_1: write-var gs#428200 <= s_123_0
        fn_state.gs_428200 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#428200:u8
        let s_124_0: bool = fn_state.gs_428200;
        // D s_124_1: not s_124_0
        let s_124_1: bool = !s_124_0;
        // N s_124_2: branch s_124_1 b126 b125
        if s_124_1 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #2906s : i
        let s_125_0: i128 = 2906;
        // C s_125_1: const #14696u : u32
        let s_125_1: u32 = 14696;
        // N s_125_2: write-reg s_125_1 <= s_125_0
        let s_125_2: () = {
            state.write_register::<i128>(s_125_1 as isize, s_125_0);
            tracer.write_register(s_125_1 as isize, s_125_0);
        };
        // C s_125_3: const #3s : i
        let s_125_3: i128 = 3;
        // C s_125_4: const #3s : i
        let s_125_4: i128 = 3;
        // D s_125_5: read-var u#38270:u16
        let s_125_5: u16 = fn_state.u_38270;
        // D s_125_6: cast zx s_125_5 -> bv
        let s_125_6: Bits = Bits::new(s_125_5 as u128, 16u16);
        // D s_125_7: bit-extract s_125_6 s_125_3 s_125_4
        let s_125_7: Bits = (Bits::new(
            ((s_125_6) >> (s_125_3)).value(),
            u16::try_from(s_125_4).unwrap(),
        ));
        // D s_125_8: cast reint s_125_7 -> u8
        let s_125_8: u8 = (s_125_7.value() as u8);
        // C s_125_9: const #0s : i
        let s_125_9: i128 = 0;
        // C s_125_10: const #3s : i
        let s_125_10: i128 = 3;
        // D s_125_11: read-var u#38270:u16
        let s_125_11: u16 = fn_state.u_38270;
        // D s_125_12: cast zx s_125_11 -> bv
        let s_125_12: Bits = Bits::new(s_125_11 as u128, 16u16);
        // D s_125_13: bit-extract s_125_12 s_125_9 s_125_10
        let s_125_13: Bits = (Bits::new(
            ((s_125_12) >> (s_125_9)).value(),
            u16::try_from(s_125_10).unwrap(),
        ));
        // D s_125_14: cast reint s_125_13 -> u8
        let s_125_14: u8 = (s_125_13.value() as u8);
        // D s_125_15: call decode_aarch32_instrs_EOR_r_T1enc_A_txt(s_125_8, s_125_14)
        let s_125_15: () = decode_aarch32_instrs_EOR_r_T1enc_A_txt(
            state,
            tracer,
            s_125_8,
            s_125_14,
        );
        // N s_125_16: return
        return;
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var merge#var.1:struct
        let s_126_0: u16 = fn_state.merge_var._1;
        // D s_126_1: write-var u#38274 <= s_126_0
        fn_state.u_38274 = s_126_0;
        // C s_126_2: const #8s : i
        let s_126_2: i128 = 8;
        // D s_126_3: read-var u#38274:u16
        let s_126_3: u16 = fn_state.u_38274;
        // D s_126_4: cast zx s_126_3 -> bv
        let s_126_4: Bits = Bits::new(s_126_3 as u128, 16u16);
        // C s_126_5: const #1s : i64
        let s_126_5: i64 = 1;
        // C s_126_6: cast zx s_126_5 -> i
        let s_126_6: i128 = (i128::try_from(s_126_5).unwrap());
        // C s_126_7: const #7s : i
        let s_126_7: i128 = 7;
        // C s_126_8: add s_126_7 s_126_6
        let s_126_8: i128 = (s_126_7 + s_126_6);
        // D s_126_9: bit-extract s_126_4 s_126_2 s_126_8
        let s_126_9: Bits = (Bits::new(
            ((s_126_4) >> (s_126_2)).value(),
            u16::try_from(s_126_8).unwrap(),
        ));
        // D s_126_10: cast reint s_126_9 -> u8
        let s_126_10: u8 = (s_126_9.value() as u8);
        // D s_126_11: cast zx s_126_10 -> bv
        let s_126_11: Bits = Bits::new(s_126_10 as u128, 8u16);
        // C s_126_12: const #191u : u8
        let s_126_12: u8 = 191;
        // C s_126_13: cast zx s_126_12 -> bv
        let s_126_13: Bits = Bits::new(s_126_12 as u128, 8u16);
        // D s_126_14: cmp-eq s_126_11 s_126_13
        let s_126_14: bool = ((s_126_11) == (s_126_13));
        // N s_126_15: branch s_126_14 b454 b127
        if s_126_14 {
            return block_454(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #0u : u8
        let s_127_0: bool = false;
        // D s_127_1: write-var gs#428211 <= s_127_0
        fn_state.gs_428211 = s_127_0;
        // N s_127_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#428211:u8
        let s_128_0: bool = fn_state.gs_428211;
        // D s_128_1: not s_128_0
        let s_128_1: bool = !s_128_0;
        // N s_128_2: branch s_128_1 b130 b129
        if s_128_1 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #2911s : i
        let s_129_0: i128 = 2911;
        // C s_129_1: const #14696u : u32
        let s_129_1: u32 = 14696;
        // N s_129_2: write-reg s_129_1 <= s_129_0
        let s_129_2: () = {
            state.write_register::<i128>(s_129_1 as isize, s_129_0);
            tracer.write_register(s_129_1 as isize, s_129_0);
        };
        // C s_129_3: const #4s : i
        let s_129_3: i128 = 4;
        // C s_129_4: const #4s : i
        let s_129_4: i128 = 4;
        // D s_129_5: read-var u#38274:u16
        let s_129_5: u16 = fn_state.u_38274;
        // D s_129_6: cast zx s_129_5 -> bv
        let s_129_6: Bits = Bits::new(s_129_5 as u128, 16u16);
        // D s_129_7: bit-extract s_129_6 s_129_3 s_129_4
        let s_129_7: Bits = (Bits::new(
            ((s_129_6) >> (s_129_3)).value(),
            u16::try_from(s_129_4).unwrap(),
        ));
        // D s_129_8: cast reint s_129_7 -> u8
        let s_129_8: u8 = (s_129_7.value() as u8);
        // C s_129_9: const #0s : i
        let s_129_9: i128 = 0;
        // C s_129_10: const #4s : i
        let s_129_10: i128 = 4;
        // D s_129_11: read-var u#38274:u16
        let s_129_11: u16 = fn_state.u_38274;
        // D s_129_12: cast zx s_129_11 -> bv
        let s_129_12: Bits = Bits::new(s_129_11 as u128, 16u16);
        // D s_129_13: bit-extract s_129_12 s_129_9 s_129_10
        let s_129_13: Bits = (Bits::new(
            ((s_129_12) >> (s_129_9)).value(),
            u16::try_from(s_129_10).unwrap(),
        ));
        // D s_129_14: cast reint s_129_13 -> u8
        let s_129_14: u8 = (s_129_13.value() as u8);
        // D s_129_15: call decode_aarch32_instrs_IT_T1enc_A_txt(s_129_8, s_129_14)
        let s_129_15: () = decode_aarch32_instrs_IT_T1enc_A_txt(
            state,
            tracer,
            s_129_8,
            s_129_14,
        );
        // N s_129_16: return
        return;
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var merge#var.1:struct
        let s_130_0: u16 = fn_state.merge_var._1;
        // D s_130_1: write-var u#38276 <= s_130_0
        fn_state.u_38276 = s_130_0;
        // C s_130_2: const #11s : i
        let s_130_2: i128 = 11;
        // D s_130_3: read-var u#38276:u16
        let s_130_3: u16 = fn_state.u_38276;
        // D s_130_4: cast zx s_130_3 -> bv
        let s_130_4: Bits = Bits::new(s_130_3 as u128, 16u16);
        // C s_130_5: const #1s : i64
        let s_130_5: i64 = 1;
        // C s_130_6: cast zx s_130_5 -> i
        let s_130_6: i128 = (i128::try_from(s_130_5).unwrap());
        // C s_130_7: const #4s : i
        let s_130_7: i128 = 4;
        // C s_130_8: add s_130_7 s_130_6
        let s_130_8: i128 = (s_130_7 + s_130_6);
        // D s_130_9: bit-extract s_130_4 s_130_2 s_130_8
        let s_130_9: Bits = (Bits::new(
            ((s_130_4) >> (s_130_2)).value(),
            u16::try_from(s_130_8).unwrap(),
        ));
        // D s_130_10: cast reint s_130_9 -> u8
        let s_130_10: u8 = (s_130_9.value() as u8);
        // D s_130_11: cast zx s_130_10 -> bv
        let s_130_11: Bits = Bits::new(s_130_10 as u128, 5u16);
        // C s_130_12: const #25u : u8
        let s_130_12: u8 = 25;
        // C s_130_13: cast zx s_130_12 -> bv
        let s_130_13: Bits = Bits::new(s_130_12 as u128, 5u16);
        // D s_130_14: cmp-eq s_130_11 s_130_13
        let s_130_14: bool = ((s_130_11) == (s_130_13));
        // N s_130_15: branch s_130_14 b453 b131
        if s_130_14 {
            return block_453(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #0u : u8
        let s_131_0: bool = false;
        // D s_131_1: write-var gs#428222 <= s_131_0
        fn_state.gs_428222 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#428222:u8
        let s_132_0: bool = fn_state.gs_428222;
        // D s_132_1: not s_132_0
        let s_132_1: bool = !s_132_0;
        // N s_132_2: branch s_132_1 b134 b133
        if s_132_1 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #2917s : i
        let s_133_0: i128 = 2917;
        // C s_133_1: const #14696u : u32
        let s_133_1: u32 = 14696;
        // N s_133_2: write-reg s_133_1 <= s_133_0
        let s_133_2: () = {
            state.write_register::<i128>(s_133_1 as isize, s_133_0);
            tracer.write_register(s_133_1 as isize, s_133_0);
        };
        // C s_133_3: const #8s : i
        let s_133_3: i128 = 8;
        // C s_133_4: const #3s : i
        let s_133_4: i128 = 3;
        // D s_133_5: read-var u#38276:u16
        let s_133_5: u16 = fn_state.u_38276;
        // D s_133_6: cast zx s_133_5 -> bv
        let s_133_6: Bits = Bits::new(s_133_5 as u128, 16u16);
        // D s_133_7: bit-extract s_133_6 s_133_3 s_133_4
        let s_133_7: Bits = (Bits::new(
            ((s_133_6) >> (s_133_3)).value(),
            u16::try_from(s_133_4).unwrap(),
        ));
        // D s_133_8: cast reint s_133_7 -> u8
        let s_133_8: u8 = (s_133_7.value() as u8);
        // C s_133_9: const #0s : i
        let s_133_9: i128 = 0;
        // C s_133_10: const #8s : i
        let s_133_10: i128 = 8;
        // D s_133_11: read-var u#38276:u16
        let s_133_11: u16 = fn_state.u_38276;
        // D s_133_12: cast zx s_133_11 -> bv
        let s_133_12: Bits = Bits::new(s_133_11 as u128, 16u16);
        // D s_133_13: bit-extract s_133_12 s_133_9 s_133_10
        let s_133_13: Bits = (Bits::new(
            ((s_133_12) >> (s_133_9)).value(),
            u16::try_from(s_133_10).unwrap(),
        ));
        // D s_133_14: cast reint s_133_13 -> u8
        let s_133_14: u8 = (s_133_13.value() as u8);
        // D s_133_15: call decode_aarch32_instrs_LDM_T1enc_A_txt(s_133_8, s_133_14)
        let s_133_15: () = decode_aarch32_instrs_LDM_T1enc_A_txt(
            state,
            tracer,
            s_133_8,
            s_133_14,
        );
        // N s_133_16: return
        return;
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var merge#var.1:struct
        let s_134_0: u16 = fn_state.merge_var._1;
        // D s_134_1: write-var u#38279 <= s_134_0
        fn_state.u_38279 = s_134_0;
        // C s_134_2: const #11s : i
        let s_134_2: i128 = 11;
        // D s_134_3: read-var u#38279:u16
        let s_134_3: u16 = fn_state.u_38279;
        // D s_134_4: cast zx s_134_3 -> bv
        let s_134_4: Bits = Bits::new(s_134_3 as u128, 16u16);
        // C s_134_5: const #1s : i64
        let s_134_5: i64 = 1;
        // C s_134_6: cast zx s_134_5 -> i
        let s_134_6: i128 = (i128::try_from(s_134_5).unwrap());
        // C s_134_7: const #4s : i
        let s_134_7: i128 = 4;
        // C s_134_8: add s_134_7 s_134_6
        let s_134_8: i128 = (s_134_7 + s_134_6);
        // D s_134_9: bit-extract s_134_4 s_134_2 s_134_8
        let s_134_9: Bits = (Bits::new(
            ((s_134_4) >> (s_134_2)).value(),
            u16::try_from(s_134_8).unwrap(),
        ));
        // D s_134_10: cast reint s_134_9 -> u8
        let s_134_10: u8 = (s_134_9.value() as u8);
        // D s_134_11: cast zx s_134_10 -> bv
        let s_134_11: Bits = Bits::new(s_134_10 as u128, 5u16);
        // C s_134_12: const #15u : u8
        let s_134_12: u8 = 15;
        // C s_134_13: cast zx s_134_12 -> bv
        let s_134_13: Bits = Bits::new(s_134_12 as u128, 5u16);
        // D s_134_14: cmp-eq s_134_11 s_134_13
        let s_134_14: bool = ((s_134_11) == (s_134_13));
        // N s_134_15: branch s_134_14 b452 b135
        if s_134_14 {
            return block_452(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #0u : u8
        let s_135_0: bool = false;
        // D s_135_1: write-var gs#428233 <= s_135_0
        fn_state.gs_428233 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#428233:u8
        let s_136_0: bool = fn_state.gs_428233;
        // D s_136_1: not s_136_0
        let s_136_1: bool = !s_136_0;
        // N s_136_2: branch s_136_1 b138 b137
        if s_136_1 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #2924s : i
        let s_137_0: i128 = 2924;
        // C s_137_1: const #14696u : u32
        let s_137_1: u32 = 14696;
        // N s_137_2: write-reg s_137_1 <= s_137_0
        let s_137_2: () = {
            state.write_register::<i128>(s_137_1 as isize, s_137_0);
            tracer.write_register(s_137_1 as isize, s_137_0);
        };
        // C s_137_3: const #6s : i
        let s_137_3: i128 = 6;
        // C s_137_4: const #5s : i
        let s_137_4: i128 = 5;
        // D s_137_5: read-var u#38279:u16
        let s_137_5: u16 = fn_state.u_38279;
        // D s_137_6: cast zx s_137_5 -> bv
        let s_137_6: Bits = Bits::new(s_137_5 as u128, 16u16);
        // D s_137_7: bit-extract s_137_6 s_137_3 s_137_4
        let s_137_7: Bits = (Bits::new(
            ((s_137_6) >> (s_137_3)).value(),
            u16::try_from(s_137_4).unwrap(),
        ));
        // D s_137_8: cast reint s_137_7 -> u8
        let s_137_8: u8 = (s_137_7.value() as u8);
        // C s_137_9: const #3s : i
        let s_137_9: i128 = 3;
        // C s_137_10: const #3s : i
        let s_137_10: i128 = 3;
        // D s_137_11: read-var u#38279:u16
        let s_137_11: u16 = fn_state.u_38279;
        // D s_137_12: cast zx s_137_11 -> bv
        let s_137_12: Bits = Bits::new(s_137_11 as u128, 16u16);
        // D s_137_13: bit-extract s_137_12 s_137_9 s_137_10
        let s_137_13: Bits = (Bits::new(
            ((s_137_12) >> (s_137_9)).value(),
            u16::try_from(s_137_10).unwrap(),
        ));
        // D s_137_14: cast reint s_137_13 -> u8
        let s_137_14: u8 = (s_137_13.value() as u8);
        // C s_137_15: const #0s : i
        let s_137_15: i128 = 0;
        // C s_137_16: const #3s : i
        let s_137_16: i128 = 3;
        // D s_137_17: read-var u#38279:u16
        let s_137_17: u16 = fn_state.u_38279;
        // D s_137_18: cast zx s_137_17 -> bv
        let s_137_18: Bits = Bits::new(s_137_17 as u128, 16u16);
        // D s_137_19: bit-extract s_137_18 s_137_15 s_137_16
        let s_137_19: Bits = (Bits::new(
            ((s_137_18) >> (s_137_15)).value(),
            u16::try_from(s_137_16).unwrap(),
        ));
        // D s_137_20: cast reint s_137_19 -> u8
        let s_137_20: u8 = (s_137_19.value() as u8);
        // D s_137_21: call decode_aarch32_instrs_LDRB_i_T1enc_A_txt(s_137_8, s_137_14, s_137_20)
        let s_137_21: () = decode_aarch32_instrs_LDRB_i_T1enc_A_txt(
            state,
            tracer,
            s_137_8,
            s_137_14,
            s_137_20,
        );
        // N s_137_22: return
        return;
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var merge#var.1:struct
        let s_138_0: u16 = fn_state.merge_var._1;
        // D s_138_1: write-var u#38283 <= s_138_0
        fn_state.u_38283 = s_138_0;
        // C s_138_2: const #9s : i
        let s_138_2: i128 = 9;
        // D s_138_3: read-var u#38283:u16
        let s_138_3: u16 = fn_state.u_38283;
        // D s_138_4: cast zx s_138_3 -> bv
        let s_138_4: Bits = Bits::new(s_138_3 as u128, 16u16);
        // C s_138_5: const #1s : i64
        let s_138_5: i64 = 1;
        // C s_138_6: cast zx s_138_5 -> i
        let s_138_6: i128 = (i128::try_from(s_138_5).unwrap());
        // C s_138_7: const #6s : i
        let s_138_7: i128 = 6;
        // C s_138_8: add s_138_7 s_138_6
        let s_138_8: i128 = (s_138_7 + s_138_6);
        // D s_138_9: bit-extract s_138_4 s_138_2 s_138_8
        let s_138_9: Bits = (Bits::new(
            ((s_138_4) >> (s_138_2)).value(),
            u16::try_from(s_138_8).unwrap(),
        ));
        // D s_138_10: cast reint s_138_9 -> u8
        let s_138_10: u8 = (s_138_9.value() as u8);
        // D s_138_11: cast zx s_138_10 -> bv
        let s_138_11: Bits = Bits::new(s_138_10 as u128, 7u16);
        // C s_138_12: const #46u : u8
        let s_138_12: u8 = 46;
        // C s_138_13: cast zx s_138_12 -> bv
        let s_138_13: Bits = Bits::new(s_138_12 as u128, 7u16);
        // D s_138_14: cmp-eq s_138_11 s_138_13
        let s_138_14: bool = ((s_138_11) == (s_138_13));
        // N s_138_15: branch s_138_14 b451 b139
        if s_138_14 {
            return block_451(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #0u : u8
        let s_139_0: bool = false;
        // D s_139_1: write-var gs#428246 <= s_139_0
        fn_state.gs_428246 = s_139_0;
        // N s_139_2: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var gs#428246:u8
        let s_140_0: bool = fn_state.gs_428246;
        // D s_140_1: not s_140_0
        let s_140_1: bool = !s_140_0;
        // N s_140_2: branch s_140_1 b142 b141
        if s_140_1 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #2930s : i
        let s_141_0: i128 = 2930;
        // C s_141_1: const #14696u : u32
        let s_141_1: u32 = 14696;
        // N s_141_2: write-reg s_141_1 <= s_141_0
        let s_141_2: () = {
            state.write_register::<i128>(s_141_1 as isize, s_141_0);
            tracer.write_register(s_141_1 as isize, s_141_0);
        };
        // C s_141_3: const #6s : i
        let s_141_3: i128 = 6;
        // C s_141_4: const #3s : i
        let s_141_4: i128 = 3;
        // D s_141_5: read-var u#38283:u16
        let s_141_5: u16 = fn_state.u_38283;
        // D s_141_6: cast zx s_141_5 -> bv
        let s_141_6: Bits = Bits::new(s_141_5 as u128, 16u16);
        // D s_141_7: bit-extract s_141_6 s_141_3 s_141_4
        let s_141_7: Bits = (Bits::new(
            ((s_141_6) >> (s_141_3)).value(),
            u16::try_from(s_141_4).unwrap(),
        ));
        // D s_141_8: cast reint s_141_7 -> u8
        let s_141_8: u8 = (s_141_7.value() as u8);
        // C s_141_9: const #3s : i
        let s_141_9: i128 = 3;
        // C s_141_10: const #3s : i
        let s_141_10: i128 = 3;
        // D s_141_11: read-var u#38283:u16
        let s_141_11: u16 = fn_state.u_38283;
        // D s_141_12: cast zx s_141_11 -> bv
        let s_141_12: Bits = Bits::new(s_141_11 as u128, 16u16);
        // D s_141_13: bit-extract s_141_12 s_141_9 s_141_10
        let s_141_13: Bits = (Bits::new(
            ((s_141_12) >> (s_141_9)).value(),
            u16::try_from(s_141_10).unwrap(),
        ));
        // D s_141_14: cast reint s_141_13 -> u8
        let s_141_14: u8 = (s_141_13.value() as u8);
        // C s_141_15: const #0s : i
        let s_141_15: i128 = 0;
        // C s_141_16: const #3s : i
        let s_141_16: i128 = 3;
        // D s_141_17: read-var u#38283:u16
        let s_141_17: u16 = fn_state.u_38283;
        // D s_141_18: cast zx s_141_17 -> bv
        let s_141_18: Bits = Bits::new(s_141_17 as u128, 16u16);
        // D s_141_19: bit-extract s_141_18 s_141_15 s_141_16
        let s_141_19: Bits = (Bits::new(
            ((s_141_18) >> (s_141_15)).value(),
            u16::try_from(s_141_16).unwrap(),
        ));
        // D s_141_20: cast reint s_141_19 -> u8
        let s_141_20: u8 = (s_141_19.value() as u8);
        // D s_141_21: call decode_aarch32_instrs_LDRB_r_T1enc_A_txt(s_141_8, s_141_14, s_141_20)
        let s_141_21: () = decode_aarch32_instrs_LDRB_r_T1enc_A_txt(
            state,
            tracer,
            s_141_8,
            s_141_14,
            s_141_20,
        );
        // N s_141_22: return
        return;
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var merge#var.1:struct
        let s_142_0: u16 = fn_state.merge_var._1;
        // D s_142_1: write-var u#38288 <= s_142_0
        fn_state.u_38288 = s_142_0;
        // C s_142_2: const #11s : i
        let s_142_2: i128 = 11;
        // D s_142_3: read-var u#38288:u16
        let s_142_3: u16 = fn_state.u_38288;
        // D s_142_4: cast zx s_142_3 -> bv
        let s_142_4: Bits = Bits::new(s_142_3 as u128, 16u16);
        // C s_142_5: const #1s : i64
        let s_142_5: i64 = 1;
        // C s_142_6: cast zx s_142_5 -> i
        let s_142_6: i128 = (i128::try_from(s_142_5).unwrap());
        // C s_142_7: const #4s : i
        let s_142_7: i128 = 4;
        // C s_142_8: add s_142_7 s_142_6
        let s_142_8: i128 = (s_142_7 + s_142_6);
        // D s_142_9: bit-extract s_142_4 s_142_2 s_142_8
        let s_142_9: Bits = (Bits::new(
            ((s_142_4) >> (s_142_2)).value(),
            u16::try_from(s_142_8).unwrap(),
        ));
        // D s_142_10: cast reint s_142_9 -> u8
        let s_142_10: u8 = (s_142_9.value() as u8);
        // D s_142_11: cast zx s_142_10 -> bv
        let s_142_11: Bits = Bits::new(s_142_10 as u128, 5u16);
        // C s_142_12: const #17u : u8
        let s_142_12: u8 = 17;
        // C s_142_13: cast zx s_142_12 -> bv
        let s_142_13: Bits = Bits::new(s_142_12 as u128, 5u16);
        // D s_142_14: cmp-eq s_142_11 s_142_13
        let s_142_14: bool = ((s_142_11) == (s_142_13));
        // N s_142_15: branch s_142_14 b450 b143
        if s_142_14 {
            return block_450(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #0u : u8
        let s_143_0: bool = false;
        // D s_143_1: write-var gs#428259 <= s_143_0
        fn_state.gs_428259 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#428259:u8
        let s_144_0: bool = fn_state.gs_428259;
        // D s_144_1: not s_144_0
        let s_144_1: bool = !s_144_0;
        // N s_144_2: branch s_144_1 b146 b145
        if s_144_1 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #2949s : i
        let s_145_0: i128 = 2949;
        // C s_145_1: const #14696u : u32
        let s_145_1: u32 = 14696;
        // N s_145_2: write-reg s_145_1 <= s_145_0
        let s_145_2: () = {
            state.write_register::<i128>(s_145_1 as isize, s_145_0);
            tracer.write_register(s_145_1 as isize, s_145_0);
        };
        // C s_145_3: const #6s : i
        let s_145_3: i128 = 6;
        // C s_145_4: const #5s : i
        let s_145_4: i128 = 5;
        // D s_145_5: read-var u#38288:u16
        let s_145_5: u16 = fn_state.u_38288;
        // D s_145_6: cast zx s_145_5 -> bv
        let s_145_6: Bits = Bits::new(s_145_5 as u128, 16u16);
        // D s_145_7: bit-extract s_145_6 s_145_3 s_145_4
        let s_145_7: Bits = (Bits::new(
            ((s_145_6) >> (s_145_3)).value(),
            u16::try_from(s_145_4).unwrap(),
        ));
        // D s_145_8: cast reint s_145_7 -> u8
        let s_145_8: u8 = (s_145_7.value() as u8);
        // C s_145_9: const #3s : i
        let s_145_9: i128 = 3;
        // C s_145_10: const #3s : i
        let s_145_10: i128 = 3;
        // D s_145_11: read-var u#38288:u16
        let s_145_11: u16 = fn_state.u_38288;
        // D s_145_12: cast zx s_145_11 -> bv
        let s_145_12: Bits = Bits::new(s_145_11 as u128, 16u16);
        // D s_145_13: bit-extract s_145_12 s_145_9 s_145_10
        let s_145_13: Bits = (Bits::new(
            ((s_145_12) >> (s_145_9)).value(),
            u16::try_from(s_145_10).unwrap(),
        ));
        // D s_145_14: cast reint s_145_13 -> u8
        let s_145_14: u8 = (s_145_13.value() as u8);
        // C s_145_15: const #0s : i
        let s_145_15: i128 = 0;
        // C s_145_16: const #3s : i
        let s_145_16: i128 = 3;
        // D s_145_17: read-var u#38288:u16
        let s_145_17: u16 = fn_state.u_38288;
        // D s_145_18: cast zx s_145_17 -> bv
        let s_145_18: Bits = Bits::new(s_145_17 as u128, 16u16);
        // D s_145_19: bit-extract s_145_18 s_145_15 s_145_16
        let s_145_19: Bits = (Bits::new(
            ((s_145_18) >> (s_145_15)).value(),
            u16::try_from(s_145_16).unwrap(),
        ));
        // D s_145_20: cast reint s_145_19 -> u8
        let s_145_20: u8 = (s_145_19.value() as u8);
        // D s_145_21: call decode_aarch32_instrs_LDRH_i_T1enc_A_txt(s_145_8, s_145_14, s_145_20)
        let s_145_21: () = decode_aarch32_instrs_LDRH_i_T1enc_A_txt(
            state,
            tracer,
            s_145_8,
            s_145_14,
            s_145_20,
        );
        // N s_145_22: return
        return;
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var merge#var.1:struct
        let s_146_0: u16 = fn_state.merge_var._1;
        // D s_146_1: write-var u#38293 <= s_146_0
        fn_state.u_38293 = s_146_0;
        // C s_146_2: const #9s : i
        let s_146_2: i128 = 9;
        // D s_146_3: read-var u#38293:u16
        let s_146_3: u16 = fn_state.u_38293;
        // D s_146_4: cast zx s_146_3 -> bv
        let s_146_4: Bits = Bits::new(s_146_3 as u128, 16u16);
        // C s_146_5: const #1s : i64
        let s_146_5: i64 = 1;
        // C s_146_6: cast zx s_146_5 -> i
        let s_146_6: i128 = (i128::try_from(s_146_5).unwrap());
        // C s_146_7: const #6s : i
        let s_146_7: i128 = 6;
        // C s_146_8: add s_146_7 s_146_6
        let s_146_8: i128 = (s_146_7 + s_146_6);
        // D s_146_9: bit-extract s_146_4 s_146_2 s_146_8
        let s_146_9: Bits = (Bits::new(
            ((s_146_4) >> (s_146_2)).value(),
            u16::try_from(s_146_8).unwrap(),
        ));
        // D s_146_10: cast reint s_146_9 -> u8
        let s_146_10: u8 = (s_146_9.value() as u8);
        // D s_146_11: cast zx s_146_10 -> bv
        let s_146_11: Bits = Bits::new(s_146_10 as u128, 7u16);
        // C s_146_12: const #45u : u8
        let s_146_12: u8 = 45;
        // C s_146_13: cast zx s_146_12 -> bv
        let s_146_13: Bits = Bits::new(s_146_12 as u128, 7u16);
        // D s_146_14: cmp-eq s_146_11 s_146_13
        let s_146_14: bool = ((s_146_11) == (s_146_13));
        // N s_146_15: branch s_146_14 b449 b147
        if s_146_14 {
            return block_449(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #0u : u8
        let s_147_0: bool = false;
        // D s_147_1: write-var gs#428272 <= s_147_0
        fn_state.gs_428272 = s_147_0;
        // N s_147_2: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var gs#428272:u8
        let s_148_0: bool = fn_state.gs_428272;
        // D s_148_1: not s_148_0
        let s_148_1: bool = !s_148_0;
        // N s_148_2: branch s_148_1 b150 b149
        if s_148_1 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_149(state, tracer, fn_state);
        };
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #2955s : i
        let s_149_0: i128 = 2955;
        // C s_149_1: const #14696u : u32
        let s_149_1: u32 = 14696;
        // N s_149_2: write-reg s_149_1 <= s_149_0
        let s_149_2: () = {
            state.write_register::<i128>(s_149_1 as isize, s_149_0);
            tracer.write_register(s_149_1 as isize, s_149_0);
        };
        // C s_149_3: const #6s : i
        let s_149_3: i128 = 6;
        // C s_149_4: const #3s : i
        let s_149_4: i128 = 3;
        // D s_149_5: read-var u#38293:u16
        let s_149_5: u16 = fn_state.u_38293;
        // D s_149_6: cast zx s_149_5 -> bv
        let s_149_6: Bits = Bits::new(s_149_5 as u128, 16u16);
        // D s_149_7: bit-extract s_149_6 s_149_3 s_149_4
        let s_149_7: Bits = (Bits::new(
            ((s_149_6) >> (s_149_3)).value(),
            u16::try_from(s_149_4).unwrap(),
        ));
        // D s_149_8: cast reint s_149_7 -> u8
        let s_149_8: u8 = (s_149_7.value() as u8);
        // C s_149_9: const #3s : i
        let s_149_9: i128 = 3;
        // C s_149_10: const #3s : i
        let s_149_10: i128 = 3;
        // D s_149_11: read-var u#38293:u16
        let s_149_11: u16 = fn_state.u_38293;
        // D s_149_12: cast zx s_149_11 -> bv
        let s_149_12: Bits = Bits::new(s_149_11 as u128, 16u16);
        // D s_149_13: bit-extract s_149_12 s_149_9 s_149_10
        let s_149_13: Bits = (Bits::new(
            ((s_149_12) >> (s_149_9)).value(),
            u16::try_from(s_149_10).unwrap(),
        ));
        // D s_149_14: cast reint s_149_13 -> u8
        let s_149_14: u8 = (s_149_13.value() as u8);
        // C s_149_15: const #0s : i
        let s_149_15: i128 = 0;
        // C s_149_16: const #3s : i
        let s_149_16: i128 = 3;
        // D s_149_17: read-var u#38293:u16
        let s_149_17: u16 = fn_state.u_38293;
        // D s_149_18: cast zx s_149_17 -> bv
        let s_149_18: Bits = Bits::new(s_149_17 as u128, 16u16);
        // D s_149_19: bit-extract s_149_18 s_149_15 s_149_16
        let s_149_19: Bits = (Bits::new(
            ((s_149_18) >> (s_149_15)).value(),
            u16::try_from(s_149_16).unwrap(),
        ));
        // D s_149_20: cast reint s_149_19 -> u8
        let s_149_20: u8 = (s_149_19.value() as u8);
        // D s_149_21: call decode_aarch32_instrs_LDRH_r_T1enc_A_txt(s_149_8, s_149_14, s_149_20)
        let s_149_21: () = decode_aarch32_instrs_LDRH_r_T1enc_A_txt(
            state,
            tracer,
            s_149_8,
            s_149_14,
            s_149_20,
        );
        // N s_149_22: return
        return;
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var merge#var.1:struct
        let s_150_0: u16 = fn_state.merge_var._1;
        // D s_150_1: write-var u#38298 <= s_150_0
        fn_state.u_38298 = s_150_0;
        // C s_150_2: const #11s : i
        let s_150_2: i128 = 11;
        // D s_150_3: read-var u#38298:u16
        let s_150_3: u16 = fn_state.u_38298;
        // D s_150_4: cast zx s_150_3 -> bv
        let s_150_4: Bits = Bits::new(s_150_3 as u128, 16u16);
        // C s_150_5: const #1s : i64
        let s_150_5: i64 = 1;
        // C s_150_6: cast zx s_150_5 -> i
        let s_150_6: i128 = (i128::try_from(s_150_5).unwrap());
        // C s_150_7: const #4s : i
        let s_150_7: i128 = 4;
        // C s_150_8: add s_150_7 s_150_6
        let s_150_8: i128 = (s_150_7 + s_150_6);
        // D s_150_9: bit-extract s_150_4 s_150_2 s_150_8
        let s_150_9: Bits = (Bits::new(
            ((s_150_4) >> (s_150_2)).value(),
            u16::try_from(s_150_8).unwrap(),
        ));
        // D s_150_10: cast reint s_150_9 -> u8
        let s_150_10: u8 = (s_150_9.value() as u8);
        // D s_150_11: cast zx s_150_10 -> bv
        let s_150_11: Bits = Bits::new(s_150_10 as u128, 5u16);
        // C s_150_12: const #13u : u8
        let s_150_12: u8 = 13;
        // C s_150_13: cast zx s_150_12 -> bv
        let s_150_13: Bits = Bits::new(s_150_12 as u128, 5u16);
        // D s_150_14: cmp-eq s_150_11 s_150_13
        let s_150_14: bool = ((s_150_11) == (s_150_13));
        // N s_150_15: branch s_150_14 b448 b151
        if s_150_14 {
            return block_448(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #0u : u8
        let s_151_0: bool = false;
        // D s_151_1: write-var gs#428285 <= s_151_0
        fn_state.gs_428285 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#428285:u8
        let s_152_0: bool = fn_state.gs_428285;
        // D s_152_1: not s_152_0
        let s_152_1: bool = !s_152_0;
        // N s_152_2: branch s_152_1 b154 b153
        if s_152_1 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #2961s : i
        let s_153_0: i128 = 2961;
        // C s_153_1: const #14696u : u32
        let s_153_1: u32 = 14696;
        // N s_153_2: write-reg s_153_1 <= s_153_0
        let s_153_2: () = {
            state.write_register::<i128>(s_153_1 as isize, s_153_0);
            tracer.write_register(s_153_1 as isize, s_153_0);
        };
        // C s_153_3: const #6s : i
        let s_153_3: i128 = 6;
        // C s_153_4: const #5s : i
        let s_153_4: i128 = 5;
        // D s_153_5: read-var u#38298:u16
        let s_153_5: u16 = fn_state.u_38298;
        // D s_153_6: cast zx s_153_5 -> bv
        let s_153_6: Bits = Bits::new(s_153_5 as u128, 16u16);
        // D s_153_7: bit-extract s_153_6 s_153_3 s_153_4
        let s_153_7: Bits = (Bits::new(
            ((s_153_6) >> (s_153_3)).value(),
            u16::try_from(s_153_4).unwrap(),
        ));
        // D s_153_8: cast reint s_153_7 -> u8
        let s_153_8: u8 = (s_153_7.value() as u8);
        // C s_153_9: const #3s : i
        let s_153_9: i128 = 3;
        // C s_153_10: const #3s : i
        let s_153_10: i128 = 3;
        // D s_153_11: read-var u#38298:u16
        let s_153_11: u16 = fn_state.u_38298;
        // D s_153_12: cast zx s_153_11 -> bv
        let s_153_12: Bits = Bits::new(s_153_11 as u128, 16u16);
        // D s_153_13: bit-extract s_153_12 s_153_9 s_153_10
        let s_153_13: Bits = (Bits::new(
            ((s_153_12) >> (s_153_9)).value(),
            u16::try_from(s_153_10).unwrap(),
        ));
        // D s_153_14: cast reint s_153_13 -> u8
        let s_153_14: u8 = (s_153_13.value() as u8);
        // C s_153_15: const #0s : i
        let s_153_15: i128 = 0;
        // C s_153_16: const #3s : i
        let s_153_16: i128 = 3;
        // D s_153_17: read-var u#38298:u16
        let s_153_17: u16 = fn_state.u_38298;
        // D s_153_18: cast zx s_153_17 -> bv
        let s_153_18: Bits = Bits::new(s_153_17 as u128, 16u16);
        // D s_153_19: bit-extract s_153_18 s_153_15 s_153_16
        let s_153_19: Bits = (Bits::new(
            ((s_153_18) >> (s_153_15)).value(),
            u16::try_from(s_153_16).unwrap(),
        ));
        // D s_153_20: cast reint s_153_19 -> u8
        let s_153_20: u8 = (s_153_19.value() as u8);
        // D s_153_21: call decode_aarch32_instrs_LDR_i_T1enc_A_txt(s_153_8, s_153_14, s_153_20)
        let s_153_21: () = decode_aarch32_instrs_LDR_i_T1enc_A_txt(
            state,
            tracer,
            s_153_8,
            s_153_14,
            s_153_20,
        );
        // N s_153_22: return
        return;
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var merge#var.1:struct
        let s_154_0: u16 = fn_state.merge_var._1;
        // D s_154_1: write-var u#38303 <= s_154_0
        fn_state.u_38303 = s_154_0;
        // C s_154_2: const #11s : i
        let s_154_2: i128 = 11;
        // D s_154_3: read-var u#38303:u16
        let s_154_3: u16 = fn_state.u_38303;
        // D s_154_4: cast zx s_154_3 -> bv
        let s_154_4: Bits = Bits::new(s_154_3 as u128, 16u16);
        // C s_154_5: const #1s : i64
        let s_154_5: i64 = 1;
        // C s_154_6: cast zx s_154_5 -> i
        let s_154_6: i128 = (i128::try_from(s_154_5).unwrap());
        // C s_154_7: const #4s : i
        let s_154_7: i128 = 4;
        // C s_154_8: add s_154_7 s_154_6
        let s_154_8: i128 = (s_154_7 + s_154_6);
        // D s_154_9: bit-extract s_154_4 s_154_2 s_154_8
        let s_154_9: Bits = (Bits::new(
            ((s_154_4) >> (s_154_2)).value(),
            u16::try_from(s_154_8).unwrap(),
        ));
        // D s_154_10: cast reint s_154_9 -> u8
        let s_154_10: u8 = (s_154_9.value() as u8);
        // D s_154_11: cast zx s_154_10 -> bv
        let s_154_11: Bits = Bits::new(s_154_10 as u128, 5u16);
        // C s_154_12: const #19u : u8
        let s_154_12: u8 = 19;
        // C s_154_13: cast zx s_154_12 -> bv
        let s_154_13: Bits = Bits::new(s_154_12 as u128, 5u16);
        // D s_154_14: cmp-eq s_154_11 s_154_13
        let s_154_14: bool = ((s_154_11) == (s_154_13));
        // N s_154_15: branch s_154_14 b447 b155
        if s_154_14 {
            return block_447(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #0u : u8
        let s_155_0: bool = false;
        // D s_155_1: write-var gs#428298 <= s_155_0
        fn_state.gs_428298 = s_155_0;
        // N s_155_2: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var gs#428298:u8
        let s_156_0: bool = fn_state.gs_428298;
        // D s_156_1: not s_156_0
        let s_156_1: bool = !s_156_0;
        // N s_156_2: branch s_156_1 b158 b157
        if s_156_1 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #2962s : i
        let s_157_0: i128 = 2962;
        // C s_157_1: const #14696u : u32
        let s_157_1: u32 = 14696;
        // N s_157_2: write-reg s_157_1 <= s_157_0
        let s_157_2: () = {
            state.write_register::<i128>(s_157_1 as isize, s_157_0);
            tracer.write_register(s_157_1 as isize, s_157_0);
        };
        // C s_157_3: const #8s : i
        let s_157_3: i128 = 8;
        // C s_157_4: const #3s : i
        let s_157_4: i128 = 3;
        // D s_157_5: read-var u#38303:u16
        let s_157_5: u16 = fn_state.u_38303;
        // D s_157_6: cast zx s_157_5 -> bv
        let s_157_6: Bits = Bits::new(s_157_5 as u128, 16u16);
        // D s_157_7: bit-extract s_157_6 s_157_3 s_157_4
        let s_157_7: Bits = (Bits::new(
            ((s_157_6) >> (s_157_3)).value(),
            u16::try_from(s_157_4).unwrap(),
        ));
        // D s_157_8: cast reint s_157_7 -> u8
        let s_157_8: u8 = (s_157_7.value() as u8);
        // C s_157_9: const #0s : i
        let s_157_9: i128 = 0;
        // C s_157_10: const #8s : i
        let s_157_10: i128 = 8;
        // D s_157_11: read-var u#38303:u16
        let s_157_11: u16 = fn_state.u_38303;
        // D s_157_12: cast zx s_157_11 -> bv
        let s_157_12: Bits = Bits::new(s_157_11 as u128, 16u16);
        // D s_157_13: bit-extract s_157_12 s_157_9 s_157_10
        let s_157_13: Bits = (Bits::new(
            ((s_157_12) >> (s_157_9)).value(),
            u16::try_from(s_157_10).unwrap(),
        ));
        // D s_157_14: cast reint s_157_13 -> u8
        let s_157_14: u8 = (s_157_13.value() as u8);
        // D s_157_15: call decode_aarch32_instrs_LDR_i_T2enc_A_txt(s_157_8, s_157_14)
        let s_157_15: () = decode_aarch32_instrs_LDR_i_T2enc_A_txt(
            state,
            tracer,
            s_157_8,
            s_157_14,
        );
        // N s_157_16: return
        return;
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var merge#var.1:struct
        let s_158_0: u16 = fn_state.merge_var._1;
        // D s_158_1: write-var u#38307 <= s_158_0
        fn_state.u_38307 = s_158_0;
        // C s_158_2: const #11s : i
        let s_158_2: i128 = 11;
        // D s_158_3: read-var u#38307:u16
        let s_158_3: u16 = fn_state.u_38307;
        // D s_158_4: cast zx s_158_3 -> bv
        let s_158_4: Bits = Bits::new(s_158_3 as u128, 16u16);
        // C s_158_5: const #1s : i64
        let s_158_5: i64 = 1;
        // C s_158_6: cast zx s_158_5 -> i
        let s_158_6: i128 = (i128::try_from(s_158_5).unwrap());
        // C s_158_7: const #4s : i
        let s_158_7: i128 = 4;
        // C s_158_8: add s_158_7 s_158_6
        let s_158_8: i128 = (s_158_7 + s_158_6);
        // D s_158_9: bit-extract s_158_4 s_158_2 s_158_8
        let s_158_9: Bits = (Bits::new(
            ((s_158_4) >> (s_158_2)).value(),
            u16::try_from(s_158_8).unwrap(),
        ));
        // D s_158_10: cast reint s_158_9 -> u8
        let s_158_10: u8 = (s_158_9.value() as u8);
        // D s_158_11: cast zx s_158_10 -> bv
        let s_158_11: Bits = Bits::new(s_158_10 as u128, 5u16);
        // C s_158_12: const #9u : u8
        let s_158_12: u8 = 9;
        // C s_158_13: cast zx s_158_12 -> bv
        let s_158_13: Bits = Bits::new(s_158_12 as u128, 5u16);
        // D s_158_14: cmp-eq s_158_11 s_158_13
        let s_158_14: bool = ((s_158_11) == (s_158_13));
        // N s_158_15: branch s_158_14 b446 b159
        if s_158_14 {
            return block_446(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #0u : u8
        let s_159_0: bool = false;
        // D s_159_1: write-var gs#428309 <= s_159_0
        fn_state.gs_428309 = s_159_0;
        // N s_159_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var gs#428309:u8
        let s_160_0: bool = fn_state.gs_428309;
        // D s_160_1: not s_160_0
        let s_160_1: bool = !s_160_0;
        // N s_160_2: branch s_160_1 b162 b161
        if s_160_1 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #2966s : i
        let s_161_0: i128 = 2966;
        // C s_161_1: const #14696u : u32
        let s_161_1: u32 = 14696;
        // N s_161_2: write-reg s_161_1 <= s_161_0
        let s_161_2: () = {
            state.write_register::<i128>(s_161_1 as isize, s_161_0);
            tracer.write_register(s_161_1 as isize, s_161_0);
        };
        // C s_161_3: const #8s : i
        let s_161_3: i128 = 8;
        // C s_161_4: const #3s : i
        let s_161_4: i128 = 3;
        // D s_161_5: read-var u#38307:u16
        let s_161_5: u16 = fn_state.u_38307;
        // D s_161_6: cast zx s_161_5 -> bv
        let s_161_6: Bits = Bits::new(s_161_5 as u128, 16u16);
        // D s_161_7: bit-extract s_161_6 s_161_3 s_161_4
        let s_161_7: Bits = (Bits::new(
            ((s_161_6) >> (s_161_3)).value(),
            u16::try_from(s_161_4).unwrap(),
        ));
        // D s_161_8: cast reint s_161_7 -> u8
        let s_161_8: u8 = (s_161_7.value() as u8);
        // C s_161_9: const #0s : i
        let s_161_9: i128 = 0;
        // C s_161_10: const #8s : i
        let s_161_10: i128 = 8;
        // D s_161_11: read-var u#38307:u16
        let s_161_11: u16 = fn_state.u_38307;
        // D s_161_12: cast zx s_161_11 -> bv
        let s_161_12: Bits = Bits::new(s_161_11 as u128, 16u16);
        // D s_161_13: bit-extract s_161_12 s_161_9 s_161_10
        let s_161_13: Bits = (Bits::new(
            ((s_161_12) >> (s_161_9)).value(),
            u16::try_from(s_161_10).unwrap(),
        ));
        // D s_161_14: cast reint s_161_13 -> u8
        let s_161_14: u8 = (s_161_13.value() as u8);
        // D s_161_15: call decode_aarch32_instrs_LDR_l_T1enc_A_txt(s_161_8, s_161_14)
        let s_161_15: () = decode_aarch32_instrs_LDR_l_T1enc_A_txt(
            state,
            tracer,
            s_161_8,
            s_161_14,
        );
        // N s_161_16: return
        return;
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var merge#var.1:struct
        let s_162_0: u16 = fn_state.merge_var._1;
        // D s_162_1: write-var u#38311 <= s_162_0
        fn_state.u_38311 = s_162_0;
        // C s_162_2: const #9s : i
        let s_162_2: i128 = 9;
        // D s_162_3: read-var u#38311:u16
        let s_162_3: u16 = fn_state.u_38311;
        // D s_162_4: cast zx s_162_3 -> bv
        let s_162_4: Bits = Bits::new(s_162_3 as u128, 16u16);
        // C s_162_5: const #1s : i64
        let s_162_5: i64 = 1;
        // C s_162_6: cast zx s_162_5 -> i
        let s_162_6: i128 = (i128::try_from(s_162_5).unwrap());
        // C s_162_7: const #6s : i
        let s_162_7: i128 = 6;
        // C s_162_8: add s_162_7 s_162_6
        let s_162_8: i128 = (s_162_7 + s_162_6);
        // D s_162_9: bit-extract s_162_4 s_162_2 s_162_8
        let s_162_9: Bits = (Bits::new(
            ((s_162_4) >> (s_162_2)).value(),
            u16::try_from(s_162_8).unwrap(),
        ));
        // D s_162_10: cast reint s_162_9 -> u8
        let s_162_10: u8 = (s_162_9.value() as u8);
        // D s_162_11: cast zx s_162_10 -> bv
        let s_162_11: Bits = Bits::new(s_162_10 as u128, 7u16);
        // C s_162_12: const #44u : u8
        let s_162_12: u8 = 44;
        // C s_162_13: cast zx s_162_12 -> bv
        let s_162_13: Bits = Bits::new(s_162_12 as u128, 7u16);
        // D s_162_14: cmp-eq s_162_11 s_162_13
        let s_162_14: bool = ((s_162_11) == (s_162_13));
        // N s_162_15: branch s_162_14 b445 b163
        if s_162_14 {
            return block_445(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #0u : u8
        let s_163_0: bool = false;
        // D s_163_1: write-var gs#428320 <= s_163_0
        fn_state.gs_428320 = s_163_0;
        // N s_163_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var gs#428320:u8
        let s_164_0: bool = fn_state.gs_428320;
        // D s_164_1: not s_164_0
        let s_164_1: bool = !s_164_0;
        // N s_164_2: branch s_164_1 b166 b165
        if s_164_1 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_165(state, tracer, fn_state);
        };
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #2969s : i
        let s_165_0: i128 = 2969;
        // C s_165_1: const #14696u : u32
        let s_165_1: u32 = 14696;
        // N s_165_2: write-reg s_165_1 <= s_165_0
        let s_165_2: () = {
            state.write_register::<i128>(s_165_1 as isize, s_165_0);
            tracer.write_register(s_165_1 as isize, s_165_0);
        };
        // C s_165_3: const #6s : i
        let s_165_3: i128 = 6;
        // C s_165_4: const #3s : i
        let s_165_4: i128 = 3;
        // D s_165_5: read-var u#38311:u16
        let s_165_5: u16 = fn_state.u_38311;
        // D s_165_6: cast zx s_165_5 -> bv
        let s_165_6: Bits = Bits::new(s_165_5 as u128, 16u16);
        // D s_165_7: bit-extract s_165_6 s_165_3 s_165_4
        let s_165_7: Bits = (Bits::new(
            ((s_165_6) >> (s_165_3)).value(),
            u16::try_from(s_165_4).unwrap(),
        ));
        // D s_165_8: cast reint s_165_7 -> u8
        let s_165_8: u8 = (s_165_7.value() as u8);
        // C s_165_9: const #3s : i
        let s_165_9: i128 = 3;
        // C s_165_10: const #3s : i
        let s_165_10: i128 = 3;
        // D s_165_11: read-var u#38311:u16
        let s_165_11: u16 = fn_state.u_38311;
        // D s_165_12: cast zx s_165_11 -> bv
        let s_165_12: Bits = Bits::new(s_165_11 as u128, 16u16);
        // D s_165_13: bit-extract s_165_12 s_165_9 s_165_10
        let s_165_13: Bits = (Bits::new(
            ((s_165_12) >> (s_165_9)).value(),
            u16::try_from(s_165_10).unwrap(),
        ));
        // D s_165_14: cast reint s_165_13 -> u8
        let s_165_14: u8 = (s_165_13.value() as u8);
        // C s_165_15: const #0s : i
        let s_165_15: i128 = 0;
        // C s_165_16: const #3s : i
        let s_165_16: i128 = 3;
        // D s_165_17: read-var u#38311:u16
        let s_165_17: u16 = fn_state.u_38311;
        // D s_165_18: cast zx s_165_17 -> bv
        let s_165_18: Bits = Bits::new(s_165_17 as u128, 16u16);
        // D s_165_19: bit-extract s_165_18 s_165_15 s_165_16
        let s_165_19: Bits = (Bits::new(
            ((s_165_18) >> (s_165_15)).value(),
            u16::try_from(s_165_16).unwrap(),
        ));
        // D s_165_20: cast reint s_165_19 -> u8
        let s_165_20: u8 = (s_165_19.value() as u8);
        // D s_165_21: call decode_aarch32_instrs_LDR_r_T1enc_A_txt(s_165_8, s_165_14, s_165_20)
        let s_165_21: () = decode_aarch32_instrs_LDR_r_T1enc_A_txt(
            state,
            tracer,
            s_165_8,
            s_165_14,
            s_165_20,
        );
        // N s_165_22: return
        return;
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var merge#var.1:struct
        let s_166_0: u16 = fn_state.merge_var._1;
        // D s_166_1: write-var u#38316 <= s_166_0
        fn_state.u_38316 = s_166_0;
        // C s_166_2: const #9s : i
        let s_166_2: i128 = 9;
        // D s_166_3: read-var u#38316:u16
        let s_166_3: u16 = fn_state.u_38316;
        // D s_166_4: cast zx s_166_3 -> bv
        let s_166_4: Bits = Bits::new(s_166_3 as u128, 16u16);
        // C s_166_5: const #1s : i64
        let s_166_5: i64 = 1;
        // C s_166_6: cast zx s_166_5 -> i
        let s_166_6: i128 = (i128::try_from(s_166_5).unwrap());
        // C s_166_7: const #6s : i
        let s_166_7: i128 = 6;
        // C s_166_8: add s_166_7 s_166_6
        let s_166_8: i128 = (s_166_7 + s_166_6);
        // D s_166_9: bit-extract s_166_4 s_166_2 s_166_8
        let s_166_9: Bits = (Bits::new(
            ((s_166_4) >> (s_166_2)).value(),
            u16::try_from(s_166_8).unwrap(),
        ));
        // D s_166_10: cast reint s_166_9 -> u8
        let s_166_10: u8 = (s_166_9.value() as u8);
        // D s_166_11: cast zx s_166_10 -> bv
        let s_166_11: Bits = Bits::new(s_166_10 as u128, 7u16);
        // C s_166_12: const #43u : u8
        let s_166_12: u8 = 43;
        // C s_166_13: cast zx s_166_12 -> bv
        let s_166_13: Bits = Bits::new(s_166_12 as u128, 7u16);
        // D s_166_14: cmp-eq s_166_11 s_166_13
        let s_166_14: bool = ((s_166_11) == (s_166_13));
        // N s_166_15: branch s_166_14 b444 b167
        if s_166_14 {
            return block_444(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #0u : u8
        let s_167_0: bool = false;
        // D s_167_1: write-var gs#428333 <= s_167_0
        fn_state.gs_428333 = s_167_0;
        // N s_167_2: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var gs#428333:u8
        let s_168_0: bool = fn_state.gs_428333;
        // D s_168_1: not s_168_0
        let s_168_1: bool = !s_168_0;
        // N s_168_2: branch s_168_1 b170 b169
        if s_168_1 {
            return block_170(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #2977s : i
        let s_169_0: i128 = 2977;
        // C s_169_1: const #14696u : u32
        let s_169_1: u32 = 14696;
        // N s_169_2: write-reg s_169_1 <= s_169_0
        let s_169_2: () = {
            state.write_register::<i128>(s_169_1 as isize, s_169_0);
            tracer.write_register(s_169_1 as isize, s_169_0);
        };
        // C s_169_3: const #6s : i
        let s_169_3: i128 = 6;
        // C s_169_4: const #3s : i
        let s_169_4: i128 = 3;
        // D s_169_5: read-var u#38316:u16
        let s_169_5: u16 = fn_state.u_38316;
        // D s_169_6: cast zx s_169_5 -> bv
        let s_169_6: Bits = Bits::new(s_169_5 as u128, 16u16);
        // D s_169_7: bit-extract s_169_6 s_169_3 s_169_4
        let s_169_7: Bits = (Bits::new(
            ((s_169_6) >> (s_169_3)).value(),
            u16::try_from(s_169_4).unwrap(),
        ));
        // D s_169_8: cast reint s_169_7 -> u8
        let s_169_8: u8 = (s_169_7.value() as u8);
        // C s_169_9: const #3s : i
        let s_169_9: i128 = 3;
        // C s_169_10: const #3s : i
        let s_169_10: i128 = 3;
        // D s_169_11: read-var u#38316:u16
        let s_169_11: u16 = fn_state.u_38316;
        // D s_169_12: cast zx s_169_11 -> bv
        let s_169_12: Bits = Bits::new(s_169_11 as u128, 16u16);
        // D s_169_13: bit-extract s_169_12 s_169_9 s_169_10
        let s_169_13: Bits = (Bits::new(
            ((s_169_12) >> (s_169_9)).value(),
            u16::try_from(s_169_10).unwrap(),
        ));
        // D s_169_14: cast reint s_169_13 -> u8
        let s_169_14: u8 = (s_169_13.value() as u8);
        // C s_169_15: const #0s : i
        let s_169_15: i128 = 0;
        // C s_169_16: const #3s : i
        let s_169_16: i128 = 3;
        // D s_169_17: read-var u#38316:u16
        let s_169_17: u16 = fn_state.u_38316;
        // D s_169_18: cast zx s_169_17 -> bv
        let s_169_18: Bits = Bits::new(s_169_17 as u128, 16u16);
        // D s_169_19: bit-extract s_169_18 s_169_15 s_169_16
        let s_169_19: Bits = (Bits::new(
            ((s_169_18) >> (s_169_15)).value(),
            u16::try_from(s_169_16).unwrap(),
        ));
        // D s_169_20: cast reint s_169_19 -> u8
        let s_169_20: u8 = (s_169_19.value() as u8);
        // D s_169_21: call decode_aarch32_instrs_LDRSB_r_T1enc_A_txt(s_169_8, s_169_14, s_169_20)
        let s_169_21: () = decode_aarch32_instrs_LDRSB_r_T1enc_A_txt(
            state,
            tracer,
            s_169_8,
            s_169_14,
            s_169_20,
        );
        // N s_169_22: return
        return;
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var merge#var.1:struct
        let s_170_0: u16 = fn_state.merge_var._1;
        // D s_170_1: write-var u#38321 <= s_170_0
        fn_state.u_38321 = s_170_0;
        // C s_170_2: const #9s : i
        let s_170_2: i128 = 9;
        // D s_170_3: read-var u#38321:u16
        let s_170_3: u16 = fn_state.u_38321;
        // D s_170_4: cast zx s_170_3 -> bv
        let s_170_4: Bits = Bits::new(s_170_3 as u128, 16u16);
        // C s_170_5: const #1s : i64
        let s_170_5: i64 = 1;
        // C s_170_6: cast zx s_170_5 -> i
        let s_170_6: i128 = (i128::try_from(s_170_5).unwrap());
        // C s_170_7: const #6s : i
        let s_170_7: i128 = 6;
        // C s_170_8: add s_170_7 s_170_6
        let s_170_8: i128 = (s_170_7 + s_170_6);
        // D s_170_9: bit-extract s_170_4 s_170_2 s_170_8
        let s_170_9: Bits = (Bits::new(
            ((s_170_4) >> (s_170_2)).value(),
            u16::try_from(s_170_8).unwrap(),
        ));
        // D s_170_10: cast reint s_170_9 -> u8
        let s_170_10: u8 = (s_170_9.value() as u8);
        // D s_170_11: cast zx s_170_10 -> bv
        let s_170_11: Bits = Bits::new(s_170_10 as u128, 7u16);
        // C s_170_12: const #47u : u8
        let s_170_12: u8 = 47;
        // C s_170_13: cast zx s_170_12 -> bv
        let s_170_13: Bits = Bits::new(s_170_12 as u128, 7u16);
        // D s_170_14: cmp-eq s_170_11 s_170_13
        let s_170_14: bool = ((s_170_11) == (s_170_13));
        // N s_170_15: branch s_170_14 b443 b171
        if s_170_14 {
            return block_443(state, tracer, fn_state);
        } else {
            return block_171(state, tracer, fn_state);
        };
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #0u : u8
        let s_171_0: bool = false;
        // D s_171_1: write-var gs#428346 <= s_171_0
        fn_state.gs_428346 = s_171_0;
        // N s_171_2: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var gs#428346:u8
        let s_172_0: bool = fn_state.gs_428346;
        // D s_172_1: not s_172_0
        let s_172_1: bool = !s_172_0;
        // N s_172_2: branch s_172_1 b174 b173
        if s_172_1 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #2988s : i
        let s_173_0: i128 = 2988;
        // C s_173_1: const #14696u : u32
        let s_173_1: u32 = 14696;
        // N s_173_2: write-reg s_173_1 <= s_173_0
        let s_173_2: () = {
            state.write_register::<i128>(s_173_1 as isize, s_173_0);
            tracer.write_register(s_173_1 as isize, s_173_0);
        };
        // C s_173_3: const #6s : i
        let s_173_3: i128 = 6;
        // C s_173_4: const #3s : i
        let s_173_4: i128 = 3;
        // D s_173_5: read-var u#38321:u16
        let s_173_5: u16 = fn_state.u_38321;
        // D s_173_6: cast zx s_173_5 -> bv
        let s_173_6: Bits = Bits::new(s_173_5 as u128, 16u16);
        // D s_173_7: bit-extract s_173_6 s_173_3 s_173_4
        let s_173_7: Bits = (Bits::new(
            ((s_173_6) >> (s_173_3)).value(),
            u16::try_from(s_173_4).unwrap(),
        ));
        // D s_173_8: cast reint s_173_7 -> u8
        let s_173_8: u8 = (s_173_7.value() as u8);
        // C s_173_9: const #3s : i
        let s_173_9: i128 = 3;
        // C s_173_10: const #3s : i
        let s_173_10: i128 = 3;
        // D s_173_11: read-var u#38321:u16
        let s_173_11: u16 = fn_state.u_38321;
        // D s_173_12: cast zx s_173_11 -> bv
        let s_173_12: Bits = Bits::new(s_173_11 as u128, 16u16);
        // D s_173_13: bit-extract s_173_12 s_173_9 s_173_10
        let s_173_13: Bits = (Bits::new(
            ((s_173_12) >> (s_173_9)).value(),
            u16::try_from(s_173_10).unwrap(),
        ));
        // D s_173_14: cast reint s_173_13 -> u8
        let s_173_14: u8 = (s_173_13.value() as u8);
        // C s_173_15: const #0s : i
        let s_173_15: i128 = 0;
        // C s_173_16: const #3s : i
        let s_173_16: i128 = 3;
        // D s_173_17: read-var u#38321:u16
        let s_173_17: u16 = fn_state.u_38321;
        // D s_173_18: cast zx s_173_17 -> bv
        let s_173_18: Bits = Bits::new(s_173_17 as u128, 16u16);
        // D s_173_19: bit-extract s_173_18 s_173_15 s_173_16
        let s_173_19: Bits = (Bits::new(
            ((s_173_18) >> (s_173_15)).value(),
            u16::try_from(s_173_16).unwrap(),
        ));
        // D s_173_20: cast reint s_173_19 -> u8
        let s_173_20: u8 = (s_173_19.value() as u8);
        // D s_173_21: call decode_aarch32_instrs_LDRSH_r_T1enc_A_txt(s_173_8, s_173_14, s_173_20)
        let s_173_21: () = decode_aarch32_instrs_LDRSH_r_T1enc_A_txt(
            state,
            tracer,
            s_173_8,
            s_173_14,
            s_173_20,
        );
        // N s_173_22: return
        return;
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var merge#var.1:struct
        let s_174_0: u16 = fn_state.merge_var._1;
        // D s_174_1: write-var u#38326 <= s_174_0
        fn_state.u_38326 = s_174_0;
        // C s_174_2: const #11s : i
        let s_174_2: i128 = 11;
        // D s_174_3: read-var u#38326:u16
        let s_174_3: u16 = fn_state.u_38326;
        // D s_174_4: cast zx s_174_3 -> bv
        let s_174_4: Bits = Bits::new(s_174_3 as u128, 16u16);
        // C s_174_5: const #1s : i64
        let s_174_5: i64 = 1;
        // C s_174_6: cast zx s_174_5 -> i
        let s_174_6: i128 = (i128::try_from(s_174_5).unwrap());
        // C s_174_7: const #4s : i
        let s_174_7: i128 = 4;
        // C s_174_8: add s_174_7 s_174_6
        let s_174_8: i128 = (s_174_7 + s_174_6);
        // D s_174_9: bit-extract s_174_4 s_174_2 s_174_8
        let s_174_9: Bits = (Bits::new(
            ((s_174_4) >> (s_174_2)).value(),
            u16::try_from(s_174_8).unwrap(),
        ));
        // D s_174_10: cast reint s_174_9 -> u8
        let s_174_10: u8 = (s_174_9.value() as u8);
        // D s_174_11: cast zx s_174_10 -> bv
        let s_174_11: Bits = Bits::new(s_174_10 as u128, 5u16);
        // C s_174_12: const #0u : u8
        let s_174_12: u8 = 0;
        // C s_174_13: cast zx s_174_12 -> bv
        let s_174_13: Bits = Bits::new(s_174_12 as u128, 5u16);
        // D s_174_14: cmp-eq s_174_11 s_174_13
        let s_174_14: bool = ((s_174_11) == (s_174_13));
        // N s_174_15: branch s_174_14 b442 b175
        if s_174_14 {
            return block_442(state, tracer, fn_state);
        } else {
            return block_175(state, tracer, fn_state);
        };
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #0u : u8
        let s_175_0: bool = false;
        // D s_175_1: write-var gs#428359 <= s_175_0
        fn_state.gs_428359 = s_175_0;
        // N s_175_2: jump b176
        return block_176(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var gs#428359:u8
        let s_176_0: bool = fn_state.gs_428359;
        // D s_176_1: not s_176_0
        let s_176_1: bool = !s_176_0;
        // N s_176_2: branch s_176_1 b178 b177
        if s_176_1 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #2996s : i
        let s_177_0: i128 = 2996;
        // C s_177_1: const #14696u : u32
        let s_177_1: u32 = 14696;
        // N s_177_2: write-reg s_177_1 <= s_177_0
        let s_177_2: () = {
            state.write_register::<i128>(s_177_1 as isize, s_177_0);
            tracer.write_register(s_177_1 as isize, s_177_0);
        };
        // C s_177_3: const #6s : i
        let s_177_3: i128 = 6;
        // C s_177_4: const #5s : i
        let s_177_4: i128 = 5;
        // D s_177_5: read-var u#38326:u16
        let s_177_5: u16 = fn_state.u_38326;
        // D s_177_6: cast zx s_177_5 -> bv
        let s_177_6: Bits = Bits::new(s_177_5 as u128, 16u16);
        // D s_177_7: bit-extract s_177_6 s_177_3 s_177_4
        let s_177_7: Bits = (Bits::new(
            ((s_177_6) >> (s_177_3)).value(),
            u16::try_from(s_177_4).unwrap(),
        ));
        // D s_177_8: cast reint s_177_7 -> u8
        let s_177_8: u8 = (s_177_7.value() as u8);
        // C s_177_9: const #3s : i
        let s_177_9: i128 = 3;
        // C s_177_10: const #3s : i
        let s_177_10: i128 = 3;
        // D s_177_11: read-var u#38326:u16
        let s_177_11: u16 = fn_state.u_38326;
        // D s_177_12: cast zx s_177_11 -> bv
        let s_177_12: Bits = Bits::new(s_177_11 as u128, 16u16);
        // D s_177_13: bit-extract s_177_12 s_177_9 s_177_10
        let s_177_13: Bits = (Bits::new(
            ((s_177_12) >> (s_177_9)).value(),
            u16::try_from(s_177_10).unwrap(),
        ));
        // D s_177_14: cast reint s_177_13 -> u8
        let s_177_14: u8 = (s_177_13.value() as u8);
        // C s_177_15: const #0s : i
        let s_177_15: i128 = 0;
        // C s_177_16: const #3s : i
        let s_177_16: i128 = 3;
        // D s_177_17: read-var u#38326:u16
        let s_177_17: u16 = fn_state.u_38326;
        // D s_177_18: cast zx s_177_17 -> bv
        let s_177_18: Bits = Bits::new(s_177_17 as u128, 16u16);
        // D s_177_19: bit-extract s_177_18 s_177_15 s_177_16
        let s_177_19: Bits = (Bits::new(
            ((s_177_18) >> (s_177_15)).value(),
            u16::try_from(s_177_16).unwrap(),
        ));
        // D s_177_20: cast reint s_177_19 -> u8
        let s_177_20: u8 = (s_177_19.value() as u8);
        // D s_177_21: call decode_aarch32_instrs_LSL_i_T1enc_A_txt(s_177_8, s_177_14, s_177_20)
        let s_177_21: () = decode_aarch32_instrs_LSL_i_T1enc_A_txt(
            state,
            tracer,
            s_177_8,
            s_177_14,
            s_177_20,
        );
        // N s_177_22: return
        return;
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var merge#var.1:struct
        let s_178_0: u16 = fn_state.merge_var._1;
        // D s_178_1: write-var u#38331 <= s_178_0
        fn_state.u_38331 = s_178_0;
        // C s_178_2: const #6s : i
        let s_178_2: i128 = 6;
        // D s_178_3: read-var u#38331:u16
        let s_178_3: u16 = fn_state.u_38331;
        // D s_178_4: cast zx s_178_3 -> bv
        let s_178_4: Bits = Bits::new(s_178_3 as u128, 16u16);
        // C s_178_5: const #1s : i64
        let s_178_5: i64 = 1;
        // C s_178_6: cast zx s_178_5 -> i
        let s_178_6: i128 = (i128::try_from(s_178_5).unwrap());
        // C s_178_7: const #9s : i
        let s_178_7: i128 = 9;
        // C s_178_8: add s_178_7 s_178_6
        let s_178_8: i128 = (s_178_7 + s_178_6);
        // D s_178_9: bit-extract s_178_4 s_178_2 s_178_8
        let s_178_9: Bits = (Bits::new(
            ((s_178_4) >> (s_178_2)).value(),
            u16::try_from(s_178_8).unwrap(),
        ));
        // D s_178_10: cast reint s_178_9 -> u10
        let s_178_10: u16 = (s_178_9.value() as u16);
        // D s_178_11: cast zx s_178_10 -> bv
        let s_178_11: Bits = Bits::new(s_178_10 as u128, 10u16);
        // C s_178_12: const #258u : u10
        let s_178_12: u16 = 258;
        // C s_178_13: cast zx s_178_12 -> bv
        let s_178_13: Bits = Bits::new(s_178_12 as u128, 10u16);
        // D s_178_14: cmp-eq s_178_11 s_178_13
        let s_178_14: bool = ((s_178_11) == (s_178_13));
        // N s_178_15: branch s_178_14 b441 b179
        if s_178_14 {
            return block_441(state, tracer, fn_state);
        } else {
            return block_179(state, tracer, fn_state);
        };
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #0u : u8
        let s_179_0: bool = false;
        // D s_179_1: write-var gs#428372 <= s_179_0
        fn_state.gs_428372 = s_179_0;
        // N s_179_2: jump b180
        return block_180(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var gs#428372:u8
        let s_180_0: bool = fn_state.gs_428372;
        // D s_180_1: not s_180_0
        let s_180_1: bool = !s_180_0;
        // N s_180_2: branch s_180_1 b182 b181
        if s_180_1 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_181(state, tracer, fn_state);
        };
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #2997s : i
        let s_181_0: i128 = 2997;
        // C s_181_1: const #14696u : u32
        let s_181_1: u32 = 14696;
        // N s_181_2: write-reg s_181_1 <= s_181_0
        let s_181_2: () = {
            state.write_register::<i128>(s_181_1 as isize, s_181_0);
            tracer.write_register(s_181_1 as isize, s_181_0);
        };
        // C s_181_3: const #3s : i
        let s_181_3: i128 = 3;
        // C s_181_4: const #3s : i
        let s_181_4: i128 = 3;
        // D s_181_5: read-var u#38331:u16
        let s_181_5: u16 = fn_state.u_38331;
        // D s_181_6: cast zx s_181_5 -> bv
        let s_181_6: Bits = Bits::new(s_181_5 as u128, 16u16);
        // D s_181_7: bit-extract s_181_6 s_181_3 s_181_4
        let s_181_7: Bits = (Bits::new(
            ((s_181_6) >> (s_181_3)).value(),
            u16::try_from(s_181_4).unwrap(),
        ));
        // D s_181_8: cast reint s_181_7 -> u8
        let s_181_8: u8 = (s_181_7.value() as u8);
        // C s_181_9: const #0s : i
        let s_181_9: i128 = 0;
        // C s_181_10: const #3s : i
        let s_181_10: i128 = 3;
        // D s_181_11: read-var u#38331:u16
        let s_181_11: u16 = fn_state.u_38331;
        // D s_181_12: cast zx s_181_11 -> bv
        let s_181_12: Bits = Bits::new(s_181_11 as u128, 16u16);
        // D s_181_13: bit-extract s_181_12 s_181_9 s_181_10
        let s_181_13: Bits = (Bits::new(
            ((s_181_12) >> (s_181_9)).value(),
            u16::try_from(s_181_10).unwrap(),
        ));
        // D s_181_14: cast reint s_181_13 -> u8
        let s_181_14: u8 = (s_181_13.value() as u8);
        // D s_181_15: call decode_aarch32_instrs_LSL_r_T1enc_A_txt(s_181_8, s_181_14)
        let s_181_15: () = decode_aarch32_instrs_LSL_r_T1enc_A_txt(
            state,
            tracer,
            s_181_8,
            s_181_14,
        );
        // N s_181_16: return
        return;
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var merge#var.1:struct
        let s_182_0: u16 = fn_state.merge_var._1;
        // D s_182_1: write-var u#38335 <= s_182_0
        fn_state.u_38335 = s_182_0;
        // C s_182_2: const #11s : i
        let s_182_2: i128 = 11;
        // D s_182_3: read-var u#38335:u16
        let s_182_3: u16 = fn_state.u_38335;
        // D s_182_4: cast zx s_182_3 -> bv
        let s_182_4: Bits = Bits::new(s_182_3 as u128, 16u16);
        // C s_182_5: const #1s : i64
        let s_182_5: i64 = 1;
        // C s_182_6: cast zx s_182_5 -> i
        let s_182_6: i128 = (i128::try_from(s_182_5).unwrap());
        // C s_182_7: const #4s : i
        let s_182_7: i128 = 4;
        // C s_182_8: add s_182_7 s_182_6
        let s_182_8: i128 = (s_182_7 + s_182_6);
        // D s_182_9: bit-extract s_182_4 s_182_2 s_182_8
        let s_182_9: Bits = (Bits::new(
            ((s_182_4) >> (s_182_2)).value(),
            u16::try_from(s_182_8).unwrap(),
        ));
        // D s_182_10: cast reint s_182_9 -> u8
        let s_182_10: u8 = (s_182_9.value() as u8);
        // D s_182_11: cast zx s_182_10 -> bv
        let s_182_11: Bits = Bits::new(s_182_10 as u128, 5u16);
        // C s_182_12: const #1u : u8
        let s_182_12: u8 = 1;
        // C s_182_13: cast zx s_182_12 -> bv
        let s_182_13: Bits = Bits::new(s_182_12 as u128, 5u16);
        // D s_182_14: cmp-eq s_182_11 s_182_13
        let s_182_14: bool = ((s_182_11) == (s_182_13));
        // N s_182_15: branch s_182_14 b440 b183
        if s_182_14 {
            return block_440(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #0u : u8
        let s_183_0: bool = false;
        // D s_183_1: write-var gs#428383 <= s_183_0
        fn_state.gs_428383 = s_183_0;
        // N s_183_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var gs#428383:u8
        let s_184_0: bool = fn_state.gs_428383;
        // D s_184_1: not s_184_0
        let s_184_1: bool = !s_184_0;
        // N s_184_2: branch s_184_1 b186 b185
        if s_184_1 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_185(state, tracer, fn_state);
        };
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #2999s : i
        let s_185_0: i128 = 2999;
        // C s_185_1: const #14696u : u32
        let s_185_1: u32 = 14696;
        // N s_185_2: write-reg s_185_1 <= s_185_0
        let s_185_2: () = {
            state.write_register::<i128>(s_185_1 as isize, s_185_0);
            tracer.write_register(s_185_1 as isize, s_185_0);
        };
        // C s_185_3: const #6s : i
        let s_185_3: i128 = 6;
        // C s_185_4: const #5s : i
        let s_185_4: i128 = 5;
        // D s_185_5: read-var u#38335:u16
        let s_185_5: u16 = fn_state.u_38335;
        // D s_185_6: cast zx s_185_5 -> bv
        let s_185_6: Bits = Bits::new(s_185_5 as u128, 16u16);
        // D s_185_7: bit-extract s_185_6 s_185_3 s_185_4
        let s_185_7: Bits = (Bits::new(
            ((s_185_6) >> (s_185_3)).value(),
            u16::try_from(s_185_4).unwrap(),
        ));
        // D s_185_8: cast reint s_185_7 -> u8
        let s_185_8: u8 = (s_185_7.value() as u8);
        // C s_185_9: const #3s : i
        let s_185_9: i128 = 3;
        // C s_185_10: const #3s : i
        let s_185_10: i128 = 3;
        // D s_185_11: read-var u#38335:u16
        let s_185_11: u16 = fn_state.u_38335;
        // D s_185_12: cast zx s_185_11 -> bv
        let s_185_12: Bits = Bits::new(s_185_11 as u128, 16u16);
        // D s_185_13: bit-extract s_185_12 s_185_9 s_185_10
        let s_185_13: Bits = (Bits::new(
            ((s_185_12) >> (s_185_9)).value(),
            u16::try_from(s_185_10).unwrap(),
        ));
        // D s_185_14: cast reint s_185_13 -> u8
        let s_185_14: u8 = (s_185_13.value() as u8);
        // C s_185_15: const #0s : i
        let s_185_15: i128 = 0;
        // C s_185_16: const #3s : i
        let s_185_16: i128 = 3;
        // D s_185_17: read-var u#38335:u16
        let s_185_17: u16 = fn_state.u_38335;
        // D s_185_18: cast zx s_185_17 -> bv
        let s_185_18: Bits = Bits::new(s_185_17 as u128, 16u16);
        // D s_185_19: bit-extract s_185_18 s_185_15 s_185_16
        let s_185_19: Bits = (Bits::new(
            ((s_185_18) >> (s_185_15)).value(),
            u16::try_from(s_185_16).unwrap(),
        ));
        // D s_185_20: cast reint s_185_19 -> u8
        let s_185_20: u8 = (s_185_19.value() as u8);
        // D s_185_21: call decode_aarch32_instrs_LSR_i_T1enc_A_txt(s_185_8, s_185_14, s_185_20)
        let s_185_21: () = decode_aarch32_instrs_LSR_i_T1enc_A_txt(
            state,
            tracer,
            s_185_8,
            s_185_14,
            s_185_20,
        );
        // N s_185_22: return
        return;
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var merge#var.1:struct
        let s_186_0: u16 = fn_state.merge_var._1;
        // D s_186_1: write-var u#38340 <= s_186_0
        fn_state.u_38340 = s_186_0;
        // C s_186_2: const #6s : i
        let s_186_2: i128 = 6;
        // D s_186_3: read-var u#38340:u16
        let s_186_3: u16 = fn_state.u_38340;
        // D s_186_4: cast zx s_186_3 -> bv
        let s_186_4: Bits = Bits::new(s_186_3 as u128, 16u16);
        // C s_186_5: const #1s : i64
        let s_186_5: i64 = 1;
        // C s_186_6: cast zx s_186_5 -> i
        let s_186_6: i128 = (i128::try_from(s_186_5).unwrap());
        // C s_186_7: const #9s : i
        let s_186_7: i128 = 9;
        // C s_186_8: add s_186_7 s_186_6
        let s_186_8: i128 = (s_186_7 + s_186_6);
        // D s_186_9: bit-extract s_186_4 s_186_2 s_186_8
        let s_186_9: Bits = (Bits::new(
            ((s_186_4) >> (s_186_2)).value(),
            u16::try_from(s_186_8).unwrap(),
        ));
        // D s_186_10: cast reint s_186_9 -> u10
        let s_186_10: u16 = (s_186_9.value() as u16);
        // D s_186_11: cast zx s_186_10 -> bv
        let s_186_11: Bits = Bits::new(s_186_10 as u128, 10u16);
        // C s_186_12: const #259u : u10
        let s_186_12: u16 = 259;
        // C s_186_13: cast zx s_186_12 -> bv
        let s_186_13: Bits = Bits::new(s_186_12 as u128, 10u16);
        // D s_186_14: cmp-eq s_186_11 s_186_13
        let s_186_14: bool = ((s_186_11) == (s_186_13));
        // N s_186_15: branch s_186_14 b439 b187
        if s_186_14 {
            return block_439(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #0u : u8
        let s_187_0: bool = false;
        // D s_187_1: write-var gs#428396 <= s_187_0
        fn_state.gs_428396 = s_187_0;
        // N s_187_2: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var gs#428396:u8
        let s_188_0: bool = fn_state.gs_428396;
        // D s_188_1: not s_188_0
        let s_188_1: bool = !s_188_0;
        // N s_188_2: branch s_188_1 b190 b189
        if s_188_1 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_189(state, tracer, fn_state);
        };
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #3000s : i
        let s_189_0: i128 = 3000;
        // C s_189_1: const #14696u : u32
        let s_189_1: u32 = 14696;
        // N s_189_2: write-reg s_189_1 <= s_189_0
        let s_189_2: () = {
            state.write_register::<i128>(s_189_1 as isize, s_189_0);
            tracer.write_register(s_189_1 as isize, s_189_0);
        };
        // C s_189_3: const #3s : i
        let s_189_3: i128 = 3;
        // C s_189_4: const #3s : i
        let s_189_4: i128 = 3;
        // D s_189_5: read-var u#38340:u16
        let s_189_5: u16 = fn_state.u_38340;
        // D s_189_6: cast zx s_189_5 -> bv
        let s_189_6: Bits = Bits::new(s_189_5 as u128, 16u16);
        // D s_189_7: bit-extract s_189_6 s_189_3 s_189_4
        let s_189_7: Bits = (Bits::new(
            ((s_189_6) >> (s_189_3)).value(),
            u16::try_from(s_189_4).unwrap(),
        ));
        // D s_189_8: cast reint s_189_7 -> u8
        let s_189_8: u8 = (s_189_7.value() as u8);
        // C s_189_9: const #0s : i
        let s_189_9: i128 = 0;
        // C s_189_10: const #3s : i
        let s_189_10: i128 = 3;
        // D s_189_11: read-var u#38340:u16
        let s_189_11: u16 = fn_state.u_38340;
        // D s_189_12: cast zx s_189_11 -> bv
        let s_189_12: Bits = Bits::new(s_189_11 as u128, 16u16);
        // D s_189_13: bit-extract s_189_12 s_189_9 s_189_10
        let s_189_13: Bits = (Bits::new(
            ((s_189_12) >> (s_189_9)).value(),
            u16::try_from(s_189_10).unwrap(),
        ));
        // D s_189_14: cast reint s_189_13 -> u8
        let s_189_14: u8 = (s_189_13.value() as u8);
        // D s_189_15: call decode_aarch32_instrs_LSR_r_T1enc_A_txt(s_189_8, s_189_14)
        let s_189_15: () = decode_aarch32_instrs_LSR_r_T1enc_A_txt(
            state,
            tracer,
            s_189_8,
            s_189_14,
        );
        // N s_189_16: return
        return;
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var merge#var.1:struct
        let s_190_0: u16 = fn_state.merge_var._1;
        // D s_190_1: write-var u#38344 <= s_190_0
        fn_state.u_38344 = s_190_0;
        // C s_190_2: const #11s : i
        let s_190_2: i128 = 11;
        // D s_190_3: read-var u#38344:u16
        let s_190_3: u16 = fn_state.u_38344;
        // D s_190_4: cast zx s_190_3 -> bv
        let s_190_4: Bits = Bits::new(s_190_3 as u128, 16u16);
        // C s_190_5: const #1s : i64
        let s_190_5: i64 = 1;
        // C s_190_6: cast zx s_190_5 -> i
        let s_190_6: i128 = (i128::try_from(s_190_5).unwrap());
        // C s_190_7: const #4s : i
        let s_190_7: i128 = 4;
        // C s_190_8: add s_190_7 s_190_6
        let s_190_8: i128 = (s_190_7 + s_190_6);
        // D s_190_9: bit-extract s_190_4 s_190_2 s_190_8
        let s_190_9: Bits = (Bits::new(
            ((s_190_4) >> (s_190_2)).value(),
            u16::try_from(s_190_8).unwrap(),
        ));
        // D s_190_10: cast reint s_190_9 -> u8
        let s_190_10: u8 = (s_190_9.value() as u8);
        // D s_190_11: cast zx s_190_10 -> bv
        let s_190_11: Bits = Bits::new(s_190_10 as u128, 5u16);
        // C s_190_12: const #4u : u8
        let s_190_12: u8 = 4;
        // C s_190_13: cast zx s_190_12 -> bv
        let s_190_13: Bits = Bits::new(s_190_12 as u128, 5u16);
        // D s_190_14: cmp-eq s_190_11 s_190_13
        let s_190_14: bool = ((s_190_11) == (s_190_13));
        // N s_190_15: branch s_190_14 b438 b191
        if s_190_14 {
            return block_438(state, tracer, fn_state);
        } else {
            return block_191(state, tracer, fn_state);
        };
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #0u : u8
        let s_191_0: bool = false;
        // D s_191_1: write-var gs#428407 <= s_191_0
        fn_state.gs_428407 = s_191_0;
        // N s_191_2: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_192_0: read-var gs#428407:u8
        let s_192_0: bool = fn_state.gs_428407;
        // D s_192_1: not s_192_0
        let s_192_1: bool = !s_192_0;
        // N s_192_2: branch s_192_1 b194 b193
        if s_192_1 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_193(state, tracer, fn_state);
        };
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #3011s : i
        let s_193_0: i128 = 3011;
        // C s_193_1: const #14696u : u32
        let s_193_1: u32 = 14696;
        // N s_193_2: write-reg s_193_1 <= s_193_0
        let s_193_2: () = {
            state.write_register::<i128>(s_193_1 as isize, s_193_0);
            tracer.write_register(s_193_1 as isize, s_193_0);
        };
        // C s_193_3: const #8s : i
        let s_193_3: i128 = 8;
        // C s_193_4: const #3s : i
        let s_193_4: i128 = 3;
        // D s_193_5: read-var u#38344:u16
        let s_193_5: u16 = fn_state.u_38344;
        // D s_193_6: cast zx s_193_5 -> bv
        let s_193_6: Bits = Bits::new(s_193_5 as u128, 16u16);
        // D s_193_7: bit-extract s_193_6 s_193_3 s_193_4
        let s_193_7: Bits = (Bits::new(
            ((s_193_6) >> (s_193_3)).value(),
            u16::try_from(s_193_4).unwrap(),
        ));
        // D s_193_8: cast reint s_193_7 -> u8
        let s_193_8: u8 = (s_193_7.value() as u8);
        // C s_193_9: const #0s : i
        let s_193_9: i128 = 0;
        // C s_193_10: const #8s : i
        let s_193_10: i128 = 8;
        // D s_193_11: read-var u#38344:u16
        let s_193_11: u16 = fn_state.u_38344;
        // D s_193_12: cast zx s_193_11 -> bv
        let s_193_12: Bits = Bits::new(s_193_11 as u128, 16u16);
        // D s_193_13: bit-extract s_193_12 s_193_9 s_193_10
        let s_193_13: Bits = (Bits::new(
            ((s_193_12) >> (s_193_9)).value(),
            u16::try_from(s_193_10).unwrap(),
        ));
        // D s_193_14: cast reint s_193_13 -> u8
        let s_193_14: u8 = (s_193_13.value() as u8);
        // D s_193_15: call decode_aarch32_instrs_MOV_i_T1enc_A_txt(s_193_8, s_193_14)
        let s_193_15: () = decode_aarch32_instrs_MOV_i_T1enc_A_txt(
            state,
            tracer,
            s_193_8,
            s_193_14,
        );
        // N s_193_16: return
        return;
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var merge#var.1:struct
        let s_194_0: u16 = fn_state.merge_var._1;
        // D s_194_1: write-var u#38348 <= s_194_0
        fn_state.u_38348 = s_194_0;
        // C s_194_2: const #8s : i
        let s_194_2: i128 = 8;
        // D s_194_3: read-var u#38348:u16
        let s_194_3: u16 = fn_state.u_38348;
        // D s_194_4: cast zx s_194_3 -> bv
        let s_194_4: Bits = Bits::new(s_194_3 as u128, 16u16);
        // C s_194_5: const #1s : i64
        let s_194_5: i64 = 1;
        // C s_194_6: cast zx s_194_5 -> i
        let s_194_6: i128 = (i128::try_from(s_194_5).unwrap());
        // C s_194_7: const #7s : i
        let s_194_7: i128 = 7;
        // C s_194_8: add s_194_7 s_194_6
        let s_194_8: i128 = (s_194_7 + s_194_6);
        // D s_194_9: bit-extract s_194_4 s_194_2 s_194_8
        let s_194_9: Bits = (Bits::new(
            ((s_194_4) >> (s_194_2)).value(),
            u16::try_from(s_194_8).unwrap(),
        ));
        // D s_194_10: cast reint s_194_9 -> u8
        let s_194_10: u8 = (s_194_9.value() as u8);
        // D s_194_11: cast zx s_194_10 -> bv
        let s_194_11: Bits = Bits::new(s_194_10 as u128, 8u16);
        // C s_194_12: const #70u : u8
        let s_194_12: u8 = 70;
        // C s_194_13: cast zx s_194_12 -> bv
        let s_194_13: Bits = Bits::new(s_194_12 as u128, 8u16);
        // D s_194_14: cmp-eq s_194_11 s_194_13
        let s_194_14: bool = ((s_194_11) == (s_194_13));
        // N s_194_15: branch s_194_14 b437 b195
        if s_194_14 {
            return block_437(state, tracer, fn_state);
        } else {
            return block_195(state, tracer, fn_state);
        };
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #0u : u8
        let s_195_0: bool = false;
        // D s_195_1: write-var gs#428418 <= s_195_0
        fn_state.gs_428418 = s_195_0;
        // N s_195_2: jump b196
        return block_196(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_196_0: read-var gs#428418:u8
        let s_196_0: bool = fn_state.gs_428418;
        // D s_196_1: not s_196_0
        let s_196_1: bool = !s_196_0;
        // N s_196_2: branch s_196_1 b198 b197
        if s_196_1 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_197(state, tracer, fn_state);
        };
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #3015s : i
        let s_197_0: i128 = 3015;
        // C s_197_1: const #14696u : u32
        let s_197_1: u32 = 14696;
        // N s_197_2: write-reg s_197_1 <= s_197_0
        let s_197_2: () = {
            state.write_register::<i128>(s_197_1 as isize, s_197_0);
            tracer.write_register(s_197_1 as isize, s_197_0);
        };
        // C s_197_3: const #7s : i
        let s_197_3: i128 = 7;
        // C s_197_4: const #1s : i
        let s_197_4: i128 = 1;
        // D s_197_5: read-var u#38348:u16
        let s_197_5: u16 = fn_state.u_38348;
        // D s_197_6: cast zx s_197_5 -> bv
        let s_197_6: Bits = Bits::new(s_197_5 as u128, 16u16);
        // D s_197_7: bit-extract s_197_6 s_197_3 s_197_4
        let s_197_7: Bits = (Bits::new(
            ((s_197_6) >> (s_197_3)).value(),
            u16::try_from(s_197_4).unwrap(),
        ));
        // D s_197_8: cast reint s_197_7 -> u8
        let s_197_8: bool = ((s_197_7.value()) != 0);
        // C s_197_9: const #3s : i
        let s_197_9: i128 = 3;
        // C s_197_10: const #4s : i
        let s_197_10: i128 = 4;
        // D s_197_11: read-var u#38348:u16
        let s_197_11: u16 = fn_state.u_38348;
        // D s_197_12: cast zx s_197_11 -> bv
        let s_197_12: Bits = Bits::new(s_197_11 as u128, 16u16);
        // D s_197_13: bit-extract s_197_12 s_197_9 s_197_10
        let s_197_13: Bits = (Bits::new(
            ((s_197_12) >> (s_197_9)).value(),
            u16::try_from(s_197_10).unwrap(),
        ));
        // D s_197_14: cast reint s_197_13 -> u8
        let s_197_14: u8 = (s_197_13.value() as u8);
        // C s_197_15: const #0s : i
        let s_197_15: i128 = 0;
        // C s_197_16: const #3s : i
        let s_197_16: i128 = 3;
        // D s_197_17: read-var u#38348:u16
        let s_197_17: u16 = fn_state.u_38348;
        // D s_197_18: cast zx s_197_17 -> bv
        let s_197_18: Bits = Bits::new(s_197_17 as u128, 16u16);
        // D s_197_19: bit-extract s_197_18 s_197_15 s_197_16
        let s_197_19: Bits = (Bits::new(
            ((s_197_18) >> (s_197_15)).value(),
            u16::try_from(s_197_16).unwrap(),
        ));
        // D s_197_20: cast reint s_197_19 -> u8
        let s_197_20: u8 = (s_197_19.value() as u8);
        // D s_197_21: call decode_aarch32_instrs_MOV_r_T1enc_A_txt(s_197_8, s_197_14, s_197_20)
        let s_197_21: () = decode_aarch32_instrs_MOV_r_T1enc_A_txt(
            state,
            tracer,
            s_197_8,
            s_197_14,
            s_197_20,
        );
        // N s_197_22: return
        return;
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_198_0: read-var merge#var.1:struct
        let s_198_0: u16 = fn_state.merge_var._1;
        // D s_198_1: write-var u#38352 <= s_198_0
        fn_state.u_38352 = s_198_0;
        // C s_198_2: const #6s : i
        let s_198_2: i128 = 6;
        // D s_198_3: read-var u#38352:u16
        let s_198_3: u16 = fn_state.u_38352;
        // D s_198_4: cast zx s_198_3 -> bv
        let s_198_4: Bits = Bits::new(s_198_3 as u128, 16u16);
        // C s_198_5: const #1s : i64
        let s_198_5: i64 = 1;
        // C s_198_6: cast zx s_198_5 -> i
        let s_198_6: i128 = (i128::try_from(s_198_5).unwrap());
        // C s_198_7: const #9s : i
        let s_198_7: i128 = 9;
        // C s_198_8: add s_198_7 s_198_6
        let s_198_8: i128 = (s_198_7 + s_198_6);
        // D s_198_9: bit-extract s_198_4 s_198_2 s_198_8
        let s_198_9: Bits = (Bits::new(
            ((s_198_4) >> (s_198_2)).value(),
            u16::try_from(s_198_8).unwrap(),
        ));
        // D s_198_10: cast reint s_198_9 -> u10
        let s_198_10: u16 = (s_198_9.value() as u16);
        // D s_198_11: cast zx s_198_10 -> bv
        let s_198_11: Bits = Bits::new(s_198_10 as u128, 10u16);
        // C s_198_12: const #0u : u10
        let s_198_12: u16 = 0;
        // C s_198_13: cast zx s_198_12 -> bv
        let s_198_13: Bits = Bits::new(s_198_12 as u128, 10u16);
        // D s_198_14: cmp-eq s_198_11 s_198_13
        let s_198_14: bool = ((s_198_11) == (s_198_13));
        // N s_198_15: branch s_198_14 b436 b199
        if s_198_14 {
            return block_436(state, tracer, fn_state);
        } else {
            return block_199(state, tracer, fn_state);
        };
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #0u : u8
        let s_199_0: bool = false;
        // D s_199_1: write-var gs#428431 <= s_199_0
        fn_state.gs_428431 = s_199_0;
        // N s_199_2: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var gs#428431:u8
        let s_200_0: bool = fn_state.gs_428431;
        // D s_200_1: not s_200_0
        let s_200_1: bool = !s_200_0;
        // N s_200_2: branch s_200_1 b202 b201
        if s_200_1 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_201(state, tracer, fn_state);
        };
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #3016s : i
        let s_201_0: i128 = 3016;
        // C s_201_1: const #14696u : u32
        let s_201_1: u32 = 14696;
        // N s_201_2: write-reg s_201_1 <= s_201_0
        let s_201_2: () = {
            state.write_register::<i128>(s_201_1 as isize, s_201_0);
            tracer.write_register(s_201_1 as isize, s_201_0);
        };
        // C s_201_3: const #3s : i
        let s_201_3: i128 = 3;
        // C s_201_4: const #3s : i
        let s_201_4: i128 = 3;
        // D s_201_5: read-var u#38352:u16
        let s_201_5: u16 = fn_state.u_38352;
        // D s_201_6: cast zx s_201_5 -> bv
        let s_201_6: Bits = Bits::new(s_201_5 as u128, 16u16);
        // D s_201_7: bit-extract s_201_6 s_201_3 s_201_4
        let s_201_7: Bits = (Bits::new(
            ((s_201_6) >> (s_201_3)).value(),
            u16::try_from(s_201_4).unwrap(),
        ));
        // D s_201_8: cast reint s_201_7 -> u8
        let s_201_8: u8 = (s_201_7.value() as u8);
        // C s_201_9: const #0s : i
        let s_201_9: i128 = 0;
        // C s_201_10: const #3s : i
        let s_201_10: i128 = 3;
        // D s_201_11: read-var u#38352:u16
        let s_201_11: u16 = fn_state.u_38352;
        // D s_201_12: cast zx s_201_11 -> bv
        let s_201_12: Bits = Bits::new(s_201_11 as u128, 16u16);
        // D s_201_13: bit-extract s_201_12 s_201_9 s_201_10
        let s_201_13: Bits = (Bits::new(
            ((s_201_12) >> (s_201_9)).value(),
            u16::try_from(s_201_10).unwrap(),
        ));
        // D s_201_14: cast reint s_201_13 -> u8
        let s_201_14: u8 = (s_201_13.value() as u8);
        // D s_201_15: call decode_aarch32_instrs_MOV_r_T2enc_A_txt(s_201_8, s_201_14)
        let s_201_15: () = decode_aarch32_instrs_MOV_r_T2enc_A_txt(
            state,
            tracer,
            s_201_8,
            s_201_14,
        );
        // N s_201_16: return
        return;
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_202_0: read-var merge#var.1:struct
        let s_202_0: u16 = fn_state.merge_var._1;
        // D s_202_1: write-var u#38356 <= s_202_0
        fn_state.u_38356 = s_202_0;
        // C s_202_2: const #10s : i
        let s_202_2: i128 = 10;
        // D s_202_3: read-var u#38356:u16
        let s_202_3: u16 = fn_state.u_38356;
        // D s_202_4: cast zx s_202_3 -> bv
        let s_202_4: Bits = Bits::new(s_202_3 as u128, 16u16);
        // C s_202_5: const #1s : i64
        let s_202_5: i64 = 1;
        // C s_202_6: cast zx s_202_5 -> i
        let s_202_6: i128 = (i128::try_from(s_202_5).unwrap());
        // C s_202_7: const #5s : i
        let s_202_7: i128 = 5;
        // C s_202_8: add s_202_7 s_202_6
        let s_202_8: i128 = (s_202_7 + s_202_6);
        // D s_202_9: bit-extract s_202_4 s_202_2 s_202_8
        let s_202_9: Bits = (Bits::new(
            ((s_202_4) >> (s_202_2)).value(),
            u16::try_from(s_202_8).unwrap(),
        ));
        // D s_202_10: cast reint s_202_9 -> u8
        let s_202_10: u8 = (s_202_9.value() as u8);
        // D s_202_11: cast zx s_202_10 -> bv
        let s_202_11: Bits = Bits::new(s_202_10 as u128, 6u16);
        // C s_202_12: const #16u : u8
        let s_202_12: u8 = 16;
        // C s_202_13: cast zx s_202_12 -> bv
        let s_202_13: Bits = Bits::new(s_202_12 as u128, 6u16);
        // D s_202_14: cmp-eq s_202_11 s_202_13
        let s_202_14: bool = ((s_202_11) == (s_202_13));
        // N s_202_15: branch s_202_14 b435 b203
        if s_202_14 {
            return block_435(state, tracer, fn_state);
        } else {
            return block_203(state, tracer, fn_state);
        };
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #0u : u8
        let s_203_0: bool = false;
        // D s_203_1: write-var gs#428442 <= s_203_0
        fn_state.gs_428442 = s_203_0;
        // N s_203_2: jump b204
        return block_204(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_204_0: read-var gs#428442:u8
        let s_204_0: bool = fn_state.gs_428442;
        // D s_204_1: not s_204_0
        let s_204_1: bool = !s_204_0;
        // N s_204_2: branch s_204_1 b206 b205
        if s_204_1 {
            return block_206(state, tracer, fn_state);
        } else {
            return block_205(state, tracer, fn_state);
        };
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #3019s : i
        let s_205_0: i128 = 3019;
        // C s_205_1: const #14696u : u32
        let s_205_1: u32 = 14696;
        // N s_205_2: write-reg s_205_1 <= s_205_0
        let s_205_2: () = {
            state.write_register::<i128>(s_205_1 as isize, s_205_0);
            tracer.write_register(s_205_1 as isize, s_205_0);
        };
        // C s_205_3: const #6s : i
        let s_205_3: i128 = 6;
        // C s_205_4: const #4s : i
        let s_205_4: i128 = 4;
        // D s_205_5: read-var u#38356:u16
        let s_205_5: u16 = fn_state.u_38356;
        // D s_205_6: cast zx s_205_5 -> bv
        let s_205_6: Bits = Bits::new(s_205_5 as u128, 16u16);
        // D s_205_7: bit-extract s_205_6 s_205_3 s_205_4
        let s_205_7: Bits = (Bits::new(
            ((s_205_6) >> (s_205_3)).value(),
            u16::try_from(s_205_4).unwrap(),
        ));
        // D s_205_8: cast reint s_205_7 -> u8
        let s_205_8: u8 = (s_205_7.value() as u8);
        // C s_205_9: const #3s : i
        let s_205_9: i128 = 3;
        // C s_205_10: const #3s : i
        let s_205_10: i128 = 3;
        // D s_205_11: read-var u#38356:u16
        let s_205_11: u16 = fn_state.u_38356;
        // D s_205_12: cast zx s_205_11 -> bv
        let s_205_12: Bits = Bits::new(s_205_11 as u128, 16u16);
        // D s_205_13: bit-extract s_205_12 s_205_9 s_205_10
        let s_205_13: Bits = (Bits::new(
            ((s_205_12) >> (s_205_9)).value(),
            u16::try_from(s_205_10).unwrap(),
        ));
        // D s_205_14: cast reint s_205_13 -> u8
        let s_205_14: u8 = (s_205_13.value() as u8);
        // C s_205_15: const #0s : i
        let s_205_15: i128 = 0;
        // C s_205_16: const #3s : i
        let s_205_16: i128 = 3;
        // D s_205_17: read-var u#38356:u16
        let s_205_17: u16 = fn_state.u_38356;
        // D s_205_18: cast zx s_205_17 -> bv
        let s_205_18: Bits = Bits::new(s_205_17 as u128, 16u16);
        // D s_205_19: bit-extract s_205_18 s_205_15 s_205_16
        let s_205_19: Bits = (Bits::new(
            ((s_205_18) >> (s_205_15)).value(),
            u16::try_from(s_205_16).unwrap(),
        ));
        // D s_205_20: cast reint s_205_19 -> u8
        let s_205_20: u8 = (s_205_19.value() as u8);
        // D s_205_21: call decode_aarch32_instrs_MOV_rr_T1enc_A_txt(s_205_8, s_205_14, s_205_20)
        let s_205_21: () = decode_aarch32_instrs_MOV_rr_T1enc_A_txt(
            state,
            tracer,
            s_205_8,
            s_205_14,
            s_205_20,
        );
        // N s_205_22: return
        return;
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var merge#var.1:struct
        let s_206_0: u16 = fn_state.merge_var._1;
        // D s_206_1: write-var u#38360 <= s_206_0
        fn_state.u_38360 = s_206_0;
        // C s_206_2: const #6s : i
        let s_206_2: i128 = 6;
        // D s_206_3: read-var u#38360:u16
        let s_206_3: u16 = fn_state.u_38360;
        // D s_206_4: cast zx s_206_3 -> bv
        let s_206_4: Bits = Bits::new(s_206_3 as u128, 16u16);
        // C s_206_5: const #1s : i64
        let s_206_5: i64 = 1;
        // C s_206_6: cast zx s_206_5 -> i
        let s_206_6: i128 = (i128::try_from(s_206_5).unwrap());
        // C s_206_7: const #9s : i
        let s_206_7: i128 = 9;
        // C s_206_8: add s_206_7 s_206_6
        let s_206_8: i128 = (s_206_7 + s_206_6);
        // D s_206_9: bit-extract s_206_4 s_206_2 s_206_8
        let s_206_9: Bits = (Bits::new(
            ((s_206_4) >> (s_206_2)).value(),
            u16::try_from(s_206_8).unwrap(),
        ));
        // D s_206_10: cast reint s_206_9 -> u10
        let s_206_10: u16 = (s_206_9.value() as u16);
        // D s_206_11: cast zx s_206_10 -> bv
        let s_206_11: Bits = Bits::new(s_206_10 as u128, 10u16);
        // C s_206_12: const #269u : u10
        let s_206_12: u16 = 269;
        // C s_206_13: cast zx s_206_12 -> bv
        let s_206_13: Bits = Bits::new(s_206_12 as u128, 10u16);
        // D s_206_14: cmp-eq s_206_11 s_206_13
        let s_206_14: bool = ((s_206_11) == (s_206_13));
        // N s_206_15: branch s_206_14 b434 b207
        if s_206_14 {
            return block_434(state, tracer, fn_state);
        } else {
            return block_207(state, tracer, fn_state);
        };
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #0u : u8
        let s_207_0: bool = false;
        // D s_207_1: write-var gs#428455 <= s_207_0
        fn_state.gs_428455 = s_207_0;
        // N s_207_2: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var gs#428455:u8
        let s_208_0: bool = fn_state.gs_428455;
        // D s_208_1: not s_208_0
        let s_208_1: bool = !s_208_0;
        // N s_208_2: branch s_208_1 b210 b209
        if s_208_1 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_209(state, tracer, fn_state);
        };
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #3028s : i
        let s_209_0: i128 = 3028;
        // C s_209_1: const #14696u : u32
        let s_209_1: u32 = 14696;
        // N s_209_2: write-reg s_209_1 <= s_209_0
        let s_209_2: () = {
            state.write_register::<i128>(s_209_1 as isize, s_209_0);
            tracer.write_register(s_209_1 as isize, s_209_0);
        };
        // C s_209_3: const #3s : i
        let s_209_3: i128 = 3;
        // C s_209_4: const #3s : i
        let s_209_4: i128 = 3;
        // D s_209_5: read-var u#38360:u16
        let s_209_5: u16 = fn_state.u_38360;
        // D s_209_6: cast zx s_209_5 -> bv
        let s_209_6: Bits = Bits::new(s_209_5 as u128, 16u16);
        // D s_209_7: bit-extract s_209_6 s_209_3 s_209_4
        let s_209_7: Bits = (Bits::new(
            ((s_209_6) >> (s_209_3)).value(),
            u16::try_from(s_209_4).unwrap(),
        ));
        // D s_209_8: cast reint s_209_7 -> u8
        let s_209_8: u8 = (s_209_7.value() as u8);
        // C s_209_9: const #0s : i
        let s_209_9: i128 = 0;
        // C s_209_10: const #3s : i
        let s_209_10: i128 = 3;
        // D s_209_11: read-var u#38360:u16
        let s_209_11: u16 = fn_state.u_38360;
        // D s_209_12: cast zx s_209_11 -> bv
        let s_209_12: Bits = Bits::new(s_209_11 as u128, 16u16);
        // D s_209_13: bit-extract s_209_12 s_209_9 s_209_10
        let s_209_13: Bits = (Bits::new(
            ((s_209_12) >> (s_209_9)).value(),
            u16::try_from(s_209_10).unwrap(),
        ));
        // D s_209_14: cast reint s_209_13 -> u8
        let s_209_14: u8 = (s_209_13.value() as u8);
        // D s_209_15: call decode_aarch32_instrs_MUL_T1enc_A_txt(s_209_8, s_209_14)
        let s_209_15: () = decode_aarch32_instrs_MUL_T1enc_A_txt(
            state,
            tracer,
            s_209_8,
            s_209_14,
        );
        // N s_209_16: return
        return;
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_210_0: read-var merge#var.1:struct
        let s_210_0: u16 = fn_state.merge_var._1;
        // D s_210_1: write-var u#38364 <= s_210_0
        fn_state.u_38364 = s_210_0;
        // C s_210_2: const #6s : i
        let s_210_2: i128 = 6;
        // D s_210_3: read-var u#38364:u16
        let s_210_3: u16 = fn_state.u_38364;
        // D s_210_4: cast zx s_210_3 -> bv
        let s_210_4: Bits = Bits::new(s_210_3 as u128, 16u16);
        // C s_210_5: const #1s : i64
        let s_210_5: i64 = 1;
        // C s_210_6: cast zx s_210_5 -> i
        let s_210_6: i128 = (i128::try_from(s_210_5).unwrap());
        // C s_210_7: const #9s : i
        let s_210_7: i128 = 9;
        // C s_210_8: add s_210_7 s_210_6
        let s_210_8: i128 = (s_210_7 + s_210_6);
        // D s_210_9: bit-extract s_210_4 s_210_2 s_210_8
        let s_210_9: Bits = (Bits::new(
            ((s_210_4) >> (s_210_2)).value(),
            u16::try_from(s_210_8).unwrap(),
        ));
        // D s_210_10: cast reint s_210_9 -> u10
        let s_210_10: u16 = (s_210_9.value() as u16);
        // D s_210_11: cast zx s_210_10 -> bv
        let s_210_11: Bits = Bits::new(s_210_10 as u128, 10u16);
        // C s_210_12: const #271u : u10
        let s_210_12: u16 = 271;
        // C s_210_13: cast zx s_210_12 -> bv
        let s_210_13: Bits = Bits::new(s_210_12 as u128, 10u16);
        // D s_210_14: cmp-eq s_210_11 s_210_13
        let s_210_14: bool = ((s_210_11) == (s_210_13));
        // N s_210_15: branch s_210_14 b433 b211
        if s_210_14 {
            return block_433(state, tracer, fn_state);
        } else {
            return block_211(state, tracer, fn_state);
        };
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #0u : u8
        let s_211_0: bool = false;
        // D s_211_1: write-var gs#428466 <= s_211_0
        fn_state.gs_428466 = s_211_0;
        // N s_211_2: jump b212
        return block_212(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_212_0: read-var gs#428466:u8
        let s_212_0: bool = fn_state.gs_428466;
        // D s_212_1: not s_212_0
        let s_212_1: bool = !s_212_0;
        // N s_212_2: branch s_212_1 b214 b213
        if s_212_1 {
            return block_214(state, tracer, fn_state);
        } else {
            return block_213(state, tracer, fn_state);
        };
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #3033s : i
        let s_213_0: i128 = 3033;
        // C s_213_1: const #14696u : u32
        let s_213_1: u32 = 14696;
        // N s_213_2: write-reg s_213_1 <= s_213_0
        let s_213_2: () = {
            state.write_register::<i128>(s_213_1 as isize, s_213_0);
            tracer.write_register(s_213_1 as isize, s_213_0);
        };
        // C s_213_3: const #3s : i
        let s_213_3: i128 = 3;
        // C s_213_4: const #3s : i
        let s_213_4: i128 = 3;
        // D s_213_5: read-var u#38364:u16
        let s_213_5: u16 = fn_state.u_38364;
        // D s_213_6: cast zx s_213_5 -> bv
        let s_213_6: Bits = Bits::new(s_213_5 as u128, 16u16);
        // D s_213_7: bit-extract s_213_6 s_213_3 s_213_4
        let s_213_7: Bits = (Bits::new(
            ((s_213_6) >> (s_213_3)).value(),
            u16::try_from(s_213_4).unwrap(),
        ));
        // D s_213_8: cast reint s_213_7 -> u8
        let s_213_8: u8 = (s_213_7.value() as u8);
        // C s_213_9: const #0s : i
        let s_213_9: i128 = 0;
        // C s_213_10: const #3s : i
        let s_213_10: i128 = 3;
        // D s_213_11: read-var u#38364:u16
        let s_213_11: u16 = fn_state.u_38364;
        // D s_213_12: cast zx s_213_11 -> bv
        let s_213_12: Bits = Bits::new(s_213_11 as u128, 16u16);
        // D s_213_13: bit-extract s_213_12 s_213_9 s_213_10
        let s_213_13: Bits = (Bits::new(
            ((s_213_12) >> (s_213_9)).value(),
            u16::try_from(s_213_10).unwrap(),
        ));
        // D s_213_14: cast reint s_213_13 -> u8
        let s_213_14: u8 = (s_213_13.value() as u8);
        // D s_213_15: call decode_aarch32_instrs_MVN_r_T1enc_A_txt(s_213_8, s_213_14)
        let s_213_15: () = decode_aarch32_instrs_MVN_r_T1enc_A_txt(
            state,
            tracer,
            s_213_8,
            s_213_14,
        );
        // N s_213_16: return
        return;
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_214_0: read-var merge#var.1:struct
        let s_214_0: u16 = fn_state.merge_var._1;
        // D s_214_1: cast zx s_214_0 -> bv
        let s_214_1: Bits = Bits::new(s_214_0 as u128, 16u16);
        // C s_214_2: const #48896u : u16
        let s_214_2: u16 = 48896;
        // C s_214_3: cast zx s_214_2 -> bv
        let s_214_3: Bits = Bits::new(s_214_2 as u128, 16u16);
        // D s_214_4: cmp-eq s_214_1 s_214_3
        let s_214_4: bool = ((s_214_1) == (s_214_3));
        // N s_214_5: branch s_214_4 b432 b215
        if s_214_4 {
            return block_432(state, tracer, fn_state);
        } else {
            return block_215(state, tracer, fn_state);
        };
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_215_0: const #0u : u8
        let s_215_0: bool = false;
        // D s_215_1: write-var gs#428475 <= s_215_0
        fn_state.gs_428475 = s_215_0;
        // N s_215_2: jump b216
        return block_216(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_216_0: read-var gs#428475:u8
        let s_216_0: bool = fn_state.gs_428475;
        // D s_216_1: not s_216_0
        let s_216_1: bool = !s_216_0;
        // N s_216_2: branch s_216_1 b218 b217
        if s_216_1 {
            return block_218(state, tracer, fn_state);
        } else {
            return block_217(state, tracer, fn_state);
        };
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #3037s : i
        let s_217_0: i128 = 3037;
        // C s_217_1: const #14696u : u32
        let s_217_1: u32 = 14696;
        // N s_217_2: write-reg s_217_1 <= s_217_0
        let s_217_2: () = {
            state.write_register::<i128>(s_217_1 as isize, s_217_0);
            tracer.write_register(s_217_1 as isize, s_217_0);
        };
        // C s_217_3: const #() : ()
        let s_217_3: () = ();
        // S s_217_4: call decode_aarch32_instrs_NOP_T1enc_A_txt(s_217_3)
        let s_217_4: () = decode_aarch32_instrs_NOP_T1enc_A_txt(state, tracer, s_217_3);
        // N s_217_5: return
        return;
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_218_0: read-var merge#var.1:struct
        let s_218_0: u16 = fn_state.merge_var._1;
        // D s_218_1: write-var u#38370 <= s_218_0
        fn_state.u_38370 = s_218_0;
        // C s_218_2: const #6s : i
        let s_218_2: i128 = 6;
        // D s_218_3: read-var u#38370:u16
        let s_218_3: u16 = fn_state.u_38370;
        // D s_218_4: cast zx s_218_3 -> bv
        let s_218_4: Bits = Bits::new(s_218_3 as u128, 16u16);
        // C s_218_5: const #1s : i64
        let s_218_5: i64 = 1;
        // C s_218_6: cast zx s_218_5 -> i
        let s_218_6: i128 = (i128::try_from(s_218_5).unwrap());
        // C s_218_7: const #9s : i
        let s_218_7: i128 = 9;
        // C s_218_8: add s_218_7 s_218_6
        let s_218_8: i128 = (s_218_7 + s_218_6);
        // D s_218_9: bit-extract s_218_4 s_218_2 s_218_8
        let s_218_9: Bits = (Bits::new(
            ((s_218_4) >> (s_218_2)).value(),
            u16::try_from(s_218_8).unwrap(),
        ));
        // D s_218_10: cast reint s_218_9 -> u10
        let s_218_10: u16 = (s_218_9.value() as u16);
        // D s_218_11: cast zx s_218_10 -> bv
        let s_218_11: Bits = Bits::new(s_218_10 as u128, 10u16);
        // C s_218_12: const #268u : u10
        let s_218_12: u16 = 268;
        // C s_218_13: cast zx s_218_12 -> bv
        let s_218_13: Bits = Bits::new(s_218_12 as u128, 10u16);
        // D s_218_14: cmp-eq s_218_11 s_218_13
        let s_218_14: bool = ((s_218_11) == (s_218_13));
        // N s_218_15: branch s_218_14 b431 b219
        if s_218_14 {
            return block_431(state, tracer, fn_state);
        } else {
            return block_219(state, tracer, fn_state);
        };
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #0u : u8
        let s_219_0: bool = false;
        // D s_219_1: write-var gs#428482 <= s_219_0
        fn_state.gs_428482 = s_219_0;
        // N s_219_2: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_220_0: read-var gs#428482:u8
        let s_220_0: bool = fn_state.gs_428482;
        // D s_220_1: not s_220_0
        let s_220_1: bool = !s_220_0;
        // N s_220_2: branch s_220_1 b222 b221
        if s_220_1 {
            return block_222(state, tracer, fn_state);
        } else {
            return block_221(state, tracer, fn_state);
        };
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_221_0: const #3044s : i
        let s_221_0: i128 = 3044;
        // C s_221_1: const #14696u : u32
        let s_221_1: u32 = 14696;
        // N s_221_2: write-reg s_221_1 <= s_221_0
        let s_221_2: () = {
            state.write_register::<i128>(s_221_1 as isize, s_221_0);
            tracer.write_register(s_221_1 as isize, s_221_0);
        };
        // C s_221_3: const #3s : i
        let s_221_3: i128 = 3;
        // C s_221_4: const #3s : i
        let s_221_4: i128 = 3;
        // D s_221_5: read-var u#38370:u16
        let s_221_5: u16 = fn_state.u_38370;
        // D s_221_6: cast zx s_221_5 -> bv
        let s_221_6: Bits = Bits::new(s_221_5 as u128, 16u16);
        // D s_221_7: bit-extract s_221_6 s_221_3 s_221_4
        let s_221_7: Bits = (Bits::new(
            ((s_221_6) >> (s_221_3)).value(),
            u16::try_from(s_221_4).unwrap(),
        ));
        // D s_221_8: cast reint s_221_7 -> u8
        let s_221_8: u8 = (s_221_7.value() as u8);
        // C s_221_9: const #0s : i
        let s_221_9: i128 = 0;
        // C s_221_10: const #3s : i
        let s_221_10: i128 = 3;
        // D s_221_11: read-var u#38370:u16
        let s_221_11: u16 = fn_state.u_38370;
        // D s_221_12: cast zx s_221_11 -> bv
        let s_221_12: Bits = Bits::new(s_221_11 as u128, 16u16);
        // D s_221_13: bit-extract s_221_12 s_221_9 s_221_10
        let s_221_13: Bits = (Bits::new(
            ((s_221_12) >> (s_221_9)).value(),
            u16::try_from(s_221_10).unwrap(),
        ));
        // D s_221_14: cast reint s_221_13 -> u8
        let s_221_14: u8 = (s_221_13.value() as u8);
        // D s_221_15: call decode_aarch32_instrs_ORR_r_T1enc_A_txt(s_221_8, s_221_14)
        let s_221_15: () = decode_aarch32_instrs_ORR_r_T1enc_A_txt(
            state,
            tracer,
            s_221_8,
            s_221_14,
        );
        // N s_221_16: return
        return;
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_222_0: read-var merge#var.1:struct
        let s_222_0: u16 = fn_state.merge_var._1;
        // D s_222_1: write-var u#38374 <= s_222_0
        fn_state.u_38374 = s_222_0;
        // C s_222_2: const #9s : i
        let s_222_2: i128 = 9;
        // D s_222_3: read-var u#38374:u16
        let s_222_3: u16 = fn_state.u_38374;
        // D s_222_4: cast zx s_222_3 -> bv
        let s_222_4: Bits = Bits::new(s_222_3 as u128, 16u16);
        // C s_222_5: const #1s : i64
        let s_222_5: i64 = 1;
        // C s_222_6: cast zx s_222_5 -> i
        let s_222_6: i128 = (i128::try_from(s_222_5).unwrap());
        // C s_222_7: const #6s : i
        let s_222_7: i128 = 6;
        // C s_222_8: add s_222_7 s_222_6
        let s_222_8: i128 = (s_222_7 + s_222_6);
        // D s_222_9: bit-extract s_222_4 s_222_2 s_222_8
        let s_222_9: Bits = (Bits::new(
            ((s_222_4) >> (s_222_2)).value(),
            u16::try_from(s_222_8).unwrap(),
        ));
        // D s_222_10: cast reint s_222_9 -> u8
        let s_222_10: u8 = (s_222_9.value() as u8);
        // D s_222_11: cast zx s_222_10 -> bv
        let s_222_11: Bits = Bits::new(s_222_10 as u128, 7u16);
        // C s_222_12: const #94u : u8
        let s_222_12: u8 = 94;
        // C s_222_13: cast zx s_222_12 -> bv
        let s_222_13: Bits = Bits::new(s_222_12 as u128, 7u16);
        // D s_222_14: cmp-eq s_222_11 s_222_13
        let s_222_14: bool = ((s_222_11) == (s_222_13));
        // N s_222_15: branch s_222_14 b430 b223
        if s_222_14 {
            return block_430(state, tracer, fn_state);
        } else {
            return block_223(state, tracer, fn_state);
        };
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_223_0: const #0u : u8
        let s_223_0: bool = false;
        // D s_223_1: write-var gs#428493 <= s_223_0
        fn_state.gs_428493 = s_223_0;
        // N s_223_2: jump b224
        return block_224(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_224_0: read-var gs#428493:u8
        let s_224_0: bool = fn_state.gs_428493;
        // D s_224_1: not s_224_0
        let s_224_1: bool = !s_224_0;
        // N s_224_2: branch s_224_1 b226 b225
        if s_224_1 {
            return block_226(state, tracer, fn_state);
        } else {
            return block_225(state, tracer, fn_state);
        };
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_225_0: const #3062s : i
        let s_225_0: i128 = 3062;
        // C s_225_1: const #14696u : u32
        let s_225_1: u32 = 14696;
        // N s_225_2: write-reg s_225_1 <= s_225_0
        let s_225_2: () = {
            state.write_register::<i128>(s_225_1 as isize, s_225_0);
            tracer.write_register(s_225_1 as isize, s_225_0);
        };
        // C s_225_3: const #8s : i
        let s_225_3: i128 = 8;
        // C s_225_4: const #1s : i
        let s_225_4: i128 = 1;
        // D s_225_5: read-var u#38374:u16
        let s_225_5: u16 = fn_state.u_38374;
        // D s_225_6: cast zx s_225_5 -> bv
        let s_225_6: Bits = Bits::new(s_225_5 as u128, 16u16);
        // D s_225_7: bit-extract s_225_6 s_225_3 s_225_4
        let s_225_7: Bits = (Bits::new(
            ((s_225_6) >> (s_225_3)).value(),
            u16::try_from(s_225_4).unwrap(),
        ));
        // D s_225_8: cast reint s_225_7 -> u8
        let s_225_8: bool = ((s_225_7.value()) != 0);
        // C s_225_9: const #0s : i
        let s_225_9: i128 = 0;
        // C s_225_10: const #8s : i
        let s_225_10: i128 = 8;
        // D s_225_11: read-var u#38374:u16
        let s_225_11: u16 = fn_state.u_38374;
        // D s_225_12: cast zx s_225_11 -> bv
        let s_225_12: Bits = Bits::new(s_225_11 as u128, 16u16);
        // D s_225_13: bit-extract s_225_12 s_225_9 s_225_10
        let s_225_13: Bits = (Bits::new(
            ((s_225_12) >> (s_225_9)).value(),
            u16::try_from(s_225_10).unwrap(),
        ));
        // D s_225_14: cast reint s_225_13 -> u8
        let s_225_14: u8 = (s_225_13.value() as u8);
        // D s_225_15: call decode_aarch32_instrs_POP_T1enc_A_txt(s_225_8, s_225_14)
        let s_225_15: () = decode_aarch32_instrs_POP_T1enc_A_txt(
            state,
            tracer,
            s_225_8,
            s_225_14,
        );
        // N s_225_16: return
        return;
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_226_0: read-var merge#var.1:struct
        let s_226_0: u16 = fn_state.merge_var._1;
        // D s_226_1: write-var u#38377 <= s_226_0
        fn_state.u_38377 = s_226_0;
        // C s_226_2: const #9s : i
        let s_226_2: i128 = 9;
        // D s_226_3: read-var u#38377:u16
        let s_226_3: u16 = fn_state.u_38377;
        // D s_226_4: cast zx s_226_3 -> bv
        let s_226_4: Bits = Bits::new(s_226_3 as u128, 16u16);
        // C s_226_5: const #1s : i64
        let s_226_5: i64 = 1;
        // C s_226_6: cast zx s_226_5 -> i
        let s_226_6: i128 = (i128::try_from(s_226_5).unwrap());
        // C s_226_7: const #6s : i
        let s_226_7: i128 = 6;
        // C s_226_8: add s_226_7 s_226_6
        let s_226_8: i128 = (s_226_7 + s_226_6);
        // D s_226_9: bit-extract s_226_4 s_226_2 s_226_8
        let s_226_9: Bits = (Bits::new(
            ((s_226_4) >> (s_226_2)).value(),
            u16::try_from(s_226_8).unwrap(),
        ));
        // D s_226_10: cast reint s_226_9 -> u8
        let s_226_10: u8 = (s_226_9.value() as u8);
        // D s_226_11: cast zx s_226_10 -> bv
        let s_226_11: Bits = Bits::new(s_226_10 as u128, 7u16);
        // C s_226_12: const #90u : u8
        let s_226_12: u8 = 90;
        // C s_226_13: cast zx s_226_12 -> bv
        let s_226_13: Bits = Bits::new(s_226_12 as u128, 7u16);
        // D s_226_14: cmp-eq s_226_11 s_226_13
        let s_226_14: bool = ((s_226_11) == (s_226_13));
        // N s_226_15: branch s_226_14 b429 b227
        if s_226_14 {
            return block_429(state, tracer, fn_state);
        } else {
            return block_227(state, tracer, fn_state);
        };
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_227_0: const #0u : u8
        let s_227_0: bool = false;
        // D s_227_1: write-var gs#428504 <= s_227_0
        fn_state.gs_428504 = s_227_0;
        // N s_227_2: jump b228
        return block_228(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_228_0: read-var gs#428504:u8
        let s_228_0: bool = fn_state.gs_428504;
        // D s_228_1: not s_228_0
        let s_228_1: bool = !s_228_0;
        // N s_228_2: branch s_228_1 b230 b229
        if s_228_1 {
            return block_230(state, tracer, fn_state);
        } else {
            return block_229(state, tracer, fn_state);
        };
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #3063s : i
        let s_229_0: i128 = 3063;
        // C s_229_1: const #14696u : u32
        let s_229_1: u32 = 14696;
        // N s_229_2: write-reg s_229_1 <= s_229_0
        let s_229_2: () = {
            state.write_register::<i128>(s_229_1 as isize, s_229_0);
            tracer.write_register(s_229_1 as isize, s_229_0);
        };
        // C s_229_3: const #8s : i
        let s_229_3: i128 = 8;
        // C s_229_4: const #1s : i
        let s_229_4: i128 = 1;
        // D s_229_5: read-var u#38377:u16
        let s_229_5: u16 = fn_state.u_38377;
        // D s_229_6: cast zx s_229_5 -> bv
        let s_229_6: Bits = Bits::new(s_229_5 as u128, 16u16);
        // D s_229_7: bit-extract s_229_6 s_229_3 s_229_4
        let s_229_7: Bits = (Bits::new(
            ((s_229_6) >> (s_229_3)).value(),
            u16::try_from(s_229_4).unwrap(),
        ));
        // D s_229_8: cast reint s_229_7 -> u8
        let s_229_8: bool = ((s_229_7.value()) != 0);
        // C s_229_9: const #0s : i
        let s_229_9: i128 = 0;
        // C s_229_10: const #8s : i
        let s_229_10: i128 = 8;
        // D s_229_11: read-var u#38377:u16
        let s_229_11: u16 = fn_state.u_38377;
        // D s_229_12: cast zx s_229_11 -> bv
        let s_229_12: Bits = Bits::new(s_229_11 as u128, 16u16);
        // D s_229_13: bit-extract s_229_12 s_229_9 s_229_10
        let s_229_13: Bits = (Bits::new(
            ((s_229_12) >> (s_229_9)).value(),
            u16::try_from(s_229_10).unwrap(),
        ));
        // D s_229_14: cast reint s_229_13 -> u8
        let s_229_14: u8 = (s_229_13.value() as u8);
        // D s_229_15: call decode_aarch32_instrs_PUSH_T1enc_A_txt(s_229_8, s_229_14)
        let s_229_15: () = decode_aarch32_instrs_PUSH_T1enc_A_txt(
            state,
            tracer,
            s_229_8,
            s_229_14,
        );
        // N s_229_16: return
        return;
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_230_0: read-var merge#var.1:struct
        let s_230_0: u16 = fn_state.merge_var._1;
        // D s_230_1: write-var u#38380 <= s_230_0
        fn_state.u_38380 = s_230_0;
        // C s_230_2: const #6s : i
        let s_230_2: i128 = 6;
        // D s_230_3: read-var u#38380:u16
        let s_230_3: u16 = fn_state.u_38380;
        // D s_230_4: cast zx s_230_3 -> bv
        let s_230_4: Bits = Bits::new(s_230_3 as u128, 16u16);
        // C s_230_5: const #1s : i64
        let s_230_5: i64 = 1;
        // C s_230_6: cast zx s_230_5 -> i
        let s_230_6: i128 = (i128::try_from(s_230_5).unwrap());
        // C s_230_7: const #9s : i
        let s_230_7: i128 = 9;
        // C s_230_8: add s_230_7 s_230_6
        let s_230_8: i128 = (s_230_7 + s_230_6);
        // D s_230_9: bit-extract s_230_4 s_230_2 s_230_8
        let s_230_9: Bits = (Bits::new(
            ((s_230_4) >> (s_230_2)).value(),
            u16::try_from(s_230_8).unwrap(),
        ));
        // D s_230_10: cast reint s_230_9 -> u10
        let s_230_10: u16 = (s_230_9.value() as u16);
        // D s_230_11: cast zx s_230_10 -> bv
        let s_230_11: Bits = Bits::new(s_230_10 as u128, 10u16);
        // C s_230_12: const #745u : u10
        let s_230_12: u16 = 745;
        // C s_230_13: cast zx s_230_12 -> bv
        let s_230_13: Bits = Bits::new(s_230_12 as u128, 10u16);
        // D s_230_14: cmp-eq s_230_11 s_230_13
        let s_230_14: bool = ((s_230_11) == (s_230_13));
        // N s_230_15: branch s_230_14 b428 b231
        if s_230_14 {
            return block_428(state, tracer, fn_state);
        } else {
            return block_231(state, tracer, fn_state);
        };
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_231_0: const #0u : u8
        let s_231_0: bool = false;
        // D s_231_1: write-var gs#428515 <= s_231_0
        fn_state.gs_428515 = s_231_0;
        // N s_231_2: jump b232
        return block_232(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_232_0: read-var gs#428515:u8
        let s_232_0: bool = fn_state.gs_428515;
        // D s_232_1: not s_232_0
        let s_232_1: bool = !s_232_0;
        // N s_232_2: branch s_232_1 b234 b233
        if s_232_1 {
            return block_234(state, tracer, fn_state);
        } else {
            return block_233(state, tracer, fn_state);
        };
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_233_0: const #3087s : i
        let s_233_0: i128 = 3087;
        // C s_233_1: const #14696u : u32
        let s_233_1: u32 = 14696;
        // N s_233_2: write-reg s_233_1 <= s_233_0
        let s_233_2: () = {
            state.write_register::<i128>(s_233_1 as isize, s_233_0);
            tracer.write_register(s_233_1 as isize, s_233_0);
        };
        // C s_233_3: const #3s : i
        let s_233_3: i128 = 3;
        // C s_233_4: const #3s : i
        let s_233_4: i128 = 3;
        // D s_233_5: read-var u#38380:u16
        let s_233_5: u16 = fn_state.u_38380;
        // D s_233_6: cast zx s_233_5 -> bv
        let s_233_6: Bits = Bits::new(s_233_5 as u128, 16u16);
        // D s_233_7: bit-extract s_233_6 s_233_3 s_233_4
        let s_233_7: Bits = (Bits::new(
            ((s_233_6) >> (s_233_3)).value(),
            u16::try_from(s_233_4).unwrap(),
        ));
        // D s_233_8: cast reint s_233_7 -> u8
        let s_233_8: u8 = (s_233_7.value() as u8);
        // C s_233_9: const #0s : i
        let s_233_9: i128 = 0;
        // C s_233_10: const #3s : i
        let s_233_10: i128 = 3;
        // D s_233_11: read-var u#38380:u16
        let s_233_11: u16 = fn_state.u_38380;
        // D s_233_12: cast zx s_233_11 -> bv
        let s_233_12: Bits = Bits::new(s_233_11 as u128, 16u16);
        // D s_233_13: bit-extract s_233_12 s_233_9 s_233_10
        let s_233_13: Bits = (Bits::new(
            ((s_233_12) >> (s_233_9)).value(),
            u16::try_from(s_233_10).unwrap(),
        ));
        // D s_233_14: cast reint s_233_13 -> u8
        let s_233_14: u8 = (s_233_13.value() as u8);
        // D s_233_15: call decode_aarch32_instrs_REV16_T1enc_A_txt(s_233_8, s_233_14)
        let s_233_15: () = decode_aarch32_instrs_REV16_T1enc_A_txt(
            state,
            tracer,
            s_233_8,
            s_233_14,
        );
        // N s_233_16: return
        return;
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_234_0: read-var merge#var.1:struct
        let s_234_0: u16 = fn_state.merge_var._1;
        // D s_234_1: write-var u#38384 <= s_234_0
        fn_state.u_38384 = s_234_0;
        // C s_234_2: const #6s : i
        let s_234_2: i128 = 6;
        // D s_234_3: read-var u#38384:u16
        let s_234_3: u16 = fn_state.u_38384;
        // D s_234_4: cast zx s_234_3 -> bv
        let s_234_4: Bits = Bits::new(s_234_3 as u128, 16u16);
        // C s_234_5: const #1s : i64
        let s_234_5: i64 = 1;
        // C s_234_6: cast zx s_234_5 -> i
        let s_234_6: i128 = (i128::try_from(s_234_5).unwrap());
        // C s_234_7: const #9s : i
        let s_234_7: i128 = 9;
        // C s_234_8: add s_234_7 s_234_6
        let s_234_8: i128 = (s_234_7 + s_234_6);
        // D s_234_9: bit-extract s_234_4 s_234_2 s_234_8
        let s_234_9: Bits = (Bits::new(
            ((s_234_4) >> (s_234_2)).value(),
            u16::try_from(s_234_8).unwrap(),
        ));
        // D s_234_10: cast reint s_234_9 -> u10
        let s_234_10: u16 = (s_234_9.value() as u16);
        // D s_234_11: cast zx s_234_10 -> bv
        let s_234_11: Bits = Bits::new(s_234_10 as u128, 10u16);
        // C s_234_12: const #744u : u10
        let s_234_12: u16 = 744;
        // C s_234_13: cast zx s_234_12 -> bv
        let s_234_13: Bits = Bits::new(s_234_12 as u128, 10u16);
        // D s_234_14: cmp-eq s_234_11 s_234_13
        let s_234_14: bool = ((s_234_11) == (s_234_13));
        // N s_234_15: branch s_234_14 b427 b235
        if s_234_14 {
            return block_427(state, tracer, fn_state);
        } else {
            return block_235(state, tracer, fn_state);
        };
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_235_0: const #0u : u8
        let s_235_0: bool = false;
        // D s_235_1: write-var gs#428526 <= s_235_0
        fn_state.gs_428526 = s_235_0;
        // N s_235_2: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_236_0: read-var gs#428526:u8
        let s_236_0: bool = fn_state.gs_428526;
        // D s_236_1: not s_236_0
        let s_236_1: bool = !s_236_0;
        // N s_236_2: branch s_236_1 b238 b237
        if s_236_1 {
            return block_238(state, tracer, fn_state);
        } else {
            return block_237(state, tracer, fn_state);
        };
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_237_0: const #3090s : i
        let s_237_0: i128 = 3090;
        // C s_237_1: const #14696u : u32
        let s_237_1: u32 = 14696;
        // N s_237_2: write-reg s_237_1 <= s_237_0
        let s_237_2: () = {
            state.write_register::<i128>(s_237_1 as isize, s_237_0);
            tracer.write_register(s_237_1 as isize, s_237_0);
        };
        // C s_237_3: const #3s : i
        let s_237_3: i128 = 3;
        // C s_237_4: const #3s : i
        let s_237_4: i128 = 3;
        // D s_237_5: read-var u#38384:u16
        let s_237_5: u16 = fn_state.u_38384;
        // D s_237_6: cast zx s_237_5 -> bv
        let s_237_6: Bits = Bits::new(s_237_5 as u128, 16u16);
        // D s_237_7: bit-extract s_237_6 s_237_3 s_237_4
        let s_237_7: Bits = (Bits::new(
            ((s_237_6) >> (s_237_3)).value(),
            u16::try_from(s_237_4).unwrap(),
        ));
        // D s_237_8: cast reint s_237_7 -> u8
        let s_237_8: u8 = (s_237_7.value() as u8);
        // C s_237_9: const #0s : i
        let s_237_9: i128 = 0;
        // C s_237_10: const #3s : i
        let s_237_10: i128 = 3;
        // D s_237_11: read-var u#38384:u16
        let s_237_11: u16 = fn_state.u_38384;
        // D s_237_12: cast zx s_237_11 -> bv
        let s_237_12: Bits = Bits::new(s_237_11 as u128, 16u16);
        // D s_237_13: bit-extract s_237_12 s_237_9 s_237_10
        let s_237_13: Bits = (Bits::new(
            ((s_237_12) >> (s_237_9)).value(),
            u16::try_from(s_237_10).unwrap(),
        ));
        // D s_237_14: cast reint s_237_13 -> u8
        let s_237_14: u8 = (s_237_13.value() as u8);
        // D s_237_15: call decode_aarch32_instrs_REV_T1enc_A_txt(s_237_8, s_237_14)
        let s_237_15: () = decode_aarch32_instrs_REV_T1enc_A_txt(
            state,
            tracer,
            s_237_8,
            s_237_14,
        );
        // N s_237_16: return
        return;
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_238_0: read-var merge#var.1:struct
        let s_238_0: u16 = fn_state.merge_var._1;
        // D s_238_1: write-var u#38388 <= s_238_0
        fn_state.u_38388 = s_238_0;
        // C s_238_2: const #6s : i
        let s_238_2: i128 = 6;
        // D s_238_3: read-var u#38388:u16
        let s_238_3: u16 = fn_state.u_38388;
        // D s_238_4: cast zx s_238_3 -> bv
        let s_238_4: Bits = Bits::new(s_238_3 as u128, 16u16);
        // C s_238_5: const #1s : i64
        let s_238_5: i64 = 1;
        // C s_238_6: cast zx s_238_5 -> i
        let s_238_6: i128 = (i128::try_from(s_238_5).unwrap());
        // C s_238_7: const #9s : i
        let s_238_7: i128 = 9;
        // C s_238_8: add s_238_7 s_238_6
        let s_238_8: i128 = (s_238_7 + s_238_6);
        // D s_238_9: bit-extract s_238_4 s_238_2 s_238_8
        let s_238_9: Bits = (Bits::new(
            ((s_238_4) >> (s_238_2)).value(),
            u16::try_from(s_238_8).unwrap(),
        ));
        // D s_238_10: cast reint s_238_9 -> u10
        let s_238_10: u16 = (s_238_9.value() as u16);
        // D s_238_11: cast zx s_238_10 -> bv
        let s_238_11: Bits = Bits::new(s_238_10 as u128, 10u16);
        // C s_238_12: const #747u : u10
        let s_238_12: u16 = 747;
        // C s_238_13: cast zx s_238_12 -> bv
        let s_238_13: Bits = Bits::new(s_238_12 as u128, 10u16);
        // D s_238_14: cmp-eq s_238_11 s_238_13
        let s_238_14: bool = ((s_238_11) == (s_238_13));
        // N s_238_15: branch s_238_14 b426 b239
        if s_238_14 {
            return block_426(state, tracer, fn_state);
        } else {
            return block_239(state, tracer, fn_state);
        };
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_239_0: const #0u : u8
        let s_239_0: bool = false;
        // D s_239_1: write-var gs#428537 <= s_239_0
        fn_state.gs_428537 = s_239_0;
        // N s_239_2: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_240_0: read-var gs#428537:u8
        let s_240_0: bool = fn_state.gs_428537;
        // D s_240_1: not s_240_0
        let s_240_1: bool = !s_240_0;
        // N s_240_2: branch s_240_1 b242 b241
        if s_240_1 {
            return block_242(state, tracer, fn_state);
        } else {
            return block_241(state, tracer, fn_state);
        };
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_241_0: const #3093s : i
        let s_241_0: i128 = 3093;
        // C s_241_1: const #14696u : u32
        let s_241_1: u32 = 14696;
        // N s_241_2: write-reg s_241_1 <= s_241_0
        let s_241_2: () = {
            state.write_register::<i128>(s_241_1 as isize, s_241_0);
            tracer.write_register(s_241_1 as isize, s_241_0);
        };
        // C s_241_3: const #3s : i
        let s_241_3: i128 = 3;
        // C s_241_4: const #3s : i
        let s_241_4: i128 = 3;
        // D s_241_5: read-var u#38388:u16
        let s_241_5: u16 = fn_state.u_38388;
        // D s_241_6: cast zx s_241_5 -> bv
        let s_241_6: Bits = Bits::new(s_241_5 as u128, 16u16);
        // D s_241_7: bit-extract s_241_6 s_241_3 s_241_4
        let s_241_7: Bits = (Bits::new(
            ((s_241_6) >> (s_241_3)).value(),
            u16::try_from(s_241_4).unwrap(),
        ));
        // D s_241_8: cast reint s_241_7 -> u8
        let s_241_8: u8 = (s_241_7.value() as u8);
        // C s_241_9: const #0s : i
        let s_241_9: i128 = 0;
        // C s_241_10: const #3s : i
        let s_241_10: i128 = 3;
        // D s_241_11: read-var u#38388:u16
        let s_241_11: u16 = fn_state.u_38388;
        // D s_241_12: cast zx s_241_11 -> bv
        let s_241_12: Bits = Bits::new(s_241_11 as u128, 16u16);
        // D s_241_13: bit-extract s_241_12 s_241_9 s_241_10
        let s_241_13: Bits = (Bits::new(
            ((s_241_12) >> (s_241_9)).value(),
            u16::try_from(s_241_10).unwrap(),
        ));
        // D s_241_14: cast reint s_241_13 -> u8
        let s_241_14: u8 = (s_241_13.value() as u8);
        // D s_241_15: call decode_aarch32_instrs_REVSH_T1enc_A_txt(s_241_8, s_241_14)
        let s_241_15: () = decode_aarch32_instrs_REVSH_T1enc_A_txt(
            state,
            tracer,
            s_241_8,
            s_241_14,
        );
        // N s_241_16: return
        return;
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_242_0: read-var merge#var.1:struct
        let s_242_0: u16 = fn_state.merge_var._1;
        // D s_242_1: write-var u#38392 <= s_242_0
        fn_state.u_38392 = s_242_0;
        // C s_242_2: const #6s : i
        let s_242_2: i128 = 6;
        // D s_242_3: read-var u#38392:u16
        let s_242_3: u16 = fn_state.u_38392;
        // D s_242_4: cast zx s_242_3 -> bv
        let s_242_4: Bits = Bits::new(s_242_3 as u128, 16u16);
        // C s_242_5: const #1s : i64
        let s_242_5: i64 = 1;
        // C s_242_6: cast zx s_242_5 -> i
        let s_242_6: i128 = (i128::try_from(s_242_5).unwrap());
        // C s_242_7: const #9s : i
        let s_242_7: i128 = 9;
        // C s_242_8: add s_242_7 s_242_6
        let s_242_8: i128 = (s_242_7 + s_242_6);
        // D s_242_9: bit-extract s_242_4 s_242_2 s_242_8
        let s_242_9: Bits = (Bits::new(
            ((s_242_4) >> (s_242_2)).value(),
            u16::try_from(s_242_8).unwrap(),
        ));
        // D s_242_10: cast reint s_242_9 -> u10
        let s_242_10: u16 = (s_242_9.value() as u16);
        // D s_242_11: cast zx s_242_10 -> bv
        let s_242_11: Bits = Bits::new(s_242_10 as u128, 10u16);
        // C s_242_12: const #263u : u10
        let s_242_12: u16 = 263;
        // C s_242_13: cast zx s_242_12 -> bv
        let s_242_13: Bits = Bits::new(s_242_12 as u128, 10u16);
        // D s_242_14: cmp-eq s_242_11 s_242_13
        let s_242_14: bool = ((s_242_11) == (s_242_13));
        // N s_242_15: branch s_242_14 b425 b243
        if s_242_14 {
            return block_425(state, tracer, fn_state);
        } else {
            return block_243(state, tracer, fn_state);
        };
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_243_0: const #0u : u8
        let s_243_0: bool = false;
        // D s_243_1: write-var gs#428548 <= s_243_0
        fn_state.gs_428548 = s_243_0;
        // N s_243_2: jump b244
        return block_244(state, tracer, fn_state);
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_244_0: read-var gs#428548:u8
        let s_244_0: bool = fn_state.gs_428548;
        // D s_244_1: not s_244_0
        let s_244_1: bool = !s_244_0;
        // N s_244_2: branch s_244_1 b246 b245
        if s_244_1 {
            return block_246(state, tracer, fn_state);
        } else {
            return block_245(state, tracer, fn_state);
        };
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_245_0: const #3095s : i
        let s_245_0: i128 = 3095;
        // C s_245_1: const #14696u : u32
        let s_245_1: u32 = 14696;
        // N s_245_2: write-reg s_245_1 <= s_245_0
        let s_245_2: () = {
            state.write_register::<i128>(s_245_1 as isize, s_245_0);
            tracer.write_register(s_245_1 as isize, s_245_0);
        };
        // C s_245_3: const #3s : i
        let s_245_3: i128 = 3;
        // C s_245_4: const #3s : i
        let s_245_4: i128 = 3;
        // D s_245_5: read-var u#38392:u16
        let s_245_5: u16 = fn_state.u_38392;
        // D s_245_6: cast zx s_245_5 -> bv
        let s_245_6: Bits = Bits::new(s_245_5 as u128, 16u16);
        // D s_245_7: bit-extract s_245_6 s_245_3 s_245_4
        let s_245_7: Bits = (Bits::new(
            ((s_245_6) >> (s_245_3)).value(),
            u16::try_from(s_245_4).unwrap(),
        ));
        // D s_245_8: cast reint s_245_7 -> u8
        let s_245_8: u8 = (s_245_7.value() as u8);
        // C s_245_9: const #0s : i
        let s_245_9: i128 = 0;
        // C s_245_10: const #3s : i
        let s_245_10: i128 = 3;
        // D s_245_11: read-var u#38392:u16
        let s_245_11: u16 = fn_state.u_38392;
        // D s_245_12: cast zx s_245_11 -> bv
        let s_245_12: Bits = Bits::new(s_245_11 as u128, 16u16);
        // D s_245_13: bit-extract s_245_12 s_245_9 s_245_10
        let s_245_13: Bits = (Bits::new(
            ((s_245_12) >> (s_245_9)).value(),
            u16::try_from(s_245_10).unwrap(),
        ));
        // D s_245_14: cast reint s_245_13 -> u8
        let s_245_14: u8 = (s_245_13.value() as u8);
        // D s_245_15: call decode_aarch32_instrs_ROR_r_T1enc_A_txt(s_245_8, s_245_14)
        let s_245_15: () = decode_aarch32_instrs_ROR_r_T1enc_A_txt(
            state,
            tracer,
            s_245_8,
            s_245_14,
        );
        // N s_245_16: return
        return;
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_246_0: read-var merge#var.1:struct
        let s_246_0: u16 = fn_state.merge_var._1;
        // D s_246_1: write-var u#38396 <= s_246_0
        fn_state.u_38396 = s_246_0;
        // C s_246_2: const #6s : i
        let s_246_2: i128 = 6;
        // D s_246_3: read-var u#38396:u16
        let s_246_3: u16 = fn_state.u_38396;
        // D s_246_4: cast zx s_246_3 -> bv
        let s_246_4: Bits = Bits::new(s_246_3 as u128, 16u16);
        // C s_246_5: const #1s : i64
        let s_246_5: i64 = 1;
        // C s_246_6: cast zx s_246_5 -> i
        let s_246_6: i128 = (i128::try_from(s_246_5).unwrap());
        // C s_246_7: const #9s : i
        let s_246_7: i128 = 9;
        // C s_246_8: add s_246_7 s_246_6
        let s_246_8: i128 = (s_246_7 + s_246_6);
        // D s_246_9: bit-extract s_246_4 s_246_2 s_246_8
        let s_246_9: Bits = (Bits::new(
            ((s_246_4) >> (s_246_2)).value(),
            u16::try_from(s_246_8).unwrap(),
        ));
        // D s_246_10: cast reint s_246_9 -> u10
        let s_246_10: u16 = (s_246_9.value() as u16);
        // D s_246_11: cast zx s_246_10 -> bv
        let s_246_11: Bits = Bits::new(s_246_10 as u128, 10u16);
        // C s_246_12: const #265u : u10
        let s_246_12: u16 = 265;
        // C s_246_13: cast zx s_246_12 -> bv
        let s_246_13: Bits = Bits::new(s_246_12 as u128, 10u16);
        // D s_246_14: cmp-eq s_246_11 s_246_13
        let s_246_14: bool = ((s_246_11) == (s_246_13));
        // N s_246_15: branch s_246_14 b424 b247
        if s_246_14 {
            return block_424(state, tracer, fn_state);
        } else {
            return block_247(state, tracer, fn_state);
        };
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_247_0: const #0u : u8
        let s_247_0: bool = false;
        // D s_247_1: write-var gs#428559 <= s_247_0
        fn_state.gs_428559 = s_247_0;
        // N s_247_2: jump b248
        return block_248(state, tracer, fn_state);
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_248_0: read-var gs#428559:u8
        let s_248_0: bool = fn_state.gs_428559;
        // D s_248_1: not s_248_0
        let s_248_1: bool = !s_248_0;
        // N s_248_2: branch s_248_1 b250 b249
        if s_248_1 {
            return block_250(state, tracer, fn_state);
        } else {
            return block_249(state, tracer, fn_state);
        };
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_249_0: const #3097s : i
        let s_249_0: i128 = 3097;
        // C s_249_1: const #14696u : u32
        let s_249_1: u32 = 14696;
        // N s_249_2: write-reg s_249_1 <= s_249_0
        let s_249_2: () = {
            state.write_register::<i128>(s_249_1 as isize, s_249_0);
            tracer.write_register(s_249_1 as isize, s_249_0);
        };
        // C s_249_3: const #3s : i
        let s_249_3: i128 = 3;
        // C s_249_4: const #3s : i
        let s_249_4: i128 = 3;
        // D s_249_5: read-var u#38396:u16
        let s_249_5: u16 = fn_state.u_38396;
        // D s_249_6: cast zx s_249_5 -> bv
        let s_249_6: Bits = Bits::new(s_249_5 as u128, 16u16);
        // D s_249_7: bit-extract s_249_6 s_249_3 s_249_4
        let s_249_7: Bits = (Bits::new(
            ((s_249_6) >> (s_249_3)).value(),
            u16::try_from(s_249_4).unwrap(),
        ));
        // D s_249_8: cast reint s_249_7 -> u8
        let s_249_8: u8 = (s_249_7.value() as u8);
        // C s_249_9: const #0s : i
        let s_249_9: i128 = 0;
        // C s_249_10: const #3s : i
        let s_249_10: i128 = 3;
        // D s_249_11: read-var u#38396:u16
        let s_249_11: u16 = fn_state.u_38396;
        // D s_249_12: cast zx s_249_11 -> bv
        let s_249_12: Bits = Bits::new(s_249_11 as u128, 16u16);
        // D s_249_13: bit-extract s_249_12 s_249_9 s_249_10
        let s_249_13: Bits = (Bits::new(
            ((s_249_12) >> (s_249_9)).value(),
            u16::try_from(s_249_10).unwrap(),
        ));
        // D s_249_14: cast reint s_249_13 -> u8
        let s_249_14: u8 = (s_249_13.value() as u8);
        // D s_249_15: call decode_aarch32_instrs_RSB_i_T1enc_A_txt(s_249_8, s_249_14)
        let s_249_15: () = decode_aarch32_instrs_RSB_i_T1enc_A_txt(
            state,
            tracer,
            s_249_8,
            s_249_14,
        );
        // N s_249_16: return
        return;
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_250_0: read-var merge#var.1:struct
        let s_250_0: u16 = fn_state.merge_var._1;
        // D s_250_1: write-var u#38400 <= s_250_0
        fn_state.u_38400 = s_250_0;
        // C s_250_2: const #6s : i
        let s_250_2: i128 = 6;
        // D s_250_3: read-var u#38400:u16
        let s_250_3: u16 = fn_state.u_38400;
        // D s_250_4: cast zx s_250_3 -> bv
        let s_250_4: Bits = Bits::new(s_250_3 as u128, 16u16);
        // C s_250_5: const #1s : i64
        let s_250_5: i64 = 1;
        // C s_250_6: cast zx s_250_5 -> i
        let s_250_6: i128 = (i128::try_from(s_250_5).unwrap());
        // C s_250_7: const #9s : i
        let s_250_7: i128 = 9;
        // C s_250_8: add s_250_7 s_250_6
        let s_250_8: i128 = (s_250_7 + s_250_6);
        // D s_250_9: bit-extract s_250_4 s_250_2 s_250_8
        let s_250_9: Bits = (Bits::new(
            ((s_250_4) >> (s_250_2)).value(),
            u16::try_from(s_250_8).unwrap(),
        ));
        // D s_250_10: cast reint s_250_9 -> u10
        let s_250_10: u16 = (s_250_9.value() as u16);
        // D s_250_11: cast zx s_250_10 -> bv
        let s_250_11: Bits = Bits::new(s_250_10 as u128, 10u16);
        // C s_250_12: const #262u : u10
        let s_250_12: u16 = 262;
        // C s_250_13: cast zx s_250_12 -> bv
        let s_250_13: Bits = Bits::new(s_250_12 as u128, 10u16);
        // D s_250_14: cmp-eq s_250_11 s_250_13
        let s_250_14: bool = ((s_250_11) == (s_250_13));
        // N s_250_15: branch s_250_14 b423 b251
        if s_250_14 {
            return block_423(state, tracer, fn_state);
        } else {
            return block_251(state, tracer, fn_state);
        };
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_251_0: const #0u : u8
        let s_251_0: bool = false;
        // D s_251_1: write-var gs#428570 <= s_251_0
        fn_state.gs_428570 = s_251_0;
        // N s_251_2: jump b252
        return block_252(state, tracer, fn_state);
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_252_0: read-var gs#428570:u8
        let s_252_0: bool = fn_state.gs_428570;
        // D s_252_1: not s_252_0
        let s_252_1: bool = !s_252_0;
        // N s_252_2: branch s_252_1 b254 b253
        if s_252_1 {
            return block_254(state, tracer, fn_state);
        } else {
            return block_253(state, tracer, fn_state);
        };
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_253_0: const #3116s : i
        let s_253_0: i128 = 3116;
        // C s_253_1: const #14696u : u32
        let s_253_1: u32 = 14696;
        // N s_253_2: write-reg s_253_1 <= s_253_0
        let s_253_2: () = {
            state.write_register::<i128>(s_253_1 as isize, s_253_0);
            tracer.write_register(s_253_1 as isize, s_253_0);
        };
        // C s_253_3: const #3s : i
        let s_253_3: i128 = 3;
        // C s_253_4: const #3s : i
        let s_253_4: i128 = 3;
        // D s_253_5: read-var u#38400:u16
        let s_253_5: u16 = fn_state.u_38400;
        // D s_253_6: cast zx s_253_5 -> bv
        let s_253_6: Bits = Bits::new(s_253_5 as u128, 16u16);
        // D s_253_7: bit-extract s_253_6 s_253_3 s_253_4
        let s_253_7: Bits = (Bits::new(
            ((s_253_6) >> (s_253_3)).value(),
            u16::try_from(s_253_4).unwrap(),
        ));
        // D s_253_8: cast reint s_253_7 -> u8
        let s_253_8: u8 = (s_253_7.value() as u8);
        // C s_253_9: const #0s : i
        let s_253_9: i128 = 0;
        // C s_253_10: const #3s : i
        let s_253_10: i128 = 3;
        // D s_253_11: read-var u#38400:u16
        let s_253_11: u16 = fn_state.u_38400;
        // D s_253_12: cast zx s_253_11 -> bv
        let s_253_12: Bits = Bits::new(s_253_11 as u128, 16u16);
        // D s_253_13: bit-extract s_253_12 s_253_9 s_253_10
        let s_253_13: Bits = (Bits::new(
            ((s_253_12) >> (s_253_9)).value(),
            u16::try_from(s_253_10).unwrap(),
        ));
        // D s_253_14: cast reint s_253_13 -> u8
        let s_253_14: u8 = (s_253_13.value() as u8);
        // D s_253_15: call decode_aarch32_instrs_SBC_r_T1enc_A_txt(s_253_8, s_253_14)
        let s_253_15: () = decode_aarch32_instrs_SBC_r_T1enc_A_txt(
            state,
            tracer,
            s_253_8,
            s_253_14,
        );
        // N s_253_16: return
        return;
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_254_0: read-var merge#var.1:struct
        let s_254_0: u16 = fn_state.merge_var._1;
        // D s_254_1: write-var u#38404 <= s_254_0
        fn_state.u_38404 = s_254_0;
        // C s_254_2: const #4s : i
        let s_254_2: i128 = 4;
        // D s_254_3: read-var u#38404:u16
        let s_254_3: u16 = fn_state.u_38404;
        // D s_254_4: cast zx s_254_3 -> bv
        let s_254_4: Bits = Bits::new(s_254_3 as u128, 16u16);
        // C s_254_5: const #1s : i64
        let s_254_5: i64 = 1;
        // C s_254_6: cast zx s_254_5 -> i
        let s_254_6: i128 = (i128::try_from(s_254_5).unwrap());
        // C s_254_7: const #11s : i
        let s_254_7: i128 = 11;
        // C s_254_8: add s_254_7 s_254_6
        let s_254_8: i128 = (s_254_7 + s_254_6);
        // D s_254_9: bit-extract s_254_4 s_254_2 s_254_8
        let s_254_9: Bits = (Bits::new(
            ((s_254_4) >> (s_254_2)).value(),
            u16::try_from(s_254_8).unwrap(),
        ));
        // D s_254_10: cast reint s_254_9 -> u12
        let s_254_10: u16 = (s_254_9.value() as u16);
        // D s_254_11: cast zx s_254_10 -> bv
        let s_254_11: Bits = Bits::new(s_254_10 as u128, 12u16);
        // C s_254_12: const #2917u : u12
        let s_254_12: u16 = 2917;
        // C s_254_13: cast zx s_254_12 -> bv
        let s_254_13: Bits = Bits::new(s_254_12 as u128, 12u16);
        // D s_254_14: cmp-eq s_254_11 s_254_13
        let s_254_14: bool = ((s_254_11) == (s_254_13));
        // N s_254_15: branch s_254_14 b422 b255
        if s_254_14 {
            return block_422(state, tracer, fn_state);
        } else {
            return block_255(state, tracer, fn_state);
        };
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_255_0: const #0u : u8
        let s_255_0: bool = false;
        // D s_255_1: write-var gs#428582 <= s_255_0
        fn_state.gs_428582 = s_255_0;
        // N s_255_2: jump b256
        return block_256(state, tracer, fn_state);
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_256_0: read-var gs#428582:u8
        let s_256_0: bool = fn_state.gs_428582;
        // N s_256_1: branch s_256_0 b421 b257
        if s_256_0 {
            return block_421(state, tracer, fn_state);
        } else {
            return block_257(state, tracer, fn_state);
        };
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_257_0: const #0u : u8
        let s_257_0: bool = false;
        // D s_257_1: write-var gs#428584 <= s_257_0
        fn_state.gs_428584 = s_257_0;
        // N s_257_2: jump b258
        return block_258(state, tracer, fn_state);
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_258_0: read-var gs#428584:u8
        let s_258_0: bool = fn_state.gs_428584;
        // D s_258_1: not s_258_0
        let s_258_1: bool = !s_258_0;
        // N s_258_2: branch s_258_1 b268 b259
        if s_258_1 {
            return block_268(state, tracer, fn_state);
        } else {
            return block_259(state, tracer, fn_state);
        };
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_259_0: const #3126s : i
        let s_259_0: i128 = 3126;
        // C s_259_1: const #14696u : u32
        let s_259_1: u32 = 14696;
        // N s_259_2: write-reg s_259_1 <= s_259_0
        let s_259_2: () = {
            state.write_register::<i128>(s_259_1 as isize, s_259_0);
            tracer.write_register(s_259_1 as isize, s_259_0);
        };
        // C s_259_3: const #3s : i
        let s_259_3: i128 = 3;
        // C s_259_4: const #1s : i
        let s_259_4: i128 = 1;
        // D s_259_5: read-var u#38404:u16
        let s_259_5: u16 = fn_state.u_38404;
        // D s_259_6: cast zx s_259_5 -> bv
        let s_259_6: Bits = Bits::new(s_259_5 as u128, 16u16);
        // D s_259_7: bit-extract s_259_6 s_259_3 s_259_4
        let s_259_7: Bits = (Bits::new(
            ((s_259_6) >> (s_259_3)).value(),
            u16::try_from(s_259_4).unwrap(),
        ));
        // D s_259_8: cast reint s_259_7 -> u8
        let s_259_8: bool = ((s_259_7.value()) != 0);
        // D s_259_9: write-var E <= s_259_8
        fn_state.E = s_259_8;
        // C s_259_10: const #0s : i
        let s_259_10: i128 = 0;
        // D s_259_11: read-var u#38404:u16
        let s_259_11: u16 = fn_state.u_38404;
        // D s_259_12: cast zx s_259_11 -> bv
        let s_259_12: Bits = Bits::new(s_259_11 as u128, 16u16);
        // C s_259_13: const #1u : u64
        let s_259_13: u64 = 1;
        // D s_259_14: bit-extract s_259_12 s_259_10 s_259_13
        let s_259_14: Bits = (Bits::new(
            ((s_259_12) >> (s_259_10)).value(),
            u16::try_from(s_259_13).unwrap(),
        ));
        // D s_259_15: cast reint s_259_14 -> u8
        let s_259_15: bool = ((s_259_14.value()) != 0);
        // C s_259_16: const #0s : i
        let s_259_16: i128 = 0;
        // C s_259_17: const #0u : u64
        let s_259_17: u64 = 0;
        // D s_259_18: cast zx s_259_15 -> u64
        let s_259_18: u64 = (s_259_15 as u64);
        // C s_259_19: const #1u : u64
        let s_259_19: u64 = 1;
        // D s_259_20: and s_259_18 s_259_19
        let s_259_20: u64 = ((s_259_18) & (s_259_19));
        // D s_259_21: cmp-eq s_259_20 s_259_19
        let s_259_21: bool = ((s_259_20) == (s_259_19));
        // D s_259_22: lsl s_259_18 s_259_16
        let s_259_22: u64 = s_259_18 << s_259_16;
        // D s_259_23: or s_259_17 s_259_22
        let s_259_23: u64 = ((s_259_17) | (s_259_22));
        // D s_259_24: cmpl s_259_22
        let s_259_24: u64 = !s_259_22;
        // D s_259_25: and s_259_17 s_259_24
        let s_259_25: u64 = ((s_259_17) & (s_259_24));
        // D s_259_26: select s_259_21 s_259_23 s_259_25
        let s_259_26: u64 = if s_259_21 { s_259_23 } else { s_259_25 };
        // D s_259_27: cast trunc s_259_26 -> u8
        let s_259_27: bool = ((s_259_26) != 0);
        // D s_259_28: cast zx s_259_27 -> bv
        let s_259_28: Bits = Bits::new(s_259_27 as u128, 1u16);
        // C s_259_29: const #0u : u8
        let s_259_29: bool = false;
        // C s_259_30: cast zx s_259_29 -> bv
        let s_259_30: Bits = Bits::new(s_259_29 as u128, 1u16);
        // D s_259_31: cmp-ne s_259_28 s_259_30
        let s_259_31: bool = ((s_259_28) != (s_259_30));
        // N s_259_32: branch s_259_31 b267 b260
        if s_259_31 {
            return block_267(state, tracer, fn_state);
        } else {
            return block_260(state, tracer, fn_state);
        };
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_260_0: const #1s : i
        let s_260_0: i128 = 1;
        // D s_260_1: read-var u#38404:u16
        let s_260_1: u16 = fn_state.u_38404;
        // D s_260_2: cast zx s_260_1 -> bv
        let s_260_2: Bits = Bits::new(s_260_1 as u128, 16u16);
        // C s_260_3: const #1u : u64
        let s_260_3: u64 = 1;
        // D s_260_4: bit-extract s_260_2 s_260_0 s_260_3
        let s_260_4: Bits = (Bits::new(
            ((s_260_2) >> (s_260_0)).value(),
            u16::try_from(s_260_3).unwrap(),
        ));
        // D s_260_5: cast reint s_260_4 -> u8
        let s_260_5: bool = ((s_260_4.value()) != 0);
        // C s_260_6: const #0s : i
        let s_260_6: i128 = 0;
        // C s_260_7: const #0u : u64
        let s_260_7: u64 = 0;
        // D s_260_8: cast zx s_260_5 -> u64
        let s_260_8: u64 = (s_260_5 as u64);
        // C s_260_9: const #1u : u64
        let s_260_9: u64 = 1;
        // D s_260_10: and s_260_8 s_260_9
        let s_260_10: u64 = ((s_260_8) & (s_260_9));
        // D s_260_11: cmp-eq s_260_10 s_260_9
        let s_260_11: bool = ((s_260_10) == (s_260_9));
        // D s_260_12: lsl s_260_8 s_260_6
        let s_260_12: u64 = s_260_8 << s_260_6;
        // D s_260_13: or s_260_7 s_260_12
        let s_260_13: u64 = ((s_260_7) | (s_260_12));
        // D s_260_14: cmpl s_260_12
        let s_260_14: u64 = !s_260_12;
        // D s_260_15: and s_260_7 s_260_14
        let s_260_15: u64 = ((s_260_7) & (s_260_14));
        // D s_260_16: select s_260_11 s_260_13 s_260_15
        let s_260_16: u64 = if s_260_11 { s_260_13 } else { s_260_15 };
        // D s_260_17: cast trunc s_260_16 -> u8
        let s_260_17: bool = ((s_260_16) != 0);
        // D s_260_18: cast zx s_260_17 -> bv
        let s_260_18: Bits = Bits::new(s_260_17 as u128, 1u16);
        // C s_260_19: const #0u : u8
        let s_260_19: bool = false;
        // C s_260_20: cast zx s_260_19 -> bv
        let s_260_20: Bits = Bits::new(s_260_19 as u128, 1u16);
        // D s_260_21: cmp-ne s_260_18 s_260_20
        let s_260_21: bool = ((s_260_18) != (s_260_20));
        // D s_260_22: write-var gs#428593 <= s_260_21
        fn_state.gs_428593 = s_260_21;
        // N s_260_23: jump b261
        return block_261(state, tracer, fn_state);
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_261_0: read-var gs#428593:u8
        let s_261_0: bool = fn_state.gs_428593;
        // N s_261_1: branch s_261_0 b266 b262
        if s_261_0 {
            return block_266(state, tracer, fn_state);
        } else {
            return block_262(state, tracer, fn_state);
        };
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_262_0: const #2s : i
        let s_262_0: i128 = 2;
        // D s_262_1: read-var u#38404:u16
        let s_262_1: u16 = fn_state.u_38404;
        // D s_262_2: cast zx s_262_1 -> bv
        let s_262_2: Bits = Bits::new(s_262_1 as u128, 16u16);
        // C s_262_3: const #1u : u64
        let s_262_3: u64 = 1;
        // D s_262_4: bit-extract s_262_2 s_262_0 s_262_3
        let s_262_4: Bits = (Bits::new(
            ((s_262_2) >> (s_262_0)).value(),
            u16::try_from(s_262_3).unwrap(),
        ));
        // D s_262_5: cast reint s_262_4 -> u8
        let s_262_5: bool = ((s_262_4.value()) != 0);
        // C s_262_6: const #0s : i
        let s_262_6: i128 = 0;
        // C s_262_7: const #0u : u64
        let s_262_7: u64 = 0;
        // D s_262_8: cast zx s_262_5 -> u64
        let s_262_8: u64 = (s_262_5 as u64);
        // C s_262_9: const #1u : u64
        let s_262_9: u64 = 1;
        // D s_262_10: and s_262_8 s_262_9
        let s_262_10: u64 = ((s_262_8) & (s_262_9));
        // D s_262_11: cmp-eq s_262_10 s_262_9
        let s_262_11: bool = ((s_262_10) == (s_262_9));
        // D s_262_12: lsl s_262_8 s_262_6
        let s_262_12: u64 = s_262_8 << s_262_6;
        // D s_262_13: or s_262_7 s_262_12
        let s_262_13: u64 = ((s_262_7) | (s_262_12));
        // D s_262_14: cmpl s_262_12
        let s_262_14: u64 = !s_262_12;
        // D s_262_15: and s_262_7 s_262_14
        let s_262_15: u64 = ((s_262_7) & (s_262_14));
        // D s_262_16: select s_262_11 s_262_13 s_262_15
        let s_262_16: u64 = if s_262_11 { s_262_13 } else { s_262_15 };
        // D s_262_17: cast trunc s_262_16 -> u8
        let s_262_17: bool = ((s_262_16) != 0);
        // D s_262_18: cast zx s_262_17 -> bv
        let s_262_18: Bits = Bits::new(s_262_17 as u128, 1u16);
        // C s_262_19: const #0u : u8
        let s_262_19: bool = false;
        // C s_262_20: cast zx s_262_19 -> bv
        let s_262_20: Bits = Bits::new(s_262_19 as u128, 1u16);
        // D s_262_21: cmp-ne s_262_18 s_262_20
        let s_262_21: bool = ((s_262_18) != (s_262_20));
        // D s_262_22: write-var gs#428596 <= s_262_21
        fn_state.gs_428596 = s_262_21;
        // N s_262_23: jump b263
        return block_263(state, tracer, fn_state);
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_263_0: read-var gs#428596:u8
        let s_263_0: bool = fn_state.gs_428596;
        // N s_263_1: branch s_263_0 b265 b264
        if s_263_0 {
            return block_265(state, tracer, fn_state);
        } else {
            return block_264(state, tracer, fn_state);
        };
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_264_0: read-var E:u8
        let s_264_0: bool = fn_state.E;
        // D s_264_1: call decode_aarch32_instrs_SETEND_T1enc_A_txt(s_264_0)
        let s_264_1: () = decode_aarch32_instrs_SETEND_T1enc_A_txt(
            state,
            tracer,
            s_264_0,
        );
        // N s_264_2: return
        return;
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_265_0: panic
        panic!("{:?}", ());
        // N s_265_1: return
        return;
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_266_0: const #1u : u8
        let s_266_0: bool = true;
        // D s_266_1: write-var gs#428596 <= s_266_0
        fn_state.gs_428596 = s_266_0;
        // N s_266_2: jump b263
        return block_263(state, tracer, fn_state);
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_267_0: const #1u : u8
        let s_267_0: bool = true;
        // D s_267_1: write-var gs#428593 <= s_267_0
        fn_state.gs_428593 = s_267_0;
        // N s_267_2: jump b261
        return block_261(state, tracer, fn_state);
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_268_0: read-var merge#var.1:struct
        let s_268_0: u16 = fn_state.merge_var._1;
        // D s_268_1: cast zx s_268_0 -> bv
        let s_268_1: Bits = Bits::new(s_268_0 as u128, 16u16);
        // C s_268_2: const #48960u : u16
        let s_268_2: u16 = 48960;
        // C s_268_3: cast zx s_268_2 -> bv
        let s_268_3: Bits = Bits::new(s_268_2 as u128, 16u16);
        // D s_268_4: cmp-eq s_268_1 s_268_3
        let s_268_4: bool = ((s_268_1) == (s_268_3));
        // N s_268_5: branch s_268_4 b420 b269
        if s_268_4 {
            return block_420(state, tracer, fn_state);
        } else {
            return block_269(state, tracer, fn_state);
        };
    }
    fn block_269<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_269_0: const #0u : u8
        let s_269_0: bool = false;
        // D s_269_1: write-var gs#428599 <= s_269_0
        fn_state.gs_428599 = s_269_0;
        // N s_269_2: jump b270
        return block_270(state, tracer, fn_state);
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_270_0: read-var gs#428599:u8
        let s_270_0: bool = fn_state.gs_428599;
        // D s_270_1: not s_270_0
        let s_270_1: bool = !s_270_0;
        // N s_270_2: branch s_270_1 b272 b271
        if s_270_1 {
            return block_272(state, tracer, fn_state);
        } else {
            return block_271(state, tracer, fn_state);
        };
    }
    fn block_271<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_271_0: const #3128s : i
        let s_271_0: i128 = 3128;
        // C s_271_1: const #14696u : u32
        let s_271_1: u32 = 14696;
        // N s_271_2: write-reg s_271_1 <= s_271_0
        let s_271_2: () = {
            state.write_register::<i128>(s_271_1 as isize, s_271_0);
            tracer.write_register(s_271_1 as isize, s_271_0);
        };
        // C s_271_3: const #() : ()
        let s_271_3: () = ();
        // S s_271_4: call decode_aarch32_instrs_SEV_T1enc_A_txt(s_271_3)
        let s_271_4: () = decode_aarch32_instrs_SEV_T1enc_A_txt(state, tracer, s_271_3);
        // N s_271_5: return
        return;
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_272_0: read-var merge#var.1:struct
        let s_272_0: u16 = fn_state.merge_var._1;
        // D s_272_1: write-var u#38408 <= s_272_0
        fn_state.u_38408 = s_272_0;
        // C s_272_2: const #11s : i
        let s_272_2: i128 = 11;
        // D s_272_3: read-var u#38408:u16
        let s_272_3: u16 = fn_state.u_38408;
        // D s_272_4: cast zx s_272_3 -> bv
        let s_272_4: Bits = Bits::new(s_272_3 as u128, 16u16);
        // C s_272_5: const #1s : i64
        let s_272_5: i64 = 1;
        // C s_272_6: cast zx s_272_5 -> i
        let s_272_6: i128 = (i128::try_from(s_272_5).unwrap());
        // C s_272_7: const #4s : i
        let s_272_7: i128 = 4;
        // C s_272_8: add s_272_7 s_272_6
        let s_272_8: i128 = (s_272_7 + s_272_6);
        // D s_272_9: bit-extract s_272_4 s_272_2 s_272_8
        let s_272_9: Bits = (Bits::new(
            ((s_272_4) >> (s_272_2)).value(),
            u16::try_from(s_272_8).unwrap(),
        ));
        // D s_272_10: cast reint s_272_9 -> u8
        let s_272_10: u8 = (s_272_9.value() as u8);
        // D s_272_11: cast zx s_272_10 -> bv
        let s_272_11: Bits = Bits::new(s_272_10 as u128, 5u16);
        // C s_272_12: const #24u : u8
        let s_272_12: u8 = 24;
        // C s_272_13: cast zx s_272_12 -> bv
        let s_272_13: Bits = Bits::new(s_272_12 as u128, 5u16);
        // D s_272_14: cmp-eq s_272_11 s_272_13
        let s_272_14: bool = ((s_272_11) == (s_272_13));
        // N s_272_15: branch s_272_14 b419 b273
        if s_272_14 {
            return block_419(state, tracer, fn_state);
        } else {
            return block_273(state, tracer, fn_state);
        };
    }
    fn block_273<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_273_0: const #0u : u8
        let s_273_0: bool = false;
        // D s_273_1: write-var gs#428606 <= s_273_0
        fn_state.gs_428606 = s_273_0;
        // N s_273_2: jump b274
        return block_274(state, tracer, fn_state);
    }
    fn block_274<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_274_0: read-var gs#428606:u8
        let s_274_0: bool = fn_state.gs_428606;
        // D s_274_1: not s_274_0
        let s_274_1: bool = !s_274_0;
        // N s_274_2: branch s_274_1 b276 b275
        if s_274_1 {
            return block_276(state, tracer, fn_state);
        } else {
            return block_275(state, tracer, fn_state);
        };
    }
    fn block_275<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_275_0: const #3187s : i
        let s_275_0: i128 = 3187;
        // C s_275_1: const #14696u : u32
        let s_275_1: u32 = 14696;
        // N s_275_2: write-reg s_275_1 <= s_275_0
        let s_275_2: () = {
            state.write_register::<i128>(s_275_1 as isize, s_275_0);
            tracer.write_register(s_275_1 as isize, s_275_0);
        };
        // C s_275_3: const #8s : i
        let s_275_3: i128 = 8;
        // C s_275_4: const #3s : i
        let s_275_4: i128 = 3;
        // D s_275_5: read-var u#38408:u16
        let s_275_5: u16 = fn_state.u_38408;
        // D s_275_6: cast zx s_275_5 -> bv
        let s_275_6: Bits = Bits::new(s_275_5 as u128, 16u16);
        // D s_275_7: bit-extract s_275_6 s_275_3 s_275_4
        let s_275_7: Bits = (Bits::new(
            ((s_275_6) >> (s_275_3)).value(),
            u16::try_from(s_275_4).unwrap(),
        ));
        // D s_275_8: cast reint s_275_7 -> u8
        let s_275_8: u8 = (s_275_7.value() as u8);
        // C s_275_9: const #0s : i
        let s_275_9: i128 = 0;
        // C s_275_10: const #8s : i
        let s_275_10: i128 = 8;
        // D s_275_11: read-var u#38408:u16
        let s_275_11: u16 = fn_state.u_38408;
        // D s_275_12: cast zx s_275_11 -> bv
        let s_275_12: Bits = Bits::new(s_275_11 as u128, 16u16);
        // D s_275_13: bit-extract s_275_12 s_275_9 s_275_10
        let s_275_13: Bits = (Bits::new(
            ((s_275_12) >> (s_275_9)).value(),
            u16::try_from(s_275_10).unwrap(),
        ));
        // D s_275_14: cast reint s_275_13 -> u8
        let s_275_14: u8 = (s_275_13.value() as u8);
        // D s_275_15: call decode_aarch32_instrs_STM_T1enc_A_txt(s_275_8, s_275_14)
        let s_275_15: () = decode_aarch32_instrs_STM_T1enc_A_txt(
            state,
            tracer,
            s_275_8,
            s_275_14,
        );
        // N s_275_16: return
        return;
    }
    fn block_276<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_276_0: read-var merge#var.1:struct
        let s_276_0: u16 = fn_state.merge_var._1;
        // D s_276_1: write-var u#38412 <= s_276_0
        fn_state.u_38412 = s_276_0;
        // C s_276_2: const #11s : i
        let s_276_2: i128 = 11;
        // D s_276_3: read-var u#38412:u16
        let s_276_3: u16 = fn_state.u_38412;
        // D s_276_4: cast zx s_276_3 -> bv
        let s_276_4: Bits = Bits::new(s_276_3 as u128, 16u16);
        // C s_276_5: const #1s : i64
        let s_276_5: i64 = 1;
        // C s_276_6: cast zx s_276_5 -> i
        let s_276_6: i128 = (i128::try_from(s_276_5).unwrap());
        // C s_276_7: const #4s : i
        let s_276_7: i128 = 4;
        // C s_276_8: add s_276_7 s_276_6
        let s_276_8: i128 = (s_276_7 + s_276_6);
        // D s_276_9: bit-extract s_276_4 s_276_2 s_276_8
        let s_276_9: Bits = (Bits::new(
            ((s_276_4) >> (s_276_2)).value(),
            u16::try_from(s_276_8).unwrap(),
        ));
        // D s_276_10: cast reint s_276_9 -> u8
        let s_276_10: u8 = (s_276_9.value() as u8);
        // D s_276_11: cast zx s_276_10 -> bv
        let s_276_11: Bits = Bits::new(s_276_10 as u128, 5u16);
        // C s_276_12: const #14u : u8
        let s_276_12: u8 = 14;
        // C s_276_13: cast zx s_276_12 -> bv
        let s_276_13: Bits = Bits::new(s_276_12 as u128, 5u16);
        // D s_276_14: cmp-eq s_276_11 s_276_13
        let s_276_14: bool = ((s_276_11) == (s_276_13));
        // N s_276_15: branch s_276_14 b418 b277
        if s_276_14 {
            return block_418(state, tracer, fn_state);
        } else {
            return block_277(state, tracer, fn_state);
        };
    }
    fn block_277<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_277_0: const #0u : u8
        let s_277_0: bool = false;
        // D s_277_1: write-var gs#428617 <= s_277_0
        fn_state.gs_428617 = s_277_0;
        // N s_277_2: jump b278
        return block_278(state, tracer, fn_state);
    }
    fn block_278<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_278_0: read-var gs#428617:u8
        let s_278_0: bool = fn_state.gs_428617;
        // D s_278_1: not s_278_0
        let s_278_1: bool = !s_278_0;
        // N s_278_2: branch s_278_1 b280 b279
        if s_278_1 {
            return block_280(state, tracer, fn_state);
        } else {
            return block_279(state, tracer, fn_state);
        };
    }
    fn block_279<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_279_0: const #3194s : i
        let s_279_0: i128 = 3194;
        // C s_279_1: const #14696u : u32
        let s_279_1: u32 = 14696;
        // N s_279_2: write-reg s_279_1 <= s_279_0
        let s_279_2: () = {
            state.write_register::<i128>(s_279_1 as isize, s_279_0);
            tracer.write_register(s_279_1 as isize, s_279_0);
        };
        // C s_279_3: const #6s : i
        let s_279_3: i128 = 6;
        // C s_279_4: const #5s : i
        let s_279_4: i128 = 5;
        // D s_279_5: read-var u#38412:u16
        let s_279_5: u16 = fn_state.u_38412;
        // D s_279_6: cast zx s_279_5 -> bv
        let s_279_6: Bits = Bits::new(s_279_5 as u128, 16u16);
        // D s_279_7: bit-extract s_279_6 s_279_3 s_279_4
        let s_279_7: Bits = (Bits::new(
            ((s_279_6) >> (s_279_3)).value(),
            u16::try_from(s_279_4).unwrap(),
        ));
        // D s_279_8: cast reint s_279_7 -> u8
        let s_279_8: u8 = (s_279_7.value() as u8);
        // C s_279_9: const #3s : i
        let s_279_9: i128 = 3;
        // C s_279_10: const #3s : i
        let s_279_10: i128 = 3;
        // D s_279_11: read-var u#38412:u16
        let s_279_11: u16 = fn_state.u_38412;
        // D s_279_12: cast zx s_279_11 -> bv
        let s_279_12: Bits = Bits::new(s_279_11 as u128, 16u16);
        // D s_279_13: bit-extract s_279_12 s_279_9 s_279_10
        let s_279_13: Bits = (Bits::new(
            ((s_279_12) >> (s_279_9)).value(),
            u16::try_from(s_279_10).unwrap(),
        ));
        // D s_279_14: cast reint s_279_13 -> u8
        let s_279_14: u8 = (s_279_13.value() as u8);
        // C s_279_15: const #0s : i
        let s_279_15: i128 = 0;
        // C s_279_16: const #3s : i
        let s_279_16: i128 = 3;
        // D s_279_17: read-var u#38412:u16
        let s_279_17: u16 = fn_state.u_38412;
        // D s_279_18: cast zx s_279_17 -> bv
        let s_279_18: Bits = Bits::new(s_279_17 as u128, 16u16);
        // D s_279_19: bit-extract s_279_18 s_279_15 s_279_16
        let s_279_19: Bits = (Bits::new(
            ((s_279_18) >> (s_279_15)).value(),
            u16::try_from(s_279_16).unwrap(),
        ));
        // D s_279_20: cast reint s_279_19 -> u8
        let s_279_20: u8 = (s_279_19.value() as u8);
        // D s_279_21: call decode_aarch32_instrs_STRB_i_T1enc_A_txt(s_279_8, s_279_14, s_279_20)
        let s_279_21: () = decode_aarch32_instrs_STRB_i_T1enc_A_txt(
            state,
            tracer,
            s_279_8,
            s_279_14,
            s_279_20,
        );
        // N s_279_22: return
        return;
    }
    fn block_280<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_280_0: read-var merge#var.1:struct
        let s_280_0: u16 = fn_state.merge_var._1;
        // D s_280_1: write-var u#38417 <= s_280_0
        fn_state.u_38417 = s_280_0;
        // C s_280_2: const #9s : i
        let s_280_2: i128 = 9;
        // D s_280_3: read-var u#38417:u16
        let s_280_3: u16 = fn_state.u_38417;
        // D s_280_4: cast zx s_280_3 -> bv
        let s_280_4: Bits = Bits::new(s_280_3 as u128, 16u16);
        // C s_280_5: const #1s : i64
        let s_280_5: i64 = 1;
        // C s_280_6: cast zx s_280_5 -> i
        let s_280_6: i128 = (i128::try_from(s_280_5).unwrap());
        // C s_280_7: const #6s : i
        let s_280_7: i128 = 6;
        // C s_280_8: add s_280_7 s_280_6
        let s_280_8: i128 = (s_280_7 + s_280_6);
        // D s_280_9: bit-extract s_280_4 s_280_2 s_280_8
        let s_280_9: Bits = (Bits::new(
            ((s_280_4) >> (s_280_2)).value(),
            u16::try_from(s_280_8).unwrap(),
        ));
        // D s_280_10: cast reint s_280_9 -> u8
        let s_280_10: u8 = (s_280_9.value() as u8);
        // D s_280_11: cast zx s_280_10 -> bv
        let s_280_11: Bits = Bits::new(s_280_10 as u128, 7u16);
        // C s_280_12: const #42u : u8
        let s_280_12: u8 = 42;
        // C s_280_13: cast zx s_280_12 -> bv
        let s_280_13: Bits = Bits::new(s_280_12 as u128, 7u16);
        // D s_280_14: cmp-eq s_280_11 s_280_13
        let s_280_14: bool = ((s_280_11) == (s_280_13));
        // N s_280_15: branch s_280_14 b417 b281
        if s_280_14 {
            return block_417(state, tracer, fn_state);
        } else {
            return block_281(state, tracer, fn_state);
        };
    }
    fn block_281<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_281_0: const #0u : u8
        let s_281_0: bool = false;
        // D s_281_1: write-var gs#428630 <= s_281_0
        fn_state.gs_428630 = s_281_0;
        // N s_281_2: jump b282
        return block_282(state, tracer, fn_state);
    }
    fn block_282<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_282_0: read-var gs#428630:u8
        let s_282_0: bool = fn_state.gs_428630;
        // D s_282_1: not s_282_0
        let s_282_1: bool = !s_282_0;
        // N s_282_2: branch s_282_1 b284 b283
        if s_282_1 {
            return block_284(state, tracer, fn_state);
        } else {
            return block_283(state, tracer, fn_state);
        };
    }
    fn block_283<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_283_0: const #3198s : i
        let s_283_0: i128 = 3198;
        // C s_283_1: const #14696u : u32
        let s_283_1: u32 = 14696;
        // N s_283_2: write-reg s_283_1 <= s_283_0
        let s_283_2: () = {
            state.write_register::<i128>(s_283_1 as isize, s_283_0);
            tracer.write_register(s_283_1 as isize, s_283_0);
        };
        // C s_283_3: const #6s : i
        let s_283_3: i128 = 6;
        // C s_283_4: const #3s : i
        let s_283_4: i128 = 3;
        // D s_283_5: read-var u#38417:u16
        let s_283_5: u16 = fn_state.u_38417;
        // D s_283_6: cast zx s_283_5 -> bv
        let s_283_6: Bits = Bits::new(s_283_5 as u128, 16u16);
        // D s_283_7: bit-extract s_283_6 s_283_3 s_283_4
        let s_283_7: Bits = (Bits::new(
            ((s_283_6) >> (s_283_3)).value(),
            u16::try_from(s_283_4).unwrap(),
        ));
        // D s_283_8: cast reint s_283_7 -> u8
        let s_283_8: u8 = (s_283_7.value() as u8);
        // C s_283_9: const #3s : i
        let s_283_9: i128 = 3;
        // C s_283_10: const #3s : i
        let s_283_10: i128 = 3;
        // D s_283_11: read-var u#38417:u16
        let s_283_11: u16 = fn_state.u_38417;
        // D s_283_12: cast zx s_283_11 -> bv
        let s_283_12: Bits = Bits::new(s_283_11 as u128, 16u16);
        // D s_283_13: bit-extract s_283_12 s_283_9 s_283_10
        let s_283_13: Bits = (Bits::new(
            ((s_283_12) >> (s_283_9)).value(),
            u16::try_from(s_283_10).unwrap(),
        ));
        // D s_283_14: cast reint s_283_13 -> u8
        let s_283_14: u8 = (s_283_13.value() as u8);
        // C s_283_15: const #0s : i
        let s_283_15: i128 = 0;
        // C s_283_16: const #3s : i
        let s_283_16: i128 = 3;
        // D s_283_17: read-var u#38417:u16
        let s_283_17: u16 = fn_state.u_38417;
        // D s_283_18: cast zx s_283_17 -> bv
        let s_283_18: Bits = Bits::new(s_283_17 as u128, 16u16);
        // D s_283_19: bit-extract s_283_18 s_283_15 s_283_16
        let s_283_19: Bits = (Bits::new(
            ((s_283_18) >> (s_283_15)).value(),
            u16::try_from(s_283_16).unwrap(),
        ));
        // D s_283_20: cast reint s_283_19 -> u8
        let s_283_20: u8 = (s_283_19.value() as u8);
        // D s_283_21: call decode_aarch32_instrs_STRB_r_T1enc_A_txt(s_283_8, s_283_14, s_283_20)
        let s_283_21: () = decode_aarch32_instrs_STRB_r_T1enc_A_txt(
            state,
            tracer,
            s_283_8,
            s_283_14,
            s_283_20,
        );
        // N s_283_22: return
        return;
    }
    fn block_284<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_284_0: read-var merge#var.1:struct
        let s_284_0: u16 = fn_state.merge_var._1;
        // D s_284_1: write-var u#38422 <= s_284_0
        fn_state.u_38422 = s_284_0;
        // C s_284_2: const #11s : i
        let s_284_2: i128 = 11;
        // D s_284_3: read-var u#38422:u16
        let s_284_3: u16 = fn_state.u_38422;
        // D s_284_4: cast zx s_284_3 -> bv
        let s_284_4: Bits = Bits::new(s_284_3 as u128, 16u16);
        // C s_284_5: const #1s : i64
        let s_284_5: i64 = 1;
        // C s_284_6: cast zx s_284_5 -> i
        let s_284_6: i128 = (i128::try_from(s_284_5).unwrap());
        // C s_284_7: const #4s : i
        let s_284_7: i128 = 4;
        // C s_284_8: add s_284_7 s_284_6
        let s_284_8: i128 = (s_284_7 + s_284_6);
        // D s_284_9: bit-extract s_284_4 s_284_2 s_284_8
        let s_284_9: Bits = (Bits::new(
            ((s_284_4) >> (s_284_2)).value(),
            u16::try_from(s_284_8).unwrap(),
        ));
        // D s_284_10: cast reint s_284_9 -> u8
        let s_284_10: u8 = (s_284_9.value() as u8);
        // D s_284_11: cast zx s_284_10 -> bv
        let s_284_11: Bits = Bits::new(s_284_10 as u128, 5u16);
        // C s_284_12: const #16u : u8
        let s_284_12: u8 = 16;
        // C s_284_13: cast zx s_284_12 -> bv
        let s_284_13: Bits = Bits::new(s_284_12 as u128, 5u16);
        // D s_284_14: cmp-eq s_284_11 s_284_13
        let s_284_14: bool = ((s_284_11) == (s_284_13));
        // N s_284_15: branch s_284_14 b416 b285
        if s_284_14 {
            return block_416(state, tracer, fn_state);
        } else {
            return block_285(state, tracer, fn_state);
        };
    }
    fn block_285<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_285_0: const #0u : u8
        let s_285_0: bool = false;
        // D s_285_1: write-var gs#428643 <= s_285_0
        fn_state.gs_428643 = s_285_0;
        // N s_285_2: jump b286
        return block_286(state, tracer, fn_state);
    }
    fn block_286<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_286_0: read-var gs#428643:u8
        let s_286_0: bool = fn_state.gs_428643;
        // D s_286_1: not s_286_0
        let s_286_1: bool = !s_286_0;
        // N s_286_2: branch s_286_1 b288 b287
        if s_286_1 {
            return block_288(state, tracer, fn_state);
        } else {
            return block_287(state, tracer, fn_state);
        };
    }
    fn block_287<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_287_0: const #3215s : i
        let s_287_0: i128 = 3215;
        // C s_287_1: const #14696u : u32
        let s_287_1: u32 = 14696;
        // N s_287_2: write-reg s_287_1 <= s_287_0
        let s_287_2: () = {
            state.write_register::<i128>(s_287_1 as isize, s_287_0);
            tracer.write_register(s_287_1 as isize, s_287_0);
        };
        // C s_287_3: const #6s : i
        let s_287_3: i128 = 6;
        // C s_287_4: const #5s : i
        let s_287_4: i128 = 5;
        // D s_287_5: read-var u#38422:u16
        let s_287_5: u16 = fn_state.u_38422;
        // D s_287_6: cast zx s_287_5 -> bv
        let s_287_6: Bits = Bits::new(s_287_5 as u128, 16u16);
        // D s_287_7: bit-extract s_287_6 s_287_3 s_287_4
        let s_287_7: Bits = (Bits::new(
            ((s_287_6) >> (s_287_3)).value(),
            u16::try_from(s_287_4).unwrap(),
        ));
        // D s_287_8: cast reint s_287_7 -> u8
        let s_287_8: u8 = (s_287_7.value() as u8);
        // C s_287_9: const #3s : i
        let s_287_9: i128 = 3;
        // C s_287_10: const #3s : i
        let s_287_10: i128 = 3;
        // D s_287_11: read-var u#38422:u16
        let s_287_11: u16 = fn_state.u_38422;
        // D s_287_12: cast zx s_287_11 -> bv
        let s_287_12: Bits = Bits::new(s_287_11 as u128, 16u16);
        // D s_287_13: bit-extract s_287_12 s_287_9 s_287_10
        let s_287_13: Bits = (Bits::new(
            ((s_287_12) >> (s_287_9)).value(),
            u16::try_from(s_287_10).unwrap(),
        ));
        // D s_287_14: cast reint s_287_13 -> u8
        let s_287_14: u8 = (s_287_13.value() as u8);
        // C s_287_15: const #0s : i
        let s_287_15: i128 = 0;
        // C s_287_16: const #3s : i
        let s_287_16: i128 = 3;
        // D s_287_17: read-var u#38422:u16
        let s_287_17: u16 = fn_state.u_38422;
        // D s_287_18: cast zx s_287_17 -> bv
        let s_287_18: Bits = Bits::new(s_287_17 as u128, 16u16);
        // D s_287_19: bit-extract s_287_18 s_287_15 s_287_16
        let s_287_19: Bits = (Bits::new(
            ((s_287_18) >> (s_287_15)).value(),
            u16::try_from(s_287_16).unwrap(),
        ));
        // D s_287_20: cast reint s_287_19 -> u8
        let s_287_20: u8 = (s_287_19.value() as u8);
        // D s_287_21: call decode_aarch32_instrs_STRH_i_T1enc_A_txt(s_287_8, s_287_14, s_287_20)
        let s_287_21: () = decode_aarch32_instrs_STRH_i_T1enc_A_txt(
            state,
            tracer,
            s_287_8,
            s_287_14,
            s_287_20,
        );
        // N s_287_22: return
        return;
    }
    fn block_288<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_288_0: read-var merge#var.1:struct
        let s_288_0: u16 = fn_state.merge_var._1;
        // D s_288_1: write-var u#38427 <= s_288_0
        fn_state.u_38427 = s_288_0;
        // C s_288_2: const #9s : i
        let s_288_2: i128 = 9;
        // D s_288_3: read-var u#38427:u16
        let s_288_3: u16 = fn_state.u_38427;
        // D s_288_4: cast zx s_288_3 -> bv
        let s_288_4: Bits = Bits::new(s_288_3 as u128, 16u16);
        // C s_288_5: const #1s : i64
        let s_288_5: i64 = 1;
        // C s_288_6: cast zx s_288_5 -> i
        let s_288_6: i128 = (i128::try_from(s_288_5).unwrap());
        // C s_288_7: const #6s : i
        let s_288_7: i128 = 6;
        // C s_288_8: add s_288_7 s_288_6
        let s_288_8: i128 = (s_288_7 + s_288_6);
        // D s_288_9: bit-extract s_288_4 s_288_2 s_288_8
        let s_288_9: Bits = (Bits::new(
            ((s_288_4) >> (s_288_2)).value(),
            u16::try_from(s_288_8).unwrap(),
        ));
        // D s_288_10: cast reint s_288_9 -> u8
        let s_288_10: u8 = (s_288_9.value() as u8);
        // D s_288_11: cast zx s_288_10 -> bv
        let s_288_11: Bits = Bits::new(s_288_10 as u128, 7u16);
        // C s_288_12: const #41u : u8
        let s_288_12: u8 = 41;
        // C s_288_13: cast zx s_288_12 -> bv
        let s_288_13: Bits = Bits::new(s_288_12 as u128, 7u16);
        // D s_288_14: cmp-eq s_288_11 s_288_13
        let s_288_14: bool = ((s_288_11) == (s_288_13));
        // N s_288_15: branch s_288_14 b415 b289
        if s_288_14 {
            return block_415(state, tracer, fn_state);
        } else {
            return block_289(state, tracer, fn_state);
        };
    }
    fn block_289<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_289_0: const #0u : u8
        let s_289_0: bool = false;
        // D s_289_1: write-var gs#428656 <= s_289_0
        fn_state.gs_428656 = s_289_0;
        // N s_289_2: jump b290
        return block_290(state, tracer, fn_state);
    }
    fn block_290<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_290_0: read-var gs#428656:u8
        let s_290_0: bool = fn_state.gs_428656;
        // D s_290_1: not s_290_0
        let s_290_1: bool = !s_290_0;
        // N s_290_2: branch s_290_1 b292 b291
        if s_290_1 {
            return block_292(state, tracer, fn_state);
        } else {
            return block_291(state, tracer, fn_state);
        };
    }
    fn block_291<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_291_0: const #3219s : i
        let s_291_0: i128 = 3219;
        // C s_291_1: const #14696u : u32
        let s_291_1: u32 = 14696;
        // N s_291_2: write-reg s_291_1 <= s_291_0
        let s_291_2: () = {
            state.write_register::<i128>(s_291_1 as isize, s_291_0);
            tracer.write_register(s_291_1 as isize, s_291_0);
        };
        // C s_291_3: const #6s : i
        let s_291_3: i128 = 6;
        // C s_291_4: const #3s : i
        let s_291_4: i128 = 3;
        // D s_291_5: read-var u#38427:u16
        let s_291_5: u16 = fn_state.u_38427;
        // D s_291_6: cast zx s_291_5 -> bv
        let s_291_6: Bits = Bits::new(s_291_5 as u128, 16u16);
        // D s_291_7: bit-extract s_291_6 s_291_3 s_291_4
        let s_291_7: Bits = (Bits::new(
            ((s_291_6) >> (s_291_3)).value(),
            u16::try_from(s_291_4).unwrap(),
        ));
        // D s_291_8: cast reint s_291_7 -> u8
        let s_291_8: u8 = (s_291_7.value() as u8);
        // C s_291_9: const #3s : i
        let s_291_9: i128 = 3;
        // C s_291_10: const #3s : i
        let s_291_10: i128 = 3;
        // D s_291_11: read-var u#38427:u16
        let s_291_11: u16 = fn_state.u_38427;
        // D s_291_12: cast zx s_291_11 -> bv
        let s_291_12: Bits = Bits::new(s_291_11 as u128, 16u16);
        // D s_291_13: bit-extract s_291_12 s_291_9 s_291_10
        let s_291_13: Bits = (Bits::new(
            ((s_291_12) >> (s_291_9)).value(),
            u16::try_from(s_291_10).unwrap(),
        ));
        // D s_291_14: cast reint s_291_13 -> u8
        let s_291_14: u8 = (s_291_13.value() as u8);
        // C s_291_15: const #0s : i
        let s_291_15: i128 = 0;
        // C s_291_16: const #3s : i
        let s_291_16: i128 = 3;
        // D s_291_17: read-var u#38427:u16
        let s_291_17: u16 = fn_state.u_38427;
        // D s_291_18: cast zx s_291_17 -> bv
        let s_291_18: Bits = Bits::new(s_291_17 as u128, 16u16);
        // D s_291_19: bit-extract s_291_18 s_291_15 s_291_16
        let s_291_19: Bits = (Bits::new(
            ((s_291_18) >> (s_291_15)).value(),
            u16::try_from(s_291_16).unwrap(),
        ));
        // D s_291_20: cast reint s_291_19 -> u8
        let s_291_20: u8 = (s_291_19.value() as u8);
        // D s_291_21: call decode_aarch32_instrs_STRH_r_T1enc_A_txt(s_291_8, s_291_14, s_291_20)
        let s_291_21: () = decode_aarch32_instrs_STRH_r_T1enc_A_txt(
            state,
            tracer,
            s_291_8,
            s_291_14,
            s_291_20,
        );
        // N s_291_22: return
        return;
    }
    fn block_292<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_292_0: read-var merge#var.1:struct
        let s_292_0: u16 = fn_state.merge_var._1;
        // D s_292_1: write-var u#38432 <= s_292_0
        fn_state.u_38432 = s_292_0;
        // C s_292_2: const #11s : i
        let s_292_2: i128 = 11;
        // D s_292_3: read-var u#38432:u16
        let s_292_3: u16 = fn_state.u_38432;
        // D s_292_4: cast zx s_292_3 -> bv
        let s_292_4: Bits = Bits::new(s_292_3 as u128, 16u16);
        // C s_292_5: const #1s : i64
        let s_292_5: i64 = 1;
        // C s_292_6: cast zx s_292_5 -> i
        let s_292_6: i128 = (i128::try_from(s_292_5).unwrap());
        // C s_292_7: const #4s : i
        let s_292_7: i128 = 4;
        // C s_292_8: add s_292_7 s_292_6
        let s_292_8: i128 = (s_292_7 + s_292_6);
        // D s_292_9: bit-extract s_292_4 s_292_2 s_292_8
        let s_292_9: Bits = (Bits::new(
            ((s_292_4) >> (s_292_2)).value(),
            u16::try_from(s_292_8).unwrap(),
        ));
        // D s_292_10: cast reint s_292_9 -> u8
        let s_292_10: u8 = (s_292_9.value() as u8);
        // D s_292_11: cast zx s_292_10 -> bv
        let s_292_11: Bits = Bits::new(s_292_10 as u128, 5u16);
        // C s_292_12: const #12u : u8
        let s_292_12: u8 = 12;
        // C s_292_13: cast zx s_292_12 -> bv
        let s_292_13: Bits = Bits::new(s_292_12 as u128, 5u16);
        // D s_292_14: cmp-eq s_292_11 s_292_13
        let s_292_14: bool = ((s_292_11) == (s_292_13));
        // N s_292_15: branch s_292_14 b414 b293
        if s_292_14 {
            return block_414(state, tracer, fn_state);
        } else {
            return block_293(state, tracer, fn_state);
        };
    }
    fn block_293<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_293_0: const #0u : u8
        let s_293_0: bool = false;
        // D s_293_1: write-var gs#428669 <= s_293_0
        fn_state.gs_428669 = s_293_0;
        // N s_293_2: jump b294
        return block_294(state, tracer, fn_state);
    }
    fn block_294<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_294_0: read-var gs#428669:u8
        let s_294_0: bool = fn_state.gs_428669;
        // D s_294_1: not s_294_0
        let s_294_1: bool = !s_294_0;
        // N s_294_2: branch s_294_1 b296 b295
        if s_294_1 {
            return block_296(state, tracer, fn_state);
        } else {
            return block_295(state, tracer, fn_state);
        };
    }
    fn block_295<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_295_0: const #3225s : i
        let s_295_0: i128 = 3225;
        // C s_295_1: const #14696u : u32
        let s_295_1: u32 = 14696;
        // N s_295_2: write-reg s_295_1 <= s_295_0
        let s_295_2: () = {
            state.write_register::<i128>(s_295_1 as isize, s_295_0);
            tracer.write_register(s_295_1 as isize, s_295_0);
        };
        // C s_295_3: const #6s : i
        let s_295_3: i128 = 6;
        // C s_295_4: const #5s : i
        let s_295_4: i128 = 5;
        // D s_295_5: read-var u#38432:u16
        let s_295_5: u16 = fn_state.u_38432;
        // D s_295_6: cast zx s_295_5 -> bv
        let s_295_6: Bits = Bits::new(s_295_5 as u128, 16u16);
        // D s_295_7: bit-extract s_295_6 s_295_3 s_295_4
        let s_295_7: Bits = (Bits::new(
            ((s_295_6) >> (s_295_3)).value(),
            u16::try_from(s_295_4).unwrap(),
        ));
        // D s_295_8: cast reint s_295_7 -> u8
        let s_295_8: u8 = (s_295_7.value() as u8);
        // C s_295_9: const #3s : i
        let s_295_9: i128 = 3;
        // C s_295_10: const #3s : i
        let s_295_10: i128 = 3;
        // D s_295_11: read-var u#38432:u16
        let s_295_11: u16 = fn_state.u_38432;
        // D s_295_12: cast zx s_295_11 -> bv
        let s_295_12: Bits = Bits::new(s_295_11 as u128, 16u16);
        // D s_295_13: bit-extract s_295_12 s_295_9 s_295_10
        let s_295_13: Bits = (Bits::new(
            ((s_295_12) >> (s_295_9)).value(),
            u16::try_from(s_295_10).unwrap(),
        ));
        // D s_295_14: cast reint s_295_13 -> u8
        let s_295_14: u8 = (s_295_13.value() as u8);
        // C s_295_15: const #0s : i
        let s_295_15: i128 = 0;
        // C s_295_16: const #3s : i
        let s_295_16: i128 = 3;
        // D s_295_17: read-var u#38432:u16
        let s_295_17: u16 = fn_state.u_38432;
        // D s_295_18: cast zx s_295_17 -> bv
        let s_295_18: Bits = Bits::new(s_295_17 as u128, 16u16);
        // D s_295_19: bit-extract s_295_18 s_295_15 s_295_16
        let s_295_19: Bits = (Bits::new(
            ((s_295_18) >> (s_295_15)).value(),
            u16::try_from(s_295_16).unwrap(),
        ));
        // D s_295_20: cast reint s_295_19 -> u8
        let s_295_20: u8 = (s_295_19.value() as u8);
        // D s_295_21: call decode_aarch32_instrs_STR_i_T1enc_A_txt(s_295_8, s_295_14, s_295_20)
        let s_295_21: () = decode_aarch32_instrs_STR_i_T1enc_A_txt(
            state,
            tracer,
            s_295_8,
            s_295_14,
            s_295_20,
        );
        // N s_295_22: return
        return;
    }
    fn block_296<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_296_0: read-var merge#var.1:struct
        let s_296_0: u16 = fn_state.merge_var._1;
        // D s_296_1: write-var u#38437 <= s_296_0
        fn_state.u_38437 = s_296_0;
        // C s_296_2: const #11s : i
        let s_296_2: i128 = 11;
        // D s_296_3: read-var u#38437:u16
        let s_296_3: u16 = fn_state.u_38437;
        // D s_296_4: cast zx s_296_3 -> bv
        let s_296_4: Bits = Bits::new(s_296_3 as u128, 16u16);
        // C s_296_5: const #1s : i64
        let s_296_5: i64 = 1;
        // C s_296_6: cast zx s_296_5 -> i
        let s_296_6: i128 = (i128::try_from(s_296_5).unwrap());
        // C s_296_7: const #4s : i
        let s_296_7: i128 = 4;
        // C s_296_8: add s_296_7 s_296_6
        let s_296_8: i128 = (s_296_7 + s_296_6);
        // D s_296_9: bit-extract s_296_4 s_296_2 s_296_8
        let s_296_9: Bits = (Bits::new(
            ((s_296_4) >> (s_296_2)).value(),
            u16::try_from(s_296_8).unwrap(),
        ));
        // D s_296_10: cast reint s_296_9 -> u8
        let s_296_10: u8 = (s_296_9.value() as u8);
        // D s_296_11: cast zx s_296_10 -> bv
        let s_296_11: Bits = Bits::new(s_296_10 as u128, 5u16);
        // C s_296_12: const #18u : u8
        let s_296_12: u8 = 18;
        // C s_296_13: cast zx s_296_12 -> bv
        let s_296_13: Bits = Bits::new(s_296_12 as u128, 5u16);
        // D s_296_14: cmp-eq s_296_11 s_296_13
        let s_296_14: bool = ((s_296_11) == (s_296_13));
        // N s_296_15: branch s_296_14 b413 b297
        if s_296_14 {
            return block_413(state, tracer, fn_state);
        } else {
            return block_297(state, tracer, fn_state);
        };
    }
    fn block_297<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_297_0: const #0u : u8
        let s_297_0: bool = false;
        // D s_297_1: write-var gs#428682 <= s_297_0
        fn_state.gs_428682 = s_297_0;
        // N s_297_2: jump b298
        return block_298(state, tracer, fn_state);
    }
    fn block_298<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_298_0: read-var gs#428682:u8
        let s_298_0: bool = fn_state.gs_428682;
        // D s_298_1: not s_298_0
        let s_298_1: bool = !s_298_0;
        // N s_298_2: branch s_298_1 b300 b299
        if s_298_1 {
            return block_300(state, tracer, fn_state);
        } else {
            return block_299(state, tracer, fn_state);
        };
    }
    fn block_299<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_299_0: const #3226s : i
        let s_299_0: i128 = 3226;
        // C s_299_1: const #14696u : u32
        let s_299_1: u32 = 14696;
        // N s_299_2: write-reg s_299_1 <= s_299_0
        let s_299_2: () = {
            state.write_register::<i128>(s_299_1 as isize, s_299_0);
            tracer.write_register(s_299_1 as isize, s_299_0);
        };
        // C s_299_3: const #8s : i
        let s_299_3: i128 = 8;
        // C s_299_4: const #3s : i
        let s_299_4: i128 = 3;
        // D s_299_5: read-var u#38437:u16
        let s_299_5: u16 = fn_state.u_38437;
        // D s_299_6: cast zx s_299_5 -> bv
        let s_299_6: Bits = Bits::new(s_299_5 as u128, 16u16);
        // D s_299_7: bit-extract s_299_6 s_299_3 s_299_4
        let s_299_7: Bits = (Bits::new(
            ((s_299_6) >> (s_299_3)).value(),
            u16::try_from(s_299_4).unwrap(),
        ));
        // D s_299_8: cast reint s_299_7 -> u8
        let s_299_8: u8 = (s_299_7.value() as u8);
        // C s_299_9: const #0s : i
        let s_299_9: i128 = 0;
        // C s_299_10: const #8s : i
        let s_299_10: i128 = 8;
        // D s_299_11: read-var u#38437:u16
        let s_299_11: u16 = fn_state.u_38437;
        // D s_299_12: cast zx s_299_11 -> bv
        let s_299_12: Bits = Bits::new(s_299_11 as u128, 16u16);
        // D s_299_13: bit-extract s_299_12 s_299_9 s_299_10
        let s_299_13: Bits = (Bits::new(
            ((s_299_12) >> (s_299_9)).value(),
            u16::try_from(s_299_10).unwrap(),
        ));
        // D s_299_14: cast reint s_299_13 -> u8
        let s_299_14: u8 = (s_299_13.value() as u8);
        // D s_299_15: call decode_aarch32_instrs_STR_i_T2enc_A_txt(s_299_8, s_299_14)
        let s_299_15: () = decode_aarch32_instrs_STR_i_T2enc_A_txt(
            state,
            tracer,
            s_299_8,
            s_299_14,
        );
        // N s_299_16: return
        return;
    }
    fn block_300<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_300_0: read-var merge#var.1:struct
        let s_300_0: u16 = fn_state.merge_var._1;
        // D s_300_1: write-var u#38441 <= s_300_0
        fn_state.u_38441 = s_300_0;
        // C s_300_2: const #9s : i
        let s_300_2: i128 = 9;
        // D s_300_3: read-var u#38441:u16
        let s_300_3: u16 = fn_state.u_38441;
        // D s_300_4: cast zx s_300_3 -> bv
        let s_300_4: Bits = Bits::new(s_300_3 as u128, 16u16);
        // C s_300_5: const #1s : i64
        let s_300_5: i64 = 1;
        // C s_300_6: cast zx s_300_5 -> i
        let s_300_6: i128 = (i128::try_from(s_300_5).unwrap());
        // C s_300_7: const #6s : i
        let s_300_7: i128 = 6;
        // C s_300_8: add s_300_7 s_300_6
        let s_300_8: i128 = (s_300_7 + s_300_6);
        // D s_300_9: bit-extract s_300_4 s_300_2 s_300_8
        let s_300_9: Bits = (Bits::new(
            ((s_300_4) >> (s_300_2)).value(),
            u16::try_from(s_300_8).unwrap(),
        ));
        // D s_300_10: cast reint s_300_9 -> u8
        let s_300_10: u8 = (s_300_9.value() as u8);
        // D s_300_11: cast zx s_300_10 -> bv
        let s_300_11: Bits = Bits::new(s_300_10 as u128, 7u16);
        // C s_300_12: const #40u : u8
        let s_300_12: u8 = 40;
        // C s_300_13: cast zx s_300_12 -> bv
        let s_300_13: Bits = Bits::new(s_300_12 as u128, 7u16);
        // D s_300_14: cmp-eq s_300_11 s_300_13
        let s_300_14: bool = ((s_300_11) == (s_300_13));
        // N s_300_15: branch s_300_14 b412 b301
        if s_300_14 {
            return block_412(state, tracer, fn_state);
        } else {
            return block_301(state, tracer, fn_state);
        };
    }
    fn block_301<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_301_0: const #0u : u8
        let s_301_0: bool = false;
        // D s_301_1: write-var gs#428693 <= s_301_0
        fn_state.gs_428693 = s_301_0;
        // N s_301_2: jump b302
        return block_302(state, tracer, fn_state);
    }
    fn block_302<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_302_0: read-var gs#428693:u8
        let s_302_0: bool = fn_state.gs_428693;
        // D s_302_1: not s_302_0
        let s_302_1: bool = !s_302_0;
        // N s_302_2: branch s_302_1 b304 b303
        if s_302_1 {
            return block_304(state, tracer, fn_state);
        } else {
            return block_303(state, tracer, fn_state);
        };
    }
    fn block_303<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_303_0: const #3230s : i
        let s_303_0: i128 = 3230;
        // C s_303_1: const #14696u : u32
        let s_303_1: u32 = 14696;
        // N s_303_2: write-reg s_303_1 <= s_303_0
        let s_303_2: () = {
            state.write_register::<i128>(s_303_1 as isize, s_303_0);
            tracer.write_register(s_303_1 as isize, s_303_0);
        };
        // C s_303_3: const #6s : i
        let s_303_3: i128 = 6;
        // C s_303_4: const #3s : i
        let s_303_4: i128 = 3;
        // D s_303_5: read-var u#38441:u16
        let s_303_5: u16 = fn_state.u_38441;
        // D s_303_6: cast zx s_303_5 -> bv
        let s_303_6: Bits = Bits::new(s_303_5 as u128, 16u16);
        // D s_303_7: bit-extract s_303_6 s_303_3 s_303_4
        let s_303_7: Bits = (Bits::new(
            ((s_303_6) >> (s_303_3)).value(),
            u16::try_from(s_303_4).unwrap(),
        ));
        // D s_303_8: cast reint s_303_7 -> u8
        let s_303_8: u8 = (s_303_7.value() as u8);
        // C s_303_9: const #3s : i
        let s_303_9: i128 = 3;
        // C s_303_10: const #3s : i
        let s_303_10: i128 = 3;
        // D s_303_11: read-var u#38441:u16
        let s_303_11: u16 = fn_state.u_38441;
        // D s_303_12: cast zx s_303_11 -> bv
        let s_303_12: Bits = Bits::new(s_303_11 as u128, 16u16);
        // D s_303_13: bit-extract s_303_12 s_303_9 s_303_10
        let s_303_13: Bits = (Bits::new(
            ((s_303_12) >> (s_303_9)).value(),
            u16::try_from(s_303_10).unwrap(),
        ));
        // D s_303_14: cast reint s_303_13 -> u8
        let s_303_14: u8 = (s_303_13.value() as u8);
        // C s_303_15: const #0s : i
        let s_303_15: i128 = 0;
        // C s_303_16: const #3s : i
        let s_303_16: i128 = 3;
        // D s_303_17: read-var u#38441:u16
        let s_303_17: u16 = fn_state.u_38441;
        // D s_303_18: cast zx s_303_17 -> bv
        let s_303_18: Bits = Bits::new(s_303_17 as u128, 16u16);
        // D s_303_19: bit-extract s_303_18 s_303_15 s_303_16
        let s_303_19: Bits = (Bits::new(
            ((s_303_18) >> (s_303_15)).value(),
            u16::try_from(s_303_16).unwrap(),
        ));
        // D s_303_20: cast reint s_303_19 -> u8
        let s_303_20: u8 = (s_303_19.value() as u8);
        // D s_303_21: call decode_aarch32_instrs_STR_r_T1enc_A_txt(s_303_8, s_303_14, s_303_20)
        let s_303_21: () = decode_aarch32_instrs_STR_r_T1enc_A_txt(
            state,
            tracer,
            s_303_8,
            s_303_14,
            s_303_20,
        );
        // N s_303_22: return
        return;
    }
    fn block_304<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_304_0: read-var merge#var.1:struct
        let s_304_0: u16 = fn_state.merge_var._1;
        // D s_304_1: write-var u#38446 <= s_304_0
        fn_state.u_38446 = s_304_0;
        // C s_304_2: const #9s : i
        let s_304_2: i128 = 9;
        // D s_304_3: read-var u#38446:u16
        let s_304_3: u16 = fn_state.u_38446;
        // D s_304_4: cast zx s_304_3 -> bv
        let s_304_4: Bits = Bits::new(s_304_3 as u128, 16u16);
        // C s_304_5: const #1s : i64
        let s_304_5: i64 = 1;
        // C s_304_6: cast zx s_304_5 -> i
        let s_304_6: i128 = (i128::try_from(s_304_5).unwrap());
        // C s_304_7: const #6s : i
        let s_304_7: i128 = 6;
        // C s_304_8: add s_304_7 s_304_6
        let s_304_8: i128 = (s_304_7 + s_304_6);
        // D s_304_9: bit-extract s_304_4 s_304_2 s_304_8
        let s_304_9: Bits = (Bits::new(
            ((s_304_4) >> (s_304_2)).value(),
            u16::try_from(s_304_8).unwrap(),
        ));
        // D s_304_10: cast reint s_304_9 -> u8
        let s_304_10: u8 = (s_304_9.value() as u8);
        // D s_304_11: cast zx s_304_10 -> bv
        let s_304_11: Bits = Bits::new(s_304_10 as u128, 7u16);
        // C s_304_12: const #15u : u8
        let s_304_12: u8 = 15;
        // C s_304_13: cast zx s_304_12 -> bv
        let s_304_13: Bits = Bits::new(s_304_12 as u128, 7u16);
        // D s_304_14: cmp-eq s_304_11 s_304_13
        let s_304_14: bool = ((s_304_11) == (s_304_13));
        // N s_304_15: branch s_304_14 b411 b305
        if s_304_14 {
            return block_411(state, tracer, fn_state);
        } else {
            return block_305(state, tracer, fn_state);
        };
    }
    fn block_305<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_305_0: const #0u : u8
        let s_305_0: bool = false;
        // D s_305_1: write-var gs#428706 <= s_305_0
        fn_state.gs_428706 = s_305_0;
        // N s_305_2: jump b306
        return block_306(state, tracer, fn_state);
    }
    fn block_306<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_306_0: read-var gs#428706:u8
        let s_306_0: bool = fn_state.gs_428706;
        // D s_306_1: not s_306_0
        let s_306_1: bool = !s_306_0;
        // N s_306_2: branch s_306_1 b308 b307
        if s_306_1 {
            return block_308(state, tracer, fn_state);
        } else {
            return block_307(state, tracer, fn_state);
        };
    }
    fn block_307<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_307_0: const #3236s : i
        let s_307_0: i128 = 3236;
        // C s_307_1: const #14696u : u32
        let s_307_1: u32 = 14696;
        // N s_307_2: write-reg s_307_1 <= s_307_0
        let s_307_2: () = {
            state.write_register::<i128>(s_307_1 as isize, s_307_0);
            tracer.write_register(s_307_1 as isize, s_307_0);
        };
        // C s_307_3: const #6s : i
        let s_307_3: i128 = 6;
        // C s_307_4: const #3s : i
        let s_307_4: i128 = 3;
        // D s_307_5: read-var u#38446:u16
        let s_307_5: u16 = fn_state.u_38446;
        // D s_307_6: cast zx s_307_5 -> bv
        let s_307_6: Bits = Bits::new(s_307_5 as u128, 16u16);
        // D s_307_7: bit-extract s_307_6 s_307_3 s_307_4
        let s_307_7: Bits = (Bits::new(
            ((s_307_6) >> (s_307_3)).value(),
            u16::try_from(s_307_4).unwrap(),
        ));
        // D s_307_8: cast reint s_307_7 -> u8
        let s_307_8: u8 = (s_307_7.value() as u8);
        // C s_307_9: const #3s : i
        let s_307_9: i128 = 3;
        // C s_307_10: const #3s : i
        let s_307_10: i128 = 3;
        // D s_307_11: read-var u#38446:u16
        let s_307_11: u16 = fn_state.u_38446;
        // D s_307_12: cast zx s_307_11 -> bv
        let s_307_12: Bits = Bits::new(s_307_11 as u128, 16u16);
        // D s_307_13: bit-extract s_307_12 s_307_9 s_307_10
        let s_307_13: Bits = (Bits::new(
            ((s_307_12) >> (s_307_9)).value(),
            u16::try_from(s_307_10).unwrap(),
        ));
        // D s_307_14: cast reint s_307_13 -> u8
        let s_307_14: u8 = (s_307_13.value() as u8);
        // C s_307_15: const #0s : i
        let s_307_15: i128 = 0;
        // C s_307_16: const #3s : i
        let s_307_16: i128 = 3;
        // D s_307_17: read-var u#38446:u16
        let s_307_17: u16 = fn_state.u_38446;
        // D s_307_18: cast zx s_307_17 -> bv
        let s_307_18: Bits = Bits::new(s_307_17 as u128, 16u16);
        // D s_307_19: bit-extract s_307_18 s_307_15 s_307_16
        let s_307_19: Bits = (Bits::new(
            ((s_307_18) >> (s_307_15)).value(),
            u16::try_from(s_307_16).unwrap(),
        ));
        // D s_307_20: cast reint s_307_19 -> u8
        let s_307_20: u8 = (s_307_19.value() as u8);
        // D s_307_21: call decode_aarch32_instrs_SUB_i_T1enc_A_txt(s_307_8, s_307_14, s_307_20)
        let s_307_21: () = decode_aarch32_instrs_SUB_i_T1enc_A_txt(
            state,
            tracer,
            s_307_8,
            s_307_14,
            s_307_20,
        );
        // N s_307_22: return
        return;
    }
    fn block_308<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_308_0: read-var merge#var.1:struct
        let s_308_0: u16 = fn_state.merge_var._1;
        // D s_308_1: write-var u#38451 <= s_308_0
        fn_state.u_38451 = s_308_0;
        // C s_308_2: const #11s : i
        let s_308_2: i128 = 11;
        // D s_308_3: read-var u#38451:u16
        let s_308_3: u16 = fn_state.u_38451;
        // D s_308_4: cast zx s_308_3 -> bv
        let s_308_4: Bits = Bits::new(s_308_3 as u128, 16u16);
        // C s_308_5: const #1s : i64
        let s_308_5: i64 = 1;
        // C s_308_6: cast zx s_308_5 -> i
        let s_308_6: i128 = (i128::try_from(s_308_5).unwrap());
        // C s_308_7: const #4s : i
        let s_308_7: i128 = 4;
        // C s_308_8: add s_308_7 s_308_6
        let s_308_8: i128 = (s_308_7 + s_308_6);
        // D s_308_9: bit-extract s_308_4 s_308_2 s_308_8
        let s_308_9: Bits = (Bits::new(
            ((s_308_4) >> (s_308_2)).value(),
            u16::try_from(s_308_8).unwrap(),
        ));
        // D s_308_10: cast reint s_308_9 -> u8
        let s_308_10: u8 = (s_308_9.value() as u8);
        // D s_308_11: cast zx s_308_10 -> bv
        let s_308_11: Bits = Bits::new(s_308_10 as u128, 5u16);
        // C s_308_12: const #7u : u8
        let s_308_12: u8 = 7;
        // C s_308_13: cast zx s_308_12 -> bv
        let s_308_13: Bits = Bits::new(s_308_12 as u128, 5u16);
        // D s_308_14: cmp-eq s_308_11 s_308_13
        let s_308_14: bool = ((s_308_11) == (s_308_13));
        // N s_308_15: branch s_308_14 b410 b309
        if s_308_14 {
            return block_410(state, tracer, fn_state);
        } else {
            return block_309(state, tracer, fn_state);
        };
    }
    fn block_309<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_309_0: const #0u : u8
        let s_309_0: bool = false;
        // D s_309_1: write-var gs#428719 <= s_309_0
        fn_state.gs_428719 = s_309_0;
        // N s_309_2: jump b310
        return block_310(state, tracer, fn_state);
    }
    fn block_310<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_310_0: read-var gs#428719:u8
        let s_310_0: bool = fn_state.gs_428719;
        // D s_310_1: not s_310_0
        let s_310_1: bool = !s_310_0;
        // N s_310_2: branch s_310_1 b312 b311
        if s_310_1 {
            return block_312(state, tracer, fn_state);
        } else {
            return block_311(state, tracer, fn_state);
        };
    }
    fn block_311<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_311_0: const #3237s : i
        let s_311_0: i128 = 3237;
        // C s_311_1: const #14696u : u32
        let s_311_1: u32 = 14696;
        // N s_311_2: write-reg s_311_1 <= s_311_0
        let s_311_2: () = {
            state.write_register::<i128>(s_311_1 as isize, s_311_0);
            tracer.write_register(s_311_1 as isize, s_311_0);
        };
        // C s_311_3: const #8s : i
        let s_311_3: i128 = 8;
        // C s_311_4: const #3s : i
        let s_311_4: i128 = 3;
        // D s_311_5: read-var u#38451:u16
        let s_311_5: u16 = fn_state.u_38451;
        // D s_311_6: cast zx s_311_5 -> bv
        let s_311_6: Bits = Bits::new(s_311_5 as u128, 16u16);
        // D s_311_7: bit-extract s_311_6 s_311_3 s_311_4
        let s_311_7: Bits = (Bits::new(
            ((s_311_6) >> (s_311_3)).value(),
            u16::try_from(s_311_4).unwrap(),
        ));
        // D s_311_8: cast reint s_311_7 -> u8
        let s_311_8: u8 = (s_311_7.value() as u8);
        // C s_311_9: const #0s : i
        let s_311_9: i128 = 0;
        // C s_311_10: const #8s : i
        let s_311_10: i128 = 8;
        // D s_311_11: read-var u#38451:u16
        let s_311_11: u16 = fn_state.u_38451;
        // D s_311_12: cast zx s_311_11 -> bv
        let s_311_12: Bits = Bits::new(s_311_11 as u128, 16u16);
        // D s_311_13: bit-extract s_311_12 s_311_9 s_311_10
        let s_311_13: Bits = (Bits::new(
            ((s_311_12) >> (s_311_9)).value(),
            u16::try_from(s_311_10).unwrap(),
        ));
        // D s_311_14: cast reint s_311_13 -> u8
        let s_311_14: u8 = (s_311_13.value() as u8);
        // D s_311_15: call decode_aarch32_instrs_SUB_i_T2enc_A_txt(s_311_8, s_311_14)
        let s_311_15: () = decode_aarch32_instrs_SUB_i_T2enc_A_txt(
            state,
            tracer,
            s_311_8,
            s_311_14,
        );
        // N s_311_16: return
        return;
    }
    fn block_312<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_312_0: read-var merge#var.1:struct
        let s_312_0: u16 = fn_state.merge_var._1;
        // D s_312_1: write-var u#38455 <= s_312_0
        fn_state.u_38455 = s_312_0;
        // C s_312_2: const #9s : i
        let s_312_2: i128 = 9;
        // D s_312_3: read-var u#38455:u16
        let s_312_3: u16 = fn_state.u_38455;
        // D s_312_4: cast zx s_312_3 -> bv
        let s_312_4: Bits = Bits::new(s_312_3 as u128, 16u16);
        // C s_312_5: const #1s : i64
        let s_312_5: i64 = 1;
        // C s_312_6: cast zx s_312_5 -> i
        let s_312_6: i128 = (i128::try_from(s_312_5).unwrap());
        // C s_312_7: const #6s : i
        let s_312_7: i128 = 6;
        // C s_312_8: add s_312_7 s_312_6
        let s_312_8: i128 = (s_312_7 + s_312_6);
        // D s_312_9: bit-extract s_312_4 s_312_2 s_312_8
        let s_312_9: Bits = (Bits::new(
            ((s_312_4) >> (s_312_2)).value(),
            u16::try_from(s_312_8).unwrap(),
        ));
        // D s_312_10: cast reint s_312_9 -> u8
        let s_312_10: u8 = (s_312_9.value() as u8);
        // D s_312_11: cast zx s_312_10 -> bv
        let s_312_11: Bits = Bits::new(s_312_10 as u128, 7u16);
        // C s_312_12: const #13u : u8
        let s_312_12: u8 = 13;
        // C s_312_13: cast zx s_312_12 -> bv
        let s_312_13: Bits = Bits::new(s_312_12 as u128, 7u16);
        // D s_312_14: cmp-eq s_312_11 s_312_13
        let s_312_14: bool = ((s_312_11) == (s_312_13));
        // N s_312_15: branch s_312_14 b409 b313
        if s_312_14 {
            return block_409(state, tracer, fn_state);
        } else {
            return block_313(state, tracer, fn_state);
        };
    }
    fn block_313<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_313_0: const #0u : u8
        let s_313_0: bool = false;
        // D s_313_1: write-var gs#428730 <= s_313_0
        fn_state.gs_428730 = s_313_0;
        // N s_313_2: jump b314
        return block_314(state, tracer, fn_state);
    }
    fn block_314<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_314_0: read-var gs#428730:u8
        let s_314_0: bool = fn_state.gs_428730;
        // D s_314_1: not s_314_0
        let s_314_1: bool = !s_314_0;
        // N s_314_2: branch s_314_1 b316 b315
        if s_314_1 {
            return block_316(state, tracer, fn_state);
        } else {
            return block_315(state, tracer, fn_state);
        };
    }
    fn block_315<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_315_0: const #3242s : i
        let s_315_0: i128 = 3242;
        // C s_315_1: const #14696u : u32
        let s_315_1: u32 = 14696;
        // N s_315_2: write-reg s_315_1 <= s_315_0
        let s_315_2: () = {
            state.write_register::<i128>(s_315_1 as isize, s_315_0);
            tracer.write_register(s_315_1 as isize, s_315_0);
        };
        // C s_315_3: const #6s : i
        let s_315_3: i128 = 6;
        // C s_315_4: const #3s : i
        let s_315_4: i128 = 3;
        // D s_315_5: read-var u#38455:u16
        let s_315_5: u16 = fn_state.u_38455;
        // D s_315_6: cast zx s_315_5 -> bv
        let s_315_6: Bits = Bits::new(s_315_5 as u128, 16u16);
        // D s_315_7: bit-extract s_315_6 s_315_3 s_315_4
        let s_315_7: Bits = (Bits::new(
            ((s_315_6) >> (s_315_3)).value(),
            u16::try_from(s_315_4).unwrap(),
        ));
        // D s_315_8: cast reint s_315_7 -> u8
        let s_315_8: u8 = (s_315_7.value() as u8);
        // C s_315_9: const #3s : i
        let s_315_9: i128 = 3;
        // C s_315_10: const #3s : i
        let s_315_10: i128 = 3;
        // D s_315_11: read-var u#38455:u16
        let s_315_11: u16 = fn_state.u_38455;
        // D s_315_12: cast zx s_315_11 -> bv
        let s_315_12: Bits = Bits::new(s_315_11 as u128, 16u16);
        // D s_315_13: bit-extract s_315_12 s_315_9 s_315_10
        let s_315_13: Bits = (Bits::new(
            ((s_315_12) >> (s_315_9)).value(),
            u16::try_from(s_315_10).unwrap(),
        ));
        // D s_315_14: cast reint s_315_13 -> u8
        let s_315_14: u8 = (s_315_13.value() as u8);
        // C s_315_15: const #0s : i
        let s_315_15: i128 = 0;
        // C s_315_16: const #3s : i
        let s_315_16: i128 = 3;
        // D s_315_17: read-var u#38455:u16
        let s_315_17: u16 = fn_state.u_38455;
        // D s_315_18: cast zx s_315_17 -> bv
        let s_315_18: Bits = Bits::new(s_315_17 as u128, 16u16);
        // D s_315_19: bit-extract s_315_18 s_315_15 s_315_16
        let s_315_19: Bits = (Bits::new(
            ((s_315_18) >> (s_315_15)).value(),
            u16::try_from(s_315_16).unwrap(),
        ));
        // D s_315_20: cast reint s_315_19 -> u8
        let s_315_20: u8 = (s_315_19.value() as u8);
        // D s_315_21: call decode_aarch32_instrs_SUB_r_T1enc_A_txt(s_315_8, s_315_14, s_315_20)
        let s_315_21: () = decode_aarch32_instrs_SUB_r_T1enc_A_txt(
            state,
            tracer,
            s_315_8,
            s_315_14,
            s_315_20,
        );
        // N s_315_22: return
        return;
    }
    fn block_316<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_316_0: read-var merge#var.1:struct
        let s_316_0: u16 = fn_state.merge_var._1;
        // D s_316_1: write-var u#38460 <= s_316_0
        fn_state.u_38460 = s_316_0;
        // C s_316_2: const #7s : i
        let s_316_2: i128 = 7;
        // D s_316_3: read-var u#38460:u16
        let s_316_3: u16 = fn_state.u_38460;
        // D s_316_4: cast zx s_316_3 -> bv
        let s_316_4: Bits = Bits::new(s_316_3 as u128, 16u16);
        // C s_316_5: const #1s : i64
        let s_316_5: i64 = 1;
        // C s_316_6: cast zx s_316_5 -> i
        let s_316_6: i128 = (i128::try_from(s_316_5).unwrap());
        // C s_316_7: const #8s : i
        let s_316_7: i128 = 8;
        // C s_316_8: add s_316_7 s_316_6
        let s_316_8: i128 = (s_316_7 + s_316_6);
        // D s_316_9: bit-extract s_316_4 s_316_2 s_316_8
        let s_316_9: Bits = (Bits::new(
            ((s_316_4) >> (s_316_2)).value(),
            u16::try_from(s_316_8).unwrap(),
        ));
        // D s_316_10: cast reint s_316_9 -> u9
        let s_316_10: u16 = (s_316_9.value() as u16);
        // D s_316_11: cast zx s_316_10 -> bv
        let s_316_11: Bits = Bits::new(s_316_10 as u128, 9u16);
        // C s_316_12: const #353u : u9
        let s_316_12: u16 = 353;
        // C s_316_13: cast zx s_316_12 -> bv
        let s_316_13: Bits = Bits::new(s_316_12 as u128, 9u16);
        // D s_316_14: cmp-eq s_316_11 s_316_13
        let s_316_14: bool = ((s_316_11) == (s_316_13));
        // N s_316_15: branch s_316_14 b408 b317
        if s_316_14 {
            return block_408(state, tracer, fn_state);
        } else {
            return block_317(state, tracer, fn_state);
        };
    }
    fn block_317<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_317_0: const #0u : u8
        let s_317_0: bool = false;
        // D s_317_1: write-var gs#428743 <= s_317_0
        fn_state.gs_428743 = s_317_0;
        // N s_317_2: jump b318
        return block_318(state, tracer, fn_state);
    }
    fn block_318<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_318_0: read-var gs#428743:u8
        let s_318_0: bool = fn_state.gs_428743;
        // D s_318_1: not s_318_0
        let s_318_1: bool = !s_318_0;
        // N s_318_2: branch s_318_1 b320 b319
        if s_318_1 {
            return block_320(state, tracer, fn_state);
        } else {
            return block_319(state, tracer, fn_state);
        };
    }
    fn block_319<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_319_0: const #3246s : i
        let s_319_0: i128 = 3246;
        // C s_319_1: const #14696u : u32
        let s_319_1: u32 = 14696;
        // N s_319_2: write-reg s_319_1 <= s_319_0
        let s_319_2: () = {
            state.write_register::<i128>(s_319_1 as isize, s_319_0);
            tracer.write_register(s_319_1 as isize, s_319_0);
        };
        // C s_319_3: const #0s : i
        let s_319_3: i128 = 0;
        // C s_319_4: const #7s : i
        let s_319_4: i128 = 7;
        // D s_319_5: read-var u#38460:u16
        let s_319_5: u16 = fn_state.u_38460;
        // D s_319_6: cast zx s_319_5 -> bv
        let s_319_6: Bits = Bits::new(s_319_5 as u128, 16u16);
        // D s_319_7: bit-extract s_319_6 s_319_3 s_319_4
        let s_319_7: Bits = (Bits::new(
            ((s_319_6) >> (s_319_3)).value(),
            u16::try_from(s_319_4).unwrap(),
        ));
        // D s_319_8: cast reint s_319_7 -> u8
        let s_319_8: u8 = (s_319_7.value() as u8);
        // D s_319_9: call decode_aarch32_instrs_SUB_SP_i_T1enc_A_txt(s_319_8)
        let s_319_9: () = decode_aarch32_instrs_SUB_SP_i_T1enc_A_txt(
            state,
            tracer,
            s_319_8,
        );
        // N s_319_10: return
        return;
    }
    fn block_320<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_320_0: read-var merge#var.1:struct
        let s_320_0: u16 = fn_state.merge_var._1;
        // D s_320_1: write-var u#38463 <= s_320_0
        fn_state.u_38463 = s_320_0;
        // C s_320_2: const #8s : i
        let s_320_2: i128 = 8;
        // D s_320_3: read-var u#38463:u16
        let s_320_3: u16 = fn_state.u_38463;
        // D s_320_4: cast zx s_320_3 -> bv
        let s_320_4: Bits = Bits::new(s_320_3 as u128, 16u16);
        // C s_320_5: const #1s : i64
        let s_320_5: i64 = 1;
        // C s_320_6: cast zx s_320_5 -> i
        let s_320_6: i128 = (i128::try_from(s_320_5).unwrap());
        // C s_320_7: const #7s : i
        let s_320_7: i128 = 7;
        // C s_320_8: add s_320_7 s_320_6
        let s_320_8: i128 = (s_320_7 + s_320_6);
        // D s_320_9: bit-extract s_320_4 s_320_2 s_320_8
        let s_320_9: Bits = (Bits::new(
            ((s_320_4) >> (s_320_2)).value(),
            u16::try_from(s_320_8).unwrap(),
        ));
        // D s_320_10: cast reint s_320_9 -> u8
        let s_320_10: u8 = (s_320_9.value() as u8);
        // D s_320_11: cast zx s_320_10 -> bv
        let s_320_11: Bits = Bits::new(s_320_10 as u128, 8u16);
        // C s_320_12: const #223u : u8
        let s_320_12: u8 = 223;
        // C s_320_13: cast zx s_320_12 -> bv
        let s_320_13: Bits = Bits::new(s_320_12 as u128, 8u16);
        // D s_320_14: cmp-eq s_320_11 s_320_13
        let s_320_14: bool = ((s_320_11) == (s_320_13));
        // N s_320_15: branch s_320_14 b407 b321
        if s_320_14 {
            return block_407(state, tracer, fn_state);
        } else {
            return block_321(state, tracer, fn_state);
        };
    }
    fn block_321<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_321_0: const #0u : u8
        let s_321_0: bool = false;
        // D s_321_1: write-var gs#428752 <= s_321_0
        fn_state.gs_428752 = s_321_0;
        // N s_321_2: jump b322
        return block_322(state, tracer, fn_state);
    }
    fn block_322<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_322_0: read-var gs#428752:u8
        let s_322_0: bool = fn_state.gs_428752;
        // D s_322_1: not s_322_0
        let s_322_1: bool = !s_322_0;
        // N s_322_2: branch s_322_1 b324 b323
        if s_322_1 {
            return block_324(state, tracer, fn_state);
        } else {
            return block_323(state, tracer, fn_state);
        };
    }
    fn block_323<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_323_0: const #3252s : i
        let s_323_0: i128 = 3252;
        // C s_323_1: const #14696u : u32
        let s_323_1: u32 = 14696;
        // N s_323_2: write-reg s_323_1 <= s_323_0
        let s_323_2: () = {
            state.write_register::<i128>(s_323_1 as isize, s_323_0);
            tracer.write_register(s_323_1 as isize, s_323_0);
        };
        // C s_323_3: const #0s : i
        let s_323_3: i128 = 0;
        // C s_323_4: const #8s : i
        let s_323_4: i128 = 8;
        // D s_323_5: read-var u#38463:u16
        let s_323_5: u16 = fn_state.u_38463;
        // D s_323_6: cast zx s_323_5 -> bv
        let s_323_6: Bits = Bits::new(s_323_5 as u128, 16u16);
        // D s_323_7: bit-extract s_323_6 s_323_3 s_323_4
        let s_323_7: Bits = (Bits::new(
            ((s_323_6) >> (s_323_3)).value(),
            u16::try_from(s_323_4).unwrap(),
        ));
        // D s_323_8: cast reint s_323_7 -> u8
        let s_323_8: u8 = (s_323_7.value() as u8);
        // D s_323_9: call decode_aarch32_instrs_SVC_T1enc_A_txt(s_323_8)
        let s_323_9: () = decode_aarch32_instrs_SVC_T1enc_A_txt(state, tracer, s_323_8);
        // N s_323_10: return
        return;
    }
    fn block_324<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_324_0: read-var merge#var.1:struct
        let s_324_0: u16 = fn_state.merge_var._1;
        // D s_324_1: write-var u#38466 <= s_324_0
        fn_state.u_38466 = s_324_0;
        // C s_324_2: const #6s : i
        let s_324_2: i128 = 6;
        // D s_324_3: read-var u#38466:u16
        let s_324_3: u16 = fn_state.u_38466;
        // D s_324_4: cast zx s_324_3 -> bv
        let s_324_4: Bits = Bits::new(s_324_3 as u128, 16u16);
        // C s_324_5: const #1s : i64
        let s_324_5: i64 = 1;
        // C s_324_6: cast zx s_324_5 -> i
        let s_324_6: i128 = (i128::try_from(s_324_5).unwrap());
        // C s_324_7: const #9s : i
        let s_324_7: i128 = 9;
        // C s_324_8: add s_324_7 s_324_6
        let s_324_8: i128 = (s_324_7 + s_324_6);
        // D s_324_9: bit-extract s_324_4 s_324_2 s_324_8
        let s_324_9: Bits = (Bits::new(
            ((s_324_4) >> (s_324_2)).value(),
            u16::try_from(s_324_8).unwrap(),
        ));
        // D s_324_10: cast reint s_324_9 -> u10
        let s_324_10: u16 = (s_324_9.value() as u16);
        // D s_324_11: cast zx s_324_10 -> bv
        let s_324_11: Bits = Bits::new(s_324_10 as u128, 10u16);
        // C s_324_12: const #713u : u10
        let s_324_12: u16 = 713;
        // C s_324_13: cast zx s_324_12 -> bv
        let s_324_13: Bits = Bits::new(s_324_12 as u128, 10u16);
        // D s_324_14: cmp-eq s_324_11 s_324_13
        let s_324_14: bool = ((s_324_11) == (s_324_13));
        // N s_324_15: branch s_324_14 b406 b325
        if s_324_14 {
            return block_406(state, tracer, fn_state);
        } else {
            return block_325(state, tracer, fn_state);
        };
    }
    fn block_325<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_325_0: const #0u : u8
        let s_325_0: bool = false;
        // D s_325_1: write-var gs#428761 <= s_325_0
        fn_state.gs_428761 = s_325_0;
        // N s_325_2: jump b326
        return block_326(state, tracer, fn_state);
    }
    fn block_326<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_326_0: read-var gs#428761:u8
        let s_326_0: bool = fn_state.gs_428761;
        // D s_326_1: not s_326_0
        let s_326_1: bool = !s_326_0;
        // N s_326_2: branch s_326_1 b328 b327
        if s_326_1 {
            return block_328(state, tracer, fn_state);
        } else {
            return block_327(state, tracer, fn_state);
        };
    }
    fn block_327<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_327_0: const #3262s : i
        let s_327_0: i128 = 3262;
        // C s_327_1: const #14696u : u32
        let s_327_1: u32 = 14696;
        // N s_327_2: write-reg s_327_1 <= s_327_0
        let s_327_2: () = {
            state.write_register::<i128>(s_327_1 as isize, s_327_0);
            tracer.write_register(s_327_1 as isize, s_327_0);
        };
        // C s_327_3: const #3s : i
        let s_327_3: i128 = 3;
        // C s_327_4: const #3s : i
        let s_327_4: i128 = 3;
        // D s_327_5: read-var u#38466:u16
        let s_327_5: u16 = fn_state.u_38466;
        // D s_327_6: cast zx s_327_5 -> bv
        let s_327_6: Bits = Bits::new(s_327_5 as u128, 16u16);
        // D s_327_7: bit-extract s_327_6 s_327_3 s_327_4
        let s_327_7: Bits = (Bits::new(
            ((s_327_6) >> (s_327_3)).value(),
            u16::try_from(s_327_4).unwrap(),
        ));
        // D s_327_8: cast reint s_327_7 -> u8
        let s_327_8: u8 = (s_327_7.value() as u8);
        // C s_327_9: const #0s : i
        let s_327_9: i128 = 0;
        // C s_327_10: const #3s : i
        let s_327_10: i128 = 3;
        // D s_327_11: read-var u#38466:u16
        let s_327_11: u16 = fn_state.u_38466;
        // D s_327_12: cast zx s_327_11 -> bv
        let s_327_12: Bits = Bits::new(s_327_11 as u128, 16u16);
        // D s_327_13: bit-extract s_327_12 s_327_9 s_327_10
        let s_327_13: Bits = (Bits::new(
            ((s_327_12) >> (s_327_9)).value(),
            u16::try_from(s_327_10).unwrap(),
        ));
        // D s_327_14: cast reint s_327_13 -> u8
        let s_327_14: u8 = (s_327_13.value() as u8);
        // D s_327_15: call decode_aarch32_instrs_SXTB_T1enc_A_txt(s_327_8, s_327_14)
        let s_327_15: () = decode_aarch32_instrs_SXTB_T1enc_A_txt(
            state,
            tracer,
            s_327_8,
            s_327_14,
        );
        // N s_327_16: return
        return;
    }
    fn block_328<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_328_0: read-var merge#var.1:struct
        let s_328_0: u16 = fn_state.merge_var._1;
        // D s_328_1: write-var u#38470 <= s_328_0
        fn_state.u_38470 = s_328_0;
        // C s_328_2: const #6s : i
        let s_328_2: i128 = 6;
        // D s_328_3: read-var u#38470:u16
        let s_328_3: u16 = fn_state.u_38470;
        // D s_328_4: cast zx s_328_3 -> bv
        let s_328_4: Bits = Bits::new(s_328_3 as u128, 16u16);
        // C s_328_5: const #1s : i64
        let s_328_5: i64 = 1;
        // C s_328_6: cast zx s_328_5 -> i
        let s_328_6: i128 = (i128::try_from(s_328_5).unwrap());
        // C s_328_7: const #9s : i
        let s_328_7: i128 = 9;
        // C s_328_8: add s_328_7 s_328_6
        let s_328_8: i128 = (s_328_7 + s_328_6);
        // D s_328_9: bit-extract s_328_4 s_328_2 s_328_8
        let s_328_9: Bits = (Bits::new(
            ((s_328_4) >> (s_328_2)).value(),
            u16::try_from(s_328_8).unwrap(),
        ));
        // D s_328_10: cast reint s_328_9 -> u10
        let s_328_10: u16 = (s_328_9.value() as u16);
        // D s_328_11: cast zx s_328_10 -> bv
        let s_328_11: Bits = Bits::new(s_328_10 as u128, 10u16);
        // C s_328_12: const #712u : u10
        let s_328_12: u16 = 712;
        // C s_328_13: cast zx s_328_12 -> bv
        let s_328_13: Bits = Bits::new(s_328_12 as u128, 10u16);
        // D s_328_14: cmp-eq s_328_11 s_328_13
        let s_328_14: bool = ((s_328_11) == (s_328_13));
        // N s_328_15: branch s_328_14 b405 b329
        if s_328_14 {
            return block_405(state, tracer, fn_state);
        } else {
            return block_329(state, tracer, fn_state);
        };
    }
    fn block_329<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_329_0: const #0u : u8
        let s_329_0: bool = false;
        // D s_329_1: write-var gs#428772 <= s_329_0
        fn_state.gs_428772 = s_329_0;
        // N s_329_2: jump b330
        return block_330(state, tracer, fn_state);
    }
    fn block_330<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_330_0: read-var gs#428772:u8
        let s_330_0: bool = fn_state.gs_428772;
        // D s_330_1: not s_330_0
        let s_330_1: bool = !s_330_0;
        // N s_330_2: branch s_330_1 b332 b331
        if s_330_1 {
            return block_332(state, tracer, fn_state);
        } else {
            return block_331(state, tracer, fn_state);
        };
    }
    fn block_331<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_331_0: const #3265s : i
        let s_331_0: i128 = 3265;
        // C s_331_1: const #14696u : u32
        let s_331_1: u32 = 14696;
        // N s_331_2: write-reg s_331_1 <= s_331_0
        let s_331_2: () = {
            state.write_register::<i128>(s_331_1 as isize, s_331_0);
            tracer.write_register(s_331_1 as isize, s_331_0);
        };
        // C s_331_3: const #3s : i
        let s_331_3: i128 = 3;
        // C s_331_4: const #3s : i
        let s_331_4: i128 = 3;
        // D s_331_5: read-var u#38470:u16
        let s_331_5: u16 = fn_state.u_38470;
        // D s_331_6: cast zx s_331_5 -> bv
        let s_331_6: Bits = Bits::new(s_331_5 as u128, 16u16);
        // D s_331_7: bit-extract s_331_6 s_331_3 s_331_4
        let s_331_7: Bits = (Bits::new(
            ((s_331_6) >> (s_331_3)).value(),
            u16::try_from(s_331_4).unwrap(),
        ));
        // D s_331_8: cast reint s_331_7 -> u8
        let s_331_8: u8 = (s_331_7.value() as u8);
        // C s_331_9: const #0s : i
        let s_331_9: i128 = 0;
        // C s_331_10: const #3s : i
        let s_331_10: i128 = 3;
        // D s_331_11: read-var u#38470:u16
        let s_331_11: u16 = fn_state.u_38470;
        // D s_331_12: cast zx s_331_11 -> bv
        let s_331_12: Bits = Bits::new(s_331_11 as u128, 16u16);
        // D s_331_13: bit-extract s_331_12 s_331_9 s_331_10
        let s_331_13: Bits = (Bits::new(
            ((s_331_12) >> (s_331_9)).value(),
            u16::try_from(s_331_10).unwrap(),
        ));
        // D s_331_14: cast reint s_331_13 -> u8
        let s_331_14: u8 = (s_331_13.value() as u8);
        // D s_331_15: call decode_aarch32_instrs_SXTH_T1enc_A_txt(s_331_8, s_331_14)
        let s_331_15: () = decode_aarch32_instrs_SXTH_T1enc_A_txt(
            state,
            tracer,
            s_331_8,
            s_331_14,
        );
        // N s_331_16: return
        return;
    }
    fn block_332<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_332_0: read-var merge#var.1:struct
        let s_332_0: u16 = fn_state.merge_var._1;
        // D s_332_1: write-var u#38474 <= s_332_0
        fn_state.u_38474 = s_332_0;
        // C s_332_2: const #6s : i
        let s_332_2: i128 = 6;
        // D s_332_3: read-var u#38474:u16
        let s_332_3: u16 = fn_state.u_38474;
        // D s_332_4: cast zx s_332_3 -> bv
        let s_332_4: Bits = Bits::new(s_332_3 as u128, 16u16);
        // C s_332_5: const #1s : i64
        let s_332_5: i64 = 1;
        // C s_332_6: cast zx s_332_5 -> i
        let s_332_6: i128 = (i128::try_from(s_332_5).unwrap());
        // C s_332_7: const #9s : i
        let s_332_7: i128 = 9;
        // C s_332_8: add s_332_7 s_332_6
        let s_332_8: i128 = (s_332_7 + s_332_6);
        // D s_332_9: bit-extract s_332_4 s_332_2 s_332_8
        let s_332_9: Bits = (Bits::new(
            ((s_332_4) >> (s_332_2)).value(),
            u16::try_from(s_332_8).unwrap(),
        ));
        // D s_332_10: cast reint s_332_9 -> u10
        let s_332_10: u16 = (s_332_9.value() as u16);
        // D s_332_11: cast zx s_332_10 -> bv
        let s_332_11: Bits = Bits::new(s_332_10 as u128, 10u16);
        // C s_332_12: const #264u : u10
        let s_332_12: u16 = 264;
        // C s_332_13: cast zx s_332_12 -> bv
        let s_332_13: Bits = Bits::new(s_332_12 as u128, 10u16);
        // D s_332_14: cmp-eq s_332_11 s_332_13
        let s_332_14: bool = ((s_332_11) == (s_332_13));
        // N s_332_15: branch s_332_14 b404 b333
        if s_332_14 {
            return block_404(state, tracer, fn_state);
        } else {
            return block_333(state, tracer, fn_state);
        };
    }
    fn block_333<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_333_0: const #0u : u8
        let s_333_0: bool = false;
        // D s_333_1: write-var gs#428783 <= s_333_0
        fn_state.gs_428783 = s_333_0;
        // N s_333_2: jump b334
        return block_334(state, tracer, fn_state);
    }
    fn block_334<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_334_0: read-var gs#428783:u8
        let s_334_0: bool = fn_state.gs_428783;
        // D s_334_1: not s_334_0
        let s_334_1: bool = !s_334_0;
        // N s_334_2: branch s_334_1 b336 b335
        if s_334_1 {
            return block_336(state, tracer, fn_state);
        } else {
            return block_335(state, tracer, fn_state);
        };
    }
    fn block_335<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_335_0: const #3276s : i
        let s_335_0: i128 = 3276;
        // C s_335_1: const #14696u : u32
        let s_335_1: u32 = 14696;
        // N s_335_2: write-reg s_335_1 <= s_335_0
        let s_335_2: () = {
            state.write_register::<i128>(s_335_1 as isize, s_335_0);
            tracer.write_register(s_335_1 as isize, s_335_0);
        };
        // C s_335_3: const #3s : i
        let s_335_3: i128 = 3;
        // C s_335_4: const #3s : i
        let s_335_4: i128 = 3;
        // D s_335_5: read-var u#38474:u16
        let s_335_5: u16 = fn_state.u_38474;
        // D s_335_6: cast zx s_335_5 -> bv
        let s_335_6: Bits = Bits::new(s_335_5 as u128, 16u16);
        // D s_335_7: bit-extract s_335_6 s_335_3 s_335_4
        let s_335_7: Bits = (Bits::new(
            ((s_335_6) >> (s_335_3)).value(),
            u16::try_from(s_335_4).unwrap(),
        ));
        // D s_335_8: cast reint s_335_7 -> u8
        let s_335_8: u8 = (s_335_7.value() as u8);
        // C s_335_9: const #0s : i
        let s_335_9: i128 = 0;
        // C s_335_10: const #3s : i
        let s_335_10: i128 = 3;
        // D s_335_11: read-var u#38474:u16
        let s_335_11: u16 = fn_state.u_38474;
        // D s_335_12: cast zx s_335_11 -> bv
        let s_335_12: Bits = Bits::new(s_335_11 as u128, 16u16);
        // D s_335_13: bit-extract s_335_12 s_335_9 s_335_10
        let s_335_13: Bits = (Bits::new(
            ((s_335_12) >> (s_335_9)).value(),
            u16::try_from(s_335_10).unwrap(),
        ));
        // D s_335_14: cast reint s_335_13 -> u8
        let s_335_14: u8 = (s_335_13.value() as u8);
        // D s_335_15: call decode_aarch32_instrs_TST_r_T1enc_A_txt(s_335_8, s_335_14)
        let s_335_15: () = decode_aarch32_instrs_TST_r_T1enc_A_txt(
            state,
            tracer,
            s_335_8,
            s_335_14,
        );
        // N s_335_16: return
        return;
    }
    fn block_336<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_336_0: read-var merge#var.1:struct
        let s_336_0: u16 = fn_state.merge_var._1;
        // D s_336_1: write-var u#38478 <= s_336_0
        fn_state.u_38478 = s_336_0;
        // C s_336_2: const #8s : i
        let s_336_2: i128 = 8;
        // D s_336_3: read-var u#38478:u16
        let s_336_3: u16 = fn_state.u_38478;
        // D s_336_4: cast zx s_336_3 -> bv
        let s_336_4: Bits = Bits::new(s_336_3 as u128, 16u16);
        // C s_336_5: const #1s : i64
        let s_336_5: i64 = 1;
        // C s_336_6: cast zx s_336_5 -> i
        let s_336_6: i128 = (i128::try_from(s_336_5).unwrap());
        // C s_336_7: const #7s : i
        let s_336_7: i128 = 7;
        // C s_336_8: add s_336_7 s_336_6
        let s_336_8: i128 = (s_336_7 + s_336_6);
        // D s_336_9: bit-extract s_336_4 s_336_2 s_336_8
        let s_336_9: Bits = (Bits::new(
            ((s_336_4) >> (s_336_2)).value(),
            u16::try_from(s_336_8).unwrap(),
        ));
        // D s_336_10: cast reint s_336_9 -> u8
        let s_336_10: u8 = (s_336_9.value() as u8);
        // D s_336_11: cast zx s_336_10 -> bv
        let s_336_11: Bits = Bits::new(s_336_10 as u128, 8u16);
        // C s_336_12: const #222u : u8
        let s_336_12: u8 = 222;
        // C s_336_13: cast zx s_336_12 -> bv
        let s_336_13: Bits = Bits::new(s_336_12 as u128, 8u16);
        // D s_336_14: cmp-eq s_336_11 s_336_13
        let s_336_14: bool = ((s_336_11) == (s_336_13));
        // N s_336_15: branch s_336_14 b403 b337
        if s_336_14 {
            return block_403(state, tracer, fn_state);
        } else {
            return block_337(state, tracer, fn_state);
        };
    }
    fn block_337<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_337_0: const #0u : u8
        let s_337_0: bool = false;
        // D s_337_1: write-var gs#428794 <= s_337_0
        fn_state.gs_428794 = s_337_0;
        // N s_337_2: jump b338
        return block_338(state, tracer, fn_state);
    }
    fn block_338<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_338_0: read-var gs#428794:u8
        let s_338_0: bool = fn_state.gs_428794;
        // D s_338_1: not s_338_0
        let s_338_1: bool = !s_338_0;
        // N s_338_2: branch s_338_1 b340 b339
        if s_338_1 {
            return block_340(state, tracer, fn_state);
        } else {
            return block_339(state, tracer, fn_state);
        };
    }
    fn block_339<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_339_0: const #3288s : i
        let s_339_0: i128 = 3288;
        // C s_339_1: const #14696u : u32
        let s_339_1: u32 = 14696;
        // N s_339_2: write-reg s_339_1 <= s_339_0
        let s_339_2: () = {
            state.write_register::<i128>(s_339_1 as isize, s_339_0);
            tracer.write_register(s_339_1 as isize, s_339_0);
        };
        // C s_339_3: const #0s : i
        let s_339_3: i128 = 0;
        // C s_339_4: const #8s : i
        let s_339_4: i128 = 8;
        // D s_339_5: read-var u#38478:u16
        let s_339_5: u16 = fn_state.u_38478;
        // D s_339_6: cast zx s_339_5 -> bv
        let s_339_6: Bits = Bits::new(s_339_5 as u128, 16u16);
        // D s_339_7: bit-extract s_339_6 s_339_3 s_339_4
        let s_339_7: Bits = (Bits::new(
            ((s_339_6) >> (s_339_3)).value(),
            u16::try_from(s_339_4).unwrap(),
        ));
        // D s_339_8: cast reint s_339_7 -> u8
        let s_339_8: u8 = (s_339_7.value() as u8);
        // D s_339_9: call decode_aarch32_instrs_UDF_T1enc_A_txt(s_339_8)
        let s_339_9: () = decode_aarch32_instrs_UDF_T1enc_A_txt(state, tracer, s_339_8);
        // N s_339_10: return
        return;
    }
    fn block_340<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_340_0: read-var merge#var.1:struct
        let s_340_0: u16 = fn_state.merge_var._1;
        // D s_340_1: write-var u#38481 <= s_340_0
        fn_state.u_38481 = s_340_0;
        // C s_340_2: const #6s : i
        let s_340_2: i128 = 6;
        // D s_340_3: read-var u#38481:u16
        let s_340_3: u16 = fn_state.u_38481;
        // D s_340_4: cast zx s_340_3 -> bv
        let s_340_4: Bits = Bits::new(s_340_3 as u128, 16u16);
        // C s_340_5: const #1s : i64
        let s_340_5: i64 = 1;
        // C s_340_6: cast zx s_340_5 -> i
        let s_340_6: i128 = (i128::try_from(s_340_5).unwrap());
        // C s_340_7: const #9s : i
        let s_340_7: i128 = 9;
        // C s_340_8: add s_340_7 s_340_6
        let s_340_8: i128 = (s_340_7 + s_340_6);
        // D s_340_9: bit-extract s_340_4 s_340_2 s_340_8
        let s_340_9: Bits = (Bits::new(
            ((s_340_4) >> (s_340_2)).value(),
            u16::try_from(s_340_8).unwrap(),
        ));
        // D s_340_10: cast reint s_340_9 -> u10
        let s_340_10: u16 = (s_340_9.value() as u16);
        // D s_340_11: cast zx s_340_10 -> bv
        let s_340_11: Bits = Bits::new(s_340_10 as u128, 10u16);
        // C s_340_12: const #715u : u10
        let s_340_12: u16 = 715;
        // C s_340_13: cast zx s_340_12 -> bv
        let s_340_13: Bits = Bits::new(s_340_12 as u128, 10u16);
        // D s_340_14: cmp-eq s_340_11 s_340_13
        let s_340_14: bool = ((s_340_11) == (s_340_13));
        // N s_340_15: branch s_340_14 b402 b341
        if s_340_14 {
            return block_402(state, tracer, fn_state);
        } else {
            return block_341(state, tracer, fn_state);
        };
    }
    fn block_341<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_341_0: const #0u : u8
        let s_341_0: bool = false;
        // D s_341_1: write-var gs#428803 <= s_341_0
        fn_state.gs_428803 = s_341_0;
        // N s_341_2: jump b342
        return block_342(state, tracer, fn_state);
    }
    fn block_342<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_342_0: read-var gs#428803:u8
        let s_342_0: bool = fn_state.gs_428803;
        // D s_342_1: not s_342_0
        let s_342_1: bool = !s_342_0;
        // N s_342_2: branch s_342_1 b344 b343
        if s_342_1 {
            return block_344(state, tracer, fn_state);
        } else {
            return block_343(state, tracer, fn_state);
        };
    }
    fn block_343<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_343_0: const #3345s : i
        let s_343_0: i128 = 3345;
        // C s_343_1: const #14696u : u32
        let s_343_1: u32 = 14696;
        // N s_343_2: write-reg s_343_1 <= s_343_0
        let s_343_2: () = {
            state.write_register::<i128>(s_343_1 as isize, s_343_0);
            tracer.write_register(s_343_1 as isize, s_343_0);
        };
        // C s_343_3: const #3s : i
        let s_343_3: i128 = 3;
        // C s_343_4: const #3s : i
        let s_343_4: i128 = 3;
        // D s_343_5: read-var u#38481:u16
        let s_343_5: u16 = fn_state.u_38481;
        // D s_343_6: cast zx s_343_5 -> bv
        let s_343_6: Bits = Bits::new(s_343_5 as u128, 16u16);
        // D s_343_7: bit-extract s_343_6 s_343_3 s_343_4
        let s_343_7: Bits = (Bits::new(
            ((s_343_6) >> (s_343_3)).value(),
            u16::try_from(s_343_4).unwrap(),
        ));
        // D s_343_8: cast reint s_343_7 -> u8
        let s_343_8: u8 = (s_343_7.value() as u8);
        // C s_343_9: const #0s : i
        let s_343_9: i128 = 0;
        // C s_343_10: const #3s : i
        let s_343_10: i128 = 3;
        // D s_343_11: read-var u#38481:u16
        let s_343_11: u16 = fn_state.u_38481;
        // D s_343_12: cast zx s_343_11 -> bv
        let s_343_12: Bits = Bits::new(s_343_11 as u128, 16u16);
        // D s_343_13: bit-extract s_343_12 s_343_9 s_343_10
        let s_343_13: Bits = (Bits::new(
            ((s_343_12) >> (s_343_9)).value(),
            u16::try_from(s_343_10).unwrap(),
        ));
        // D s_343_14: cast reint s_343_13 -> u8
        let s_343_14: u8 = (s_343_13.value() as u8);
        // D s_343_15: call decode_aarch32_instrs_UXTB_T1enc_A_txt(s_343_8, s_343_14)
        let s_343_15: () = decode_aarch32_instrs_UXTB_T1enc_A_txt(
            state,
            tracer,
            s_343_8,
            s_343_14,
        );
        // N s_343_16: return
        return;
    }
    fn block_344<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_344_0: read-var merge#var.1:struct
        let s_344_0: u16 = fn_state.merge_var._1;
        // D s_344_1: write-var u#38485 <= s_344_0
        fn_state.u_38485 = s_344_0;
        // C s_344_2: const #6s : i
        let s_344_2: i128 = 6;
        // D s_344_3: read-var u#38485:u16
        let s_344_3: u16 = fn_state.u_38485;
        // D s_344_4: cast zx s_344_3 -> bv
        let s_344_4: Bits = Bits::new(s_344_3 as u128, 16u16);
        // C s_344_5: const #1s : i64
        let s_344_5: i64 = 1;
        // C s_344_6: cast zx s_344_5 -> i
        let s_344_6: i128 = (i128::try_from(s_344_5).unwrap());
        // C s_344_7: const #9s : i
        let s_344_7: i128 = 9;
        // C s_344_8: add s_344_7 s_344_6
        let s_344_8: i128 = (s_344_7 + s_344_6);
        // D s_344_9: bit-extract s_344_4 s_344_2 s_344_8
        let s_344_9: Bits = (Bits::new(
            ((s_344_4) >> (s_344_2)).value(),
            u16::try_from(s_344_8).unwrap(),
        ));
        // D s_344_10: cast reint s_344_9 -> u10
        let s_344_10: u16 = (s_344_9.value() as u16);
        // D s_344_11: cast zx s_344_10 -> bv
        let s_344_11: Bits = Bits::new(s_344_10 as u128, 10u16);
        // C s_344_12: const #714u : u10
        let s_344_12: u16 = 714;
        // C s_344_13: cast zx s_344_12 -> bv
        let s_344_13: Bits = Bits::new(s_344_12 as u128, 10u16);
        // D s_344_14: cmp-eq s_344_11 s_344_13
        let s_344_14: bool = ((s_344_11) == (s_344_13));
        // N s_344_15: branch s_344_14 b401 b345
        if s_344_14 {
            return block_401(state, tracer, fn_state);
        } else {
            return block_345(state, tracer, fn_state);
        };
    }
    fn block_345<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_345_0: const #0u : u8
        let s_345_0: bool = false;
        // D s_345_1: write-var gs#428814 <= s_345_0
        fn_state.gs_428814 = s_345_0;
        // N s_345_2: jump b346
        return block_346(state, tracer, fn_state);
    }
    fn block_346<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_346_0: read-var gs#428814:u8
        let s_346_0: bool = fn_state.gs_428814;
        // D s_346_1: not s_346_0
        let s_346_1: bool = !s_346_0;
        // N s_346_2: branch s_346_1 b348 b347
        if s_346_1 {
            return block_348(state, tracer, fn_state);
        } else {
            return block_347(state, tracer, fn_state);
        };
    }
    fn block_347<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_347_0: const #3348s : i
        let s_347_0: i128 = 3348;
        // C s_347_1: const #14696u : u32
        let s_347_1: u32 = 14696;
        // N s_347_2: write-reg s_347_1 <= s_347_0
        let s_347_2: () = {
            state.write_register::<i128>(s_347_1 as isize, s_347_0);
            tracer.write_register(s_347_1 as isize, s_347_0);
        };
        // C s_347_3: const #3s : i
        let s_347_3: i128 = 3;
        // C s_347_4: const #3s : i
        let s_347_4: i128 = 3;
        // D s_347_5: read-var u#38485:u16
        let s_347_5: u16 = fn_state.u_38485;
        // D s_347_6: cast zx s_347_5 -> bv
        let s_347_6: Bits = Bits::new(s_347_5 as u128, 16u16);
        // D s_347_7: bit-extract s_347_6 s_347_3 s_347_4
        let s_347_7: Bits = (Bits::new(
            ((s_347_6) >> (s_347_3)).value(),
            u16::try_from(s_347_4).unwrap(),
        ));
        // D s_347_8: cast reint s_347_7 -> u8
        let s_347_8: u8 = (s_347_7.value() as u8);
        // C s_347_9: const #0s : i
        let s_347_9: i128 = 0;
        // C s_347_10: const #3s : i
        let s_347_10: i128 = 3;
        // D s_347_11: read-var u#38485:u16
        let s_347_11: u16 = fn_state.u_38485;
        // D s_347_12: cast zx s_347_11 -> bv
        let s_347_12: Bits = Bits::new(s_347_11 as u128, 16u16);
        // D s_347_13: bit-extract s_347_12 s_347_9 s_347_10
        let s_347_13: Bits = (Bits::new(
            ((s_347_12) >> (s_347_9)).value(),
            u16::try_from(s_347_10).unwrap(),
        ));
        // D s_347_14: cast reint s_347_13 -> u8
        let s_347_14: u8 = (s_347_13.value() as u8);
        // D s_347_15: call decode_aarch32_instrs_UXTH_T1enc_A_txt(s_347_8, s_347_14)
        let s_347_15: () = decode_aarch32_instrs_UXTH_T1enc_A_txt(
            state,
            tracer,
            s_347_8,
            s_347_14,
        );
        // N s_347_16: return
        return;
    }
    fn block_348<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_348_0: read-var merge#var.1:struct
        let s_348_0: u16 = fn_state.merge_var._1;
        // D s_348_1: cast zx s_348_0 -> bv
        let s_348_1: Bits = Bits::new(s_348_0 as u128, 16u16);
        // C s_348_2: const #48928u : u16
        let s_348_2: u16 = 48928;
        // C s_348_3: cast zx s_348_2 -> bv
        let s_348_3: Bits = Bits::new(s_348_2 as u128, 16u16);
        // D s_348_4: cmp-eq s_348_1 s_348_3
        let s_348_4: bool = ((s_348_1) == (s_348_3));
        // N s_348_5: branch s_348_4 b400 b349
        if s_348_4 {
            return block_400(state, tracer, fn_state);
        } else {
            return block_349(state, tracer, fn_state);
        };
    }
    fn block_349<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_349_0: const #0u : u8
        let s_349_0: bool = false;
        // D s_349_1: write-var gs#428823 <= s_349_0
        fn_state.gs_428823 = s_349_0;
        // N s_349_2: jump b350
        return block_350(state, tracer, fn_state);
    }
    fn block_350<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_350_0: read-var gs#428823:u8
        let s_350_0: bool = fn_state.gs_428823;
        // D s_350_1: not s_350_0
        let s_350_1: bool = !s_350_0;
        // N s_350_2: branch s_350_1 b352 b351
        if s_350_1 {
            return block_352(state, tracer, fn_state);
        } else {
            return block_351(state, tracer, fn_state);
        };
    }
    fn block_351<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_351_0: const #3761s : i
        let s_351_0: i128 = 3761;
        // C s_351_1: const #14696u : u32
        let s_351_1: u32 = 14696;
        // N s_351_2: write-reg s_351_1 <= s_351_0
        let s_351_2: () = {
            state.write_register::<i128>(s_351_1 as isize, s_351_0);
            tracer.write_register(s_351_1 as isize, s_351_0);
        };
        // C s_351_3: const #() : ()
        let s_351_3: () = ();
        // S s_351_4: call decode_aarch32_instrs_WFE_T1enc_A_txt(s_351_3)
        let s_351_4: () = decode_aarch32_instrs_WFE_T1enc_A_txt(state, tracer, s_351_3);
        // N s_351_5: return
        return;
    }
    fn block_352<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_352_0: read-var merge#var.1:struct
        let s_352_0: u16 = fn_state.merge_var._1;
        // D s_352_1: cast zx s_352_0 -> bv
        let s_352_1: Bits = Bits::new(s_352_0 as u128, 16u16);
        // C s_352_2: const #48944u : u16
        let s_352_2: u16 = 48944;
        // C s_352_3: cast zx s_352_2 -> bv
        let s_352_3: Bits = Bits::new(s_352_2 as u128, 16u16);
        // D s_352_4: cmp-eq s_352_1 s_352_3
        let s_352_4: bool = ((s_352_1) == (s_352_3));
        // N s_352_5: branch s_352_4 b399 b353
        if s_352_4 {
            return block_399(state, tracer, fn_state);
        } else {
            return block_353(state, tracer, fn_state);
        };
    }
    fn block_353<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_353_0: const #0u : u8
        let s_353_0: bool = false;
        // D s_353_1: write-var gs#428828 <= s_353_0
        fn_state.gs_428828 = s_353_0;
        // N s_353_2: jump b354
        return block_354(state, tracer, fn_state);
    }
    fn block_354<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_354_0: read-var gs#428828:u8
        let s_354_0: bool = fn_state.gs_428828;
        // D s_354_1: not s_354_0
        let s_354_1: bool = !s_354_0;
        // N s_354_2: branch s_354_1 b356 b355
        if s_354_1 {
            return block_356(state, tracer, fn_state);
        } else {
            return block_355(state, tracer, fn_state);
        };
    }
    fn block_355<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_355_0: const #3764s : i
        let s_355_0: i128 = 3764;
        // C s_355_1: const #14696u : u32
        let s_355_1: u32 = 14696;
        // N s_355_2: write-reg s_355_1 <= s_355_0
        let s_355_2: () = {
            state.write_register::<i128>(s_355_1 as isize, s_355_0);
            tracer.write_register(s_355_1 as isize, s_355_0);
        };
        // C s_355_3: const #() : ()
        let s_355_3: () = ();
        // S s_355_4: call decode_aarch32_instrs_WFI_T1enc_A_txt(s_355_3)
        let s_355_4: () = decode_aarch32_instrs_WFI_T1enc_A_txt(state, tracer, s_355_3);
        // N s_355_5: return
        return;
    }
    fn block_356<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_356_0: read-var merge#var.1:struct
        let s_356_0: u16 = fn_state.merge_var._1;
        // D s_356_1: cast zx s_356_0 -> bv
        let s_356_1: Bits = Bits::new(s_356_0 as u128, 16u16);
        // C s_356_2: const #48912u : u16
        let s_356_2: u16 = 48912;
        // C s_356_3: cast zx s_356_2 -> bv
        let s_356_3: Bits = Bits::new(s_356_2 as u128, 16u16);
        // D s_356_4: cmp-eq s_356_1 s_356_3
        let s_356_4: bool = ((s_356_1) == (s_356_3));
        // N s_356_5: branch s_356_4 b398 b357
        if s_356_4 {
            return block_398(state, tracer, fn_state);
        } else {
            return block_357(state, tracer, fn_state);
        };
    }
    fn block_357<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_357_0: const #0u : u8
        let s_357_0: bool = false;
        // D s_357_1: write-var gs#428833 <= s_357_0
        fn_state.gs_428833 = s_357_0;
        // N s_357_2: jump b358
        return block_358(state, tracer, fn_state);
    }
    fn block_358<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_358_0: read-var gs#428833:u8
        let s_358_0: bool = fn_state.gs_428833;
        // D s_358_1: not s_358_0
        let s_358_1: bool = !s_358_0;
        // N s_358_2: branch s_358_1 b360 b359
        if s_358_1 {
            return block_360(state, tracer, fn_state);
        } else {
            return block_359(state, tracer, fn_state);
        };
    }
    fn block_359<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_359_0: const #3767s : i
        let s_359_0: i128 = 3767;
        // C s_359_1: const #14696u : u32
        let s_359_1: u32 = 14696;
        // N s_359_2: write-reg s_359_1 <= s_359_0
        let s_359_2: () = {
            state.write_register::<i128>(s_359_1 as isize, s_359_0);
            tracer.write_register(s_359_1 as isize, s_359_0);
        };
        // C s_359_3: const #() : ()
        let s_359_3: () = ();
        // S s_359_4: call decode_aarch32_instrs_YIELD_T1enc_A_txt(s_359_3)
        let s_359_4: () = decode_aarch32_instrs_YIELD_T1enc_A_txt(
            state,
            tracer,
            s_359_3,
        );
        // N s_359_5: return
        return;
    }
    fn block_360<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_360_0: read-var merge#var.1:struct
        let s_360_0: u16 = fn_state.merge_var._1;
        // D s_360_1: write-var u#38495 <= s_360_0
        fn_state.u_38495 = s_360_0;
        // C s_360_2: const #5s : i
        let s_360_2: i128 = 5;
        // D s_360_3: read-var u#38495:u16
        let s_360_3: u16 = fn_state.u_38495;
        // D s_360_4: cast zx s_360_3 -> bv
        let s_360_4: Bits = Bits::new(s_360_3 as u128, 16u16);
        // C s_360_5: const #1s : i64
        let s_360_5: i64 = 1;
        // C s_360_6: cast zx s_360_5 -> i
        let s_360_6: i128 = (i128::try_from(s_360_5).unwrap());
        // C s_360_7: const #10s : i
        let s_360_7: i128 = 10;
        // C s_360_8: add s_360_7 s_360_6
        let s_360_8: i128 = (s_360_7 + s_360_6);
        // D s_360_9: bit-extract s_360_4 s_360_2 s_360_8
        let s_360_9: Bits = (Bits::new(
            ((s_360_4) >> (s_360_2)).value(),
            u16::try_from(s_360_8).unwrap(),
        ));
        // D s_360_10: cast reint s_360_9 -> u11
        let s_360_10: u16 = (s_360_9.value() as u16);
        // D s_360_11: cast zx s_360_10 -> bv
        let s_360_11: Bits = Bits::new(s_360_10 as u128, 11u16);
        // C s_360_12: const #1459u : u11
        let s_360_12: u16 = 1459;
        // C s_360_13: cast zx s_360_12 -> bv
        let s_360_13: Bits = Bits::new(s_360_12 as u128, 11u16);
        // D s_360_14: cmp-eq s_360_11 s_360_13
        let s_360_14: bool = ((s_360_11) == (s_360_13));
        // N s_360_15: branch s_360_14 b397 b361
        if s_360_14 {
            return block_397(state, tracer, fn_state);
        } else {
            return block_361(state, tracer, fn_state);
        };
    }
    fn block_361<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_361_0: const #0u : u8
        let s_361_0: bool = false;
        // D s_361_1: write-var gs#428841 <= s_361_0
        fn_state.gs_428841 = s_361_0;
        // N s_361_2: jump b362
        return block_362(state, tracer, fn_state);
    }
    fn block_362<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_362_0: read-var gs#428841:u8
        let s_362_0: bool = fn_state.gs_428841;
        // N s_362_1: branch s_362_0 b396 b363
        if s_362_0 {
            return block_396(state, tracer, fn_state);
        } else {
            return block_363(state, tracer, fn_state);
        };
    }
    fn block_363<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_363_0: const #0u : u8
        let s_363_0: bool = false;
        // D s_363_1: write-var gs#428843 <= s_363_0
        fn_state.gs_428843 = s_363_0;
        // N s_363_2: jump b364
        return block_364(state, tracer, fn_state);
    }
    fn block_364<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_364_0: read-var gs#428843:u8
        let s_364_0: bool = fn_state.gs_428843;
        // D s_364_1: not s_364_0
        let s_364_1: bool = !s_364_0;
        // N s_364_2: branch s_364_1 b366 b365
        if s_364_1 {
            return block_366(state, tracer, fn_state);
        } else {
            return block_365(state, tracer, fn_state);
        };
    }
    fn block_365<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_365_0: const #3770s : i
        let s_365_0: i128 = 3770;
        // C s_365_1: const #14696u : u32
        let s_365_1: u32 = 14696;
        // N s_365_2: write-reg s_365_1 <= s_365_0
        let s_365_2: () = {
            state.write_register::<i128>(s_365_1 as isize, s_365_0);
            tracer.write_register(s_365_1 as isize, s_365_0);
        };
        // C s_365_3: const #4s : i
        let s_365_3: i128 = 4;
        // C s_365_4: const #1s : i
        let s_365_4: i128 = 1;
        // D s_365_5: read-var u#38495:u16
        let s_365_5: u16 = fn_state.u_38495;
        // D s_365_6: cast zx s_365_5 -> bv
        let s_365_6: Bits = Bits::new(s_365_5 as u128, 16u16);
        // D s_365_7: bit-extract s_365_6 s_365_3 s_365_4
        let s_365_7: Bits = (Bits::new(
            ((s_365_6) >> (s_365_3)).value(),
            u16::try_from(s_365_4).unwrap(),
        ));
        // D s_365_8: cast reint s_365_7 -> u8
        let s_365_8: bool = ((s_365_7.value()) != 0);
        // C s_365_9: const #2s : i
        let s_365_9: i128 = 2;
        // C s_365_10: const #1s : i
        let s_365_10: i128 = 1;
        // D s_365_11: read-var u#38495:u16
        let s_365_11: u16 = fn_state.u_38495;
        // D s_365_12: cast zx s_365_11 -> bv
        let s_365_12: Bits = Bits::new(s_365_11 as u128, 16u16);
        // D s_365_13: bit-extract s_365_12 s_365_9 s_365_10
        let s_365_13: Bits = (Bits::new(
            ((s_365_12) >> (s_365_9)).value(),
            u16::try_from(s_365_10).unwrap(),
        ));
        // D s_365_14: cast reint s_365_13 -> u8
        let s_365_14: bool = ((s_365_13.value()) != 0);
        // C s_365_15: const #1s : i
        let s_365_15: i128 = 1;
        // C s_365_16: const #1s : i
        let s_365_16: i128 = 1;
        // D s_365_17: read-var u#38495:u16
        let s_365_17: u16 = fn_state.u_38495;
        // D s_365_18: cast zx s_365_17 -> bv
        let s_365_18: Bits = Bits::new(s_365_17 as u128, 16u16);
        // D s_365_19: bit-extract s_365_18 s_365_15 s_365_16
        let s_365_19: Bits = (Bits::new(
            ((s_365_18) >> (s_365_15)).value(),
            u16::try_from(s_365_16).unwrap(),
        ));
        // D s_365_20: cast reint s_365_19 -> u8
        let s_365_20: bool = ((s_365_19.value()) != 0);
        // C s_365_21: const #0s : i
        let s_365_21: i128 = 0;
        // C s_365_22: const #1s : i
        let s_365_22: i128 = 1;
        // D s_365_23: read-var u#38495:u16
        let s_365_23: u16 = fn_state.u_38495;
        // D s_365_24: cast zx s_365_23 -> bv
        let s_365_24: Bits = Bits::new(s_365_23 as u128, 16u16);
        // D s_365_25: bit-extract s_365_24 s_365_21 s_365_22
        let s_365_25: Bits = (Bits::new(
            ((s_365_24) >> (s_365_21)).value(),
            u16::try_from(s_365_22).unwrap(),
        ));
        // D s_365_26: cast reint s_365_25 -> u8
        let s_365_26: bool = ((s_365_25.value()) != 0);
        // D s_365_27: call decode_aarch32_instrs_CPS_T1enc_AS_txt(s_365_8, s_365_14, s_365_20, s_365_26)
        let s_365_27: () = decode_aarch32_instrs_CPS_T1enc_AS_txt(
            state,
            tracer,
            s_365_8,
            s_365_14,
            s_365_20,
            s_365_26,
        );
        // N s_365_28: return
        return;
    }
    fn block_366<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_366_0: read-var merge#var.1:struct
        let s_366_0: u16 = fn_state.merge_var._1;
        // D s_366_1: write-var u#38497 <= s_366_0
        fn_state.u_38497 = s_366_0;
        // C s_366_2: const #6s : i
        let s_366_2: i128 = 6;
        // D s_366_3: read-var u#38497:u16
        let s_366_3: u16 = fn_state.u_38497;
        // D s_366_4: cast zx s_366_3 -> bv
        let s_366_4: Bits = Bits::new(s_366_3 as u128, 16u16);
        // C s_366_5: const #1s : i64
        let s_366_5: i64 = 1;
        // C s_366_6: cast zx s_366_5 -> i
        let s_366_6: i128 = (i128::try_from(s_366_5).unwrap());
        // C s_366_7: const #9s : i
        let s_366_7: i128 = 9;
        // C s_366_8: add s_366_7 s_366_6
        let s_366_8: i128 = (s_366_7 + s_366_6);
        // D s_366_9: bit-extract s_366_4 s_366_2 s_366_8
        let s_366_9: Bits = (Bits::new(
            ((s_366_4) >> (s_366_2)).value(),
            u16::try_from(s_366_8).unwrap(),
        ));
        // D s_366_10: cast reint s_366_9 -> u10
        let s_366_10: u16 = (s_366_9.value() as u16);
        // D s_366_11: cast zx s_366_10 -> bv
        let s_366_11: Bits = Bits::new(s_366_10 as u128, 10u16);
        // C s_366_12: const #746u : u10
        let s_366_12: u16 = 746;
        // C s_366_13: cast zx s_366_12 -> bv
        let s_366_13: Bits = Bits::new(s_366_12 as u128, 10u16);
        // D s_366_14: cmp-eq s_366_11 s_366_13
        let s_366_14: bool = ((s_366_11) == (s_366_13));
        // N s_366_15: branch s_366_14 b395 b367
        if s_366_14 {
            return block_395(state, tracer, fn_state);
        } else {
            return block_367(state, tracer, fn_state);
        };
    }
    fn block_367<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_367_0: const #0u : u8
        let s_367_0: bool = false;
        // D s_367_1: write-var gs#428858 <= s_367_0
        fn_state.gs_428858 = s_367_0;
        // N s_367_2: jump b368
        return block_368(state, tracer, fn_state);
    }
    fn block_368<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_368_0: read-var gs#428858:u8
        let s_368_0: bool = fn_state.gs_428858;
        // D s_368_1: not s_368_0
        let s_368_1: bool = !s_368_0;
        // N s_368_2: branch s_368_1 b370 b369
        if s_368_1 {
            return block_370(state, tracer, fn_state);
        } else {
            return block_369(state, tracer, fn_state);
        };
    }
    fn block_369<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_369_0: const #3814s : i
        let s_369_0: i128 = 3814;
        // C s_369_1: const #14696u : u32
        let s_369_1: u32 = 14696;
        // N s_369_2: write-reg s_369_1 <= s_369_0
        let s_369_2: () = {
            state.write_register::<i128>(s_369_1 as isize, s_369_0);
            tracer.write_register(s_369_1 as isize, s_369_0);
        };
        // C s_369_3: const #0s : i
        let s_369_3: i128 = 0;
        // C s_369_4: const #6s : i
        let s_369_4: i128 = 6;
        // D s_369_5: read-var u#38497:u16
        let s_369_5: u16 = fn_state.u_38497;
        // D s_369_6: cast zx s_369_5 -> bv
        let s_369_6: Bits = Bits::new(s_369_5 as u128, 16u16);
        // D s_369_7: bit-extract s_369_6 s_369_3 s_369_4
        let s_369_7: Bits = (Bits::new(
            ((s_369_6) >> (s_369_3)).value(),
            u16::try_from(s_369_4).unwrap(),
        ));
        // D s_369_8: cast reint s_369_7 -> u8
        let s_369_8: u8 = (s_369_7.value() as u8);
        // D s_369_9: call decode_aarch32_instrs_HLT_T1enc_A_txt(s_369_8)
        let s_369_9: () = decode_aarch32_instrs_HLT_T1enc_A_txt(state, tracer, s_369_8);
        // N s_369_10: return
        return;
    }
    fn block_370<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_370_0: read-var merge#var.1:struct
        let s_370_0: u16 = fn_state.merge_var._1;
        // D s_370_1: cast zx s_370_0 -> bv
        let s_370_1: Bits = Bits::new(s_370_0 as u128, 16u16);
        // C s_370_2: const #48976u : u16
        let s_370_2: u16 = 48976;
        // C s_370_3: cast zx s_370_2 -> bv
        let s_370_3: Bits = Bits::new(s_370_2 as u128, 16u16);
        // D s_370_4: cmp-eq s_370_1 s_370_3
        let s_370_4: bool = ((s_370_1) == (s_370_3));
        // N s_370_5: branch s_370_4 b394 b371
        if s_370_4 {
            return block_394(state, tracer, fn_state);
        } else {
            return block_371(state, tracer, fn_state);
        };
    }
    fn block_371<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_371_0: const #0u : u8
        let s_371_0: bool = false;
        // D s_371_1: write-var gs#428865 <= s_371_0
        fn_state.gs_428865 = s_371_0;
        // N s_371_2: jump b372
        return block_372(state, tracer, fn_state);
    }
    fn block_372<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_372_0: read-var gs#428865:u8
        let s_372_0: bool = fn_state.gs_428865;
        // D s_372_1: not s_372_0
        let s_372_1: bool = !s_372_0;
        // N s_372_2: branch s_372_1 b374 b373
        if s_372_1 {
            return block_374(state, tracer, fn_state);
        } else {
            return block_373(state, tracer, fn_state);
        };
    }
    fn block_373<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_373_0: const #3830s : i
        let s_373_0: i128 = 3830;
        // C s_373_1: const #14696u : u32
        let s_373_1: u32 = 14696;
        // N s_373_2: write-reg s_373_1 <= s_373_0
        let s_373_2: () = {
            state.write_register::<i128>(s_373_1 as isize, s_373_0);
            tracer.write_register(s_373_1 as isize, s_373_0);
        };
        // C s_373_3: const #() : ()
        let s_373_3: () = ();
        // S s_373_4: call decode_aarch32_instrs_SEVL_T1enc_A_txt(s_373_3)
        let s_373_4: () = decode_aarch32_instrs_SEVL_T1enc_A_txt(state, tracer, s_373_3);
        // N s_373_5: return
        return;
    }
    fn block_374<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_374_0: read-var merge#var.1:struct
        let s_374_0: u16 = fn_state.merge_var._1;
        // D s_374_1: write-var u#38501 <= s_374_0
        fn_state.u_38501 = s_374_0;
        // C s_374_2: const #4s : i
        let s_374_2: i128 = 4;
        // D s_374_3: read-var u#38501:u16
        let s_374_3: u16 = fn_state.u_38501;
        // D s_374_4: cast zx s_374_3 -> bv
        let s_374_4: Bits = Bits::new(s_374_3 as u128, 16u16);
        // C s_374_5: const #1s : i64
        let s_374_5: i64 = 1;
        // C s_374_6: cast zx s_374_5 -> i
        let s_374_6: i128 = (i128::try_from(s_374_5).unwrap());
        // C s_374_7: const #11s : i
        let s_374_7: i128 = 11;
        // C s_374_8: add s_374_7 s_374_6
        let s_374_8: i128 = (s_374_7 + s_374_6);
        // D s_374_9: bit-extract s_374_4 s_374_2 s_374_8
        let s_374_9: Bits = (Bits::new(
            ((s_374_4) >> (s_374_2)).value(),
            u16::try_from(s_374_8).unwrap(),
        ));
        // D s_374_10: cast reint s_374_9 -> u12
        let s_374_10: u16 = (s_374_9.value() as u16);
        // D s_374_11: cast zx s_374_10 -> bv
        let s_374_11: Bits = Bits::new(s_374_10 as u128, 12u16);
        // C s_374_12: const #2913u : u12
        let s_374_12: u16 = 2913;
        // C s_374_13: cast zx s_374_12 -> bv
        let s_374_13: Bits = Bits::new(s_374_12 as u128, 12u16);
        // D s_374_14: cmp-eq s_374_11 s_374_13
        let s_374_14: bool = ((s_374_11) == (s_374_13));
        // N s_374_15: branch s_374_14 b393 b375
        if s_374_14 {
            return block_393(state, tracer, fn_state);
        } else {
            return block_375(state, tracer, fn_state);
        };
    }
    fn block_375<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_375_0: const #0u : u8
        let s_375_0: bool = false;
        // D s_375_1: write-var gs#428873 <= s_375_0
        fn_state.gs_428873 = s_375_0;
        // N s_375_2: jump b376
        return block_376(state, tracer, fn_state);
    }
    fn block_376<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_376_0: read-var gs#428873:u8
        let s_376_0: bool = fn_state.gs_428873;
        // N s_376_1: branch s_376_0 b392 b377
        if s_376_0 {
            return block_392(state, tracer, fn_state);
        } else {
            return block_377(state, tracer, fn_state);
        };
    }
    fn block_377<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_377_0: const #0u : u8
        let s_377_0: bool = false;
        // D s_377_1: write-var gs#428875 <= s_377_0
        fn_state.gs_428875 = s_377_0;
        // N s_377_2: jump b378
        return block_378(state, tracer, fn_state);
    }
    fn block_378<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_378_0: read-var gs#428875:u8
        let s_378_0: bool = fn_state.gs_428875;
        // D s_378_1: not s_378_0
        let s_378_1: bool = !s_378_0;
        // N s_378_2: branch s_378_1 b391 b379
        if s_378_1 {
            return block_391(state, tracer, fn_state);
        } else {
            return block_379(state, tracer, fn_state);
        };
    }
    fn block_379<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_379_0: const #3889s : i
        let s_379_0: i128 = 3889;
        // C s_379_1: const #14696u : u32
        let s_379_1: u32 = 14696;
        // N s_379_2: write-reg s_379_1 <= s_379_0
        let s_379_2: () = {
            state.write_register::<i128>(s_379_1 as isize, s_379_0);
            tracer.write_register(s_379_1 as isize, s_379_0);
        };
        // C s_379_3: const #3s : i
        let s_379_3: i128 = 3;
        // C s_379_4: const #1s : i
        let s_379_4: i128 = 1;
        // D s_379_5: read-var u#38501:u16
        let s_379_5: u16 = fn_state.u_38501;
        // D s_379_6: cast zx s_379_5 -> bv
        let s_379_6: Bits = Bits::new(s_379_5 as u128, 16u16);
        // D s_379_7: bit-extract s_379_6 s_379_3 s_379_4
        let s_379_7: Bits = (Bits::new(
            ((s_379_6) >> (s_379_3)).value(),
            u16::try_from(s_379_4).unwrap(),
        ));
        // D s_379_8: cast reint s_379_7 -> u8
        let s_379_8: bool = ((s_379_7.value()) != 0);
        // D s_379_9: write-var imm1 <= s_379_8
        fn_state.imm1 = s_379_8;
        // C s_379_10: const #4s : i
        let s_379_10: i128 = 4;
        // D s_379_11: read-var u#38501:u16
        let s_379_11: u16 = fn_state.u_38501;
        // D s_379_12: cast zx s_379_11 -> bv
        let s_379_12: Bits = Bits::new(s_379_11 as u128, 16u16);
        // C s_379_13: const #1u : u64
        let s_379_13: u64 = 1;
        // D s_379_14: bit-extract s_379_12 s_379_10 s_379_13
        let s_379_14: Bits = (Bits::new(
            ((s_379_12) >> (s_379_10)).value(),
            u16::try_from(s_379_13).unwrap(),
        ));
        // D s_379_15: cast reint s_379_14 -> u8
        let s_379_15: bool = ((s_379_14.value()) != 0);
        // C s_379_16: const #0s : i
        let s_379_16: i128 = 0;
        // C s_379_17: const #0u : u64
        let s_379_17: u64 = 0;
        // D s_379_18: cast zx s_379_15 -> u64
        let s_379_18: u64 = (s_379_15 as u64);
        // C s_379_19: const #1u : u64
        let s_379_19: u64 = 1;
        // D s_379_20: and s_379_18 s_379_19
        let s_379_20: u64 = ((s_379_18) & (s_379_19));
        // D s_379_21: cmp-eq s_379_20 s_379_19
        let s_379_21: bool = ((s_379_20) == (s_379_19));
        // D s_379_22: lsl s_379_18 s_379_16
        let s_379_22: u64 = s_379_18 << s_379_16;
        // D s_379_23: or s_379_17 s_379_22
        let s_379_23: u64 = ((s_379_17) | (s_379_22));
        // D s_379_24: cmpl s_379_22
        let s_379_24: u64 = !s_379_22;
        // D s_379_25: and s_379_17 s_379_24
        let s_379_25: u64 = ((s_379_17) & (s_379_24));
        // D s_379_26: select s_379_21 s_379_23 s_379_25
        let s_379_26: u64 = if s_379_21 { s_379_23 } else { s_379_25 };
        // D s_379_27: cast trunc s_379_26 -> u8
        let s_379_27: bool = ((s_379_26) != 0);
        // D s_379_28: cast zx s_379_27 -> bv
        let s_379_28: Bits = Bits::new(s_379_27 as u128, 1u16);
        // C s_379_29: const #1u : u8
        let s_379_29: bool = true;
        // C s_379_30: cast zx s_379_29 -> bv
        let s_379_30: Bits = Bits::new(s_379_29 as u128, 1u16);
        // D s_379_31: cmp-ne s_379_28 s_379_30
        let s_379_31: bool = ((s_379_28) != (s_379_30));
        // N s_379_32: branch s_379_31 b390 b380
        if s_379_31 {
            return block_390(state, tracer, fn_state);
        } else {
            return block_380(state, tracer, fn_state);
        };
    }
    fn block_380<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_380_0: const #0s : i
        let s_380_0: i128 = 0;
        // D s_380_1: read-var u#38501:u16
        let s_380_1: u16 = fn_state.u_38501;
        // D s_380_2: cast zx s_380_1 -> bv
        let s_380_2: Bits = Bits::new(s_380_1 as u128, 16u16);
        // C s_380_3: const #1u : u64
        let s_380_3: u64 = 1;
        // D s_380_4: bit-extract s_380_2 s_380_0 s_380_3
        let s_380_4: Bits = (Bits::new(
            ((s_380_2) >> (s_380_0)).value(),
            u16::try_from(s_380_3).unwrap(),
        ));
        // D s_380_5: cast reint s_380_4 -> u8
        let s_380_5: bool = ((s_380_4.value()) != 0);
        // C s_380_6: const #0s : i
        let s_380_6: i128 = 0;
        // C s_380_7: const #0u : u64
        let s_380_7: u64 = 0;
        // D s_380_8: cast zx s_380_5 -> u64
        let s_380_8: u64 = (s_380_5 as u64);
        // C s_380_9: const #1u : u64
        let s_380_9: u64 = 1;
        // D s_380_10: and s_380_8 s_380_9
        let s_380_10: u64 = ((s_380_8) & (s_380_9));
        // D s_380_11: cmp-eq s_380_10 s_380_9
        let s_380_11: bool = ((s_380_10) == (s_380_9));
        // D s_380_12: lsl s_380_8 s_380_6
        let s_380_12: u64 = s_380_8 << s_380_6;
        // D s_380_13: or s_380_7 s_380_12
        let s_380_13: u64 = ((s_380_7) | (s_380_12));
        // D s_380_14: cmpl s_380_12
        let s_380_14: u64 = !s_380_12;
        // D s_380_15: and s_380_7 s_380_14
        let s_380_15: u64 = ((s_380_7) & (s_380_14));
        // D s_380_16: select s_380_11 s_380_13 s_380_15
        let s_380_16: u64 = if s_380_11 { s_380_13 } else { s_380_15 };
        // D s_380_17: cast trunc s_380_16 -> u8
        let s_380_17: bool = ((s_380_16) != 0);
        // D s_380_18: cast zx s_380_17 -> bv
        let s_380_18: Bits = Bits::new(s_380_17 as u128, 1u16);
        // C s_380_19: const #0u : u8
        let s_380_19: bool = false;
        // C s_380_20: cast zx s_380_19 -> bv
        let s_380_20: Bits = Bits::new(s_380_19 as u128, 1u16);
        // D s_380_21: cmp-ne s_380_18 s_380_20
        let s_380_21: bool = ((s_380_18) != (s_380_20));
        // D s_380_22: write-var gs#428884 <= s_380_21
        fn_state.gs_428884 = s_380_21;
        // N s_380_23: jump b381
        return block_381(state, tracer, fn_state);
    }
    fn block_381<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_381_0: read-var gs#428884:u8
        let s_381_0: bool = fn_state.gs_428884;
        // N s_381_1: branch s_381_0 b389 b382
        if s_381_0 {
            return block_389(state, tracer, fn_state);
        } else {
            return block_382(state, tracer, fn_state);
        };
    }
    fn block_382<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_382_0: const #1s : i
        let s_382_0: i128 = 1;
        // D s_382_1: read-var u#38501:u16
        let s_382_1: u16 = fn_state.u_38501;
        // D s_382_2: cast zx s_382_1 -> bv
        let s_382_2: Bits = Bits::new(s_382_1 as u128, 16u16);
        // C s_382_3: const #1u : u64
        let s_382_3: u64 = 1;
        // D s_382_4: bit-extract s_382_2 s_382_0 s_382_3
        let s_382_4: Bits = (Bits::new(
            ((s_382_2) >> (s_382_0)).value(),
            u16::try_from(s_382_3).unwrap(),
        ));
        // D s_382_5: cast reint s_382_4 -> u8
        let s_382_5: bool = ((s_382_4.value()) != 0);
        // C s_382_6: const #0s : i
        let s_382_6: i128 = 0;
        // C s_382_7: const #0u : u64
        let s_382_7: u64 = 0;
        // D s_382_8: cast zx s_382_5 -> u64
        let s_382_8: u64 = (s_382_5 as u64);
        // C s_382_9: const #1u : u64
        let s_382_9: u64 = 1;
        // D s_382_10: and s_382_8 s_382_9
        let s_382_10: u64 = ((s_382_8) & (s_382_9));
        // D s_382_11: cmp-eq s_382_10 s_382_9
        let s_382_11: bool = ((s_382_10) == (s_382_9));
        // D s_382_12: lsl s_382_8 s_382_6
        let s_382_12: u64 = s_382_8 << s_382_6;
        // D s_382_13: or s_382_7 s_382_12
        let s_382_13: u64 = ((s_382_7) | (s_382_12));
        // D s_382_14: cmpl s_382_12
        let s_382_14: u64 = !s_382_12;
        // D s_382_15: and s_382_7 s_382_14
        let s_382_15: u64 = ((s_382_7) & (s_382_14));
        // D s_382_16: select s_382_11 s_382_13 s_382_15
        let s_382_16: u64 = if s_382_11 { s_382_13 } else { s_382_15 };
        // D s_382_17: cast trunc s_382_16 -> u8
        let s_382_17: bool = ((s_382_16) != 0);
        // D s_382_18: cast zx s_382_17 -> bv
        let s_382_18: Bits = Bits::new(s_382_17 as u128, 1u16);
        // C s_382_19: const #0u : u8
        let s_382_19: bool = false;
        // C s_382_20: cast zx s_382_19 -> bv
        let s_382_20: Bits = Bits::new(s_382_19 as u128, 1u16);
        // D s_382_21: cmp-ne s_382_18 s_382_20
        let s_382_21: bool = ((s_382_18) != (s_382_20));
        // D s_382_22: write-var gs#428887 <= s_382_21
        fn_state.gs_428887 = s_382_21;
        // N s_382_23: jump b383
        return block_383(state, tracer, fn_state);
    }
    fn block_383<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_383_0: read-var gs#428887:u8
        let s_383_0: bool = fn_state.gs_428887;
        // N s_383_1: branch s_383_0 b388 b384
        if s_383_0 {
            return block_388(state, tracer, fn_state);
        } else {
            return block_384(state, tracer, fn_state);
        };
    }
    fn block_384<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_384_0: const #2s : i
        let s_384_0: i128 = 2;
        // D s_384_1: read-var u#38501:u16
        let s_384_1: u16 = fn_state.u_38501;
        // D s_384_2: cast zx s_384_1 -> bv
        let s_384_2: Bits = Bits::new(s_384_1 as u128, 16u16);
        // C s_384_3: const #1u : u64
        let s_384_3: u64 = 1;
        // D s_384_4: bit-extract s_384_2 s_384_0 s_384_3
        let s_384_4: Bits = (Bits::new(
            ((s_384_2) >> (s_384_0)).value(),
            u16::try_from(s_384_3).unwrap(),
        ));
        // D s_384_5: cast reint s_384_4 -> u8
        let s_384_5: bool = ((s_384_4.value()) != 0);
        // C s_384_6: const #0s : i
        let s_384_6: i128 = 0;
        // C s_384_7: const #0u : u64
        let s_384_7: u64 = 0;
        // D s_384_8: cast zx s_384_5 -> u64
        let s_384_8: u64 = (s_384_5 as u64);
        // C s_384_9: const #1u : u64
        let s_384_9: u64 = 1;
        // D s_384_10: and s_384_8 s_384_9
        let s_384_10: u64 = ((s_384_8) & (s_384_9));
        // D s_384_11: cmp-eq s_384_10 s_384_9
        let s_384_11: bool = ((s_384_10) == (s_384_9));
        // D s_384_12: lsl s_384_8 s_384_6
        let s_384_12: u64 = s_384_8 << s_384_6;
        // D s_384_13: or s_384_7 s_384_12
        let s_384_13: u64 = ((s_384_7) | (s_384_12));
        // D s_384_14: cmpl s_384_12
        let s_384_14: u64 = !s_384_12;
        // D s_384_15: and s_384_7 s_384_14
        let s_384_15: u64 = ((s_384_7) & (s_384_14));
        // D s_384_16: select s_384_11 s_384_13 s_384_15
        let s_384_16: u64 = if s_384_11 { s_384_13 } else { s_384_15 };
        // D s_384_17: cast trunc s_384_16 -> u8
        let s_384_17: bool = ((s_384_16) != 0);
        // D s_384_18: cast zx s_384_17 -> bv
        let s_384_18: Bits = Bits::new(s_384_17 as u128, 1u16);
        // C s_384_19: const #0u : u8
        let s_384_19: bool = false;
        // C s_384_20: cast zx s_384_19 -> bv
        let s_384_20: Bits = Bits::new(s_384_19 as u128, 1u16);
        // D s_384_21: cmp-ne s_384_18 s_384_20
        let s_384_21: bool = ((s_384_18) != (s_384_20));
        // D s_384_22: write-var gs#428890 <= s_384_21
        fn_state.gs_428890 = s_384_21;
        // N s_384_23: jump b385
        return block_385(state, tracer, fn_state);
    }
    fn block_385<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_385_0: read-var gs#428890:u8
        let s_385_0: bool = fn_state.gs_428890;
        // N s_385_1: branch s_385_0 b387 b386
        if s_385_0 {
            return block_387(state, tracer, fn_state);
        } else {
            return block_386(state, tracer, fn_state);
        };
    }
    fn block_386<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_386_0: read-var imm1:u8
        let s_386_0: bool = fn_state.imm1;
        // D s_386_1: call decode_aarch32_instrs_SETPAN_T1enc_A_txt(s_386_0)
        let s_386_1: () = decode_aarch32_instrs_SETPAN_T1enc_A_txt(
            state,
            tracer,
            s_386_0,
        );
        // N s_386_2: return
        return;
    }
    fn block_387<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_387_0: panic
        panic!("{:?}", ());
        // N s_387_1: return
        return;
    }
    fn block_388<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_388_0: const #1u : u8
        let s_388_0: bool = true;
        // D s_388_1: write-var gs#428890 <= s_388_0
        fn_state.gs_428890 = s_388_0;
        // N s_388_2: jump b385
        return block_385(state, tracer, fn_state);
    }
    fn block_389<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_389_0: const #1u : u8
        let s_389_0: bool = true;
        // D s_389_1: write-var gs#428887 <= s_389_0
        fn_state.gs_428887 = s_389_0;
        // N s_389_2: jump b383
        return block_383(state, tracer, fn_state);
    }
    fn block_390<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_390_0: const #1u : u8
        let s_390_0: bool = true;
        // D s_390_1: write-var gs#428884 <= s_390_0
        fn_state.gs_428884 = s_390_0;
        // N s_390_2: jump b381
        return block_381(state, tracer, fn_state);
    }
    fn block_391<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_391_0: panic
        panic!("{:?}", ());
        // N s_391_1: return
        return;
    }
    fn block_392<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_392_0: const #3889s : i
        let s_392_0: i128 = 3889;
        // C s_392_1: const #14696u : u32
        let s_392_1: u32 = 14696;
        // D s_392_2: read-reg s_392_1:i
        let s_392_2: i128 = {
            let value = state.read_register::<i128>(s_392_1 as isize);
            tracer.read_register(s_392_1 as isize, value);
            value
        };
        // D s_392_3: cmp-lt s_392_2 s_392_0
        let s_392_3: bool = ((s_392_2) < (s_392_0));
        // D s_392_4: write-var gs#428875 <= s_392_3
        fn_state.gs_428875 = s_392_3;
        // N s_392_5: jump b378
        return block_378(state, tracer, fn_state);
    }
    fn block_393<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_393_0: const #0s : i
        let s_393_0: i128 = 0;
        // D s_393_1: read-var u#38501:u16
        let s_393_1: u16 = fn_state.u_38501;
        // D s_393_2: cast zx s_393_1 -> bv
        let s_393_2: Bits = Bits::new(s_393_1 as u128, 16u16);
        // C s_393_3: const #1s : i64
        let s_393_3: i64 = 1;
        // C s_393_4: cast zx s_393_3 -> i
        let s_393_4: i128 = (i128::try_from(s_393_3).unwrap());
        // C s_393_5: const #2s : i
        let s_393_5: i128 = 2;
        // C s_393_6: add s_393_5 s_393_4
        let s_393_6: i128 = (s_393_5 + s_393_4);
        // D s_393_7: bit-extract s_393_2 s_393_0 s_393_6
        let s_393_7: Bits = (Bits::new(
            ((s_393_2) >> (s_393_0)).value(),
            u16::try_from(s_393_6).unwrap(),
        ));
        // D s_393_8: cast reint s_393_7 -> u8
        let s_393_8: u8 = (s_393_7.value() as u8);
        // D s_393_9: cast zx s_393_8 -> bv
        let s_393_9: Bits = Bits::new(s_393_8 as u128, 3u16);
        // C s_393_10: const #0u : u8
        let s_393_10: u8 = 0;
        // C s_393_11: cast zx s_393_10 -> bv
        let s_393_11: Bits = Bits::new(s_393_10 as u128, 3u16);
        // D s_393_12: cmp-eq s_393_9 s_393_11
        let s_393_12: bool = ((s_393_9) == (s_393_11));
        // D s_393_13: write-var gs#428873 <= s_393_12
        fn_state.gs_428873 = s_393_12;
        // N s_393_14: jump b376
        return block_376(state, tracer, fn_state);
    }
    fn block_394<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_394_0: const #3830s : i
        let s_394_0: i128 = 3830;
        // C s_394_1: const #14696u : u32
        let s_394_1: u32 = 14696;
        // D s_394_2: read-reg s_394_1:i
        let s_394_2: i128 = {
            let value = state.read_register::<i128>(s_394_1 as isize);
            tracer.read_register(s_394_1 as isize, value);
            value
        };
        // D s_394_3: cmp-lt s_394_2 s_394_0
        let s_394_3: bool = ((s_394_2) < (s_394_0));
        // D s_394_4: write-var gs#428865 <= s_394_3
        fn_state.gs_428865 = s_394_3;
        // N s_394_5: jump b372
        return block_372(state, tracer, fn_state);
    }
    fn block_395<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_395_0: const #3814s : i
        let s_395_0: i128 = 3814;
        // C s_395_1: const #14696u : u32
        let s_395_1: u32 = 14696;
        // D s_395_2: read-reg s_395_1:i
        let s_395_2: i128 = {
            let value = state.read_register::<i128>(s_395_1 as isize);
            tracer.read_register(s_395_1 as isize, value);
            value
        };
        // D s_395_3: cmp-lt s_395_2 s_395_0
        let s_395_3: bool = ((s_395_2) < (s_395_0));
        // D s_395_4: write-var gs#428858 <= s_395_3
        fn_state.gs_428858 = s_395_3;
        // N s_395_5: jump b368
        return block_368(state, tracer, fn_state);
    }
    fn block_396<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_396_0: const #3770s : i
        let s_396_0: i128 = 3770;
        // C s_396_1: const #14696u : u32
        let s_396_1: u32 = 14696;
        // D s_396_2: read-reg s_396_1:i
        let s_396_2: i128 = {
            let value = state.read_register::<i128>(s_396_1 as isize);
            tracer.read_register(s_396_1 as isize, value);
            value
        };
        // D s_396_3: cmp-lt s_396_2 s_396_0
        let s_396_3: bool = ((s_396_2) < (s_396_0));
        // D s_396_4: write-var gs#428843 <= s_396_3
        fn_state.gs_428843 = s_396_3;
        // N s_396_5: jump b364
        return block_364(state, tracer, fn_state);
    }
    fn block_397<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_397_0: const #3s : i
        let s_397_0: i128 = 3;
        // D s_397_1: read-var u#38495:u16
        let s_397_1: u16 = fn_state.u_38495;
        // D s_397_2: cast zx s_397_1 -> bv
        let s_397_2: Bits = Bits::new(s_397_1 as u128, 16u16);
        // C s_397_3: const #1s : i64
        let s_397_3: i64 = 1;
        // C s_397_4: cast zx s_397_3 -> i
        let s_397_4: i128 = (i128::try_from(s_397_3).unwrap());
        // C s_397_5: const #0s : i
        let s_397_5: i128 = 0;
        // C s_397_6: add s_397_5 s_397_4
        let s_397_6: i128 = (s_397_5 + s_397_4);
        // D s_397_7: bit-extract s_397_2 s_397_0 s_397_6
        let s_397_7: Bits = (Bits::new(
            ((s_397_2) >> (s_397_0)).value(),
            u16::try_from(s_397_6).unwrap(),
        ));
        // D s_397_8: cast reint s_397_7 -> u8
        let s_397_8: bool = ((s_397_7.value()) != 0);
        // D s_397_9: cast zx s_397_8 -> bv
        let s_397_9: Bits = Bits::new(s_397_8 as u128, 1u16);
        // C s_397_10: const #0u : u8
        let s_397_10: bool = false;
        // C s_397_11: cast zx s_397_10 -> bv
        let s_397_11: Bits = Bits::new(s_397_10 as u128, 1u16);
        // D s_397_12: cmp-eq s_397_9 s_397_11
        let s_397_12: bool = ((s_397_9) == (s_397_11));
        // D s_397_13: write-var gs#428841 <= s_397_12
        fn_state.gs_428841 = s_397_12;
        // N s_397_14: jump b362
        return block_362(state, tracer, fn_state);
    }
    fn block_398<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_398_0: const #3767s : i
        let s_398_0: i128 = 3767;
        // C s_398_1: const #14696u : u32
        let s_398_1: u32 = 14696;
        // D s_398_2: read-reg s_398_1:i
        let s_398_2: i128 = {
            let value = state.read_register::<i128>(s_398_1 as isize);
            tracer.read_register(s_398_1 as isize, value);
            value
        };
        // D s_398_3: cmp-lt s_398_2 s_398_0
        let s_398_3: bool = ((s_398_2) < (s_398_0));
        // D s_398_4: write-var gs#428833 <= s_398_3
        fn_state.gs_428833 = s_398_3;
        // N s_398_5: jump b358
        return block_358(state, tracer, fn_state);
    }
    fn block_399<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_399_0: const #3764s : i
        let s_399_0: i128 = 3764;
        // C s_399_1: const #14696u : u32
        let s_399_1: u32 = 14696;
        // D s_399_2: read-reg s_399_1:i
        let s_399_2: i128 = {
            let value = state.read_register::<i128>(s_399_1 as isize);
            tracer.read_register(s_399_1 as isize, value);
            value
        };
        // D s_399_3: cmp-lt s_399_2 s_399_0
        let s_399_3: bool = ((s_399_2) < (s_399_0));
        // D s_399_4: write-var gs#428828 <= s_399_3
        fn_state.gs_428828 = s_399_3;
        // N s_399_5: jump b354
        return block_354(state, tracer, fn_state);
    }
    fn block_400<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_400_0: const #3761s : i
        let s_400_0: i128 = 3761;
        // C s_400_1: const #14696u : u32
        let s_400_1: u32 = 14696;
        // D s_400_2: read-reg s_400_1:i
        let s_400_2: i128 = {
            let value = state.read_register::<i128>(s_400_1 as isize);
            tracer.read_register(s_400_1 as isize, value);
            value
        };
        // D s_400_3: cmp-lt s_400_2 s_400_0
        let s_400_3: bool = ((s_400_2) < (s_400_0));
        // D s_400_4: write-var gs#428823 <= s_400_3
        fn_state.gs_428823 = s_400_3;
        // N s_400_5: jump b350
        return block_350(state, tracer, fn_state);
    }
    fn block_401<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_401_0: const #3348s : i
        let s_401_0: i128 = 3348;
        // C s_401_1: const #14696u : u32
        let s_401_1: u32 = 14696;
        // D s_401_2: read-reg s_401_1:i
        let s_401_2: i128 = {
            let value = state.read_register::<i128>(s_401_1 as isize);
            tracer.read_register(s_401_1 as isize, value);
            value
        };
        // D s_401_3: cmp-lt s_401_2 s_401_0
        let s_401_3: bool = ((s_401_2) < (s_401_0));
        // D s_401_4: write-var gs#428814 <= s_401_3
        fn_state.gs_428814 = s_401_3;
        // N s_401_5: jump b346
        return block_346(state, tracer, fn_state);
    }
    fn block_402<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_402_0: const #3345s : i
        let s_402_0: i128 = 3345;
        // C s_402_1: const #14696u : u32
        let s_402_1: u32 = 14696;
        // D s_402_2: read-reg s_402_1:i
        let s_402_2: i128 = {
            let value = state.read_register::<i128>(s_402_1 as isize);
            tracer.read_register(s_402_1 as isize, value);
            value
        };
        // D s_402_3: cmp-lt s_402_2 s_402_0
        let s_402_3: bool = ((s_402_2) < (s_402_0));
        // D s_402_4: write-var gs#428803 <= s_402_3
        fn_state.gs_428803 = s_402_3;
        // N s_402_5: jump b342
        return block_342(state, tracer, fn_state);
    }
    fn block_403<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_403_0: const #3288s : i
        let s_403_0: i128 = 3288;
        // C s_403_1: const #14696u : u32
        let s_403_1: u32 = 14696;
        // D s_403_2: read-reg s_403_1:i
        let s_403_2: i128 = {
            let value = state.read_register::<i128>(s_403_1 as isize);
            tracer.read_register(s_403_1 as isize, value);
            value
        };
        // D s_403_3: cmp-lt s_403_2 s_403_0
        let s_403_3: bool = ((s_403_2) < (s_403_0));
        // D s_403_4: write-var gs#428794 <= s_403_3
        fn_state.gs_428794 = s_403_3;
        // N s_403_5: jump b338
        return block_338(state, tracer, fn_state);
    }
    fn block_404<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_404_0: const #3276s : i
        let s_404_0: i128 = 3276;
        // C s_404_1: const #14696u : u32
        let s_404_1: u32 = 14696;
        // D s_404_2: read-reg s_404_1:i
        let s_404_2: i128 = {
            let value = state.read_register::<i128>(s_404_1 as isize);
            tracer.read_register(s_404_1 as isize, value);
            value
        };
        // D s_404_3: cmp-lt s_404_2 s_404_0
        let s_404_3: bool = ((s_404_2) < (s_404_0));
        // D s_404_4: write-var gs#428783 <= s_404_3
        fn_state.gs_428783 = s_404_3;
        // N s_404_5: jump b334
        return block_334(state, tracer, fn_state);
    }
    fn block_405<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_405_0: const #3265s : i
        let s_405_0: i128 = 3265;
        // C s_405_1: const #14696u : u32
        let s_405_1: u32 = 14696;
        // D s_405_2: read-reg s_405_1:i
        let s_405_2: i128 = {
            let value = state.read_register::<i128>(s_405_1 as isize);
            tracer.read_register(s_405_1 as isize, value);
            value
        };
        // D s_405_3: cmp-lt s_405_2 s_405_0
        let s_405_3: bool = ((s_405_2) < (s_405_0));
        // D s_405_4: write-var gs#428772 <= s_405_3
        fn_state.gs_428772 = s_405_3;
        // N s_405_5: jump b330
        return block_330(state, tracer, fn_state);
    }
    fn block_406<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_406_0: const #3262s : i
        let s_406_0: i128 = 3262;
        // C s_406_1: const #14696u : u32
        let s_406_1: u32 = 14696;
        // D s_406_2: read-reg s_406_1:i
        let s_406_2: i128 = {
            let value = state.read_register::<i128>(s_406_1 as isize);
            tracer.read_register(s_406_1 as isize, value);
            value
        };
        // D s_406_3: cmp-lt s_406_2 s_406_0
        let s_406_3: bool = ((s_406_2) < (s_406_0));
        // D s_406_4: write-var gs#428761 <= s_406_3
        fn_state.gs_428761 = s_406_3;
        // N s_406_5: jump b326
        return block_326(state, tracer, fn_state);
    }
    fn block_407<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_407_0: const #3252s : i
        let s_407_0: i128 = 3252;
        // C s_407_1: const #14696u : u32
        let s_407_1: u32 = 14696;
        // D s_407_2: read-reg s_407_1:i
        let s_407_2: i128 = {
            let value = state.read_register::<i128>(s_407_1 as isize);
            tracer.read_register(s_407_1 as isize, value);
            value
        };
        // D s_407_3: cmp-lt s_407_2 s_407_0
        let s_407_3: bool = ((s_407_2) < (s_407_0));
        // D s_407_4: write-var gs#428752 <= s_407_3
        fn_state.gs_428752 = s_407_3;
        // N s_407_5: jump b322
        return block_322(state, tracer, fn_state);
    }
    fn block_408<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_408_0: const #3246s : i
        let s_408_0: i128 = 3246;
        // C s_408_1: const #14696u : u32
        let s_408_1: u32 = 14696;
        // D s_408_2: read-reg s_408_1:i
        let s_408_2: i128 = {
            let value = state.read_register::<i128>(s_408_1 as isize);
            tracer.read_register(s_408_1 as isize, value);
            value
        };
        // D s_408_3: cmp-lt s_408_2 s_408_0
        let s_408_3: bool = ((s_408_2) < (s_408_0));
        // D s_408_4: write-var gs#428743 <= s_408_3
        fn_state.gs_428743 = s_408_3;
        // N s_408_5: jump b318
        return block_318(state, tracer, fn_state);
    }
    fn block_409<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_409_0: const #3242s : i
        let s_409_0: i128 = 3242;
        // C s_409_1: const #14696u : u32
        let s_409_1: u32 = 14696;
        // D s_409_2: read-reg s_409_1:i
        let s_409_2: i128 = {
            let value = state.read_register::<i128>(s_409_1 as isize);
            tracer.read_register(s_409_1 as isize, value);
            value
        };
        // D s_409_3: cmp-lt s_409_2 s_409_0
        let s_409_3: bool = ((s_409_2) < (s_409_0));
        // D s_409_4: write-var gs#428730 <= s_409_3
        fn_state.gs_428730 = s_409_3;
        // N s_409_5: jump b314
        return block_314(state, tracer, fn_state);
    }
    fn block_410<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_410_0: const #3237s : i
        let s_410_0: i128 = 3237;
        // C s_410_1: const #14696u : u32
        let s_410_1: u32 = 14696;
        // D s_410_2: read-reg s_410_1:i
        let s_410_2: i128 = {
            let value = state.read_register::<i128>(s_410_1 as isize);
            tracer.read_register(s_410_1 as isize, value);
            value
        };
        // D s_410_3: cmp-lt s_410_2 s_410_0
        let s_410_3: bool = ((s_410_2) < (s_410_0));
        // D s_410_4: write-var gs#428719 <= s_410_3
        fn_state.gs_428719 = s_410_3;
        // N s_410_5: jump b310
        return block_310(state, tracer, fn_state);
    }
    fn block_411<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_411_0: const #3236s : i
        let s_411_0: i128 = 3236;
        // C s_411_1: const #14696u : u32
        let s_411_1: u32 = 14696;
        // D s_411_2: read-reg s_411_1:i
        let s_411_2: i128 = {
            let value = state.read_register::<i128>(s_411_1 as isize);
            tracer.read_register(s_411_1 as isize, value);
            value
        };
        // D s_411_3: cmp-lt s_411_2 s_411_0
        let s_411_3: bool = ((s_411_2) < (s_411_0));
        // D s_411_4: write-var gs#428706 <= s_411_3
        fn_state.gs_428706 = s_411_3;
        // N s_411_5: jump b306
        return block_306(state, tracer, fn_state);
    }
    fn block_412<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_412_0: const #3230s : i
        let s_412_0: i128 = 3230;
        // C s_412_1: const #14696u : u32
        let s_412_1: u32 = 14696;
        // D s_412_2: read-reg s_412_1:i
        let s_412_2: i128 = {
            let value = state.read_register::<i128>(s_412_1 as isize);
            tracer.read_register(s_412_1 as isize, value);
            value
        };
        // D s_412_3: cmp-lt s_412_2 s_412_0
        let s_412_3: bool = ((s_412_2) < (s_412_0));
        // D s_412_4: write-var gs#428693 <= s_412_3
        fn_state.gs_428693 = s_412_3;
        // N s_412_5: jump b302
        return block_302(state, tracer, fn_state);
    }
    fn block_413<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_413_0: const #3226s : i
        let s_413_0: i128 = 3226;
        // C s_413_1: const #14696u : u32
        let s_413_1: u32 = 14696;
        // D s_413_2: read-reg s_413_1:i
        let s_413_2: i128 = {
            let value = state.read_register::<i128>(s_413_1 as isize);
            tracer.read_register(s_413_1 as isize, value);
            value
        };
        // D s_413_3: cmp-lt s_413_2 s_413_0
        let s_413_3: bool = ((s_413_2) < (s_413_0));
        // D s_413_4: write-var gs#428682 <= s_413_3
        fn_state.gs_428682 = s_413_3;
        // N s_413_5: jump b298
        return block_298(state, tracer, fn_state);
    }
    fn block_414<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_414_0: const #3225s : i
        let s_414_0: i128 = 3225;
        // C s_414_1: const #14696u : u32
        let s_414_1: u32 = 14696;
        // D s_414_2: read-reg s_414_1:i
        let s_414_2: i128 = {
            let value = state.read_register::<i128>(s_414_1 as isize);
            tracer.read_register(s_414_1 as isize, value);
            value
        };
        // D s_414_3: cmp-lt s_414_2 s_414_0
        let s_414_3: bool = ((s_414_2) < (s_414_0));
        // D s_414_4: write-var gs#428669 <= s_414_3
        fn_state.gs_428669 = s_414_3;
        // N s_414_5: jump b294
        return block_294(state, tracer, fn_state);
    }
    fn block_415<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_415_0: const #3219s : i
        let s_415_0: i128 = 3219;
        // C s_415_1: const #14696u : u32
        let s_415_1: u32 = 14696;
        // D s_415_2: read-reg s_415_1:i
        let s_415_2: i128 = {
            let value = state.read_register::<i128>(s_415_1 as isize);
            tracer.read_register(s_415_1 as isize, value);
            value
        };
        // D s_415_3: cmp-lt s_415_2 s_415_0
        let s_415_3: bool = ((s_415_2) < (s_415_0));
        // D s_415_4: write-var gs#428656 <= s_415_3
        fn_state.gs_428656 = s_415_3;
        // N s_415_5: jump b290
        return block_290(state, tracer, fn_state);
    }
    fn block_416<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_416_0: const #3215s : i
        let s_416_0: i128 = 3215;
        // C s_416_1: const #14696u : u32
        let s_416_1: u32 = 14696;
        // D s_416_2: read-reg s_416_1:i
        let s_416_2: i128 = {
            let value = state.read_register::<i128>(s_416_1 as isize);
            tracer.read_register(s_416_1 as isize, value);
            value
        };
        // D s_416_3: cmp-lt s_416_2 s_416_0
        let s_416_3: bool = ((s_416_2) < (s_416_0));
        // D s_416_4: write-var gs#428643 <= s_416_3
        fn_state.gs_428643 = s_416_3;
        // N s_416_5: jump b286
        return block_286(state, tracer, fn_state);
    }
    fn block_417<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_417_0: const #3198s : i
        let s_417_0: i128 = 3198;
        // C s_417_1: const #14696u : u32
        let s_417_1: u32 = 14696;
        // D s_417_2: read-reg s_417_1:i
        let s_417_2: i128 = {
            let value = state.read_register::<i128>(s_417_1 as isize);
            tracer.read_register(s_417_1 as isize, value);
            value
        };
        // D s_417_3: cmp-lt s_417_2 s_417_0
        let s_417_3: bool = ((s_417_2) < (s_417_0));
        // D s_417_4: write-var gs#428630 <= s_417_3
        fn_state.gs_428630 = s_417_3;
        // N s_417_5: jump b282
        return block_282(state, tracer, fn_state);
    }
    fn block_418<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_418_0: const #3194s : i
        let s_418_0: i128 = 3194;
        // C s_418_1: const #14696u : u32
        let s_418_1: u32 = 14696;
        // D s_418_2: read-reg s_418_1:i
        let s_418_2: i128 = {
            let value = state.read_register::<i128>(s_418_1 as isize);
            tracer.read_register(s_418_1 as isize, value);
            value
        };
        // D s_418_3: cmp-lt s_418_2 s_418_0
        let s_418_3: bool = ((s_418_2) < (s_418_0));
        // D s_418_4: write-var gs#428617 <= s_418_3
        fn_state.gs_428617 = s_418_3;
        // N s_418_5: jump b278
        return block_278(state, tracer, fn_state);
    }
    fn block_419<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_419_0: const #3187s : i
        let s_419_0: i128 = 3187;
        // C s_419_1: const #14696u : u32
        let s_419_1: u32 = 14696;
        // D s_419_2: read-reg s_419_1:i
        let s_419_2: i128 = {
            let value = state.read_register::<i128>(s_419_1 as isize);
            tracer.read_register(s_419_1 as isize, value);
            value
        };
        // D s_419_3: cmp-lt s_419_2 s_419_0
        let s_419_3: bool = ((s_419_2) < (s_419_0));
        // D s_419_4: write-var gs#428606 <= s_419_3
        fn_state.gs_428606 = s_419_3;
        // N s_419_5: jump b274
        return block_274(state, tracer, fn_state);
    }
    fn block_420<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_420_0: const #3128s : i
        let s_420_0: i128 = 3128;
        // C s_420_1: const #14696u : u32
        let s_420_1: u32 = 14696;
        // D s_420_2: read-reg s_420_1:i
        let s_420_2: i128 = {
            let value = state.read_register::<i128>(s_420_1 as isize);
            tracer.read_register(s_420_1 as isize, value);
            value
        };
        // D s_420_3: cmp-lt s_420_2 s_420_0
        let s_420_3: bool = ((s_420_2) < (s_420_0));
        // D s_420_4: write-var gs#428599 <= s_420_3
        fn_state.gs_428599 = s_420_3;
        // N s_420_5: jump b270
        return block_270(state, tracer, fn_state);
    }
    fn block_421<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_421_0: const #3126s : i
        let s_421_0: i128 = 3126;
        // C s_421_1: const #14696u : u32
        let s_421_1: u32 = 14696;
        // D s_421_2: read-reg s_421_1:i
        let s_421_2: i128 = {
            let value = state.read_register::<i128>(s_421_1 as isize);
            tracer.read_register(s_421_1 as isize, value);
            value
        };
        // D s_421_3: cmp-lt s_421_2 s_421_0
        let s_421_3: bool = ((s_421_2) < (s_421_0));
        // D s_421_4: write-var gs#428584 <= s_421_3
        fn_state.gs_428584 = s_421_3;
        // N s_421_5: jump b258
        return block_258(state, tracer, fn_state);
    }
    fn block_422<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_422_0: const #0s : i
        let s_422_0: i128 = 0;
        // D s_422_1: read-var u#38404:u16
        let s_422_1: u16 = fn_state.u_38404;
        // D s_422_2: cast zx s_422_1 -> bv
        let s_422_2: Bits = Bits::new(s_422_1 as u128, 16u16);
        // C s_422_3: const #1s : i64
        let s_422_3: i64 = 1;
        // C s_422_4: cast zx s_422_3 -> i
        let s_422_4: i128 = (i128::try_from(s_422_3).unwrap());
        // C s_422_5: const #2s : i
        let s_422_5: i128 = 2;
        // C s_422_6: add s_422_5 s_422_4
        let s_422_6: i128 = (s_422_5 + s_422_4);
        // D s_422_7: bit-extract s_422_2 s_422_0 s_422_6
        let s_422_7: Bits = (Bits::new(
            ((s_422_2) >> (s_422_0)).value(),
            u16::try_from(s_422_6).unwrap(),
        ));
        // D s_422_8: cast reint s_422_7 -> u8
        let s_422_8: u8 = (s_422_7.value() as u8);
        // D s_422_9: cast zx s_422_8 -> bv
        let s_422_9: Bits = Bits::new(s_422_8 as u128, 3u16);
        // C s_422_10: const #0u : u8
        let s_422_10: u8 = 0;
        // C s_422_11: cast zx s_422_10 -> bv
        let s_422_11: Bits = Bits::new(s_422_10 as u128, 3u16);
        // D s_422_12: cmp-eq s_422_9 s_422_11
        let s_422_12: bool = ((s_422_9) == (s_422_11));
        // D s_422_13: write-var gs#428582 <= s_422_12
        fn_state.gs_428582 = s_422_12;
        // N s_422_14: jump b256
        return block_256(state, tracer, fn_state);
    }
    fn block_423<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_423_0: const #3116s : i
        let s_423_0: i128 = 3116;
        // C s_423_1: const #14696u : u32
        let s_423_1: u32 = 14696;
        // D s_423_2: read-reg s_423_1:i
        let s_423_2: i128 = {
            let value = state.read_register::<i128>(s_423_1 as isize);
            tracer.read_register(s_423_1 as isize, value);
            value
        };
        // D s_423_3: cmp-lt s_423_2 s_423_0
        let s_423_3: bool = ((s_423_2) < (s_423_0));
        // D s_423_4: write-var gs#428570 <= s_423_3
        fn_state.gs_428570 = s_423_3;
        // N s_423_5: jump b252
        return block_252(state, tracer, fn_state);
    }
    fn block_424<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_424_0: const #3097s : i
        let s_424_0: i128 = 3097;
        // C s_424_1: const #14696u : u32
        let s_424_1: u32 = 14696;
        // D s_424_2: read-reg s_424_1:i
        let s_424_2: i128 = {
            let value = state.read_register::<i128>(s_424_1 as isize);
            tracer.read_register(s_424_1 as isize, value);
            value
        };
        // D s_424_3: cmp-lt s_424_2 s_424_0
        let s_424_3: bool = ((s_424_2) < (s_424_0));
        // D s_424_4: write-var gs#428559 <= s_424_3
        fn_state.gs_428559 = s_424_3;
        // N s_424_5: jump b248
        return block_248(state, tracer, fn_state);
    }
    fn block_425<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_425_0: const #3095s : i
        let s_425_0: i128 = 3095;
        // C s_425_1: const #14696u : u32
        let s_425_1: u32 = 14696;
        // D s_425_2: read-reg s_425_1:i
        let s_425_2: i128 = {
            let value = state.read_register::<i128>(s_425_1 as isize);
            tracer.read_register(s_425_1 as isize, value);
            value
        };
        // D s_425_3: cmp-lt s_425_2 s_425_0
        let s_425_3: bool = ((s_425_2) < (s_425_0));
        // D s_425_4: write-var gs#428548 <= s_425_3
        fn_state.gs_428548 = s_425_3;
        // N s_425_5: jump b244
        return block_244(state, tracer, fn_state);
    }
    fn block_426<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_426_0: const #3093s : i
        let s_426_0: i128 = 3093;
        // C s_426_1: const #14696u : u32
        let s_426_1: u32 = 14696;
        // D s_426_2: read-reg s_426_1:i
        let s_426_2: i128 = {
            let value = state.read_register::<i128>(s_426_1 as isize);
            tracer.read_register(s_426_1 as isize, value);
            value
        };
        // D s_426_3: cmp-lt s_426_2 s_426_0
        let s_426_3: bool = ((s_426_2) < (s_426_0));
        // D s_426_4: write-var gs#428537 <= s_426_3
        fn_state.gs_428537 = s_426_3;
        // N s_426_5: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_427<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_427_0: const #3090s : i
        let s_427_0: i128 = 3090;
        // C s_427_1: const #14696u : u32
        let s_427_1: u32 = 14696;
        // D s_427_2: read-reg s_427_1:i
        let s_427_2: i128 = {
            let value = state.read_register::<i128>(s_427_1 as isize);
            tracer.read_register(s_427_1 as isize, value);
            value
        };
        // D s_427_3: cmp-lt s_427_2 s_427_0
        let s_427_3: bool = ((s_427_2) < (s_427_0));
        // D s_427_4: write-var gs#428526 <= s_427_3
        fn_state.gs_428526 = s_427_3;
        // N s_427_5: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_428<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_428_0: const #3087s : i
        let s_428_0: i128 = 3087;
        // C s_428_1: const #14696u : u32
        let s_428_1: u32 = 14696;
        // D s_428_2: read-reg s_428_1:i
        let s_428_2: i128 = {
            let value = state.read_register::<i128>(s_428_1 as isize);
            tracer.read_register(s_428_1 as isize, value);
            value
        };
        // D s_428_3: cmp-lt s_428_2 s_428_0
        let s_428_3: bool = ((s_428_2) < (s_428_0));
        // D s_428_4: write-var gs#428515 <= s_428_3
        fn_state.gs_428515 = s_428_3;
        // N s_428_5: jump b232
        return block_232(state, tracer, fn_state);
    }
    fn block_429<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_429_0: const #3063s : i
        let s_429_0: i128 = 3063;
        // C s_429_1: const #14696u : u32
        let s_429_1: u32 = 14696;
        // D s_429_2: read-reg s_429_1:i
        let s_429_2: i128 = {
            let value = state.read_register::<i128>(s_429_1 as isize);
            tracer.read_register(s_429_1 as isize, value);
            value
        };
        // D s_429_3: cmp-lt s_429_2 s_429_0
        let s_429_3: bool = ((s_429_2) < (s_429_0));
        // D s_429_4: write-var gs#428504 <= s_429_3
        fn_state.gs_428504 = s_429_3;
        // N s_429_5: jump b228
        return block_228(state, tracer, fn_state);
    }
    fn block_430<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_430_0: const #3062s : i
        let s_430_0: i128 = 3062;
        // C s_430_1: const #14696u : u32
        let s_430_1: u32 = 14696;
        // D s_430_2: read-reg s_430_1:i
        let s_430_2: i128 = {
            let value = state.read_register::<i128>(s_430_1 as isize);
            tracer.read_register(s_430_1 as isize, value);
            value
        };
        // D s_430_3: cmp-lt s_430_2 s_430_0
        let s_430_3: bool = ((s_430_2) < (s_430_0));
        // D s_430_4: write-var gs#428493 <= s_430_3
        fn_state.gs_428493 = s_430_3;
        // N s_430_5: jump b224
        return block_224(state, tracer, fn_state);
    }
    fn block_431<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_431_0: const #3044s : i
        let s_431_0: i128 = 3044;
        // C s_431_1: const #14696u : u32
        let s_431_1: u32 = 14696;
        // D s_431_2: read-reg s_431_1:i
        let s_431_2: i128 = {
            let value = state.read_register::<i128>(s_431_1 as isize);
            tracer.read_register(s_431_1 as isize, value);
            value
        };
        // D s_431_3: cmp-lt s_431_2 s_431_0
        let s_431_3: bool = ((s_431_2) < (s_431_0));
        // D s_431_4: write-var gs#428482 <= s_431_3
        fn_state.gs_428482 = s_431_3;
        // N s_431_5: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_432<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_432_0: const #3037s : i
        let s_432_0: i128 = 3037;
        // C s_432_1: const #14696u : u32
        let s_432_1: u32 = 14696;
        // D s_432_2: read-reg s_432_1:i
        let s_432_2: i128 = {
            let value = state.read_register::<i128>(s_432_1 as isize);
            tracer.read_register(s_432_1 as isize, value);
            value
        };
        // D s_432_3: cmp-lt s_432_2 s_432_0
        let s_432_3: bool = ((s_432_2) < (s_432_0));
        // D s_432_4: write-var gs#428475 <= s_432_3
        fn_state.gs_428475 = s_432_3;
        // N s_432_5: jump b216
        return block_216(state, tracer, fn_state);
    }
    fn block_433<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_433_0: const #3033s : i
        let s_433_0: i128 = 3033;
        // C s_433_1: const #14696u : u32
        let s_433_1: u32 = 14696;
        // D s_433_2: read-reg s_433_1:i
        let s_433_2: i128 = {
            let value = state.read_register::<i128>(s_433_1 as isize);
            tracer.read_register(s_433_1 as isize, value);
            value
        };
        // D s_433_3: cmp-lt s_433_2 s_433_0
        let s_433_3: bool = ((s_433_2) < (s_433_0));
        // D s_433_4: write-var gs#428466 <= s_433_3
        fn_state.gs_428466 = s_433_3;
        // N s_433_5: jump b212
        return block_212(state, tracer, fn_state);
    }
    fn block_434<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_434_0: const #3028s : i
        let s_434_0: i128 = 3028;
        // C s_434_1: const #14696u : u32
        let s_434_1: u32 = 14696;
        // D s_434_2: read-reg s_434_1:i
        let s_434_2: i128 = {
            let value = state.read_register::<i128>(s_434_1 as isize);
            tracer.read_register(s_434_1 as isize, value);
            value
        };
        // D s_434_3: cmp-lt s_434_2 s_434_0
        let s_434_3: bool = ((s_434_2) < (s_434_0));
        // D s_434_4: write-var gs#428455 <= s_434_3
        fn_state.gs_428455 = s_434_3;
        // N s_434_5: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_435<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_435_0: const #3019s : i
        let s_435_0: i128 = 3019;
        // C s_435_1: const #14696u : u32
        let s_435_1: u32 = 14696;
        // D s_435_2: read-reg s_435_1:i
        let s_435_2: i128 = {
            let value = state.read_register::<i128>(s_435_1 as isize);
            tracer.read_register(s_435_1 as isize, value);
            value
        };
        // D s_435_3: cmp-lt s_435_2 s_435_0
        let s_435_3: bool = ((s_435_2) < (s_435_0));
        // D s_435_4: write-var gs#428442 <= s_435_3
        fn_state.gs_428442 = s_435_3;
        // N s_435_5: jump b204
        return block_204(state, tracer, fn_state);
    }
    fn block_436<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_436_0: const #3016s : i
        let s_436_0: i128 = 3016;
        // C s_436_1: const #14696u : u32
        let s_436_1: u32 = 14696;
        // D s_436_2: read-reg s_436_1:i
        let s_436_2: i128 = {
            let value = state.read_register::<i128>(s_436_1 as isize);
            tracer.read_register(s_436_1 as isize, value);
            value
        };
        // D s_436_3: cmp-lt s_436_2 s_436_0
        let s_436_3: bool = ((s_436_2) < (s_436_0));
        // D s_436_4: write-var gs#428431 <= s_436_3
        fn_state.gs_428431 = s_436_3;
        // N s_436_5: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_437<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_437_0: const #3015s : i
        let s_437_0: i128 = 3015;
        // C s_437_1: const #14696u : u32
        let s_437_1: u32 = 14696;
        // D s_437_2: read-reg s_437_1:i
        let s_437_2: i128 = {
            let value = state.read_register::<i128>(s_437_1 as isize);
            tracer.read_register(s_437_1 as isize, value);
            value
        };
        // D s_437_3: cmp-lt s_437_2 s_437_0
        let s_437_3: bool = ((s_437_2) < (s_437_0));
        // D s_437_4: write-var gs#428418 <= s_437_3
        fn_state.gs_428418 = s_437_3;
        // N s_437_5: jump b196
        return block_196(state, tracer, fn_state);
    }
    fn block_438<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_438_0: const #3011s : i
        let s_438_0: i128 = 3011;
        // C s_438_1: const #14696u : u32
        let s_438_1: u32 = 14696;
        // D s_438_2: read-reg s_438_1:i
        let s_438_2: i128 = {
            let value = state.read_register::<i128>(s_438_1 as isize);
            tracer.read_register(s_438_1 as isize, value);
            value
        };
        // D s_438_3: cmp-lt s_438_2 s_438_0
        let s_438_3: bool = ((s_438_2) < (s_438_0));
        // D s_438_4: write-var gs#428407 <= s_438_3
        fn_state.gs_428407 = s_438_3;
        // N s_438_5: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_439<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_439_0: const #3000s : i
        let s_439_0: i128 = 3000;
        // C s_439_1: const #14696u : u32
        let s_439_1: u32 = 14696;
        // D s_439_2: read-reg s_439_1:i
        let s_439_2: i128 = {
            let value = state.read_register::<i128>(s_439_1 as isize);
            tracer.read_register(s_439_1 as isize, value);
            value
        };
        // D s_439_3: cmp-lt s_439_2 s_439_0
        let s_439_3: bool = ((s_439_2) < (s_439_0));
        // D s_439_4: write-var gs#428396 <= s_439_3
        fn_state.gs_428396 = s_439_3;
        // N s_439_5: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_440<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_440_0: const #2999s : i
        let s_440_0: i128 = 2999;
        // C s_440_1: const #14696u : u32
        let s_440_1: u32 = 14696;
        // D s_440_2: read-reg s_440_1:i
        let s_440_2: i128 = {
            let value = state.read_register::<i128>(s_440_1 as isize);
            tracer.read_register(s_440_1 as isize, value);
            value
        };
        // D s_440_3: cmp-lt s_440_2 s_440_0
        let s_440_3: bool = ((s_440_2) < (s_440_0));
        // D s_440_4: write-var gs#428383 <= s_440_3
        fn_state.gs_428383 = s_440_3;
        // N s_440_5: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_441<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_441_0: const #2997s : i
        let s_441_0: i128 = 2997;
        // C s_441_1: const #14696u : u32
        let s_441_1: u32 = 14696;
        // D s_441_2: read-reg s_441_1:i
        let s_441_2: i128 = {
            let value = state.read_register::<i128>(s_441_1 as isize);
            tracer.read_register(s_441_1 as isize, value);
            value
        };
        // D s_441_3: cmp-lt s_441_2 s_441_0
        let s_441_3: bool = ((s_441_2) < (s_441_0));
        // D s_441_4: write-var gs#428372 <= s_441_3
        fn_state.gs_428372 = s_441_3;
        // N s_441_5: jump b180
        return block_180(state, tracer, fn_state);
    }
    fn block_442<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_442_0: const #2996s : i
        let s_442_0: i128 = 2996;
        // C s_442_1: const #14696u : u32
        let s_442_1: u32 = 14696;
        // D s_442_2: read-reg s_442_1:i
        let s_442_2: i128 = {
            let value = state.read_register::<i128>(s_442_1 as isize);
            tracer.read_register(s_442_1 as isize, value);
            value
        };
        // D s_442_3: cmp-lt s_442_2 s_442_0
        let s_442_3: bool = ((s_442_2) < (s_442_0));
        // D s_442_4: write-var gs#428359 <= s_442_3
        fn_state.gs_428359 = s_442_3;
        // N s_442_5: jump b176
        return block_176(state, tracer, fn_state);
    }
    fn block_443<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_443_0: const #2988s : i
        let s_443_0: i128 = 2988;
        // C s_443_1: const #14696u : u32
        let s_443_1: u32 = 14696;
        // D s_443_2: read-reg s_443_1:i
        let s_443_2: i128 = {
            let value = state.read_register::<i128>(s_443_1 as isize);
            tracer.read_register(s_443_1 as isize, value);
            value
        };
        // D s_443_3: cmp-lt s_443_2 s_443_0
        let s_443_3: bool = ((s_443_2) < (s_443_0));
        // D s_443_4: write-var gs#428346 <= s_443_3
        fn_state.gs_428346 = s_443_3;
        // N s_443_5: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_444<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_444_0: const #2977s : i
        let s_444_0: i128 = 2977;
        // C s_444_1: const #14696u : u32
        let s_444_1: u32 = 14696;
        // D s_444_2: read-reg s_444_1:i
        let s_444_2: i128 = {
            let value = state.read_register::<i128>(s_444_1 as isize);
            tracer.read_register(s_444_1 as isize, value);
            value
        };
        // D s_444_3: cmp-lt s_444_2 s_444_0
        let s_444_3: bool = ((s_444_2) < (s_444_0));
        // D s_444_4: write-var gs#428333 <= s_444_3
        fn_state.gs_428333 = s_444_3;
        // N s_444_5: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_445<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_445_0: const #2969s : i
        let s_445_0: i128 = 2969;
        // C s_445_1: const #14696u : u32
        let s_445_1: u32 = 14696;
        // D s_445_2: read-reg s_445_1:i
        let s_445_2: i128 = {
            let value = state.read_register::<i128>(s_445_1 as isize);
            tracer.read_register(s_445_1 as isize, value);
            value
        };
        // D s_445_3: cmp-lt s_445_2 s_445_0
        let s_445_3: bool = ((s_445_2) < (s_445_0));
        // D s_445_4: write-var gs#428320 <= s_445_3
        fn_state.gs_428320 = s_445_3;
        // N s_445_5: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_446<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_446_0: const #2966s : i
        let s_446_0: i128 = 2966;
        // C s_446_1: const #14696u : u32
        let s_446_1: u32 = 14696;
        // D s_446_2: read-reg s_446_1:i
        let s_446_2: i128 = {
            let value = state.read_register::<i128>(s_446_1 as isize);
            tracer.read_register(s_446_1 as isize, value);
            value
        };
        // D s_446_3: cmp-lt s_446_2 s_446_0
        let s_446_3: bool = ((s_446_2) < (s_446_0));
        // D s_446_4: write-var gs#428309 <= s_446_3
        fn_state.gs_428309 = s_446_3;
        // N s_446_5: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_447<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_447_0: const #2962s : i
        let s_447_0: i128 = 2962;
        // C s_447_1: const #14696u : u32
        let s_447_1: u32 = 14696;
        // D s_447_2: read-reg s_447_1:i
        let s_447_2: i128 = {
            let value = state.read_register::<i128>(s_447_1 as isize);
            tracer.read_register(s_447_1 as isize, value);
            value
        };
        // D s_447_3: cmp-lt s_447_2 s_447_0
        let s_447_3: bool = ((s_447_2) < (s_447_0));
        // D s_447_4: write-var gs#428298 <= s_447_3
        fn_state.gs_428298 = s_447_3;
        // N s_447_5: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_448<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_448_0: const #2961s : i
        let s_448_0: i128 = 2961;
        // C s_448_1: const #14696u : u32
        let s_448_1: u32 = 14696;
        // D s_448_2: read-reg s_448_1:i
        let s_448_2: i128 = {
            let value = state.read_register::<i128>(s_448_1 as isize);
            tracer.read_register(s_448_1 as isize, value);
            value
        };
        // D s_448_3: cmp-lt s_448_2 s_448_0
        let s_448_3: bool = ((s_448_2) < (s_448_0));
        // D s_448_4: write-var gs#428285 <= s_448_3
        fn_state.gs_428285 = s_448_3;
        // N s_448_5: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_449<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_449_0: const #2955s : i
        let s_449_0: i128 = 2955;
        // C s_449_1: const #14696u : u32
        let s_449_1: u32 = 14696;
        // D s_449_2: read-reg s_449_1:i
        let s_449_2: i128 = {
            let value = state.read_register::<i128>(s_449_1 as isize);
            tracer.read_register(s_449_1 as isize, value);
            value
        };
        // D s_449_3: cmp-lt s_449_2 s_449_0
        let s_449_3: bool = ((s_449_2) < (s_449_0));
        // D s_449_4: write-var gs#428272 <= s_449_3
        fn_state.gs_428272 = s_449_3;
        // N s_449_5: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_450<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_450_0: const #2949s : i
        let s_450_0: i128 = 2949;
        // C s_450_1: const #14696u : u32
        let s_450_1: u32 = 14696;
        // D s_450_2: read-reg s_450_1:i
        let s_450_2: i128 = {
            let value = state.read_register::<i128>(s_450_1 as isize);
            tracer.read_register(s_450_1 as isize, value);
            value
        };
        // D s_450_3: cmp-lt s_450_2 s_450_0
        let s_450_3: bool = ((s_450_2) < (s_450_0));
        // D s_450_4: write-var gs#428259 <= s_450_3
        fn_state.gs_428259 = s_450_3;
        // N s_450_5: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_451<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_451_0: const #2930s : i
        let s_451_0: i128 = 2930;
        // C s_451_1: const #14696u : u32
        let s_451_1: u32 = 14696;
        // D s_451_2: read-reg s_451_1:i
        let s_451_2: i128 = {
            let value = state.read_register::<i128>(s_451_1 as isize);
            tracer.read_register(s_451_1 as isize, value);
            value
        };
        // D s_451_3: cmp-lt s_451_2 s_451_0
        let s_451_3: bool = ((s_451_2) < (s_451_0));
        // D s_451_4: write-var gs#428246 <= s_451_3
        fn_state.gs_428246 = s_451_3;
        // N s_451_5: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_452<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_452_0: const #2924s : i
        let s_452_0: i128 = 2924;
        // C s_452_1: const #14696u : u32
        let s_452_1: u32 = 14696;
        // D s_452_2: read-reg s_452_1:i
        let s_452_2: i128 = {
            let value = state.read_register::<i128>(s_452_1 as isize);
            tracer.read_register(s_452_1 as isize, value);
            value
        };
        // D s_452_3: cmp-lt s_452_2 s_452_0
        let s_452_3: bool = ((s_452_2) < (s_452_0));
        // D s_452_4: write-var gs#428233 <= s_452_3
        fn_state.gs_428233 = s_452_3;
        // N s_452_5: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_453<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_453_0: const #2917s : i
        let s_453_0: i128 = 2917;
        // C s_453_1: const #14696u : u32
        let s_453_1: u32 = 14696;
        // D s_453_2: read-reg s_453_1:i
        let s_453_2: i128 = {
            let value = state.read_register::<i128>(s_453_1 as isize);
            tracer.read_register(s_453_1 as isize, value);
            value
        };
        // D s_453_3: cmp-lt s_453_2 s_453_0
        let s_453_3: bool = ((s_453_2) < (s_453_0));
        // D s_453_4: write-var gs#428222 <= s_453_3
        fn_state.gs_428222 = s_453_3;
        // N s_453_5: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_454<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_454_0: const #2911s : i
        let s_454_0: i128 = 2911;
        // C s_454_1: const #14696u : u32
        let s_454_1: u32 = 14696;
        // D s_454_2: read-reg s_454_1:i
        let s_454_2: i128 = {
            let value = state.read_register::<i128>(s_454_1 as isize);
            tracer.read_register(s_454_1 as isize, value);
            value
        };
        // D s_454_3: cmp-lt s_454_2 s_454_0
        let s_454_3: bool = ((s_454_2) < (s_454_0));
        // D s_454_4: write-var gs#428211 <= s_454_3
        fn_state.gs_428211 = s_454_3;
        // N s_454_5: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_455<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_455_0: const #2906s : i
        let s_455_0: i128 = 2906;
        // C s_455_1: const #14696u : u32
        let s_455_1: u32 = 14696;
        // D s_455_2: read-reg s_455_1:i
        let s_455_2: i128 = {
            let value = state.read_register::<i128>(s_455_1 as isize);
            tracer.read_register(s_455_1 as isize, value);
            value
        };
        // D s_455_3: cmp-lt s_455_2 s_455_0
        let s_455_3: bool = ((s_455_2) < (s_455_0));
        // D s_455_4: write-var gs#428200 <= s_455_3
        fn_state.gs_428200 = s_455_3;
        // N s_455_5: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_456<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_456_0: const #2894s : i
        let s_456_0: i128 = 2894;
        // C s_456_1: const #14696u : u32
        let s_456_1: u32 = 14696;
        // D s_456_2: read-reg s_456_1:i
        let s_456_2: i128 = {
            let value = state.read_register::<i128>(s_456_1 as isize);
            tracer.read_register(s_456_1 as isize, value);
            value
        };
        // D s_456_3: cmp-lt s_456_2 s_456_0
        let s_456_3: bool = ((s_456_2) < (s_456_0));
        // D s_456_4: write-var gs#428187 <= s_456_3
        fn_state.gs_428187 = s_456_3;
        // N s_456_5: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_457<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_457_0: const #2893s : i
        let s_457_0: i128 = 2893;
        // C s_457_1: const #14696u : u32
        let s_457_1: u32 = 14696;
        // D s_457_2: read-reg s_457_1:i
        let s_457_2: i128 = {
            let value = state.read_register::<i128>(s_457_1 as isize);
            tracer.read_register(s_457_1 as isize, value);
            value
        };
        // D s_457_3: cmp-lt s_457_2 s_457_0
        let s_457_3: bool = ((s_457_2) < (s_457_0));
        // D s_457_4: write-var gs#428176 <= s_457_3
        fn_state.gs_428176 = s_457_3;
        // N s_457_5: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_458<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_458_0: const #2890s : i
        let s_458_0: i128 = 2890;
        // C s_458_1: const #14696u : u32
        let s_458_1: u32 = 14696;
        // D s_458_2: read-reg s_458_1:i
        let s_458_2: i128 = {
            let value = state.read_register::<i128>(s_458_1 as isize);
            tracer.read_register(s_458_1 as isize, value);
            value
        };
        // D s_458_3: cmp-lt s_458_2 s_458_0
        let s_458_3: bool = ((s_458_2) < (s_458_0));
        // D s_458_4: write-var gs#428165 <= s_458_3
        fn_state.gs_428165 = s_458_3;
        // N s_458_5: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_459<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_459_0: const #2886s : i
        let s_459_0: i128 = 2886;
        // C s_459_1: const #14696u : u32
        let s_459_1: u32 = 14696;
        // D s_459_2: read-reg s_459_1:i
        let s_459_2: i128 = {
            let value = state.read_register::<i128>(s_459_1 as isize);
            tracer.read_register(s_459_1 as isize, value);
            value
        };
        // D s_459_3: cmp-lt s_459_2 s_459_0
        let s_459_3: bool = ((s_459_2) < (s_459_0));
        // D s_459_4: write-var gs#428154 <= s_459_3
        fn_state.gs_428154 = s_459_3;
        // N s_459_5: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_460<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_460_0: const #2878s : i
        let s_460_0: i128 = 2878;
        // C s_460_1: const #14696u : u32
        let s_460_1: u32 = 14696;
        // D s_460_2: read-reg s_460_1:i
        let s_460_2: i128 = {
            let value = state.read_register::<i128>(s_460_1 as isize);
            tracer.read_register(s_460_1 as isize, value);
            value
        };
        // D s_460_3: cmp-lt s_460_2 s_460_0
        let s_460_3: bool = ((s_460_2) < (s_460_0));
        // D s_460_4: write-var gs#428139 <= s_460_3
        fn_state.gs_428139 = s_460_3;
        // N s_460_5: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_461<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_461_0: const #10s : i
        let s_461_0: i128 = 10;
        // D s_461_1: read-var u#38250:u16
        let s_461_1: u16 = fn_state.u_38250;
        // D s_461_2: cast zx s_461_1 -> bv
        let s_461_2: Bits = Bits::new(s_461_1 as u128, 16u16);
        // C s_461_3: const #1s : i64
        let s_461_3: i64 = 1;
        // C s_461_4: cast zx s_461_3 -> i
        let s_461_4: i128 = (i128::try_from(s_461_3).unwrap());
        // C s_461_5: const #0s : i
        let s_461_5: i128 = 0;
        // C s_461_6: add s_461_5 s_461_4
        let s_461_6: i128 = (s_461_5 + s_461_4);
        // D s_461_7: bit-extract s_461_2 s_461_0 s_461_6
        let s_461_7: Bits = (Bits::new(
            ((s_461_2) >> (s_461_0)).value(),
            u16::try_from(s_461_6).unwrap(),
        ));
        // D s_461_8: cast reint s_461_7 -> u8
        let s_461_8: bool = ((s_461_7.value()) != 0);
        // D s_461_9: cast zx s_461_8 -> bv
        let s_461_9: Bits = Bits::new(s_461_8 as u128, 1u16);
        // C s_461_10: const #0u : u8
        let s_461_10: bool = false;
        // C s_461_11: cast zx s_461_10 -> bv
        let s_461_11: Bits = Bits::new(s_461_10 as u128, 1u16);
        // D s_461_12: cmp-eq s_461_9 s_461_11
        let s_461_12: bool = ((s_461_9) == (s_461_11));
        // N s_461_13: branch s_461_12 b464 b462
        if s_461_12 {
            return block_464(state, tracer, fn_state);
        } else {
            return block_462(state, tracer, fn_state);
        };
    }
    fn block_462<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_462_0: const #0u : u8
        let s_462_0: bool = false;
        // D s_462_1: write-var gs#428136 <= s_462_0
        fn_state.gs_428136 = s_462_0;
        // N s_462_2: jump b463
        return block_463(state, tracer, fn_state);
    }
    fn block_463<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_463_0: read-var gs#428136:u8
        let s_463_0: bool = fn_state.gs_428136;
        // D s_463_1: write-var gs#428137 <= s_463_0
        fn_state.gs_428137 = s_463_0;
        // N s_463_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_464<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_464_0: const #8s : i
        let s_464_0: i128 = 8;
        // D s_464_1: read-var u#38250:u16
        let s_464_1: u16 = fn_state.u_38250;
        // D s_464_2: cast zx s_464_1 -> bv
        let s_464_2: Bits = Bits::new(s_464_1 as u128, 16u16);
        // C s_464_3: const #1s : i64
        let s_464_3: i64 = 1;
        // C s_464_4: cast zx s_464_3 -> i
        let s_464_4: i128 = (i128::try_from(s_464_3).unwrap());
        // C s_464_5: const #0s : i
        let s_464_5: i128 = 0;
        // C s_464_6: add s_464_5 s_464_4
        let s_464_6: i128 = (s_464_5 + s_464_4);
        // D s_464_7: bit-extract s_464_2 s_464_0 s_464_6
        let s_464_7: Bits = (Bits::new(
            ((s_464_2) >> (s_464_0)).value(),
            u16::try_from(s_464_6).unwrap(),
        ));
        // D s_464_8: cast reint s_464_7 -> u8
        let s_464_8: bool = ((s_464_7.value()) != 0);
        // D s_464_9: cast zx s_464_8 -> bv
        let s_464_9: Bits = Bits::new(s_464_8 as u128, 1u16);
        // C s_464_10: const #1u : u8
        let s_464_10: bool = true;
        // C s_464_11: cast zx s_464_10 -> bv
        let s_464_11: Bits = Bits::new(s_464_10 as u128, 1u16);
        // D s_464_12: cmp-eq s_464_9 s_464_11
        let s_464_12: bool = ((s_464_9) == (s_464_11));
        // D s_464_13: write-var gs#428136 <= s_464_12
        fn_state.gs_428136 = s_464_12;
        // N s_464_14: jump b463
        return block_463(state, tracer, fn_state);
    }
    fn block_465<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_465_0: const #2875s : i
        let s_465_0: i128 = 2875;
        // C s_465_1: const #14696u : u32
        let s_465_1: u32 = 14696;
        // D s_465_2: read-reg s_465_1:i
        let s_465_2: i128 = {
            let value = state.read_register::<i128>(s_465_1 as isize);
            tracer.read_register(s_465_1 as isize, value);
            value
        };
        // D s_465_3: cmp-lt s_465_2 s_465_0
        let s_465_3: bool = ((s_465_2) < (s_465_0));
        // D s_465_4: write-var gs#428116 <= s_465_3
        fn_state.gs_428116 = s_465_3;
        // N s_465_5: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_466<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_466_0: const #0s : i
        let s_466_0: i128 = 0;
        // D s_466_1: read-var u#38247:u16
        let s_466_1: u16 = fn_state.u_38247;
        // D s_466_2: cast zx s_466_1 -> bv
        let s_466_2: Bits = Bits::new(s_466_1 as u128, 16u16);
        // C s_466_3: const #1s : i64
        let s_466_3: i64 = 1;
        // C s_466_4: cast zx s_466_3 -> i
        let s_466_4: i128 = (i128::try_from(s_466_3).unwrap());
        // C s_466_5: const #2s : i
        let s_466_5: i128 = 2;
        // C s_466_6: add s_466_5 s_466_4
        let s_466_6: i128 = (s_466_5 + s_466_4);
        // D s_466_7: bit-extract s_466_2 s_466_0 s_466_6
        let s_466_7: Bits = (Bits::new(
            ((s_466_2) >> (s_466_0)).value(),
            u16::try_from(s_466_6).unwrap(),
        ));
        // D s_466_8: cast reint s_466_7 -> u8
        let s_466_8: u8 = (s_466_7.value() as u8);
        // D s_466_9: cast zx s_466_8 -> bv
        let s_466_9: Bits = Bits::new(s_466_8 as u128, 3u16);
        // C s_466_10: const #0u : u8
        let s_466_10: u8 = 0;
        // C s_466_11: cast zx s_466_10 -> bv
        let s_466_11: Bits = Bits::new(s_466_10 as u128, 3u16);
        // D s_466_12: cmp-eq s_466_9 s_466_11
        let s_466_12: bool = ((s_466_9) == (s_466_11));
        // D s_466_13: write-var gs#428114 <= s_466_12
        fn_state.gs_428114 = s_466_12;
        // N s_466_14: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_467<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_467_0: const #2873s : i
        let s_467_0: i128 = 2873;
        // C s_467_1: const #14696u : u32
        let s_467_1: u32 = 14696;
        // D s_467_2: read-reg s_467_1:i
        let s_467_2: i128 = {
            let value = state.read_register::<i128>(s_467_1 as isize);
            tracer.read_register(s_467_1 as isize, value);
            value
        };
        // D s_467_3: cmp-lt s_467_2 s_467_0
        let s_467_3: bool = ((s_467_2) < (s_467_0));
        // D s_467_4: write-var gs#428096 <= s_467_3
        fn_state.gs_428096 = s_467_3;
        // N s_467_5: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_468<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_468_0: const #0s : i
        let s_468_0: i128 = 0;
        // D s_468_1: read-var u#38244:u16
        let s_468_1: u16 = fn_state.u_38244;
        // D s_468_2: cast zx s_468_1 -> bv
        let s_468_2: Bits = Bits::new(s_468_1 as u128, 16u16);
        // C s_468_3: const #1s : i64
        let s_468_3: i64 = 1;
        // C s_468_4: cast zx s_468_3 -> i
        let s_468_4: i128 = (i128::try_from(s_468_3).unwrap());
        // C s_468_5: const #2s : i
        let s_468_5: i128 = 2;
        // C s_468_6: add s_468_5 s_468_4
        let s_468_6: i128 = (s_468_5 + s_468_4);
        // D s_468_7: bit-extract s_468_2 s_468_0 s_468_6
        let s_468_7: Bits = (Bits::new(
            ((s_468_2) >> (s_468_0)).value(),
            u16::try_from(s_468_6).unwrap(),
        ));
        // D s_468_8: cast reint s_468_7 -> u8
        let s_468_8: u8 = (s_468_7.value() as u8);
        // D s_468_9: cast zx s_468_8 -> bv
        let s_468_9: Bits = Bits::new(s_468_8 as u128, 3u16);
        // C s_468_10: const #0u : u8
        let s_468_10: u8 = 0;
        // C s_468_11: cast zx s_468_10 -> bv
        let s_468_11: Bits = Bits::new(s_468_10 as u128, 3u16);
        // D s_468_12: cmp-eq s_468_9 s_468_11
        let s_468_12: bool = ((s_468_9) == (s_468_11));
        // D s_468_13: write-var gs#428094 <= s_468_12
        fn_state.gs_428094 = s_468_12;
        // N s_468_14: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_469<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_469_0: const #2867s : i
        let s_469_0: i128 = 2867;
        // C s_469_1: const #14696u : u32
        let s_469_1: u32 = 14696;
        // D s_469_2: read-reg s_469_1:i
        let s_469_2: i128 = {
            let value = state.read_register::<i128>(s_469_1 as isize);
            tracer.read_register(s_469_1 as isize, value);
            value
        };
        // D s_469_3: cmp-lt s_469_2 s_469_0
        let s_469_3: bool = ((s_469_2) < (s_469_0));
        // D s_469_4: write-var gs#428084 <= s_469_3
        fn_state.gs_428084 = s_469_3;
        // N s_469_5: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_470<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_470_0: const #2863s : i
        let s_470_0: i128 = 2863;
        // C s_470_1: const #14696u : u32
        let s_470_1: u32 = 14696;
        // D s_470_2: read-reg s_470_1:i
        let s_470_2: i128 = {
            let value = state.read_register::<i128>(s_470_1 as isize);
            tracer.read_register(s_470_1 as isize, value);
            value
        };
        // D s_470_3: cmp-lt s_470_2 s_470_0
        let s_470_3: bool = ((s_470_2) < (s_470_0));
        // D s_470_4: write-var gs#428073 <= s_470_3
        fn_state.gs_428073 = s_470_3;
        // N s_470_5: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_471<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_471_0: const #2853s : i
        let s_471_0: i128 = 2853;
        // C s_471_1: const #14696u : u32
        let s_471_1: u32 = 14696;
        // D s_471_2: read-reg s_471_1:i
        let s_471_2: i128 = {
            let value = state.read_register::<i128>(s_471_1 as isize);
            tracer.read_register(s_471_1 as isize, value);
            value
        };
        // D s_471_3: cmp-lt s_471_2 s_471_0
        let s_471_3: bool = ((s_471_2) < (s_471_0));
        // D s_471_4: write-var gs#428064 <= s_471_3
        fn_state.gs_428064 = s_471_3;
        // N s_471_5: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_472<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_472_0: const #8s : i
        let s_472_0: i128 = 8;
        // C s_472_1: const #4s : i
        let s_472_1: i128 = 4;
        // D s_472_2: read-var u#38232:u16
        let s_472_2: u16 = fn_state.u_38232;
        // D s_472_3: cast zx s_472_2 -> bv
        let s_472_3: Bits = Bits::new(s_472_2 as u128, 16u16);
        // D s_472_4: bit-extract s_472_3 s_472_0 s_472_1
        let s_472_4: Bits = (Bits::new(
            ((s_472_3) >> (s_472_0)).value(),
            u16::try_from(s_472_1).unwrap(),
        ));
        // D s_472_5: cast reint s_472_4 -> u8
        let s_472_5: u8 = (s_472_4.value() as u8);
        // D s_472_6: cast zx s_472_5 -> bv
        let s_472_6: Bits = Bits::new(s_472_5 as u128, 4u16);
        // C s_472_7: const #15u : u8
        let s_472_7: u8 = 15;
        // C s_472_8: cast zx s_472_7 -> bv
        let s_472_8: Bits = Bits::new(s_472_7 as u128, 4u16);
        // D s_472_9: cmp-ne s_472_6 s_472_8
        let s_472_9: bool = ((s_472_6) != (s_472_8));
        // N s_472_10: branch s_472_9 b475 b473
        if s_472_9 {
            return block_475(state, tracer, fn_state);
        } else {
            return block_473(state, tracer, fn_state);
        };
    }
    fn block_473<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_473_0: const #0u : u8
        let s_473_0: bool = false;
        // D s_473_1: write-var gs#428052 <= s_473_0
        fn_state.gs_428052 = s_473_0;
        // N s_473_2: jump b474
        return block_474(state, tracer, fn_state);
    }
    fn block_474<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_474_0: read-var gs#428052:u8
        let s_474_0: bool = fn_state.gs_428052;
        // D s_474_1: write-var gs#428053 <= s_474_0
        fn_state.gs_428053 = s_474_0;
        // N s_474_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_475<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_475_0: const #2852s : i
        let s_475_0: i128 = 2852;
        // C s_475_1: const #14696u : u32
        let s_475_1: u32 = 14696;
        // D s_475_2: read-reg s_475_1:i
        let s_475_2: i128 = {
            let value = state.read_register::<i128>(s_475_1 as isize);
            tracer.read_register(s_475_1 as isize, value);
            value
        };
        // D s_475_3: cmp-lt s_475_2 s_475_0
        let s_475_3: bool = ((s_475_2) < (s_475_0));
        // D s_475_4: write-var gs#428052 <= s_475_3
        fn_state.gs_428052 = s_475_3;
        // N s_475_5: jump b474
        return block_474(state, tracer, fn_state);
    }
    fn block_476<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_476_0: const #2850s : i
        let s_476_0: i128 = 2850;
        // C s_476_1: const #14696u : u32
        let s_476_1: u32 = 14696;
        // D s_476_2: read-reg s_476_1:i
        let s_476_2: i128 = {
            let value = state.read_register::<i128>(s_476_1 as isize);
            tracer.read_register(s_476_1 as isize, value);
            value
        };
        // D s_476_3: cmp-lt s_476_2 s_476_0
        let s_476_3: bool = ((s_476_2) < (s_476_0));
        // D s_476_4: write-var gs#428039 <= s_476_3
        fn_state.gs_428039 = s_476_3;
        // N s_476_5: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_477<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_477_0: const #2849s : i
        let s_477_0: i128 = 2849;
        // C s_477_1: const #14696u : u32
        let s_477_1: u32 = 14696;
        // D s_477_2: read-reg s_477_1:i
        let s_477_2: i128 = {
            let value = state.read_register::<i128>(s_477_1 as isize);
            tracer.read_register(s_477_1 as isize, value);
            value
        };
        // D s_477_3: cmp-lt s_477_2 s_477_0
        let s_477_3: bool = ((s_477_2) < (s_477_0));
        // D s_477_4: write-var gs#428026 <= s_477_3
        fn_state.gs_428026 = s_477_3;
        // N s_477_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_478<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_478_0: const #2846s : i
        let s_478_0: i128 = 2846;
        // C s_478_1: const #14696u : u32
        let s_478_1: u32 = 14696;
        // D s_478_2: read-reg s_478_1:i
        let s_478_2: i128 = {
            let value = state.read_register::<i128>(s_478_1 as isize);
            tracer.read_register(s_478_1 as isize, value);
            value
        };
        // D s_478_3: cmp-lt s_478_2 s_478_0
        let s_478_3: bool = ((s_478_2) < (s_478_0));
        // D s_478_4: write-var gs#428015 <= s_478_3
        fn_state.gs_428015 = s_478_3;
        // N s_478_5: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_479<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_479_0: const #2840s : i
        let s_479_0: i128 = 2840;
        // C s_479_1: const #14696u : u32
        let s_479_1: u32 = 14696;
        // D s_479_2: read-reg s_479_1:i
        let s_479_2: i128 = {
            let value = state.read_register::<i128>(s_479_1 as isize);
            tracer.read_register(s_479_1 as isize, value);
            value
        };
        // D s_479_3: cmp-lt s_479_2 s_479_0
        let s_479_3: bool = ((s_479_2) < (s_479_0));
        // D s_479_4: write-var gs#428004 <= s_479_3
        fn_state.gs_428004 = s_479_3;
        // N s_479_5: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_480<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_480_0: const #2836s : i
        let s_480_0: i128 = 2836;
        // C s_480_1: const #14696u : u32
        let s_480_1: u32 = 14696;
        // D s_480_2: read-reg s_480_1:i
        let s_480_2: i128 = {
            let value = state.read_register::<i128>(s_480_1 as isize);
            tracer.read_register(s_480_1 as isize, value);
            value
        };
        // D s_480_3: cmp-lt s_480_2 s_480_0
        let s_480_3: bool = ((s_480_2) < (s_480_0));
        // D s_480_4: write-var gs#427995 <= s_480_3
        fn_state.gs_427995 = s_480_3;
        // N s_480_5: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_481<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_481_0: const #0s : i
        let s_481_0: i128 = 0;
        // D s_481_1: read-var u#38213:u16
        let s_481_1: u16 = fn_state.u_38213;
        // D s_481_2: cast zx s_481_1 -> bv
        let s_481_2: Bits = Bits::new(s_481_1 as u128, 16u16);
        // C s_481_3: const #1s : i64
        let s_481_3: i64 = 1;
        // C s_481_4: cast zx s_481_3 -> i
        let s_481_4: i128 = (i128::try_from(s_481_3).unwrap());
        // C s_481_5: const #2s : i
        let s_481_5: i128 = 2;
        // C s_481_6: add s_481_5 s_481_4
        let s_481_6: i128 = (s_481_5 + s_481_4);
        // D s_481_7: bit-extract s_481_2 s_481_0 s_481_6
        let s_481_7: Bits = (Bits::new(
            ((s_481_2) >> (s_481_0)).value(),
            u16::try_from(s_481_6).unwrap(),
        ));
        // D s_481_8: cast reint s_481_7 -> u8
        let s_481_8: u8 = (s_481_7.value() as u8);
        // D s_481_9: cast zx s_481_8 -> bv
        let s_481_9: Bits = Bits::new(s_481_8 as u128, 3u16);
        // C s_481_10: const #5u : u8
        let s_481_10: u8 = 5;
        // C s_481_11: cast zx s_481_10 -> bv
        let s_481_11: Bits = Bits::new(s_481_10 as u128, 3u16);
        // D s_481_12: cmp-eq s_481_9 s_481_11
        let s_481_12: bool = ((s_481_9) == (s_481_11));
        // D s_481_13: write-var gs#427993 <= s_481_12
        fn_state.gs_427993 = s_481_12;
        // N s_481_14: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_482<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_482_0: const #2835s : i
        let s_482_0: i128 = 2835;
        // C s_482_1: const #14696u : u32
        let s_482_1: u32 = 14696;
        // D s_482_2: read-reg s_482_1:i
        let s_482_2: i128 = {
            let value = state.read_register::<i128>(s_482_1 as isize);
            tracer.read_register(s_482_1 as isize, value);
            value
        };
        // D s_482_3: cmp-lt s_482_2 s_482_0
        let s_482_3: bool = ((s_482_2) < (s_482_0));
        // D s_482_4: write-var gs#427981 <= s_482_3
        fn_state.gs_427981 = s_482_3;
        // N s_482_5: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_483<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_483_0: const #3s : i
        let s_483_0: i128 = 3;
        // D s_483_1: read-var u#38211:u16
        let s_483_1: u16 = fn_state.u_38211;
        // D s_483_2: cast zx s_483_1 -> bv
        let s_483_2: Bits = Bits::new(s_483_1 as u128, 16u16);
        // C s_483_3: const #1s : i64
        let s_483_3: i64 = 1;
        // C s_483_4: cast zx s_483_3 -> i
        let s_483_4: i128 = (i128::try_from(s_483_3).unwrap());
        // C s_483_5: const #3s : i
        let s_483_5: i128 = 3;
        // C s_483_6: add s_483_5 s_483_4
        let s_483_6: i128 = (s_483_5 + s_483_4);
        // D s_483_7: bit-extract s_483_2 s_483_0 s_483_6
        let s_483_7: Bits = (Bits::new(
            ((s_483_2) >> (s_483_0)).value(),
            u16::try_from(s_483_6).unwrap(),
        ));
        // D s_483_8: cast reint s_483_7 -> u8
        let s_483_8: u8 = (s_483_7.value() as u8);
        // D s_483_9: cast zx s_483_8 -> bv
        let s_483_9: Bits = Bits::new(s_483_8 as u128, 4u16);
        // C s_483_10: const #13u : u8
        let s_483_10: u8 = 13;
        // C s_483_11: cast zx s_483_10 -> bv
        let s_483_11: Bits = Bits::new(s_483_10 as u128, 4u16);
        // D s_483_12: cmp-eq s_483_9 s_483_11
        let s_483_12: bool = ((s_483_9) == (s_483_11));
        // D s_483_13: write-var gs#427979 <= s_483_12
        fn_state.gs_427979 = s_483_12;
        // N s_483_14: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_484<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_484_0: const #2831s : i
        let s_484_0: i128 = 2831;
        // C s_484_1: const #14696u : u32
        let s_484_1: u32 = 14696;
        // D s_484_2: read-reg s_484_1:i
        let s_484_2: i128 = {
            let value = state.read_register::<i128>(s_484_1 as isize);
            tracer.read_register(s_484_1 as isize, value);
            value
        };
        // D s_484_3: cmp-lt s_484_2 s_484_0
        let s_484_3: bool = ((s_484_2) < (s_484_0));
        // D s_484_4: write-var gs#427969 <= s_484_3
        fn_state.gs_427969 = s_484_3;
        // N s_484_5: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_485<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_485_0: const #2830s : i
        let s_485_0: i128 = 2830;
        // C s_485_1: const #14696u : u32
        let s_485_1: u32 = 14696;
        // D s_485_2: read-reg s_485_1:i
        let s_485_2: i128 = {
            let value = state.read_register::<i128>(s_485_1 as isize);
            tracer.read_register(s_485_1 as isize, value);
            value
        };
        // D s_485_3: cmp-lt s_485_2 s_485_0
        let s_485_3: bool = ((s_485_2) < (s_485_0));
        // D s_485_4: write-var gs#427958 <= s_485_3
        fn_state.gs_427958 = s_485_3;
        // N s_485_5: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_486<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_486_0: const #2826s : i
        let s_486_0: i128 = 2826;
        // C s_486_1: const #14696u : u32
        let s_486_1: u32 = 14696;
        // D s_486_2: read-reg s_486_1:i
        let s_486_2: i128 = {
            let value = state.read_register::<i128>(s_486_1 as isize);
            tracer.read_register(s_486_1 as isize, value);
            value
        };
        // D s_486_3: cmp-lt s_486_2 s_486_0
        let s_486_3: bool = ((s_486_2) < (s_486_0));
        // D s_486_4: write-var gs#427945 <= s_486_3
        fn_state.gs_427945 = s_486_3;
        // N s_486_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_487<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_487_0: const #2825s : i
        let s_487_0: i128 = 2825;
        // C s_487_1: const #14696u : u32
        let s_487_1: u32 = 14696;
        // D s_487_2: read-reg s_487_1:i
        let s_487_2: i128 = {
            let value = state.read_register::<i128>(s_487_1 as isize);
            tracer.read_register(s_487_1 as isize, value);
            value
        };
        // D s_487_3: cmp-lt s_487_2 s_487_0
        let s_487_3: bool = ((s_487_2) < (s_487_0));
        // D s_487_4: write-var gs#427932 <= s_487_3
        fn_state.gs_427932 = s_487_3;
        // N s_487_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_488<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_488_0: const #2821s : i
        let s_488_0: i128 = 2821;
        // C s_488_1: const #14696u : u32
        let s_488_1: u32 = 14696;
        // D s_488_2: read-reg s_488_1:i
        let s_488_2: i128 = {
            let value = state.read_register::<i128>(s_488_1 as isize);
            tracer.read_register(s_488_1 as isize, value);
            value
        };
        // D s_488_3: cmp-lt s_488_2 s_488_0
        let s_488_3: bool = ((s_488_2) < (s_488_0));
        // D s_488_4: write-var gs#427921 <= s_488_3
        fn_state.gs_427921 = s_488_3;
        // N s_488_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_489<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_489_0: const #2820s : i
        let s_489_0: i128 = 2820;
        // C s_489_1: const #14696u : u32
        let s_489_1: u32 = 14696;
        // D s_489_2: read-reg s_489_1:i
        let s_489_2: i128 = {
            let value = state.read_register::<i128>(s_489_1 as isize);
            tracer.read_register(s_489_1 as isize, value);
            value
        };
        // D s_489_3: cmp-lt s_489_2 s_489_0
        let s_489_3: bool = ((s_489_2) < (s_489_0));
        // D s_489_4: write-var gs#427908 <= s_489_3
        fn_state.gs_427908 = s_489_3;
        // N s_489_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_490<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_490_0: const #2816s : i
        let s_490_0: i128 = 2816;
        // C s_490_1: const #14696u : u32
        let s_490_1: u32 = 14696;
        // D s_490_2: read-reg s_490_1:i
        let s_490_2: i128 = {
            let value = state.read_register::<i128>(s_490_1 as isize);
            tracer.read_register(s_490_1 as isize, value);
            value
        };
        // D s_490_3: cmp-lt s_490_2 s_490_0
        let s_490_3: bool = ((s_490_2) < (s_490_0));
        // D s_490_4: write-var gs#427897 <= s_490_3
        fn_state.gs_427897 = s_490_3;
        // N s_490_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
