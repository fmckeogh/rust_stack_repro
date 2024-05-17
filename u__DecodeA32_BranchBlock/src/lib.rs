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
use decode_aarch32_instrs_STMDA_A1enc_A_txt::*;
use decode_aarch32_instrs_STM_u_A1enc_AS_txt::*;
use decode_aarch32_instrs_STMDB_A1enc_A_txt::*;
use decode_aarch32_instrs_RFE_A1enc_AS_txt::*;
use decode_aarch32_instrs_LDMIB_A1enc_A_txt::*;
use decode_aarch32_instrs_LDM_A1enc_A_txt::*;
use decode_aarch32_instrs_LDM_e_A1enc_AS_txt::*;
use decode_aarch32_instrs_B_A1enc_A_txt::*;
use decode_aarch32_instrs_BL_i_A2enc_A_txt::*;
use decode_aarch32_instrs_BL_i_A1enc_A_txt::*;
use decode_aarch32_instrs_LDM_u_A1enc_AS_txt::*;
use decode_aarch32_instrs_SRS_A1enc_AS_txt::*;
use decode_aarch32_instrs_LDMDB_A1enc_A_txt::*;
use decode_aarch32_instrs_STM_A1enc_A_txt::*;
use decode_aarch32_instrs_LDMDA_A1enc_A_txt::*;
use decode_aarch32_instrs_STMIB_A1enc_A_txt::*;
use common::*;
pub fn u__DecodeA32_BranchBlock<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_411360: i128,
    gs_411361: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_411662: bool,
        u_33916: u32,
        gs_411431: bool,
        gs_411383: bool,
        gs_411536: bool,
        u_33954: bool,
        gs_411613: bool,
        u_33922: u32,
        gs_411448: bool,
        u_33948: bool,
        u_33904: u32,
        gs_411382: bool,
        gs_411495: bool,
        gs_411516: bool,
        gs_411730: bool,
        gs_411369: bool,
        u_33892: u32,
        u_33962: u8,
        gs_411607: bool,
        gs_411608: bool,
        u_33951: u8,
        u_33910: u32,
        u_33934: u32,
        gs_411656: bool,
        u_33943: bool,
        mode: u8,
        u_33960: bool,
        gs_411686: bool,
        gs_411584: bool,
        gs_411580: bool,
        gs_411683: bool,
        u_33953: u32,
        u_33941: u8,
        gs_411671: bool,
        gs_411680: bool,
        gs_411490: bool,
        gs_411432: bool,
        gs_411733: bool,
        u_33961: bool,
        gs_411427: bool,
        gs_411637: bool,
        gs_411694: bool,
        gs_411721: bool,
        gs_411368: bool,
        gs_411515: bool,
        u_33949: bool,
        gs_411677: bool,
        gs_411739: bool,
        gs_411659: bool,
        gs_411537: bool,
        u_33947: u32,
        u_33928: u32,
        gs_411494: bool,
        gs_411727: bool,
        gs_411674: bool,
        gs_411718: bool,
        u_33882: u32,
        gs_411532: bool,
        gs_411638: bool,
        gs_411557: bool,
        u_33958: u32,
        gs_411639: bool,
        gs_411612: bool,
        gs_411474: bool,
        gs_411695: bool,
        u_33944: u8,
        gs_411668: bool,
        u_33940: u32,
        gs_411473: bool,
        gs_411453: bool,
        u_33898: u32,
        gs_411511: bool,
        gs_411736: bool,
        u_33950: bool,
        gs_411665: bool,
        gs_411553: bool,
        gs_411394: bool,
        u_33959: u8,
        u_33942: bool,
        u__opcode: u32,
        merge_var: ProductType7b8639ca40b2f578,
        gs_411452: bool,
        gs_411715: bool,
        gs_411585: bool,
        gs_411745: bool,
        u_33889: u32,
        gs_411411: bool,
        u_33956: bool,
        gs_411469: bool,
        gs_411558: bool,
        gs_411697: bool,
        u_33963: u16,
        gs_411410: bool,
        gs_411712: bool,
        gs_411750: bool,
        u_33955: bool,
        gs_411578: bool,
        gs_411579: bool,
        gs_411641: bool,
        gs_411724: bool,
        u_33945: u16,
        u_33886: u32,
        gs_411749: bool,
        gs_411406: bool,
        gs_411360: i128,
        gs_411361: u32,
    }
    let fn_state = FunctionState {
        gs_411360,
        gs_411361,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var gs#411360:i
        let s_0_0: i128 = fn_state.gs_411360;
        // D s_0_1: write-var merge#var.0 <= s_0_0
        fn_state.merge_var._0 = s_0_0;
        // D s_0_2: read-var gs#411361:u32
        let s_0_2: u32 = fn_state.gs_411361;
        // D s_0_3: write-var merge#var.1 <= s_0_2
        fn_state.merge_var._1 = s_0_2;
        // D s_0_4: read-var merge#var.1:struct
        let s_0_4: u32 = fn_state.merge_var._1;
        // D s_0_5: write-var __opcode <= s_0_4
        fn_state.u__opcode = s_0_4;
        // C s_0_6: const #24s : i
        let s_0_6: i128 = 24;
        // D s_0_7: read-var __opcode:u32
        let s_0_7: u32 = fn_state.u__opcode;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 32u16);
        // C s_0_9: const #1s : i64
        let s_0_9: i64 = 1;
        // C s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (i128::try_from(s_0_9).unwrap());
        // C s_0_11: const #3s : i
        let s_0_11: i128 = 3;
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
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 4u16);
        // C s_0_16: const #10u : u8
        let s_0_16: u8 = 10;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 4u16);
        // D s_0_18: cmp-eq s_0_15 s_0_17
        let s_0_18: bool = ((s_0_15) == (s_0_17));
        // N s_0_19: branch s_0_18 b244 b1
        if s_0_18 {
            return block_244(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#411369 <= s_1_0
        fn_state.gs_411369 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#411369:u8
        let s_2_0: bool = fn_state.gs_411369;
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
        // C s_3_0: const #2851s : i
        let s_3_0: i128 = 2851;
        // C s_3_1: const #14696u : u32
        let s_3_1: u32 = 14696;
        // N s_3_2: write-reg s_3_1 <= s_3_0
        let s_3_2: () = {
            state.write_register::<i128>(s_3_1 as isize, s_3_0);
            tracer.write_register(s_3_1 as isize, s_3_0);
        };
        // C s_3_3: const #28s : i
        let s_3_3: i128 = 28;
        // C s_3_4: const #4s : i
        let s_3_4: i128 = 4;
        // D s_3_5: read-var __opcode:u32
        let s_3_5: u32 = fn_state.u__opcode;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 32u16);
        // D s_3_7: bit-extract s_3_6 s_3_3 s_3_4
        let s_3_7: Bits = (Bits::new(
            ((s_3_6) >> (s_3_3)).value(),
            u16::try_from(s_3_4).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: u8 = (s_3_7.value() as u8);
        // C s_3_9: const #0s : i
        let s_3_9: i128 = 0;
        // C s_3_10: const #24s : i
        let s_3_10: i128 = 24;
        // D s_3_11: read-var __opcode:u32
        let s_3_11: u32 = fn_state.u__opcode;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 32u16);
        // D s_3_13: bit-extract s_3_12 s_3_9 s_3_10
        let s_3_13: Bits = (Bits::new(
            ((s_3_12) >> (s_3_9)).value(),
            u16::try_from(s_3_10).unwrap(),
        ));
        // D s_3_14: cast reint s_3_13 -> u24
        let s_3_14: u32 = (s_3_13.value() as u32);
        // D s_3_15: call decode_aarch32_instrs_B_A1enc_A_txt(s_3_8, s_3_14)
        let s_3_15: () = decode_aarch32_instrs_B_A1enc_A_txt(
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
        let s_4_0: u32 = fn_state.merge_var._1;
        // D s_4_1: write-var u#33882 <= s_4_0
        fn_state.u_33882 = s_4_0;
        // C s_4_2: const #24s : i
        let s_4_2: i128 = 24;
        // D s_4_3: read-var u#33882:u32
        let s_4_3: u32 = fn_state.u_33882;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 32u16);
        // C s_4_5: const #1s : i64
        let s_4_5: i64 = 1;
        // C s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // C s_4_7: const #3s : i
        let s_4_7: i128 = 3;
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
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 4u16);
        // C s_4_12: const #11u : u8
        let s_4_12: u8 = 11;
        // C s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 4u16);
        // D s_4_14: cmp-eq s_4_11 s_4_13
        let s_4_14: bool = ((s_4_11) == (s_4_13));
        // N s_4_15: branch s_4_14 b240 b5
        if s_4_14 {
            return block_240(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#411383 <= s_5_0
        fn_state.gs_411383 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#411383:u8
        let s_6_0: bool = fn_state.gs_411383;
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
        // C s_7_0: const #2868s : i
        let s_7_0: i128 = 2868;
        // C s_7_1: const #14696u : u32
        let s_7_1: u32 = 14696;
        // N s_7_2: write-reg s_7_1 <= s_7_0
        let s_7_2: () = {
            state.write_register::<i128>(s_7_1 as isize, s_7_0);
            tracer.write_register(s_7_1 as isize, s_7_0);
        };
        // C s_7_3: const #28s : i
        let s_7_3: i128 = 28;
        // C s_7_4: const #4s : i
        let s_7_4: i128 = 4;
        // D s_7_5: read-var u#33882:u32
        let s_7_5: u32 = fn_state.u_33882;
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 32u16);
        // D s_7_7: bit-extract s_7_6 s_7_3 s_7_4
        let s_7_7: Bits = (Bits::new(
            ((s_7_6) >> (s_7_3)).value(),
            u16::try_from(s_7_4).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u8
        let s_7_8: u8 = (s_7_7.value() as u8);
        // C s_7_9: const #0s : i
        let s_7_9: i128 = 0;
        // C s_7_10: const #24s : i
        let s_7_10: i128 = 24;
        // D s_7_11: read-var u#33882:u32
        let s_7_11: u32 = fn_state.u_33882;
        // D s_7_12: cast zx s_7_11 -> bv
        let s_7_12: Bits = Bits::new(s_7_11 as u128, 32u16);
        // D s_7_13: bit-extract s_7_12 s_7_9 s_7_10
        let s_7_13: Bits = (Bits::new(
            ((s_7_12) >> (s_7_9)).value(),
            u16::try_from(s_7_10).unwrap(),
        ));
        // D s_7_14: cast reint s_7_13 -> u24
        let s_7_14: u32 = (s_7_13.value() as u32);
        // D s_7_15: call decode_aarch32_instrs_BL_i_A1enc_A_txt(s_7_8, s_7_14)
        let s_7_15: () = decode_aarch32_instrs_BL_i_A1enc_A_txt(
            state,
            tracer,
            s_7_8,
            s_7_14,
        );
        // N s_7_16: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var merge#var.1:struct
        let s_8_0: u32 = fn_state.merge_var._1;
        // D s_8_1: write-var u#33886 <= s_8_0
        fn_state.u_33886 = s_8_0;
        // C s_8_2: const #25s : i
        let s_8_2: i128 = 25;
        // D s_8_3: read-var u#33886:u32
        let s_8_3: u32 = fn_state.u_33886;
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 32u16);
        // C s_8_5: const #1s : i64
        let s_8_5: i64 = 1;
        // C s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // C s_8_7: const #6s : i
        let s_8_7: i128 = 6;
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
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 7u16);
        // C s_8_12: const #125u : u8
        let s_8_12: u8 = 125;
        // C s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 7u16);
        // D s_8_14: cmp-eq s_8_11 s_8_13
        let s_8_14: bool = ((s_8_11) == (s_8_13));
        // N s_8_15: branch s_8_14 b239 b9
        if s_8_14 {
            return block_239(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#411394 <= s_9_0
        fn_state.gs_411394 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#411394:u8
        let s_10_0: bool = fn_state.gs_411394;
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
        // C s_11_0: const #2869s : i
        let s_11_0: i128 = 2869;
        // C s_11_1: const #14696u : u32
        let s_11_1: u32 = 14696;
        // N s_11_2: write-reg s_11_1 <= s_11_0
        let s_11_2: () = {
            state.write_register::<i128>(s_11_1 as isize, s_11_0);
            tracer.write_register(s_11_1 as isize, s_11_0);
        };
        // C s_11_3: const #24s : i
        let s_11_3: i128 = 24;
        // C s_11_4: const #1s : i
        let s_11_4: i128 = 1;
        // D s_11_5: read-var u#33886:u32
        let s_11_5: u32 = fn_state.u_33886;
        // D s_11_6: cast zx s_11_5 -> bv
        let s_11_6: Bits = Bits::new(s_11_5 as u128, 32u16);
        // D s_11_7: bit-extract s_11_6 s_11_3 s_11_4
        let s_11_7: Bits = (Bits::new(
            ((s_11_6) >> (s_11_3)).value(),
            u16::try_from(s_11_4).unwrap(),
        ));
        // D s_11_8: cast reint s_11_7 -> u8
        let s_11_8: bool = ((s_11_7.value()) != 0);
        // C s_11_9: const #0s : i
        let s_11_9: i128 = 0;
        // C s_11_10: const #24s : i
        let s_11_10: i128 = 24;
        // D s_11_11: read-var u#33886:u32
        let s_11_11: u32 = fn_state.u_33886;
        // D s_11_12: cast zx s_11_11 -> bv
        let s_11_12: Bits = Bits::new(s_11_11 as u128, 32u16);
        // D s_11_13: bit-extract s_11_12 s_11_9 s_11_10
        let s_11_13: Bits = (Bits::new(
            ((s_11_12) >> (s_11_9)).value(),
            u16::try_from(s_11_10).unwrap(),
        ));
        // D s_11_14: cast reint s_11_13 -> u24
        let s_11_14: u32 = (s_11_13.value() as u32);
        // D s_11_15: call decode_aarch32_instrs_BL_i_A2enc_A_txt(s_11_8, s_11_14)
        let s_11_15: () = decode_aarch32_instrs_BL_i_A2enc_A_txt(
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
        let s_12_0: u32 = fn_state.merge_var._1;
        // D s_12_1: write-var u#33889 <= s_12_0
        fn_state.u_33889 = s_12_0;
        // C s_12_2: const #22s : i
        let s_12_2: i128 = 22;
        // D s_12_3: read-var u#33889:u32
        let s_12_3: u32 = fn_state.u_33889;
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 32u16);
        // C s_12_5: const #1s : i64
        let s_12_5: i64 = 1;
        // C s_12_6: cast zx s_12_5 -> i
        let s_12_6: i128 = (i128::try_from(s_12_5).unwrap());
        // C s_12_7: const #5s : i
        let s_12_7: i128 = 5;
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
        let s_12_11: Bits = Bits::new(s_12_10 as u128, 6u16);
        // C s_12_12: const #34u : u8
        let s_12_12: u8 = 34;
        // C s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 6u16);
        // D s_12_14: cmp-eq s_12_11 s_12_13
        let s_12_14: bool = ((s_12_11) == (s_12_13));
        // N s_12_15: branch s_12_14 b238 b13
        if s_12_14 {
            return block_238(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#411406 <= s_13_0
        fn_state.gs_411406 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#411406:u8
        let s_14_0: bool = fn_state.gs_411406;
        // N s_14_1: branch s_14_0 b234 b15
        if s_14_0 {
            return block_234(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#411411 <= s_15_0
        fn_state.gs_411411 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#411411:u8
        let s_16_0: bool = fn_state.gs_411411;
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
        // C s_17_0: const #2916s : i
        let s_17_0: i128 = 2916;
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
        // D s_17_5: read-var u#33889:u32
        let s_17_5: u32 = fn_state.u_33889;
        // D s_17_6: cast zx s_17_5 -> bv
        let s_17_6: Bits = Bits::new(s_17_5 as u128, 32u16);
        // D s_17_7: bit-extract s_17_6 s_17_3 s_17_4
        let s_17_7: Bits = (Bits::new(
            ((s_17_6) >> (s_17_3)).value(),
            u16::try_from(s_17_4).unwrap(),
        ));
        // D s_17_8: cast reint s_17_7 -> u8
        let s_17_8: u8 = (s_17_7.value() as u8);
        // C s_17_9: const #21s : i
        let s_17_9: i128 = 21;
        // C s_17_10: const #1s : i
        let s_17_10: i128 = 1;
        // D s_17_11: read-var u#33889:u32
        let s_17_11: u32 = fn_state.u_33889;
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
        // D s_17_17: read-var u#33889:u32
        let s_17_17: u32 = fn_state.u_33889;
        // D s_17_18: cast zx s_17_17 -> bv
        let s_17_18: Bits = Bits::new(s_17_17 as u128, 32u16);
        // D s_17_19: bit-extract s_17_18 s_17_15 s_17_16
        let s_17_19: Bits = (Bits::new(
            ((s_17_18) >> (s_17_15)).value(),
            u16::try_from(s_17_16).unwrap(),
        ));
        // D s_17_20: cast reint s_17_19 -> u8
        let s_17_20: u8 = (s_17_19.value() as u8);
        // C s_17_21: const #0s : i
        let s_17_21: i128 = 0;
        // C s_17_22: const #16s : i
        let s_17_22: i128 = 16;
        // D s_17_23: read-var u#33889:u32
        let s_17_23: u32 = fn_state.u_33889;
        // D s_17_24: cast zx s_17_23 -> bv
        let s_17_24: Bits = Bits::new(s_17_23 as u128, 32u16);
        // D s_17_25: bit-extract s_17_24 s_17_21 s_17_22
        let s_17_25: Bits = (Bits::new(
            ((s_17_24) >> (s_17_21)).value(),
            u16::try_from(s_17_22).unwrap(),
        ));
        // D s_17_26: cast reint s_17_25 -> u16
        let s_17_26: u16 = (s_17_25.value() as u16);
        // D s_17_27: call decode_aarch32_instrs_LDM_A1enc_A_txt(s_17_8, s_17_14, s_17_20, s_17_26)
        let s_17_27: () = decode_aarch32_instrs_LDM_A1enc_A_txt(
            state,
            tracer,
            s_17_8,
            s_17_14,
            s_17_20,
            s_17_26,
        );
        // N s_17_28: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var merge#var.1:struct
        let s_18_0: u32 = fn_state.merge_var._1;
        // D s_18_1: write-var u#33892 <= s_18_0
        fn_state.u_33892 = s_18_0;
        // C s_18_2: const #22s : i
        let s_18_2: i128 = 22;
        // D s_18_3: read-var u#33892:u32
        let s_18_3: u32 = fn_state.u_33892;
        // D s_18_4: cast zx s_18_3 -> bv
        let s_18_4: Bits = Bits::new(s_18_3 as u128, 32u16);
        // C s_18_5: const #1s : i64
        let s_18_5: i64 = 1;
        // C s_18_6: cast zx s_18_5 -> i
        let s_18_6: i128 = (i128::try_from(s_18_5).unwrap());
        // C s_18_7: const #5s : i
        let s_18_7: i128 = 5;
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
        let s_18_11: Bits = Bits::new(s_18_10 as u128, 6u16);
        // C s_18_12: const #32u : u8
        let s_18_12: u8 = 32;
        // C s_18_13: cast zx s_18_12 -> bv
        let s_18_13: Bits = Bits::new(s_18_12 as u128, 6u16);
        // D s_18_14: cmp-eq s_18_11 s_18_13
        let s_18_14: bool = ((s_18_11) == (s_18_13));
        // N s_18_15: branch s_18_14 b233 b19
        if s_18_14 {
            return block_233(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#411427 <= s_19_0
        fn_state.gs_411427 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#411427:u8
        let s_20_0: bool = fn_state.gs_411427;
        // N s_20_1: branch s_20_0 b229 b21
        if s_20_0 {
            return block_229(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#411432 <= s_21_0
        fn_state.gs_411432 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#411432:u8
        let s_22_0: bool = fn_state.gs_411432;
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
        // C s_23_0: const #2919s : i
        let s_23_0: i128 = 2919;
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
        // D s_23_5: read-var u#33892:u32
        let s_23_5: u32 = fn_state.u_33892;
        // D s_23_6: cast zx s_23_5 -> bv
        let s_23_6: Bits = Bits::new(s_23_5 as u128, 32u16);
        // D s_23_7: bit-extract s_23_6 s_23_3 s_23_4
        let s_23_7: Bits = (Bits::new(
            ((s_23_6) >> (s_23_3)).value(),
            u16::try_from(s_23_4).unwrap(),
        ));
        // D s_23_8: cast reint s_23_7 -> u8
        let s_23_8: u8 = (s_23_7.value() as u8);
        // C s_23_9: const #21s : i
        let s_23_9: i128 = 21;
        // C s_23_10: const #1s : i
        let s_23_10: i128 = 1;
        // D s_23_11: read-var u#33892:u32
        let s_23_11: u32 = fn_state.u_33892;
        // D s_23_12: cast zx s_23_11 -> bv
        let s_23_12: Bits = Bits::new(s_23_11 as u128, 32u16);
        // D s_23_13: bit-extract s_23_12 s_23_9 s_23_10
        let s_23_13: Bits = (Bits::new(
            ((s_23_12) >> (s_23_9)).value(),
            u16::try_from(s_23_10).unwrap(),
        ));
        // D s_23_14: cast reint s_23_13 -> u8
        let s_23_14: bool = ((s_23_13.value()) != 0);
        // C s_23_15: const #16s : i
        let s_23_15: i128 = 16;
        // C s_23_16: const #4s : i
        let s_23_16: i128 = 4;
        // D s_23_17: read-var u#33892:u32
        let s_23_17: u32 = fn_state.u_33892;
        // D s_23_18: cast zx s_23_17 -> bv
        let s_23_18: Bits = Bits::new(s_23_17 as u128, 32u16);
        // D s_23_19: bit-extract s_23_18 s_23_15 s_23_16
        let s_23_19: Bits = (Bits::new(
            ((s_23_18) >> (s_23_15)).value(),
            u16::try_from(s_23_16).unwrap(),
        ));
        // D s_23_20: cast reint s_23_19 -> u8
        let s_23_20: u8 = (s_23_19.value() as u8);
        // C s_23_21: const #0s : i
        let s_23_21: i128 = 0;
        // C s_23_22: const #16s : i
        let s_23_22: i128 = 16;
        // D s_23_23: read-var u#33892:u32
        let s_23_23: u32 = fn_state.u_33892;
        // D s_23_24: cast zx s_23_23 -> bv
        let s_23_24: Bits = Bits::new(s_23_23 as u128, 32u16);
        // D s_23_25: bit-extract s_23_24 s_23_21 s_23_22
        let s_23_25: Bits = (Bits::new(
            ((s_23_24) >> (s_23_21)).value(),
            u16::try_from(s_23_22).unwrap(),
        ));
        // D s_23_26: cast reint s_23_25 -> u16
        let s_23_26: u16 = (s_23_25.value() as u16);
        // D s_23_27: call decode_aarch32_instrs_LDMDA_A1enc_A_txt(s_23_8, s_23_14, s_23_20, s_23_26)
        let s_23_27: () = decode_aarch32_instrs_LDMDA_A1enc_A_txt(
            state,
            tracer,
            s_23_8,
            s_23_14,
            s_23_20,
            s_23_26,
        );
        // N s_23_28: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var merge#var.1:struct
        let s_24_0: u32 = fn_state.merge_var._1;
        // D s_24_1: write-var u#33898 <= s_24_0
        fn_state.u_33898 = s_24_0;
        // C s_24_2: const #22s : i
        let s_24_2: i128 = 22;
        // D s_24_3: read-var u#33898:u32
        let s_24_3: u32 = fn_state.u_33898;
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 32u16);
        // C s_24_5: const #1s : i64
        let s_24_5: i64 = 1;
        // C s_24_6: cast zx s_24_5 -> i
        let s_24_6: i128 = (i128::try_from(s_24_5).unwrap());
        // C s_24_7: const #5s : i
        let s_24_7: i128 = 5;
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
        let s_24_11: Bits = Bits::new(s_24_10 as u128, 6u16);
        // C s_24_12: const #36u : u8
        let s_24_12: u8 = 36;
        // C s_24_13: cast zx s_24_12 -> bv
        let s_24_13: Bits = Bits::new(s_24_12 as u128, 6u16);
        // D s_24_14: cmp-eq s_24_11 s_24_13
        let s_24_14: bool = ((s_24_11) == (s_24_13));
        // N s_24_15: branch s_24_14 b228 b25
        if s_24_14 {
            return block_228(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#411448 <= s_25_0
        fn_state.gs_411448 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#411448:u8
        let s_26_0: bool = fn_state.gs_411448;
        // N s_26_1: branch s_26_0 b224 b27
        if s_26_0 {
            return block_224(state, tracer, fn_state);
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
        // D s_27_1: write-var gs#411453 <= s_27_0
        fn_state.gs_411453 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#411453:u8
        let s_28_0: bool = fn_state.gs_411453;
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
        // C s_29_0: const #2920s : i
        let s_29_0: i128 = 2920;
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
        // D s_29_5: read-var u#33898:u32
        let s_29_5: u32 = fn_state.u_33898;
        // D s_29_6: cast zx s_29_5 -> bv
        let s_29_6: Bits = Bits::new(s_29_5 as u128, 32u16);
        // D s_29_7: bit-extract s_29_6 s_29_3 s_29_4
        let s_29_7: Bits = (Bits::new(
            ((s_29_6) >> (s_29_3)).value(),
            u16::try_from(s_29_4).unwrap(),
        ));
        // D s_29_8: cast reint s_29_7 -> u8
        let s_29_8: u8 = (s_29_7.value() as u8);
        // C s_29_9: const #21s : i
        let s_29_9: i128 = 21;
        // C s_29_10: const #1s : i
        let s_29_10: i128 = 1;
        // D s_29_11: read-var u#33898:u32
        let s_29_11: u32 = fn_state.u_33898;
        // D s_29_12: cast zx s_29_11 -> bv
        let s_29_12: Bits = Bits::new(s_29_11 as u128, 32u16);
        // D s_29_13: bit-extract s_29_12 s_29_9 s_29_10
        let s_29_13: Bits = (Bits::new(
            ((s_29_12) >> (s_29_9)).value(),
            u16::try_from(s_29_10).unwrap(),
        ));
        // D s_29_14: cast reint s_29_13 -> u8
        let s_29_14: bool = ((s_29_13.value()) != 0);
        // C s_29_15: const #16s : i
        let s_29_15: i128 = 16;
        // C s_29_16: const #4s : i
        let s_29_16: i128 = 4;
        // D s_29_17: read-var u#33898:u32
        let s_29_17: u32 = fn_state.u_33898;
        // D s_29_18: cast zx s_29_17 -> bv
        let s_29_18: Bits = Bits::new(s_29_17 as u128, 32u16);
        // D s_29_19: bit-extract s_29_18 s_29_15 s_29_16
        let s_29_19: Bits = (Bits::new(
            ((s_29_18) >> (s_29_15)).value(),
            u16::try_from(s_29_16).unwrap(),
        ));
        // D s_29_20: cast reint s_29_19 -> u8
        let s_29_20: u8 = (s_29_19.value() as u8);
        // C s_29_21: const #0s : i
        let s_29_21: i128 = 0;
        // C s_29_22: const #16s : i
        let s_29_22: i128 = 16;
        // D s_29_23: read-var u#33898:u32
        let s_29_23: u32 = fn_state.u_33898;
        // D s_29_24: cast zx s_29_23 -> bv
        let s_29_24: Bits = Bits::new(s_29_23 as u128, 32u16);
        // D s_29_25: bit-extract s_29_24 s_29_21 s_29_22
        let s_29_25: Bits = (Bits::new(
            ((s_29_24) >> (s_29_21)).value(),
            u16::try_from(s_29_22).unwrap(),
        ));
        // D s_29_26: cast reint s_29_25 -> u16
        let s_29_26: u16 = (s_29_25.value() as u16);
        // D s_29_27: call decode_aarch32_instrs_LDMDB_A1enc_A_txt(s_29_8, s_29_14, s_29_20, s_29_26)
        let s_29_27: () = decode_aarch32_instrs_LDMDB_A1enc_A_txt(
            state,
            tracer,
            s_29_8,
            s_29_14,
            s_29_20,
            s_29_26,
        );
        // N s_29_28: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var merge#var.1:struct
        let s_30_0: u32 = fn_state.merge_var._1;
        // D s_30_1: write-var u#33904 <= s_30_0
        fn_state.u_33904 = s_30_0;
        // C s_30_2: const #22s : i
        let s_30_2: i128 = 22;
        // D s_30_3: read-var u#33904:u32
        let s_30_3: u32 = fn_state.u_33904;
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 32u16);
        // C s_30_5: const #1s : i64
        let s_30_5: i64 = 1;
        // C s_30_6: cast zx s_30_5 -> i
        let s_30_6: i128 = (i128::try_from(s_30_5).unwrap());
        // C s_30_7: const #5s : i
        let s_30_7: i128 = 5;
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
        let s_30_11: Bits = Bits::new(s_30_10 as u128, 6u16);
        // C s_30_12: const #38u : u8
        let s_30_12: u8 = 38;
        // C s_30_13: cast zx s_30_12 -> bv
        let s_30_13: Bits = Bits::new(s_30_12 as u128, 6u16);
        // D s_30_14: cmp-eq s_30_11 s_30_13
        let s_30_14: bool = ((s_30_11) == (s_30_13));
        // N s_30_15: branch s_30_14 b223 b31
        if s_30_14 {
            return block_223(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#411469 <= s_31_0
        fn_state.gs_411469 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#411469:u8
        let s_32_0: bool = fn_state.gs_411469;
        // N s_32_1: branch s_32_0 b219 b33
        if s_32_0 {
            return block_219(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#411474 <= s_33_0
        fn_state.gs_411474 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#411474:u8
        let s_34_0: bool = fn_state.gs_411474;
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
        // C s_35_0: const #2922s : i
        let s_35_0: i128 = 2922;
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
        // D s_35_5: read-var u#33904:u32
        let s_35_5: u32 = fn_state.u_33904;
        // D s_35_6: cast zx s_35_5 -> bv
        let s_35_6: Bits = Bits::new(s_35_5 as u128, 32u16);
        // D s_35_7: bit-extract s_35_6 s_35_3 s_35_4
        let s_35_7: Bits = (Bits::new(
            ((s_35_6) >> (s_35_3)).value(),
            u16::try_from(s_35_4).unwrap(),
        ));
        // D s_35_8: cast reint s_35_7 -> u8
        let s_35_8: u8 = (s_35_7.value() as u8);
        // C s_35_9: const #21s : i
        let s_35_9: i128 = 21;
        // C s_35_10: const #1s : i
        let s_35_10: i128 = 1;
        // D s_35_11: read-var u#33904:u32
        let s_35_11: u32 = fn_state.u_33904;
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
        // D s_35_17: read-var u#33904:u32
        let s_35_17: u32 = fn_state.u_33904;
        // D s_35_18: cast zx s_35_17 -> bv
        let s_35_18: Bits = Bits::new(s_35_17 as u128, 32u16);
        // D s_35_19: bit-extract s_35_18 s_35_15 s_35_16
        let s_35_19: Bits = (Bits::new(
            ((s_35_18) >> (s_35_15)).value(),
            u16::try_from(s_35_16).unwrap(),
        ));
        // D s_35_20: cast reint s_35_19 -> u8
        let s_35_20: u8 = (s_35_19.value() as u8);
        // C s_35_21: const #0s : i
        let s_35_21: i128 = 0;
        // C s_35_22: const #16s : i
        let s_35_22: i128 = 16;
        // D s_35_23: read-var u#33904:u32
        let s_35_23: u32 = fn_state.u_33904;
        // D s_35_24: cast zx s_35_23 -> bv
        let s_35_24: Bits = Bits::new(s_35_23 as u128, 32u16);
        // D s_35_25: bit-extract s_35_24 s_35_21 s_35_22
        let s_35_25: Bits = (Bits::new(
            ((s_35_24) >> (s_35_21)).value(),
            u16::try_from(s_35_22).unwrap(),
        ));
        // D s_35_26: cast reint s_35_25 -> u16
        let s_35_26: u16 = (s_35_25.value() as u16);
        // D s_35_27: call decode_aarch32_instrs_LDMIB_A1enc_A_txt(s_35_8, s_35_14, s_35_20, s_35_26)
        let s_35_27: () = decode_aarch32_instrs_LDMIB_A1enc_A_txt(
            state,
            tracer,
            s_35_8,
            s_35_14,
            s_35_20,
            s_35_26,
        );
        // N s_35_28: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var merge#var.1:struct
        let s_36_0: u32 = fn_state.merge_var._1;
        // D s_36_1: write-var u#33910 <= s_36_0
        fn_state.u_33910 = s_36_0;
        // C s_36_2: const #22s : i
        let s_36_2: i128 = 22;
        // D s_36_3: read-var u#33910:u32
        let s_36_3: u32 = fn_state.u_33910;
        // D s_36_4: cast zx s_36_3 -> bv
        let s_36_4: Bits = Bits::new(s_36_3 as u128, 32u16);
        // C s_36_5: const #1s : i64
        let s_36_5: i64 = 1;
        // C s_36_6: cast zx s_36_5 -> i
        let s_36_6: i128 = (i128::try_from(s_36_5).unwrap());
        // C s_36_7: const #5s : i
        let s_36_7: i128 = 5;
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
        let s_36_11: Bits = Bits::new(s_36_10 as u128, 6u16);
        // C s_36_12: const #34u : u8
        let s_36_12: u8 = 34;
        // C s_36_13: cast zx s_36_12 -> bv
        let s_36_13: Bits = Bits::new(s_36_12 as u128, 6u16);
        // D s_36_14: cmp-eq s_36_11 s_36_13
        let s_36_14: bool = ((s_36_11) == (s_36_13));
        // N s_36_15: branch s_36_14 b218 b37
        if s_36_14 {
            return block_218(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#411490 <= s_37_0
        fn_state.gs_411490 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#411490:u8
        let s_38_0: bool = fn_state.gs_411490;
        // N s_38_1: branch s_38_0 b214 b39
        if s_38_0 {
            return block_214(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#411495 <= s_39_0
        fn_state.gs_411495 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#411495:u8
        let s_40_0: bool = fn_state.gs_411495;
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
        // C s_41_0: const #3186s : i
        let s_41_0: i128 = 3186;
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
        // D s_41_5: read-var u#33910:u32
        let s_41_5: u32 = fn_state.u_33910;
        // D s_41_6: cast zx s_41_5 -> bv
        let s_41_6: Bits = Bits::new(s_41_5 as u128, 32u16);
        // D s_41_7: bit-extract s_41_6 s_41_3 s_41_4
        let s_41_7: Bits = (Bits::new(
            ((s_41_6) >> (s_41_3)).value(),
            u16::try_from(s_41_4).unwrap(),
        ));
        // D s_41_8: cast reint s_41_7 -> u8
        let s_41_8: u8 = (s_41_7.value() as u8);
        // C s_41_9: const #21s : i
        let s_41_9: i128 = 21;
        // C s_41_10: const #1s : i
        let s_41_10: i128 = 1;
        // D s_41_11: read-var u#33910:u32
        let s_41_11: u32 = fn_state.u_33910;
        // D s_41_12: cast zx s_41_11 -> bv
        let s_41_12: Bits = Bits::new(s_41_11 as u128, 32u16);
        // D s_41_13: bit-extract s_41_12 s_41_9 s_41_10
        let s_41_13: Bits = (Bits::new(
            ((s_41_12) >> (s_41_9)).value(),
            u16::try_from(s_41_10).unwrap(),
        ));
        // D s_41_14: cast reint s_41_13 -> u8
        let s_41_14: bool = ((s_41_13.value()) != 0);
        // C s_41_15: const #16s : i
        let s_41_15: i128 = 16;
        // C s_41_16: const #4s : i
        let s_41_16: i128 = 4;
        // D s_41_17: read-var u#33910:u32
        let s_41_17: u32 = fn_state.u_33910;
        // D s_41_18: cast zx s_41_17 -> bv
        let s_41_18: Bits = Bits::new(s_41_17 as u128, 32u16);
        // D s_41_19: bit-extract s_41_18 s_41_15 s_41_16
        let s_41_19: Bits = (Bits::new(
            ((s_41_18) >> (s_41_15)).value(),
            u16::try_from(s_41_16).unwrap(),
        ));
        // D s_41_20: cast reint s_41_19 -> u8
        let s_41_20: u8 = (s_41_19.value() as u8);
        // C s_41_21: const #0s : i
        let s_41_21: i128 = 0;
        // C s_41_22: const #16s : i
        let s_41_22: i128 = 16;
        // D s_41_23: read-var u#33910:u32
        let s_41_23: u32 = fn_state.u_33910;
        // D s_41_24: cast zx s_41_23 -> bv
        let s_41_24: Bits = Bits::new(s_41_23 as u128, 32u16);
        // D s_41_25: bit-extract s_41_24 s_41_21 s_41_22
        let s_41_25: Bits = (Bits::new(
            ((s_41_24) >> (s_41_21)).value(),
            u16::try_from(s_41_22).unwrap(),
        ));
        // D s_41_26: cast reint s_41_25 -> u16
        let s_41_26: u16 = (s_41_25.value() as u16);
        // D s_41_27: call decode_aarch32_instrs_STM_A1enc_A_txt(s_41_8, s_41_14, s_41_20, s_41_26)
        let s_41_27: () = decode_aarch32_instrs_STM_A1enc_A_txt(
            state,
            tracer,
            s_41_8,
            s_41_14,
            s_41_20,
            s_41_26,
        );
        // N s_41_28: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var merge#var.1:struct
        let s_42_0: u32 = fn_state.merge_var._1;
        // D s_42_1: write-var u#33916 <= s_42_0
        fn_state.u_33916 = s_42_0;
        // C s_42_2: const #22s : i
        let s_42_2: i128 = 22;
        // D s_42_3: read-var u#33916:u32
        let s_42_3: u32 = fn_state.u_33916;
        // D s_42_4: cast zx s_42_3 -> bv
        let s_42_4: Bits = Bits::new(s_42_3 as u128, 32u16);
        // C s_42_5: const #1s : i64
        let s_42_5: i64 = 1;
        // C s_42_6: cast zx s_42_5 -> i
        let s_42_6: i128 = (i128::try_from(s_42_5).unwrap());
        // C s_42_7: const #5s : i
        let s_42_7: i128 = 5;
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
        let s_42_11: Bits = Bits::new(s_42_10 as u128, 6u16);
        // C s_42_12: const #32u : u8
        let s_42_12: u8 = 32;
        // C s_42_13: cast zx s_42_12 -> bv
        let s_42_13: Bits = Bits::new(s_42_12 as u128, 6u16);
        // D s_42_14: cmp-eq s_42_11 s_42_13
        let s_42_14: bool = ((s_42_11) == (s_42_13));
        // N s_42_15: branch s_42_14 b213 b43
        if s_42_14 {
            return block_213(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#411511 <= s_43_0
        fn_state.gs_411511 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#411511:u8
        let s_44_0: bool = fn_state.gs_411511;
        // N s_44_1: branch s_44_0 b209 b45
        if s_44_0 {
            return block_209(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#411516 <= s_45_0
        fn_state.gs_411516 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#411516:u8
        let s_46_0: bool = fn_state.gs_411516;
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
        // C s_47_0: const #3189s : i
        let s_47_0: i128 = 3189;
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
        // D s_47_5: read-var u#33916:u32
        let s_47_5: u32 = fn_state.u_33916;
        // D s_47_6: cast zx s_47_5 -> bv
        let s_47_6: Bits = Bits::new(s_47_5 as u128, 32u16);
        // D s_47_7: bit-extract s_47_6 s_47_3 s_47_4
        let s_47_7: Bits = (Bits::new(
            ((s_47_6) >> (s_47_3)).value(),
            u16::try_from(s_47_4).unwrap(),
        ));
        // D s_47_8: cast reint s_47_7 -> u8
        let s_47_8: u8 = (s_47_7.value() as u8);
        // C s_47_9: const #21s : i
        let s_47_9: i128 = 21;
        // C s_47_10: const #1s : i
        let s_47_10: i128 = 1;
        // D s_47_11: read-var u#33916:u32
        let s_47_11: u32 = fn_state.u_33916;
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
        // D s_47_17: read-var u#33916:u32
        let s_47_17: u32 = fn_state.u_33916;
        // D s_47_18: cast zx s_47_17 -> bv
        let s_47_18: Bits = Bits::new(s_47_17 as u128, 32u16);
        // D s_47_19: bit-extract s_47_18 s_47_15 s_47_16
        let s_47_19: Bits = (Bits::new(
            ((s_47_18) >> (s_47_15)).value(),
            u16::try_from(s_47_16).unwrap(),
        ));
        // D s_47_20: cast reint s_47_19 -> u8
        let s_47_20: u8 = (s_47_19.value() as u8);
        // C s_47_21: const #0s : i
        let s_47_21: i128 = 0;
        // C s_47_22: const #16s : i
        let s_47_22: i128 = 16;
        // D s_47_23: read-var u#33916:u32
        let s_47_23: u32 = fn_state.u_33916;
        // D s_47_24: cast zx s_47_23 -> bv
        let s_47_24: Bits = Bits::new(s_47_23 as u128, 32u16);
        // D s_47_25: bit-extract s_47_24 s_47_21 s_47_22
        let s_47_25: Bits = (Bits::new(
            ((s_47_24) >> (s_47_21)).value(),
            u16::try_from(s_47_22).unwrap(),
        ));
        // D s_47_26: cast reint s_47_25 -> u16
        let s_47_26: u16 = (s_47_25.value() as u16);
        // D s_47_27: call decode_aarch32_instrs_STMDA_A1enc_A_txt(s_47_8, s_47_14, s_47_20, s_47_26)
        let s_47_27: () = decode_aarch32_instrs_STMDA_A1enc_A_txt(
            state,
            tracer,
            s_47_8,
            s_47_14,
            s_47_20,
            s_47_26,
        );
        // N s_47_28: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var merge#var.1:struct
        let s_48_0: u32 = fn_state.merge_var._1;
        // D s_48_1: write-var u#33922 <= s_48_0
        fn_state.u_33922 = s_48_0;
        // C s_48_2: const #22s : i
        let s_48_2: i128 = 22;
        // D s_48_3: read-var u#33922:u32
        let s_48_3: u32 = fn_state.u_33922;
        // D s_48_4: cast zx s_48_3 -> bv
        let s_48_4: Bits = Bits::new(s_48_3 as u128, 32u16);
        // C s_48_5: const #1s : i64
        let s_48_5: i64 = 1;
        // C s_48_6: cast zx s_48_5 -> i
        let s_48_6: i128 = (i128::try_from(s_48_5).unwrap());
        // C s_48_7: const #5s : i
        let s_48_7: i128 = 5;
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
        let s_48_11: Bits = Bits::new(s_48_10 as u128, 6u16);
        // C s_48_12: const #36u : u8
        let s_48_12: u8 = 36;
        // C s_48_13: cast zx s_48_12 -> bv
        let s_48_13: Bits = Bits::new(s_48_12 as u128, 6u16);
        // D s_48_14: cmp-eq s_48_11 s_48_13
        let s_48_14: bool = ((s_48_11) == (s_48_13));
        // N s_48_15: branch s_48_14 b208 b49
        if s_48_14 {
            return block_208(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#411532 <= s_49_0
        fn_state.gs_411532 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#411532:u8
        let s_50_0: bool = fn_state.gs_411532;
        // N s_50_1: branch s_50_0 b204 b51
        if s_50_0 {
            return block_204(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#411537 <= s_51_0
        fn_state.gs_411537 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#411537:u8
        let s_52_0: bool = fn_state.gs_411537;
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
        // C s_53_0: const #3190s : i
        let s_53_0: i128 = 3190;
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
        // D s_53_5: read-var u#33922:u32
        let s_53_5: u32 = fn_state.u_33922;
        // D s_53_6: cast zx s_53_5 -> bv
        let s_53_6: Bits = Bits::new(s_53_5 as u128, 32u16);
        // D s_53_7: bit-extract s_53_6 s_53_3 s_53_4
        let s_53_7: Bits = (Bits::new(
            ((s_53_6) >> (s_53_3)).value(),
            u16::try_from(s_53_4).unwrap(),
        ));
        // D s_53_8: cast reint s_53_7 -> u8
        let s_53_8: u8 = (s_53_7.value() as u8);
        // C s_53_9: const #21s : i
        let s_53_9: i128 = 21;
        // C s_53_10: const #1s : i
        let s_53_10: i128 = 1;
        // D s_53_11: read-var u#33922:u32
        let s_53_11: u32 = fn_state.u_33922;
        // D s_53_12: cast zx s_53_11 -> bv
        let s_53_12: Bits = Bits::new(s_53_11 as u128, 32u16);
        // D s_53_13: bit-extract s_53_12 s_53_9 s_53_10
        let s_53_13: Bits = (Bits::new(
            ((s_53_12) >> (s_53_9)).value(),
            u16::try_from(s_53_10).unwrap(),
        ));
        // D s_53_14: cast reint s_53_13 -> u8
        let s_53_14: bool = ((s_53_13.value()) != 0);
        // C s_53_15: const #16s : i
        let s_53_15: i128 = 16;
        // C s_53_16: const #4s : i
        let s_53_16: i128 = 4;
        // D s_53_17: read-var u#33922:u32
        let s_53_17: u32 = fn_state.u_33922;
        // D s_53_18: cast zx s_53_17 -> bv
        let s_53_18: Bits = Bits::new(s_53_17 as u128, 32u16);
        // D s_53_19: bit-extract s_53_18 s_53_15 s_53_16
        let s_53_19: Bits = (Bits::new(
            ((s_53_18) >> (s_53_15)).value(),
            u16::try_from(s_53_16).unwrap(),
        ));
        // D s_53_20: cast reint s_53_19 -> u8
        let s_53_20: u8 = (s_53_19.value() as u8);
        // C s_53_21: const #0s : i
        let s_53_21: i128 = 0;
        // C s_53_22: const #16s : i
        let s_53_22: i128 = 16;
        // D s_53_23: read-var u#33922:u32
        let s_53_23: u32 = fn_state.u_33922;
        // D s_53_24: cast zx s_53_23 -> bv
        let s_53_24: Bits = Bits::new(s_53_23 as u128, 32u16);
        // D s_53_25: bit-extract s_53_24 s_53_21 s_53_22
        let s_53_25: Bits = (Bits::new(
            ((s_53_24) >> (s_53_21)).value(),
            u16::try_from(s_53_22).unwrap(),
        ));
        // D s_53_26: cast reint s_53_25 -> u16
        let s_53_26: u16 = (s_53_25.value() as u16);
        // D s_53_27: call decode_aarch32_instrs_STMDB_A1enc_A_txt(s_53_8, s_53_14, s_53_20, s_53_26)
        let s_53_27: () = decode_aarch32_instrs_STMDB_A1enc_A_txt(
            state,
            tracer,
            s_53_8,
            s_53_14,
            s_53_20,
            s_53_26,
        );
        // N s_53_28: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var merge#var.1:struct
        let s_54_0: u32 = fn_state.merge_var._1;
        // D s_54_1: write-var u#33928 <= s_54_0
        fn_state.u_33928 = s_54_0;
        // C s_54_2: const #22s : i
        let s_54_2: i128 = 22;
        // D s_54_3: read-var u#33928:u32
        let s_54_3: u32 = fn_state.u_33928;
        // D s_54_4: cast zx s_54_3 -> bv
        let s_54_4: Bits = Bits::new(s_54_3 as u128, 32u16);
        // C s_54_5: const #1s : i64
        let s_54_5: i64 = 1;
        // C s_54_6: cast zx s_54_5 -> i
        let s_54_6: i128 = (i128::try_from(s_54_5).unwrap());
        // C s_54_7: const #5s : i
        let s_54_7: i128 = 5;
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
        let s_54_11: Bits = Bits::new(s_54_10 as u128, 6u16);
        // C s_54_12: const #38u : u8
        let s_54_12: u8 = 38;
        // C s_54_13: cast zx s_54_12 -> bv
        let s_54_13: Bits = Bits::new(s_54_12 as u128, 6u16);
        // D s_54_14: cmp-eq s_54_11 s_54_13
        let s_54_14: bool = ((s_54_11) == (s_54_13));
        // N s_54_15: branch s_54_14 b203 b55
        if s_54_14 {
            return block_203(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#411553 <= s_55_0
        fn_state.gs_411553 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#411553:u8
        let s_56_0: bool = fn_state.gs_411553;
        // N s_56_1: branch s_56_0 b199 b57
        if s_56_0 {
            return block_199(state, tracer, fn_state);
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
        // D s_57_1: write-var gs#411558 <= s_57_0
        fn_state.gs_411558 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#411558:u8
        let s_58_0: bool = fn_state.gs_411558;
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
        // C s_59_0: const #3192s : i
        let s_59_0: i128 = 3192;
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
        // D s_59_5: read-var u#33928:u32
        let s_59_5: u32 = fn_state.u_33928;
        // D s_59_6: cast zx s_59_5 -> bv
        let s_59_6: Bits = Bits::new(s_59_5 as u128, 32u16);
        // D s_59_7: bit-extract s_59_6 s_59_3 s_59_4
        let s_59_7: Bits = (Bits::new(
            ((s_59_6) >> (s_59_3)).value(),
            u16::try_from(s_59_4).unwrap(),
        ));
        // D s_59_8: cast reint s_59_7 -> u8
        let s_59_8: u8 = (s_59_7.value() as u8);
        // C s_59_9: const #21s : i
        let s_59_9: i128 = 21;
        // C s_59_10: const #1s : i
        let s_59_10: i128 = 1;
        // D s_59_11: read-var u#33928:u32
        let s_59_11: u32 = fn_state.u_33928;
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
        // D s_59_17: read-var u#33928:u32
        let s_59_17: u32 = fn_state.u_33928;
        // D s_59_18: cast zx s_59_17 -> bv
        let s_59_18: Bits = Bits::new(s_59_17 as u128, 32u16);
        // D s_59_19: bit-extract s_59_18 s_59_15 s_59_16
        let s_59_19: Bits = (Bits::new(
            ((s_59_18) >> (s_59_15)).value(),
            u16::try_from(s_59_16).unwrap(),
        ));
        // D s_59_20: cast reint s_59_19 -> u8
        let s_59_20: u8 = (s_59_19.value() as u8);
        // C s_59_21: const #0s : i
        let s_59_21: i128 = 0;
        // C s_59_22: const #16s : i
        let s_59_22: i128 = 16;
        // D s_59_23: read-var u#33928:u32
        let s_59_23: u32 = fn_state.u_33928;
        // D s_59_24: cast zx s_59_23 -> bv
        let s_59_24: Bits = Bits::new(s_59_23 as u128, 32u16);
        // D s_59_25: bit-extract s_59_24 s_59_21 s_59_22
        let s_59_25: Bits = (Bits::new(
            ((s_59_24) >> (s_59_21)).value(),
            u16::try_from(s_59_22).unwrap(),
        ));
        // D s_59_26: cast reint s_59_25 -> u16
        let s_59_26: u16 = (s_59_25.value() as u16);
        // D s_59_27: call decode_aarch32_instrs_STMIB_A1enc_A_txt(s_59_8, s_59_14, s_59_20, s_59_26)
        let s_59_27: () = decode_aarch32_instrs_STMIB_A1enc_A_txt(
            state,
            tracer,
            s_59_8,
            s_59_14,
            s_59_20,
            s_59_26,
        );
        // N s_59_28: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var merge#var.1:struct
        let s_60_0: u32 = fn_state.merge_var._1;
        // D s_60_1: write-var u#33934 <= s_60_0
        fn_state.u_33934 = s_60_0;
        // C s_60_2: const #25s : i
        let s_60_2: i128 = 25;
        // D s_60_3: read-var u#33934:u32
        let s_60_3: u32 = fn_state.u_33934;
        // D s_60_4: cast zx s_60_3 -> bv
        let s_60_4: Bits = Bits::new(s_60_3 as u128, 32u16);
        // C s_60_5: const #1s : i64
        let s_60_5: i64 = 1;
        // C s_60_6: cast zx s_60_5 -> i
        let s_60_6: i128 = (i128::try_from(s_60_5).unwrap());
        // C s_60_7: const #2s : i
        let s_60_7: i128 = 2;
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
        let s_60_11: Bits = Bits::new(s_60_10 as u128, 3u16);
        // C s_60_12: const #4u : u8
        let s_60_12: u8 = 4;
        // C s_60_13: cast zx s_60_12 -> bv
        let s_60_13: Bits = Bits::new(s_60_12 as u128, 3u16);
        // D s_60_14: cmp-eq s_60_11 s_60_13
        let s_60_14: bool = ((s_60_11) == (s_60_13));
        // N s_60_15: branch s_60_14 b192 b61
        if s_60_14 {
            return block_192(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#411580 <= s_61_0
        fn_state.gs_411580 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#411580:u8
        let s_62_0: bool = fn_state.gs_411580;
        // N s_62_1: branch s_62_0 b188 b63
        if s_62_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#411585 <= s_63_0
        fn_state.gs_411585 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#411585:u8
        let s_64_0: bool = fn_state.gs_411585;
        // D s_64_1: not s_64_0
        let s_64_1: bool = !s_64_0;
        // N s_64_2: branch s_64_1 b66 b65
        if s_64_1 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #3776s : i
        let s_65_0: i128 = 3776;
        // C s_65_1: const #14696u : u32
        let s_65_1: u32 = 14696;
        // N s_65_2: write-reg s_65_1 <= s_65_0
        let s_65_2: () = {
            state.write_register::<i128>(s_65_1 as isize, s_65_0);
            tracer.write_register(s_65_1 as isize, s_65_0);
        };
        // C s_65_3: const #28s : i
        let s_65_3: i128 = 28;
        // C s_65_4: const #4s : i
        let s_65_4: i128 = 4;
        // D s_65_5: read-var u#33934:u32
        let s_65_5: u32 = fn_state.u_33934;
        // D s_65_6: cast zx s_65_5 -> bv
        let s_65_6: Bits = Bits::new(s_65_5 as u128, 32u16);
        // D s_65_7: bit-extract s_65_6 s_65_3 s_65_4
        let s_65_7: Bits = (Bits::new(
            ((s_65_6) >> (s_65_3)).value(),
            u16::try_from(s_65_4).unwrap(),
        ));
        // D s_65_8: cast reint s_65_7 -> u8
        let s_65_8: u8 = (s_65_7.value() as u8);
        // C s_65_9: const #24s : i
        let s_65_9: i128 = 24;
        // C s_65_10: const #1s : i
        let s_65_10: i128 = 1;
        // D s_65_11: read-var u#33934:u32
        let s_65_11: u32 = fn_state.u_33934;
        // D s_65_12: cast zx s_65_11 -> bv
        let s_65_12: Bits = Bits::new(s_65_11 as u128, 32u16);
        // D s_65_13: bit-extract s_65_12 s_65_9 s_65_10
        let s_65_13: Bits = (Bits::new(
            ((s_65_12) >> (s_65_9)).value(),
            u16::try_from(s_65_10).unwrap(),
        ));
        // D s_65_14: cast reint s_65_13 -> u8
        let s_65_14: bool = ((s_65_13.value()) != 0);
        // C s_65_15: const #23s : i
        let s_65_15: i128 = 23;
        // C s_65_16: const #1s : i
        let s_65_16: i128 = 1;
        // D s_65_17: read-var u#33934:u32
        let s_65_17: u32 = fn_state.u_33934;
        // D s_65_18: cast zx s_65_17 -> bv
        let s_65_18: Bits = Bits::new(s_65_17 as u128, 32u16);
        // D s_65_19: bit-extract s_65_18 s_65_15 s_65_16
        let s_65_19: Bits = (Bits::new(
            ((s_65_18) >> (s_65_15)).value(),
            u16::try_from(s_65_16).unwrap(),
        ));
        // D s_65_20: cast reint s_65_19 -> u8
        let s_65_20: bool = ((s_65_19.value()) != 0);
        // C s_65_21: const #21s : i
        let s_65_21: i128 = 21;
        // C s_65_22: const #1s : i
        let s_65_22: i128 = 1;
        // D s_65_23: read-var u#33934:u32
        let s_65_23: u32 = fn_state.u_33934;
        // D s_65_24: cast zx s_65_23 -> bv
        let s_65_24: Bits = Bits::new(s_65_23 as u128, 32u16);
        // D s_65_25: bit-extract s_65_24 s_65_21 s_65_22
        let s_65_25: Bits = (Bits::new(
            ((s_65_24) >> (s_65_21)).value(),
            u16::try_from(s_65_22).unwrap(),
        ));
        // D s_65_26: cast reint s_65_25 -> u8
        let s_65_26: bool = ((s_65_25.value()) != 0);
        // C s_65_27: const #16s : i
        let s_65_27: i128 = 16;
        // C s_65_28: const #4s : i
        let s_65_28: i128 = 4;
        // D s_65_29: read-var u#33934:u32
        let s_65_29: u32 = fn_state.u_33934;
        // D s_65_30: cast zx s_65_29 -> bv
        let s_65_30: Bits = Bits::new(s_65_29 as u128, 32u16);
        // D s_65_31: bit-extract s_65_30 s_65_27 s_65_28
        let s_65_31: Bits = (Bits::new(
            ((s_65_30) >> (s_65_27)).value(),
            u16::try_from(s_65_28).unwrap(),
        ));
        // D s_65_32: cast reint s_65_31 -> u8
        let s_65_32: u8 = (s_65_31.value() as u8);
        // C s_65_33: const #0s : i
        let s_65_33: i128 = 0;
        // C s_65_34: const #15s : i
        let s_65_34: i128 = 15;
        // D s_65_35: read-var u#33934:u32
        let s_65_35: u32 = fn_state.u_33934;
        // D s_65_36: cast zx s_65_35 -> bv
        let s_65_36: Bits = Bits::new(s_65_35 as u128, 32u16);
        // D s_65_37: bit-extract s_65_36 s_65_33 s_65_34
        let s_65_37: Bits = (Bits::new(
            ((s_65_36) >> (s_65_33)).value(),
            u16::try_from(s_65_34).unwrap(),
        ));
        // D s_65_38: cast reint s_65_37 -> u15
        let s_65_38: u16 = (s_65_37.value() as u16);
        // D s_65_39: call decode_aarch32_instrs_LDM_e_A1enc_AS_txt(s_65_8, s_65_14, s_65_20, s_65_26, s_65_32, s_65_38)
        let s_65_39: () = decode_aarch32_instrs_LDM_e_A1enc_AS_txt(
            state,
            tracer,
            s_65_8,
            s_65_14,
            s_65_20,
            s_65_26,
            s_65_32,
            s_65_38,
        );
        // N s_65_40: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var merge#var.1:struct
        let s_66_0: u32 = fn_state.merge_var._1;
        // D s_66_1: write-var u#33940 <= s_66_0
        fn_state.u_33940 = s_66_0;
        // C s_66_2: const #25s : i
        let s_66_2: i128 = 25;
        // D s_66_3: read-var u#33940:u32
        let s_66_3: u32 = fn_state.u_33940;
        // D s_66_4: cast zx s_66_3 -> bv
        let s_66_4: Bits = Bits::new(s_66_3 as u128, 32u16);
        // C s_66_5: const #1s : i64
        let s_66_5: i64 = 1;
        // C s_66_6: cast zx s_66_5 -> i
        let s_66_6: i128 = (i128::try_from(s_66_5).unwrap());
        // C s_66_7: const #2s : i
        let s_66_7: i128 = 2;
        // C s_66_8: add s_66_7 s_66_6
        let s_66_8: i128 = (s_66_7 + s_66_6);
        // D s_66_9: bit-extract s_66_4 s_66_2 s_66_8
        let s_66_9: Bits = (Bits::new(
            ((s_66_4) >> (s_66_2)).value(),
            u16::try_from(s_66_8).unwrap(),
        ));
        // D s_66_10: cast reint s_66_9 -> u8
        let s_66_10: u8 = (s_66_9.value() as u8);
        // D s_66_11: cast zx s_66_10 -> bv
        let s_66_11: Bits = Bits::new(s_66_10 as u128, 3u16);
        // C s_66_12: const #4u : u8
        let s_66_12: u8 = 4;
        // C s_66_13: cast zx s_66_12 -> bv
        let s_66_13: Bits = Bits::new(s_66_12 as u128, 3u16);
        // D s_66_14: cmp-eq s_66_11 s_66_13
        let s_66_14: bool = ((s_66_11) == (s_66_13));
        // N s_66_15: branch s_66_14 b184 b67
        if s_66_14 {
            return block_184(state, tracer, fn_state);
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
        // D s_67_1: write-var gs#411608 <= s_67_0
        fn_state.gs_411608 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#411608:u8
        let s_68_0: bool = fn_state.gs_411608;
        // N s_68_1: branch s_68_0 b180 b69
        if s_68_0 {
            return block_180(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#411613 <= s_69_0
        fn_state.gs_411613 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#411613:u8
        let s_70_0: bool = fn_state.gs_411613;
        // D s_70_1: not s_70_0
        let s_70_1: bool = !s_70_0;
        // N s_70_2: branch s_70_1 b74 b71
        if s_70_1 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #3777s : i
        let s_71_0: i128 = 3777;
        // C s_71_1: const #14696u : u32
        let s_71_1: u32 = 14696;
        // N s_71_2: write-reg s_71_1 <= s_71_0
        let s_71_2: () = {
            state.write_register::<i128>(s_71_1 as isize, s_71_0);
            tracer.write_register(s_71_1 as isize, s_71_0);
        };
        // C s_71_3: const #28s : i
        let s_71_3: i128 = 28;
        // C s_71_4: const #4s : i
        let s_71_4: i128 = 4;
        // D s_71_5: read-var u#33940:u32
        let s_71_5: u32 = fn_state.u_33940;
        // D s_71_6: cast zx s_71_5 -> bv
        let s_71_6: Bits = Bits::new(s_71_5 as u128, 32u16);
        // D s_71_7: bit-extract s_71_6 s_71_3 s_71_4
        let s_71_7: Bits = (Bits::new(
            ((s_71_6) >> (s_71_3)).value(),
            u16::try_from(s_71_4).unwrap(),
        ));
        // D s_71_8: cast reint s_71_7 -> u8
        let s_71_8: u8 = (s_71_7.value() as u8);
        // D s_71_9: write-var u#33941 <= s_71_8
        fn_state.u_33941 = s_71_8;
        // C s_71_10: const #24s : i
        let s_71_10: i128 = 24;
        // C s_71_11: const #1s : i
        let s_71_11: i128 = 1;
        // D s_71_12: read-var u#33940:u32
        let s_71_12: u32 = fn_state.u_33940;
        // D s_71_13: cast zx s_71_12 -> bv
        let s_71_13: Bits = Bits::new(s_71_12 as u128, 32u16);
        // D s_71_14: bit-extract s_71_13 s_71_10 s_71_11
        let s_71_14: Bits = (Bits::new(
            ((s_71_13) >> (s_71_10)).value(),
            u16::try_from(s_71_11).unwrap(),
        ));
        // D s_71_15: cast reint s_71_14 -> u8
        let s_71_15: bool = ((s_71_14.value()) != 0);
        // D s_71_16: write-var u#33942 <= s_71_15
        fn_state.u_33942 = s_71_15;
        // C s_71_17: const #23s : i
        let s_71_17: i128 = 23;
        // C s_71_18: const #1s : i
        let s_71_18: i128 = 1;
        // D s_71_19: read-var u#33940:u32
        let s_71_19: u32 = fn_state.u_33940;
        // D s_71_20: cast zx s_71_19 -> bv
        let s_71_20: Bits = Bits::new(s_71_19 as u128, 32u16);
        // D s_71_21: bit-extract s_71_20 s_71_17 s_71_18
        let s_71_21: Bits = (Bits::new(
            ((s_71_20) >> (s_71_17)).value(),
            u16::try_from(s_71_18).unwrap(),
        ));
        // D s_71_22: cast reint s_71_21 -> u8
        let s_71_22: bool = ((s_71_21.value()) != 0);
        // D s_71_23: write-var u#33943 <= s_71_22
        fn_state.u_33943 = s_71_22;
        // C s_71_24: const #16s : i
        let s_71_24: i128 = 16;
        // C s_71_25: const #4s : i
        let s_71_25: i128 = 4;
        // D s_71_26: read-var u#33940:u32
        let s_71_26: u32 = fn_state.u_33940;
        // D s_71_27: cast zx s_71_26 -> bv
        let s_71_27: Bits = Bits::new(s_71_26 as u128, 32u16);
        // D s_71_28: bit-extract s_71_27 s_71_24 s_71_25
        let s_71_28: Bits = (Bits::new(
            ((s_71_27) >> (s_71_24)).value(),
            u16::try_from(s_71_25).unwrap(),
        ));
        // D s_71_29: cast reint s_71_28 -> u8
        let s_71_29: u8 = (s_71_28.value() as u8);
        // D s_71_30: write-var u#33944 <= s_71_29
        fn_state.u_33944 = s_71_29;
        // C s_71_31: const #0s : i
        let s_71_31: i128 = 0;
        // C s_71_32: const #15s : i
        let s_71_32: i128 = 15;
        // D s_71_33: read-var u#33940:u32
        let s_71_33: u32 = fn_state.u_33940;
        // D s_71_34: cast zx s_71_33 -> bv
        let s_71_34: Bits = Bits::new(s_71_33 as u128, 32u16);
        // D s_71_35: bit-extract s_71_34 s_71_31 s_71_32
        let s_71_35: Bits = (Bits::new(
            ((s_71_34) >> (s_71_31)).value(),
            u16::try_from(s_71_32).unwrap(),
        ));
        // D s_71_36: cast reint s_71_35 -> u15
        let s_71_36: u16 = (s_71_35.value() as u16);
        // D s_71_37: write-var u#33945 <= s_71_36
        fn_state.u_33945 = s_71_36;
        // C s_71_38: const #21s : i
        let s_71_38: i128 = 21;
        // D s_71_39: read-var u#33940:u32
        let s_71_39: u32 = fn_state.u_33940;
        // D s_71_40: cast zx s_71_39 -> bv
        let s_71_40: Bits = Bits::new(s_71_39 as u128, 32u16);
        // C s_71_41: const #1u : u64
        let s_71_41: u64 = 1;
        // D s_71_42: bit-extract s_71_40 s_71_38 s_71_41
        let s_71_42: Bits = (Bits::new(
            ((s_71_40) >> (s_71_38)).value(),
            u16::try_from(s_71_41).unwrap(),
        ));
        // D s_71_43: cast reint s_71_42 -> u8
        let s_71_43: bool = ((s_71_42.value()) != 0);
        // C s_71_44: const #0s : i
        let s_71_44: i128 = 0;
        // C s_71_45: const #0u : u64
        let s_71_45: u64 = 0;
        // D s_71_46: cast zx s_71_43 -> u64
        let s_71_46: u64 = (s_71_43 as u64);
        // C s_71_47: const #1u : u64
        let s_71_47: u64 = 1;
        // D s_71_48: and s_71_46 s_71_47
        let s_71_48: u64 = ((s_71_46) & (s_71_47));
        // D s_71_49: cmp-eq s_71_48 s_71_47
        let s_71_49: bool = ((s_71_48) == (s_71_47));
        // D s_71_50: lsl s_71_46 s_71_44
        let s_71_50: u64 = s_71_46 << s_71_44;
        // D s_71_51: or s_71_45 s_71_50
        let s_71_51: u64 = ((s_71_45) | (s_71_50));
        // D s_71_52: cmpl s_71_50
        let s_71_52: u64 = !s_71_50;
        // D s_71_53: and s_71_45 s_71_52
        let s_71_53: u64 = ((s_71_45) & (s_71_52));
        // D s_71_54: select s_71_49 s_71_51 s_71_53
        let s_71_54: u64 = if s_71_49 { s_71_51 } else { s_71_53 };
        // D s_71_55: cast trunc s_71_54 -> u8
        let s_71_55: bool = ((s_71_54) != 0);
        // D s_71_56: cast zx s_71_55 -> bv
        let s_71_56: Bits = Bits::new(s_71_55 as u128, 1u16);
        // C s_71_57: const #0u : u8
        let s_71_57: bool = false;
        // C s_71_58: cast zx s_71_57 -> bv
        let s_71_58: Bits = Bits::new(s_71_57 as u128, 1u16);
        // D s_71_59: cmp-ne s_71_56 s_71_58
        let s_71_59: bool = ((s_71_56) != (s_71_58));
        // N s_71_60: branch s_71_59 b73 b72
        if s_71_59 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var u#33941:u8
        let s_72_0: u8 = fn_state.u_33941;
        // D s_72_1: read-var u#33942:u8
        let s_72_1: bool = fn_state.u_33942;
        // D s_72_2: read-var u#33943:u8
        let s_72_2: bool = fn_state.u_33943;
        // D s_72_3: read-var u#33944:u8
        let s_72_3: u8 = fn_state.u_33944;
        // D s_72_4: read-var u#33945:u15
        let s_72_4: u16 = fn_state.u_33945;
        // D s_72_5: call decode_aarch32_instrs_LDM_u_A1enc_AS_txt(s_72_0, s_72_1, s_72_2, s_72_3, s_72_4)
        let s_72_5: () = decode_aarch32_instrs_LDM_u_A1enc_AS_txt(
            state,
            tracer,
            s_72_0,
            s_72_1,
            s_72_2,
            s_72_3,
            s_72_4,
        );
        // N s_72_6: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_73_0: panic
        panic!("{:?}", ());
        // N s_73_1: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var merge#var.1:struct
        let s_74_0: u32 = fn_state.merge_var._1;
        // D s_74_1: write-var u#33947 <= s_74_0
        fn_state.u_33947 = s_74_0;
        // C s_74_2: const #25s : i
        let s_74_2: i128 = 25;
        // D s_74_3: read-var u#33947:u32
        let s_74_3: u32 = fn_state.u_33947;
        // D s_74_4: cast zx s_74_3 -> bv
        let s_74_4: Bits = Bits::new(s_74_3 as u128, 32u16);
        // C s_74_5: const #1s : i64
        let s_74_5: i64 = 1;
        // C s_74_6: cast zx s_74_5 -> i
        let s_74_6: i128 = (i128::try_from(s_74_5).unwrap());
        // C s_74_7: const #6s : i
        let s_74_7: i128 = 6;
        // C s_74_8: add s_74_7 s_74_6
        let s_74_8: i128 = (s_74_7 + s_74_6);
        // D s_74_9: bit-extract s_74_4 s_74_2 s_74_8
        let s_74_9: Bits = (Bits::new(
            ((s_74_4) >> (s_74_2)).value(),
            u16::try_from(s_74_8).unwrap(),
        ));
        // D s_74_10: cast reint s_74_9 -> u8
        let s_74_10: u8 = (s_74_9.value() as u8);
        // D s_74_11: cast zx s_74_10 -> bv
        let s_74_11: Bits = Bits::new(s_74_10 as u128, 7u16);
        // C s_74_12: const #124u : u8
        let s_74_12: u8 = 124;
        // C s_74_13: cast zx s_74_12 -> bv
        let s_74_13: Bits = Bits::new(s_74_12 as u128, 7u16);
        // D s_74_14: cmp-eq s_74_11 s_74_13
        let s_74_14: bool = ((s_74_11) == (s_74_13));
        // N s_74_15: branch s_74_14 b173 b75
        if s_74_14 {
            return block_173(state, tracer, fn_state);
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
        // D s_75_1: write-var gs#411639 <= s_75_0
        fn_state.gs_411639 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#411639:u8
        let s_76_0: bool = fn_state.gs_411639;
        // N s_76_1: branch s_76_0 b172 b77
        if s_76_0 {
            return block_172(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#411641 <= s_77_0
        fn_state.gs_411641 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#411641:u8
        let s_78_0: bool = fn_state.gs_411641;
        // D s_78_1: not s_78_0
        let s_78_1: bool = !s_78_0;
        // N s_78_2: branch s_78_1 b115 b79
        if s_78_1 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #3787s : i
        let s_79_0: i128 = 3787;
        // C s_79_1: const #14696u : u32
        let s_79_1: u32 = 14696;
        // N s_79_2: write-reg s_79_1 <= s_79_0
        let s_79_2: () = {
            state.write_register::<i128>(s_79_1 as isize, s_79_0);
            tracer.write_register(s_79_1 as isize, s_79_0);
        };
        // C s_79_3: const #24s : i
        let s_79_3: i128 = 24;
        // C s_79_4: const #1s : i
        let s_79_4: i128 = 1;
        // D s_79_5: read-var u#33947:u32
        let s_79_5: u32 = fn_state.u_33947;
        // D s_79_6: cast zx s_79_5 -> bv
        let s_79_6: Bits = Bits::new(s_79_5 as u128, 32u16);
        // D s_79_7: bit-extract s_79_6 s_79_3 s_79_4
        let s_79_7: Bits = (Bits::new(
            ((s_79_6) >> (s_79_3)).value(),
            u16::try_from(s_79_4).unwrap(),
        ));
        // D s_79_8: cast reint s_79_7 -> u8
        let s_79_8: bool = ((s_79_7.value()) != 0);
        // D s_79_9: write-var u#33948 <= s_79_8
        fn_state.u_33948 = s_79_8;
        // C s_79_10: const #23s : i
        let s_79_10: i128 = 23;
        // C s_79_11: const #1s : i
        let s_79_11: i128 = 1;
        // D s_79_12: read-var u#33947:u32
        let s_79_12: u32 = fn_state.u_33947;
        // D s_79_13: cast zx s_79_12 -> bv
        let s_79_13: Bits = Bits::new(s_79_12 as u128, 32u16);
        // D s_79_14: bit-extract s_79_13 s_79_10 s_79_11
        let s_79_14: Bits = (Bits::new(
            ((s_79_13) >> (s_79_10)).value(),
            u16::try_from(s_79_11).unwrap(),
        ));
        // D s_79_15: cast reint s_79_14 -> u8
        let s_79_15: bool = ((s_79_14.value()) != 0);
        // D s_79_16: write-var u#33949 <= s_79_15
        fn_state.u_33949 = s_79_15;
        // C s_79_17: const #21s : i
        let s_79_17: i128 = 21;
        // C s_79_18: const #1s : i
        let s_79_18: i128 = 1;
        // D s_79_19: read-var u#33947:u32
        let s_79_19: u32 = fn_state.u_33947;
        // D s_79_20: cast zx s_79_19 -> bv
        let s_79_20: Bits = Bits::new(s_79_19 as u128, 32u16);
        // D s_79_21: bit-extract s_79_20 s_79_17 s_79_18
        let s_79_21: Bits = (Bits::new(
            ((s_79_20) >> (s_79_17)).value(),
            u16::try_from(s_79_18).unwrap(),
        ));
        // D s_79_22: cast reint s_79_21 -> u8
        let s_79_22: bool = ((s_79_21.value()) != 0);
        // D s_79_23: write-var u#33950 <= s_79_22
        fn_state.u_33950 = s_79_22;
        // C s_79_24: const #16s : i
        let s_79_24: i128 = 16;
        // C s_79_25: const #4s : i
        let s_79_25: i128 = 4;
        // D s_79_26: read-var u#33947:u32
        let s_79_26: u32 = fn_state.u_33947;
        // D s_79_27: cast zx s_79_26 -> bv
        let s_79_27: Bits = Bits::new(s_79_26 as u128, 32u16);
        // D s_79_28: bit-extract s_79_27 s_79_24 s_79_25
        let s_79_28: Bits = (Bits::new(
            ((s_79_27) >> (s_79_24)).value(),
            u16::try_from(s_79_25).unwrap(),
        ));
        // D s_79_29: cast reint s_79_28 -> u8
        let s_79_29: u8 = (s_79_28.value() as u8);
        // D s_79_30: write-var u#33951 <= s_79_29
        fn_state.u_33951 = s_79_29;
        // C s_79_31: const #12s : i
        let s_79_31: i128 = 12;
        // D s_79_32: read-var u#33947:u32
        let s_79_32: u32 = fn_state.u_33947;
        // D s_79_33: cast zx s_79_32 -> bv
        let s_79_33: Bits = Bits::new(s_79_32 as u128, 32u16);
        // C s_79_34: const #1u : u64
        let s_79_34: u64 = 1;
        // D s_79_35: bit-extract s_79_33 s_79_31 s_79_34
        let s_79_35: Bits = (Bits::new(
            ((s_79_33) >> (s_79_31)).value(),
            u16::try_from(s_79_34).unwrap(),
        ));
        // D s_79_36: cast reint s_79_35 -> u8
        let s_79_36: bool = ((s_79_35.value()) != 0);
        // C s_79_37: const #0s : i
        let s_79_37: i128 = 0;
        // C s_79_38: const #0u : u64
        let s_79_38: u64 = 0;
        // D s_79_39: cast zx s_79_36 -> u64
        let s_79_39: u64 = (s_79_36 as u64);
        // C s_79_40: const #1u : u64
        let s_79_40: u64 = 1;
        // D s_79_41: and s_79_39 s_79_40
        let s_79_41: u64 = ((s_79_39) & (s_79_40));
        // D s_79_42: cmp-eq s_79_41 s_79_40
        let s_79_42: bool = ((s_79_41) == (s_79_40));
        // D s_79_43: lsl s_79_39 s_79_37
        let s_79_43: u64 = s_79_39 << s_79_37;
        // D s_79_44: or s_79_38 s_79_43
        let s_79_44: u64 = ((s_79_38) | (s_79_43));
        // D s_79_45: cmpl s_79_43
        let s_79_45: u64 = !s_79_43;
        // D s_79_46: and s_79_38 s_79_45
        let s_79_46: u64 = ((s_79_38) & (s_79_45));
        // D s_79_47: select s_79_42 s_79_44 s_79_46
        let s_79_47: u64 = if s_79_42 { s_79_44 } else { s_79_46 };
        // D s_79_48: cast trunc s_79_47 -> u8
        let s_79_48: bool = ((s_79_47) != 0);
        // D s_79_49: cast zx s_79_48 -> bv
        let s_79_49: Bits = Bits::new(s_79_48 as u128, 1u16);
        // C s_79_50: const #0u : u8
        let s_79_50: bool = false;
        // C s_79_51: cast zx s_79_50 -> bv
        let s_79_51: Bits = Bits::new(s_79_50 as u128, 1u16);
        // D s_79_52: cmp-ne s_79_49 s_79_51
        let s_79_52: bool = ((s_79_49) != (s_79_51));
        // N s_79_53: branch s_79_52 b114 b80
        if s_79_52 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #13s : i
        let s_80_0: i128 = 13;
        // D s_80_1: read-var u#33947:u32
        let s_80_1: u32 = fn_state.u_33947;
        // D s_80_2: cast zx s_80_1 -> bv
        let s_80_2: Bits = Bits::new(s_80_1 as u128, 32u16);
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
        // D s_80_22: write-var gs#411656 <= s_80_21
        fn_state.gs_411656 = s_80_21;
        // N s_80_23: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#411656:u8
        let s_81_0: bool = fn_state.gs_411656;
        // N s_81_1: branch s_81_0 b113 b82
        if s_81_0 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #14s : i
        let s_82_0: i128 = 14;
        // D s_82_1: read-var u#33947:u32
        let s_82_1: u32 = fn_state.u_33947;
        // D s_82_2: cast zx s_82_1 -> bv
        let s_82_2: Bits = Bits::new(s_82_1 as u128, 32u16);
        // C s_82_3: const #1u : u64
        let s_82_3: u64 = 1;
        // D s_82_4: bit-extract s_82_2 s_82_0 s_82_3
        let s_82_4: Bits = (Bits::new(
            ((s_82_2) >> (s_82_0)).value(),
            u16::try_from(s_82_3).unwrap(),
        ));
        // D s_82_5: cast reint s_82_4 -> u8
        let s_82_5: bool = ((s_82_4.value()) != 0);
        // C s_82_6: const #0s : i
        let s_82_6: i128 = 0;
        // C s_82_7: const #0u : u64
        let s_82_7: u64 = 0;
        // D s_82_8: cast zx s_82_5 -> u64
        let s_82_8: u64 = (s_82_5 as u64);
        // C s_82_9: const #1u : u64
        let s_82_9: u64 = 1;
        // D s_82_10: and s_82_8 s_82_9
        let s_82_10: u64 = ((s_82_8) & (s_82_9));
        // D s_82_11: cmp-eq s_82_10 s_82_9
        let s_82_11: bool = ((s_82_10) == (s_82_9));
        // D s_82_12: lsl s_82_8 s_82_6
        let s_82_12: u64 = s_82_8 << s_82_6;
        // D s_82_13: or s_82_7 s_82_12
        let s_82_13: u64 = ((s_82_7) | (s_82_12));
        // D s_82_14: cmpl s_82_12
        let s_82_14: u64 = !s_82_12;
        // D s_82_15: and s_82_7 s_82_14
        let s_82_15: u64 = ((s_82_7) & (s_82_14));
        // D s_82_16: select s_82_11 s_82_13 s_82_15
        let s_82_16: u64 = if s_82_11 { s_82_13 } else { s_82_15 };
        // D s_82_17: cast trunc s_82_16 -> u8
        let s_82_17: bool = ((s_82_16) != 0);
        // D s_82_18: cast zx s_82_17 -> bv
        let s_82_18: Bits = Bits::new(s_82_17 as u128, 1u16);
        // C s_82_19: const #0u : u8
        let s_82_19: bool = false;
        // C s_82_20: cast zx s_82_19 -> bv
        let s_82_20: Bits = Bits::new(s_82_19 as u128, 1u16);
        // D s_82_21: cmp-ne s_82_18 s_82_20
        let s_82_21: bool = ((s_82_18) != (s_82_20));
        // D s_82_22: write-var gs#411659 <= s_82_21
        fn_state.gs_411659 = s_82_21;
        // N s_82_23: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#411659:u8
        let s_83_0: bool = fn_state.gs_411659;
        // N s_83_1: branch s_83_0 b112 b84
        if s_83_0 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #15s : i
        let s_84_0: i128 = 15;
        // D s_84_1: read-var u#33947:u32
        let s_84_1: u32 = fn_state.u_33947;
        // D s_84_2: cast zx s_84_1 -> bv
        let s_84_2: Bits = Bits::new(s_84_1 as u128, 32u16);
        // C s_84_3: const #1u : u64
        let s_84_3: u64 = 1;
        // D s_84_4: bit-extract s_84_2 s_84_0 s_84_3
        let s_84_4: Bits = (Bits::new(
            ((s_84_2) >> (s_84_0)).value(),
            u16::try_from(s_84_3).unwrap(),
        ));
        // D s_84_5: cast reint s_84_4 -> u8
        let s_84_5: bool = ((s_84_4.value()) != 0);
        // C s_84_6: const #0s : i
        let s_84_6: i128 = 0;
        // C s_84_7: const #0u : u64
        let s_84_7: u64 = 0;
        // D s_84_8: cast zx s_84_5 -> u64
        let s_84_8: u64 = (s_84_5 as u64);
        // C s_84_9: const #1u : u64
        let s_84_9: u64 = 1;
        // D s_84_10: and s_84_8 s_84_9
        let s_84_10: u64 = ((s_84_8) & (s_84_9));
        // D s_84_11: cmp-eq s_84_10 s_84_9
        let s_84_11: bool = ((s_84_10) == (s_84_9));
        // D s_84_12: lsl s_84_8 s_84_6
        let s_84_12: u64 = s_84_8 << s_84_6;
        // D s_84_13: or s_84_7 s_84_12
        let s_84_13: u64 = ((s_84_7) | (s_84_12));
        // D s_84_14: cmpl s_84_12
        let s_84_14: u64 = !s_84_12;
        // D s_84_15: and s_84_7 s_84_14
        let s_84_15: u64 = ((s_84_7) & (s_84_14));
        // D s_84_16: select s_84_11 s_84_13 s_84_15
        let s_84_16: u64 = if s_84_11 { s_84_13 } else { s_84_15 };
        // D s_84_17: cast trunc s_84_16 -> u8
        let s_84_17: bool = ((s_84_16) != 0);
        // D s_84_18: cast zx s_84_17 -> bv
        let s_84_18: Bits = Bits::new(s_84_17 as u128, 1u16);
        // C s_84_19: const #0u : u8
        let s_84_19: bool = false;
        // C s_84_20: cast zx s_84_19 -> bv
        let s_84_20: Bits = Bits::new(s_84_19 as u128, 1u16);
        // D s_84_21: cmp-ne s_84_18 s_84_20
        let s_84_21: bool = ((s_84_18) != (s_84_20));
        // D s_84_22: write-var gs#411662 <= s_84_21
        fn_state.gs_411662 = s_84_21;
        // N s_84_23: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#411662:u8
        let s_85_0: bool = fn_state.gs_411662;
        // N s_85_1: branch s_85_0 b111 b86
        if s_85_0 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0s : i
        let s_86_0: i128 = 0;
        // D s_86_1: read-var u#33947:u32
        let s_86_1: u32 = fn_state.u_33947;
        // D s_86_2: cast zx s_86_1 -> bv
        let s_86_2: Bits = Bits::new(s_86_1 as u128, 32u16);
        // C s_86_3: const #1u : u64
        let s_86_3: u64 = 1;
        // D s_86_4: bit-extract s_86_2 s_86_0 s_86_3
        let s_86_4: Bits = (Bits::new(
            ((s_86_2) >> (s_86_0)).value(),
            u16::try_from(s_86_3).unwrap(),
        ));
        // D s_86_5: cast reint s_86_4 -> u8
        let s_86_5: bool = ((s_86_4.value()) != 0);
        // C s_86_6: const #0s : i
        let s_86_6: i128 = 0;
        // C s_86_7: const #0u : u64
        let s_86_7: u64 = 0;
        // D s_86_8: cast zx s_86_5 -> u64
        let s_86_8: u64 = (s_86_5 as u64);
        // C s_86_9: const #1u : u64
        let s_86_9: u64 = 1;
        // D s_86_10: and s_86_8 s_86_9
        let s_86_10: u64 = ((s_86_8) & (s_86_9));
        // D s_86_11: cmp-eq s_86_10 s_86_9
        let s_86_11: bool = ((s_86_10) == (s_86_9));
        // D s_86_12: lsl s_86_8 s_86_6
        let s_86_12: u64 = s_86_8 << s_86_6;
        // D s_86_13: or s_86_7 s_86_12
        let s_86_13: u64 = ((s_86_7) | (s_86_12));
        // D s_86_14: cmpl s_86_12
        let s_86_14: u64 = !s_86_12;
        // D s_86_15: and s_86_7 s_86_14
        let s_86_15: u64 = ((s_86_7) & (s_86_14));
        // D s_86_16: select s_86_11 s_86_13 s_86_15
        let s_86_16: u64 = if s_86_11 { s_86_13 } else { s_86_15 };
        // D s_86_17: cast trunc s_86_16 -> u8
        let s_86_17: bool = ((s_86_16) != 0);
        // D s_86_18: cast zx s_86_17 -> bv
        let s_86_18: Bits = Bits::new(s_86_17 as u128, 1u16);
        // C s_86_19: const #0u : u8
        let s_86_19: bool = false;
        // C s_86_20: cast zx s_86_19 -> bv
        let s_86_20: Bits = Bits::new(s_86_19 as u128, 1u16);
        // D s_86_21: cmp-ne s_86_18 s_86_20
        let s_86_21: bool = ((s_86_18) != (s_86_20));
        // D s_86_22: write-var gs#411665 <= s_86_21
        fn_state.gs_411665 = s_86_21;
        // N s_86_23: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#411665:u8
        let s_87_0: bool = fn_state.gs_411665;
        // N s_87_1: branch s_87_0 b110 b88
        if s_87_0 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #1s : i
        let s_88_0: i128 = 1;
        // D s_88_1: read-var u#33947:u32
        let s_88_1: u32 = fn_state.u_33947;
        // D s_88_2: cast zx s_88_1 -> bv
        let s_88_2: Bits = Bits::new(s_88_1 as u128, 32u16);
        // C s_88_3: const #1u : u64
        let s_88_3: u64 = 1;
        // D s_88_4: bit-extract s_88_2 s_88_0 s_88_3
        let s_88_4: Bits = (Bits::new(
            ((s_88_2) >> (s_88_0)).value(),
            u16::try_from(s_88_3).unwrap(),
        ));
        // D s_88_5: cast reint s_88_4 -> u8
        let s_88_5: bool = ((s_88_4.value()) != 0);
        // C s_88_6: const #0s : i
        let s_88_6: i128 = 0;
        // C s_88_7: const #0u : u64
        let s_88_7: u64 = 0;
        // D s_88_8: cast zx s_88_5 -> u64
        let s_88_8: u64 = (s_88_5 as u64);
        // C s_88_9: const #1u : u64
        let s_88_9: u64 = 1;
        // D s_88_10: and s_88_8 s_88_9
        let s_88_10: u64 = ((s_88_8) & (s_88_9));
        // D s_88_11: cmp-eq s_88_10 s_88_9
        let s_88_11: bool = ((s_88_10) == (s_88_9));
        // D s_88_12: lsl s_88_8 s_88_6
        let s_88_12: u64 = s_88_8 << s_88_6;
        // D s_88_13: or s_88_7 s_88_12
        let s_88_13: u64 = ((s_88_7) | (s_88_12));
        // D s_88_14: cmpl s_88_12
        let s_88_14: u64 = !s_88_12;
        // D s_88_15: and s_88_7 s_88_14
        let s_88_15: u64 = ((s_88_7) & (s_88_14));
        // D s_88_16: select s_88_11 s_88_13 s_88_15
        let s_88_16: u64 = if s_88_11 { s_88_13 } else { s_88_15 };
        // D s_88_17: cast trunc s_88_16 -> u8
        let s_88_17: bool = ((s_88_16) != 0);
        // D s_88_18: cast zx s_88_17 -> bv
        let s_88_18: Bits = Bits::new(s_88_17 as u128, 1u16);
        // C s_88_19: const #0u : u8
        let s_88_19: bool = false;
        // C s_88_20: cast zx s_88_19 -> bv
        let s_88_20: Bits = Bits::new(s_88_19 as u128, 1u16);
        // D s_88_21: cmp-ne s_88_18 s_88_20
        let s_88_21: bool = ((s_88_18) != (s_88_20));
        // D s_88_22: write-var gs#411668 <= s_88_21
        fn_state.gs_411668 = s_88_21;
        // N s_88_23: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#411668:u8
        let s_89_0: bool = fn_state.gs_411668;
        // N s_89_1: branch s_89_0 b109 b90
        if s_89_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #2s : i
        let s_90_0: i128 = 2;
        // D s_90_1: read-var u#33947:u32
        let s_90_1: u32 = fn_state.u_33947;
        // D s_90_2: cast zx s_90_1 -> bv
        let s_90_2: Bits = Bits::new(s_90_1 as u128, 32u16);
        // C s_90_3: const #1u : u64
        let s_90_3: u64 = 1;
        // D s_90_4: bit-extract s_90_2 s_90_0 s_90_3
        let s_90_4: Bits = (Bits::new(
            ((s_90_2) >> (s_90_0)).value(),
            u16::try_from(s_90_3).unwrap(),
        ));
        // D s_90_5: cast reint s_90_4 -> u8
        let s_90_5: bool = ((s_90_4.value()) != 0);
        // C s_90_6: const #0s : i
        let s_90_6: i128 = 0;
        // C s_90_7: const #0u : u64
        let s_90_7: u64 = 0;
        // D s_90_8: cast zx s_90_5 -> u64
        let s_90_8: u64 = (s_90_5 as u64);
        // C s_90_9: const #1u : u64
        let s_90_9: u64 = 1;
        // D s_90_10: and s_90_8 s_90_9
        let s_90_10: u64 = ((s_90_8) & (s_90_9));
        // D s_90_11: cmp-eq s_90_10 s_90_9
        let s_90_11: bool = ((s_90_10) == (s_90_9));
        // D s_90_12: lsl s_90_8 s_90_6
        let s_90_12: u64 = s_90_8 << s_90_6;
        // D s_90_13: or s_90_7 s_90_12
        let s_90_13: u64 = ((s_90_7) | (s_90_12));
        // D s_90_14: cmpl s_90_12
        let s_90_14: u64 = !s_90_12;
        // D s_90_15: and s_90_7 s_90_14
        let s_90_15: u64 = ((s_90_7) & (s_90_14));
        // D s_90_16: select s_90_11 s_90_13 s_90_15
        let s_90_16: u64 = if s_90_11 { s_90_13 } else { s_90_15 };
        // D s_90_17: cast trunc s_90_16 -> u8
        let s_90_17: bool = ((s_90_16) != 0);
        // D s_90_18: cast zx s_90_17 -> bv
        let s_90_18: Bits = Bits::new(s_90_17 as u128, 1u16);
        // C s_90_19: const #0u : u8
        let s_90_19: bool = false;
        // C s_90_20: cast zx s_90_19 -> bv
        let s_90_20: Bits = Bits::new(s_90_19 as u128, 1u16);
        // D s_90_21: cmp-ne s_90_18 s_90_20
        let s_90_21: bool = ((s_90_18) != (s_90_20));
        // D s_90_22: write-var gs#411671 <= s_90_21
        fn_state.gs_411671 = s_90_21;
        // N s_90_23: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#411671:u8
        let s_91_0: bool = fn_state.gs_411671;
        // N s_91_1: branch s_91_0 b108 b92
        if s_91_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #3s : i
        let s_92_0: i128 = 3;
        // D s_92_1: read-var u#33947:u32
        let s_92_1: u32 = fn_state.u_33947;
        // D s_92_2: cast zx s_92_1 -> bv
        let s_92_2: Bits = Bits::new(s_92_1 as u128, 32u16);
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
        // D s_92_22: write-var gs#411674 <= s_92_21
        fn_state.gs_411674 = s_92_21;
        // N s_92_23: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#411674:u8
        let s_93_0: bool = fn_state.gs_411674;
        // N s_93_1: branch s_93_0 b107 b94
        if s_93_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #4s : i
        let s_94_0: i128 = 4;
        // D s_94_1: read-var u#33947:u32
        let s_94_1: u32 = fn_state.u_33947;
        // D s_94_2: cast zx s_94_1 -> bv
        let s_94_2: Bits = Bits::new(s_94_1 as u128, 32u16);
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
        // D s_94_22: write-var gs#411677 <= s_94_21
        fn_state.gs_411677 = s_94_21;
        // N s_94_23: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#411677:u8
        let s_95_0: bool = fn_state.gs_411677;
        // N s_95_1: branch s_95_0 b106 b96
        if s_95_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #5s : i
        let s_96_0: i128 = 5;
        // D s_96_1: read-var u#33947:u32
        let s_96_1: u32 = fn_state.u_33947;
        // D s_96_2: cast zx s_96_1 -> bv
        let s_96_2: Bits = Bits::new(s_96_1 as u128, 32u16);
        // C s_96_3: const #1u : u64
        let s_96_3: u64 = 1;
        // D s_96_4: bit-extract s_96_2 s_96_0 s_96_3
        let s_96_4: Bits = (Bits::new(
            ((s_96_2) >> (s_96_0)).value(),
            u16::try_from(s_96_3).unwrap(),
        ));
        // D s_96_5: cast reint s_96_4 -> u8
        let s_96_5: bool = ((s_96_4.value()) != 0);
        // C s_96_6: const #0s : i
        let s_96_6: i128 = 0;
        // C s_96_7: const #0u : u64
        let s_96_7: u64 = 0;
        // D s_96_8: cast zx s_96_5 -> u64
        let s_96_8: u64 = (s_96_5 as u64);
        // C s_96_9: const #1u : u64
        let s_96_9: u64 = 1;
        // D s_96_10: and s_96_8 s_96_9
        let s_96_10: u64 = ((s_96_8) & (s_96_9));
        // D s_96_11: cmp-eq s_96_10 s_96_9
        let s_96_11: bool = ((s_96_10) == (s_96_9));
        // D s_96_12: lsl s_96_8 s_96_6
        let s_96_12: u64 = s_96_8 << s_96_6;
        // D s_96_13: or s_96_7 s_96_12
        let s_96_13: u64 = ((s_96_7) | (s_96_12));
        // D s_96_14: cmpl s_96_12
        let s_96_14: u64 = !s_96_12;
        // D s_96_15: and s_96_7 s_96_14
        let s_96_15: u64 = ((s_96_7) & (s_96_14));
        // D s_96_16: select s_96_11 s_96_13 s_96_15
        let s_96_16: u64 = if s_96_11 { s_96_13 } else { s_96_15 };
        // D s_96_17: cast trunc s_96_16 -> u8
        let s_96_17: bool = ((s_96_16) != 0);
        // D s_96_18: cast zx s_96_17 -> bv
        let s_96_18: Bits = Bits::new(s_96_17 as u128, 1u16);
        // C s_96_19: const #0u : u8
        let s_96_19: bool = false;
        // C s_96_20: cast zx s_96_19 -> bv
        let s_96_20: Bits = Bits::new(s_96_19 as u128, 1u16);
        // D s_96_21: cmp-ne s_96_18 s_96_20
        let s_96_21: bool = ((s_96_18) != (s_96_20));
        // D s_96_22: write-var gs#411680 <= s_96_21
        fn_state.gs_411680 = s_96_21;
        // N s_96_23: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#411680:u8
        let s_97_0: bool = fn_state.gs_411680;
        // N s_97_1: branch s_97_0 b105 b98
        if s_97_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #6s : i
        let s_98_0: i128 = 6;
        // D s_98_1: read-var u#33947:u32
        let s_98_1: u32 = fn_state.u_33947;
        // D s_98_2: cast zx s_98_1 -> bv
        let s_98_2: Bits = Bits::new(s_98_1 as u128, 32u16);
        // C s_98_3: const #1u : u64
        let s_98_3: u64 = 1;
        // D s_98_4: bit-extract s_98_2 s_98_0 s_98_3
        let s_98_4: Bits = (Bits::new(
            ((s_98_2) >> (s_98_0)).value(),
            u16::try_from(s_98_3).unwrap(),
        ));
        // D s_98_5: cast reint s_98_4 -> u8
        let s_98_5: bool = ((s_98_4.value()) != 0);
        // C s_98_6: const #0s : i
        let s_98_6: i128 = 0;
        // C s_98_7: const #0u : u64
        let s_98_7: u64 = 0;
        // D s_98_8: cast zx s_98_5 -> u64
        let s_98_8: u64 = (s_98_5 as u64);
        // C s_98_9: const #1u : u64
        let s_98_9: u64 = 1;
        // D s_98_10: and s_98_8 s_98_9
        let s_98_10: u64 = ((s_98_8) & (s_98_9));
        // D s_98_11: cmp-eq s_98_10 s_98_9
        let s_98_11: bool = ((s_98_10) == (s_98_9));
        // D s_98_12: lsl s_98_8 s_98_6
        let s_98_12: u64 = s_98_8 << s_98_6;
        // D s_98_13: or s_98_7 s_98_12
        let s_98_13: u64 = ((s_98_7) | (s_98_12));
        // D s_98_14: cmpl s_98_12
        let s_98_14: u64 = !s_98_12;
        // D s_98_15: and s_98_7 s_98_14
        let s_98_15: u64 = ((s_98_7) & (s_98_14));
        // D s_98_16: select s_98_11 s_98_13 s_98_15
        let s_98_16: u64 = if s_98_11 { s_98_13 } else { s_98_15 };
        // D s_98_17: cast trunc s_98_16 -> u8
        let s_98_17: bool = ((s_98_16) != 0);
        // D s_98_18: cast zx s_98_17 -> bv
        let s_98_18: Bits = Bits::new(s_98_17 as u128, 1u16);
        // C s_98_19: const #0u : u8
        let s_98_19: bool = false;
        // C s_98_20: cast zx s_98_19 -> bv
        let s_98_20: Bits = Bits::new(s_98_19 as u128, 1u16);
        // D s_98_21: cmp-ne s_98_18 s_98_20
        let s_98_21: bool = ((s_98_18) != (s_98_20));
        // D s_98_22: write-var gs#411683 <= s_98_21
        fn_state.gs_411683 = s_98_21;
        // N s_98_23: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#411683:u8
        let s_99_0: bool = fn_state.gs_411683;
        // N s_99_1: branch s_99_0 b104 b100
        if s_99_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #7s : i
        let s_100_0: i128 = 7;
        // D s_100_1: read-var u#33947:u32
        let s_100_1: u32 = fn_state.u_33947;
        // D s_100_2: cast zx s_100_1 -> bv
        let s_100_2: Bits = Bits::new(s_100_1 as u128, 32u16);
        // C s_100_3: const #1u : u64
        let s_100_3: u64 = 1;
        // D s_100_4: bit-extract s_100_2 s_100_0 s_100_3
        let s_100_4: Bits = (Bits::new(
            ((s_100_2) >> (s_100_0)).value(),
            u16::try_from(s_100_3).unwrap(),
        ));
        // D s_100_5: cast reint s_100_4 -> u8
        let s_100_5: bool = ((s_100_4.value()) != 0);
        // C s_100_6: const #0s : i
        let s_100_6: i128 = 0;
        // C s_100_7: const #0u : u64
        let s_100_7: u64 = 0;
        // D s_100_8: cast zx s_100_5 -> u64
        let s_100_8: u64 = (s_100_5 as u64);
        // C s_100_9: const #1u : u64
        let s_100_9: u64 = 1;
        // D s_100_10: and s_100_8 s_100_9
        let s_100_10: u64 = ((s_100_8) & (s_100_9));
        // D s_100_11: cmp-eq s_100_10 s_100_9
        let s_100_11: bool = ((s_100_10) == (s_100_9));
        // D s_100_12: lsl s_100_8 s_100_6
        let s_100_12: u64 = s_100_8 << s_100_6;
        // D s_100_13: or s_100_7 s_100_12
        let s_100_13: u64 = ((s_100_7) | (s_100_12));
        // D s_100_14: cmpl s_100_12
        let s_100_14: u64 = !s_100_12;
        // D s_100_15: and s_100_7 s_100_14
        let s_100_15: u64 = ((s_100_7) & (s_100_14));
        // D s_100_16: select s_100_11 s_100_13 s_100_15
        let s_100_16: u64 = if s_100_11 { s_100_13 } else { s_100_15 };
        // D s_100_17: cast trunc s_100_16 -> u8
        let s_100_17: bool = ((s_100_16) != 0);
        // D s_100_18: cast zx s_100_17 -> bv
        let s_100_18: Bits = Bits::new(s_100_17 as u128, 1u16);
        // C s_100_19: const #0u : u8
        let s_100_19: bool = false;
        // C s_100_20: cast zx s_100_19 -> bv
        let s_100_20: Bits = Bits::new(s_100_19 as u128, 1u16);
        // D s_100_21: cmp-ne s_100_18 s_100_20
        let s_100_21: bool = ((s_100_18) != (s_100_20));
        // D s_100_22: write-var gs#411686 <= s_100_21
        fn_state.gs_411686 = s_100_21;
        // N s_100_23: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#411686:u8
        let s_101_0: bool = fn_state.gs_411686;
        // N s_101_1: branch s_101_0 b103 b102
        if s_101_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var u#33948:u8
        let s_102_0: bool = fn_state.u_33948;
        // D s_102_1: read-var u#33949:u8
        let s_102_1: bool = fn_state.u_33949;
        // D s_102_2: read-var u#33950:u8
        let s_102_2: bool = fn_state.u_33950;
        // D s_102_3: read-var u#33951:u8
        let s_102_3: u8 = fn_state.u_33951;
        // D s_102_4: call decode_aarch32_instrs_RFE_A1enc_AS_txt(s_102_0, s_102_1, s_102_2, s_102_3)
        let s_102_4: () = decode_aarch32_instrs_RFE_A1enc_AS_txt(
            state,
            tracer,
            s_102_0,
            s_102_1,
            s_102_2,
            s_102_3,
        );
        // N s_102_5: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_103_0: panic
        panic!("{:?}", ());
        // N s_103_1: return
        return;
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #1u : u8
        let s_104_0: bool = true;
        // D s_104_1: write-var gs#411686 <= s_104_0
        fn_state.gs_411686 = s_104_0;
        // N s_104_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #1u : u8
        let s_105_0: bool = true;
        // D s_105_1: write-var gs#411683 <= s_105_0
        fn_state.gs_411683 = s_105_0;
        // N s_105_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #1u : u8
        let s_106_0: bool = true;
        // D s_106_1: write-var gs#411680 <= s_106_0
        fn_state.gs_411680 = s_106_0;
        // N s_106_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #1u : u8
        let s_107_0: bool = true;
        // D s_107_1: write-var gs#411677 <= s_107_0
        fn_state.gs_411677 = s_107_0;
        // N s_107_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #1u : u8
        let s_108_0: bool = true;
        // D s_108_1: write-var gs#411674 <= s_108_0
        fn_state.gs_411674 = s_108_0;
        // N s_108_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #1u : u8
        let s_109_0: bool = true;
        // D s_109_1: write-var gs#411671 <= s_109_0
        fn_state.gs_411671 = s_109_0;
        // N s_109_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #1u : u8
        let s_110_0: bool = true;
        // D s_110_1: write-var gs#411668 <= s_110_0
        fn_state.gs_411668 = s_110_0;
        // N s_110_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #1u : u8
        let s_111_0: bool = true;
        // D s_111_1: write-var gs#411665 <= s_111_0
        fn_state.gs_411665 = s_111_0;
        // N s_111_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #1u : u8
        let s_112_0: bool = true;
        // D s_112_1: write-var gs#411662 <= s_112_0
        fn_state.gs_411662 = s_112_0;
        // N s_112_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #1u : u8
        let s_113_0: bool = true;
        // D s_113_1: write-var gs#411659 <= s_113_0
        fn_state.gs_411659 = s_113_0;
        // N s_113_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #1u : u8
        let s_114_0: bool = true;
        // D s_114_1: write-var gs#411656 <= s_114_0
        fn_state.gs_411656 = s_114_0;
        // N s_114_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var merge#var.1:struct
        let s_115_0: u32 = fn_state.merge_var._1;
        // D s_115_1: write-var u#33953 <= s_115_0
        fn_state.u_33953 = s_115_0;
        // C s_115_2: const #25s : i
        let s_115_2: i128 = 25;
        // D s_115_3: read-var u#33953:u32
        let s_115_3: u32 = fn_state.u_33953;
        // D s_115_4: cast zx s_115_3 -> bv
        let s_115_4: Bits = Bits::new(s_115_3 as u128, 32u16);
        // C s_115_5: const #1s : i64
        let s_115_5: i64 = 1;
        // C s_115_6: cast zx s_115_5 -> i
        let s_115_6: i128 = (i128::try_from(s_115_5).unwrap());
        // C s_115_7: const #6s : i
        let s_115_7: i128 = 6;
        // C s_115_8: add s_115_7 s_115_6
        let s_115_8: i128 = (s_115_7 + s_115_6);
        // D s_115_9: bit-extract s_115_4 s_115_2 s_115_8
        let s_115_9: Bits = (Bits::new(
            ((s_115_4) >> (s_115_2)).value(),
            u16::try_from(s_115_8).unwrap(),
        ));
        // D s_115_10: cast reint s_115_9 -> u8
        let s_115_10: u8 = (s_115_9.value() as u8);
        // D s_115_11: cast zx s_115_10 -> bv
        let s_115_11: Bits = Bits::new(s_115_10 as u128, 7u16);
        // C s_115_12: const #124u : u8
        let s_115_12: u8 = 124;
        // C s_115_13: cast zx s_115_12 -> bv
        let s_115_13: Bits = Bits::new(s_115_12 as u128, 7u16);
        // D s_115_14: cmp-eq s_115_11 s_115_13
        let s_115_14: bool = ((s_115_11) == (s_115_13));
        // N s_115_15: branch s_115_14 b168 b116
        if s_115_14 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #0u : u8
        let s_116_0: bool = false;
        // D s_116_1: write-var gs#411695 <= s_116_0
        fn_state.gs_411695 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#411695:u8
        let s_117_0: bool = fn_state.gs_411695;
        // N s_117_1: branch s_117_0 b167 b118
        if s_117_0 {
            return block_167(state, tracer, fn_state);
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
        // D s_118_1: write-var gs#411697 <= s_118_0
        fn_state.gs_411697 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#411697:u8
        let s_119_0: bool = fn_state.gs_411697;
        // D s_119_1: not s_119_0
        let s_119_1: bool = !s_119_0;
        // N s_119_2: branch s_119_1 b153 b120
        if s_119_1 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #3792s : i
        let s_120_0: i128 = 3792;
        // C s_120_1: const #14696u : u32
        let s_120_1: u32 = 14696;
        // N s_120_2: write-reg s_120_1 <= s_120_0
        let s_120_2: () = {
            state.write_register::<i128>(s_120_1 as isize, s_120_0);
            tracer.write_register(s_120_1 as isize, s_120_0);
        };
        // C s_120_3: const #24s : i
        let s_120_3: i128 = 24;
        // C s_120_4: const #1s : i
        let s_120_4: i128 = 1;
        // D s_120_5: read-var u#33953:u32
        let s_120_5: u32 = fn_state.u_33953;
        // D s_120_6: cast zx s_120_5 -> bv
        let s_120_6: Bits = Bits::new(s_120_5 as u128, 32u16);
        // D s_120_7: bit-extract s_120_6 s_120_3 s_120_4
        let s_120_7: Bits = (Bits::new(
            ((s_120_6) >> (s_120_3)).value(),
            u16::try_from(s_120_4).unwrap(),
        ));
        // D s_120_8: cast reint s_120_7 -> u8
        let s_120_8: bool = ((s_120_7.value()) != 0);
        // D s_120_9: write-var u#33954 <= s_120_8
        fn_state.u_33954 = s_120_8;
        // C s_120_10: const #23s : i
        let s_120_10: i128 = 23;
        // C s_120_11: const #1s : i
        let s_120_11: i128 = 1;
        // D s_120_12: read-var u#33953:u32
        let s_120_12: u32 = fn_state.u_33953;
        // D s_120_13: cast zx s_120_12 -> bv
        let s_120_13: Bits = Bits::new(s_120_12 as u128, 32u16);
        // D s_120_14: bit-extract s_120_13 s_120_10 s_120_11
        let s_120_14: Bits = (Bits::new(
            ((s_120_13) >> (s_120_10)).value(),
            u16::try_from(s_120_11).unwrap(),
        ));
        // D s_120_15: cast reint s_120_14 -> u8
        let s_120_15: bool = ((s_120_14.value()) != 0);
        // D s_120_16: write-var u#33955 <= s_120_15
        fn_state.u_33955 = s_120_15;
        // C s_120_17: const #21s : i
        let s_120_17: i128 = 21;
        // C s_120_18: const #1s : i
        let s_120_18: i128 = 1;
        // D s_120_19: read-var u#33953:u32
        let s_120_19: u32 = fn_state.u_33953;
        // D s_120_20: cast zx s_120_19 -> bv
        let s_120_20: Bits = Bits::new(s_120_19 as u128, 32u16);
        // D s_120_21: bit-extract s_120_20 s_120_17 s_120_18
        let s_120_21: Bits = (Bits::new(
            ((s_120_20) >> (s_120_17)).value(),
            u16::try_from(s_120_18).unwrap(),
        ));
        // D s_120_22: cast reint s_120_21 -> u8
        let s_120_22: bool = ((s_120_21.value()) != 0);
        // D s_120_23: write-var u#33956 <= s_120_22
        fn_state.u_33956 = s_120_22;
        // C s_120_24: const #0s : i
        let s_120_24: i128 = 0;
        // C s_120_25: const #5s : i
        let s_120_25: i128 = 5;
        // D s_120_26: read-var u#33953:u32
        let s_120_26: u32 = fn_state.u_33953;
        // D s_120_27: cast zx s_120_26 -> bv
        let s_120_27: Bits = Bits::new(s_120_26 as u128, 32u16);
        // D s_120_28: bit-extract s_120_27 s_120_24 s_120_25
        let s_120_28: Bits = (Bits::new(
            ((s_120_27) >> (s_120_24)).value(),
            u16::try_from(s_120_25).unwrap(),
        ));
        // D s_120_29: cast reint s_120_28 -> u8
        let s_120_29: u8 = (s_120_28.value() as u8);
        // D s_120_30: write-var mode <= s_120_29
        fn_state.mode = s_120_29;
        // C s_120_31: const #16s : i
        let s_120_31: i128 = 16;
        // D s_120_32: read-var u#33953:u32
        let s_120_32: u32 = fn_state.u_33953;
        // D s_120_33: cast zx s_120_32 -> bv
        let s_120_33: Bits = Bits::new(s_120_32 as u128, 32u16);
        // C s_120_34: const #1u : u64
        let s_120_34: u64 = 1;
        // D s_120_35: bit-extract s_120_33 s_120_31 s_120_34
        let s_120_35: Bits = (Bits::new(
            ((s_120_33) >> (s_120_31)).value(),
            u16::try_from(s_120_34).unwrap(),
        ));
        // D s_120_36: cast reint s_120_35 -> u8
        let s_120_36: bool = ((s_120_35.value()) != 0);
        // C s_120_37: const #0s : i
        let s_120_37: i128 = 0;
        // C s_120_38: const #0u : u64
        let s_120_38: u64 = 0;
        // D s_120_39: cast zx s_120_36 -> u64
        let s_120_39: u64 = (s_120_36 as u64);
        // C s_120_40: const #1u : u64
        let s_120_40: u64 = 1;
        // D s_120_41: and s_120_39 s_120_40
        let s_120_41: u64 = ((s_120_39) & (s_120_40));
        // D s_120_42: cmp-eq s_120_41 s_120_40
        let s_120_42: bool = ((s_120_41) == (s_120_40));
        // D s_120_43: lsl s_120_39 s_120_37
        let s_120_43: u64 = s_120_39 << s_120_37;
        // D s_120_44: or s_120_38 s_120_43
        let s_120_44: u64 = ((s_120_38) | (s_120_43));
        // D s_120_45: cmpl s_120_43
        let s_120_45: u64 = !s_120_43;
        // D s_120_46: and s_120_38 s_120_45
        let s_120_46: u64 = ((s_120_38) & (s_120_45));
        // D s_120_47: select s_120_42 s_120_44 s_120_46
        let s_120_47: u64 = if s_120_42 { s_120_44 } else { s_120_46 };
        // D s_120_48: cast trunc s_120_47 -> u8
        let s_120_48: bool = ((s_120_47) != 0);
        // D s_120_49: cast zx s_120_48 -> bv
        let s_120_49: Bits = Bits::new(s_120_48 as u128, 1u16);
        // C s_120_50: const #1u : u8
        let s_120_50: bool = true;
        // C s_120_51: cast zx s_120_50 -> bv
        let s_120_51: Bits = Bits::new(s_120_50 as u128, 1u16);
        // D s_120_52: cmp-ne s_120_49 s_120_51
        let s_120_52: bool = ((s_120_49) != (s_120_51));
        // N s_120_53: branch s_120_52 b152 b121
        if s_120_52 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #17s : i
        let s_121_0: i128 = 17;
        // D s_121_1: read-var u#33953:u32
        let s_121_1: u32 = fn_state.u_33953;
        // D s_121_2: cast zx s_121_1 -> bv
        let s_121_2: Bits = Bits::new(s_121_1 as u128, 32u16);
        // C s_121_3: const #1u : u64
        let s_121_3: u64 = 1;
        // D s_121_4: bit-extract s_121_2 s_121_0 s_121_3
        let s_121_4: Bits = (Bits::new(
            ((s_121_2) >> (s_121_0)).value(),
            u16::try_from(s_121_3).unwrap(),
        ));
        // D s_121_5: cast reint s_121_4 -> u8
        let s_121_5: bool = ((s_121_4.value()) != 0);
        // C s_121_6: const #0s : i
        let s_121_6: i128 = 0;
        // C s_121_7: const #0u : u64
        let s_121_7: u64 = 0;
        // D s_121_8: cast zx s_121_5 -> u64
        let s_121_8: u64 = (s_121_5 as u64);
        // C s_121_9: const #1u : u64
        let s_121_9: u64 = 1;
        // D s_121_10: and s_121_8 s_121_9
        let s_121_10: u64 = ((s_121_8) & (s_121_9));
        // D s_121_11: cmp-eq s_121_10 s_121_9
        let s_121_11: bool = ((s_121_10) == (s_121_9));
        // D s_121_12: lsl s_121_8 s_121_6
        let s_121_12: u64 = s_121_8 << s_121_6;
        // D s_121_13: or s_121_7 s_121_12
        let s_121_13: u64 = ((s_121_7) | (s_121_12));
        // D s_121_14: cmpl s_121_12
        let s_121_14: u64 = !s_121_12;
        // D s_121_15: and s_121_7 s_121_14
        let s_121_15: u64 = ((s_121_7) & (s_121_14));
        // D s_121_16: select s_121_11 s_121_13 s_121_15
        let s_121_16: u64 = if s_121_11 { s_121_13 } else { s_121_15 };
        // D s_121_17: cast trunc s_121_16 -> u8
        let s_121_17: bool = ((s_121_16) != 0);
        // D s_121_18: cast zx s_121_17 -> bv
        let s_121_18: Bits = Bits::new(s_121_17 as u128, 1u16);
        // C s_121_19: const #1u : u8
        let s_121_19: bool = true;
        // C s_121_20: cast zx s_121_19 -> bv
        let s_121_20: Bits = Bits::new(s_121_19 as u128, 1u16);
        // D s_121_21: cmp-ne s_121_18 s_121_20
        let s_121_21: bool = ((s_121_18) != (s_121_20));
        // D s_121_22: write-var gs#411712 <= s_121_21
        fn_state.gs_411712 = s_121_21;
        // N s_121_23: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#411712:u8
        let s_122_0: bool = fn_state.gs_411712;
        // N s_122_1: branch s_122_0 b151 b123
        if s_122_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #18s : i
        let s_123_0: i128 = 18;
        // D s_123_1: read-var u#33953:u32
        let s_123_1: u32 = fn_state.u_33953;
        // D s_123_2: cast zx s_123_1 -> bv
        let s_123_2: Bits = Bits::new(s_123_1 as u128, 32u16);
        // C s_123_3: const #1u : u64
        let s_123_3: u64 = 1;
        // D s_123_4: bit-extract s_123_2 s_123_0 s_123_3
        let s_123_4: Bits = (Bits::new(
            ((s_123_2) >> (s_123_0)).value(),
            u16::try_from(s_123_3).unwrap(),
        ));
        // D s_123_5: cast reint s_123_4 -> u8
        let s_123_5: bool = ((s_123_4.value()) != 0);
        // C s_123_6: const #0s : i
        let s_123_6: i128 = 0;
        // C s_123_7: const #0u : u64
        let s_123_7: u64 = 0;
        // D s_123_8: cast zx s_123_5 -> u64
        let s_123_8: u64 = (s_123_5 as u64);
        // C s_123_9: const #1u : u64
        let s_123_9: u64 = 1;
        // D s_123_10: and s_123_8 s_123_9
        let s_123_10: u64 = ((s_123_8) & (s_123_9));
        // D s_123_11: cmp-eq s_123_10 s_123_9
        let s_123_11: bool = ((s_123_10) == (s_123_9));
        // D s_123_12: lsl s_123_8 s_123_6
        let s_123_12: u64 = s_123_8 << s_123_6;
        // D s_123_13: or s_123_7 s_123_12
        let s_123_13: u64 = ((s_123_7) | (s_123_12));
        // D s_123_14: cmpl s_123_12
        let s_123_14: u64 = !s_123_12;
        // D s_123_15: and s_123_7 s_123_14
        let s_123_15: u64 = ((s_123_7) & (s_123_14));
        // D s_123_16: select s_123_11 s_123_13 s_123_15
        let s_123_16: u64 = if s_123_11 { s_123_13 } else { s_123_15 };
        // D s_123_17: cast trunc s_123_16 -> u8
        let s_123_17: bool = ((s_123_16) != 0);
        // D s_123_18: cast zx s_123_17 -> bv
        let s_123_18: Bits = Bits::new(s_123_17 as u128, 1u16);
        // C s_123_19: const #0u : u8
        let s_123_19: bool = false;
        // C s_123_20: cast zx s_123_19 -> bv
        let s_123_20: Bits = Bits::new(s_123_19 as u128, 1u16);
        // D s_123_21: cmp-ne s_123_18 s_123_20
        let s_123_21: bool = ((s_123_18) != (s_123_20));
        // D s_123_22: write-var gs#411715 <= s_123_21
        fn_state.gs_411715 = s_123_21;
        // N s_123_23: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#411715:u8
        let s_124_0: bool = fn_state.gs_411715;
        // N s_124_1: branch s_124_0 b150 b125
        if s_124_0 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #19s : i
        let s_125_0: i128 = 19;
        // D s_125_1: read-var u#33953:u32
        let s_125_1: u32 = fn_state.u_33953;
        // D s_125_2: cast zx s_125_1 -> bv
        let s_125_2: Bits = Bits::new(s_125_1 as u128, 32u16);
        // C s_125_3: const #1u : u64
        let s_125_3: u64 = 1;
        // D s_125_4: bit-extract s_125_2 s_125_0 s_125_3
        let s_125_4: Bits = (Bits::new(
            ((s_125_2) >> (s_125_0)).value(),
            u16::try_from(s_125_3).unwrap(),
        ));
        // D s_125_5: cast reint s_125_4 -> u8
        let s_125_5: bool = ((s_125_4.value()) != 0);
        // C s_125_6: const #0s : i
        let s_125_6: i128 = 0;
        // C s_125_7: const #0u : u64
        let s_125_7: u64 = 0;
        // D s_125_8: cast zx s_125_5 -> u64
        let s_125_8: u64 = (s_125_5 as u64);
        // C s_125_9: const #1u : u64
        let s_125_9: u64 = 1;
        // D s_125_10: and s_125_8 s_125_9
        let s_125_10: u64 = ((s_125_8) & (s_125_9));
        // D s_125_11: cmp-eq s_125_10 s_125_9
        let s_125_11: bool = ((s_125_10) == (s_125_9));
        // D s_125_12: lsl s_125_8 s_125_6
        let s_125_12: u64 = s_125_8 << s_125_6;
        // D s_125_13: or s_125_7 s_125_12
        let s_125_13: u64 = ((s_125_7) | (s_125_12));
        // D s_125_14: cmpl s_125_12
        let s_125_14: u64 = !s_125_12;
        // D s_125_15: and s_125_7 s_125_14
        let s_125_15: u64 = ((s_125_7) & (s_125_14));
        // D s_125_16: select s_125_11 s_125_13 s_125_15
        let s_125_16: u64 = if s_125_11 { s_125_13 } else { s_125_15 };
        // D s_125_17: cast trunc s_125_16 -> u8
        let s_125_17: bool = ((s_125_16) != 0);
        // D s_125_18: cast zx s_125_17 -> bv
        let s_125_18: Bits = Bits::new(s_125_17 as u128, 1u16);
        // C s_125_19: const #1u : u8
        let s_125_19: bool = true;
        // C s_125_20: cast zx s_125_19 -> bv
        let s_125_20: Bits = Bits::new(s_125_19 as u128, 1u16);
        // D s_125_21: cmp-ne s_125_18 s_125_20
        let s_125_21: bool = ((s_125_18) != (s_125_20));
        // D s_125_22: write-var gs#411718 <= s_125_21
        fn_state.gs_411718 = s_125_21;
        // N s_125_23: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#411718:u8
        let s_126_0: bool = fn_state.gs_411718;
        // N s_126_1: branch s_126_0 b149 b127
        if s_126_0 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #12s : i
        let s_127_0: i128 = 12;
        // D s_127_1: read-var u#33953:u32
        let s_127_1: u32 = fn_state.u_33953;
        // D s_127_2: cast zx s_127_1 -> bv
        let s_127_2: Bits = Bits::new(s_127_1 as u128, 32u16);
        // C s_127_3: const #1u : u64
        let s_127_3: u64 = 1;
        // D s_127_4: bit-extract s_127_2 s_127_0 s_127_3
        let s_127_4: Bits = (Bits::new(
            ((s_127_2) >> (s_127_0)).value(),
            u16::try_from(s_127_3).unwrap(),
        ));
        // D s_127_5: cast reint s_127_4 -> u8
        let s_127_5: bool = ((s_127_4.value()) != 0);
        // C s_127_6: const #0s : i
        let s_127_6: i128 = 0;
        // C s_127_7: const #0u : u64
        let s_127_7: u64 = 0;
        // D s_127_8: cast zx s_127_5 -> u64
        let s_127_8: u64 = (s_127_5 as u64);
        // C s_127_9: const #1u : u64
        let s_127_9: u64 = 1;
        // D s_127_10: and s_127_8 s_127_9
        let s_127_10: u64 = ((s_127_8) & (s_127_9));
        // D s_127_11: cmp-eq s_127_10 s_127_9
        let s_127_11: bool = ((s_127_10) == (s_127_9));
        // D s_127_12: lsl s_127_8 s_127_6
        let s_127_12: u64 = s_127_8 << s_127_6;
        // D s_127_13: or s_127_7 s_127_12
        let s_127_13: u64 = ((s_127_7) | (s_127_12));
        // D s_127_14: cmpl s_127_12
        let s_127_14: u64 = !s_127_12;
        // D s_127_15: and s_127_7 s_127_14
        let s_127_15: u64 = ((s_127_7) & (s_127_14));
        // D s_127_16: select s_127_11 s_127_13 s_127_15
        let s_127_16: u64 = if s_127_11 { s_127_13 } else { s_127_15 };
        // D s_127_17: cast trunc s_127_16 -> u8
        let s_127_17: bool = ((s_127_16) != 0);
        // D s_127_18: cast zx s_127_17 -> bv
        let s_127_18: Bits = Bits::new(s_127_17 as u128, 1u16);
        // C s_127_19: const #0u : u8
        let s_127_19: bool = false;
        // C s_127_20: cast zx s_127_19 -> bv
        let s_127_20: Bits = Bits::new(s_127_19 as u128, 1u16);
        // D s_127_21: cmp-ne s_127_18 s_127_20
        let s_127_21: bool = ((s_127_18) != (s_127_20));
        // D s_127_22: write-var gs#411721 <= s_127_21
        fn_state.gs_411721 = s_127_21;
        // N s_127_23: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#411721:u8
        let s_128_0: bool = fn_state.gs_411721;
        // N s_128_1: branch s_128_0 b148 b129
        if s_128_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #13s : i
        let s_129_0: i128 = 13;
        // D s_129_1: read-var u#33953:u32
        let s_129_1: u32 = fn_state.u_33953;
        // D s_129_2: cast zx s_129_1 -> bv
        let s_129_2: Bits = Bits::new(s_129_1 as u128, 32u16);
        // C s_129_3: const #1u : u64
        let s_129_3: u64 = 1;
        // D s_129_4: bit-extract s_129_2 s_129_0 s_129_3
        let s_129_4: Bits = (Bits::new(
            ((s_129_2) >> (s_129_0)).value(),
            u16::try_from(s_129_3).unwrap(),
        ));
        // D s_129_5: cast reint s_129_4 -> u8
        let s_129_5: bool = ((s_129_4.value()) != 0);
        // C s_129_6: const #0s : i
        let s_129_6: i128 = 0;
        // C s_129_7: const #0u : u64
        let s_129_7: u64 = 0;
        // D s_129_8: cast zx s_129_5 -> u64
        let s_129_8: u64 = (s_129_5 as u64);
        // C s_129_9: const #1u : u64
        let s_129_9: u64 = 1;
        // D s_129_10: and s_129_8 s_129_9
        let s_129_10: u64 = ((s_129_8) & (s_129_9));
        // D s_129_11: cmp-eq s_129_10 s_129_9
        let s_129_11: bool = ((s_129_10) == (s_129_9));
        // D s_129_12: lsl s_129_8 s_129_6
        let s_129_12: u64 = s_129_8 << s_129_6;
        // D s_129_13: or s_129_7 s_129_12
        let s_129_13: u64 = ((s_129_7) | (s_129_12));
        // D s_129_14: cmpl s_129_12
        let s_129_14: u64 = !s_129_12;
        // D s_129_15: and s_129_7 s_129_14
        let s_129_15: u64 = ((s_129_7) & (s_129_14));
        // D s_129_16: select s_129_11 s_129_13 s_129_15
        let s_129_16: u64 = if s_129_11 { s_129_13 } else { s_129_15 };
        // D s_129_17: cast trunc s_129_16 -> u8
        let s_129_17: bool = ((s_129_16) != 0);
        // D s_129_18: cast zx s_129_17 -> bv
        let s_129_18: Bits = Bits::new(s_129_17 as u128, 1u16);
        // C s_129_19: const #0u : u8
        let s_129_19: bool = false;
        // C s_129_20: cast zx s_129_19 -> bv
        let s_129_20: Bits = Bits::new(s_129_19 as u128, 1u16);
        // D s_129_21: cmp-ne s_129_18 s_129_20
        let s_129_21: bool = ((s_129_18) != (s_129_20));
        // D s_129_22: write-var gs#411724 <= s_129_21
        fn_state.gs_411724 = s_129_21;
        // N s_129_23: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#411724:u8
        let s_130_0: bool = fn_state.gs_411724;
        // N s_130_1: branch s_130_0 b147 b131
        if s_130_0 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #14s : i
        let s_131_0: i128 = 14;
        // D s_131_1: read-var u#33953:u32
        let s_131_1: u32 = fn_state.u_33953;
        // D s_131_2: cast zx s_131_1 -> bv
        let s_131_2: Bits = Bits::new(s_131_1 as u128, 32u16);
        // C s_131_3: const #1u : u64
        let s_131_3: u64 = 1;
        // D s_131_4: bit-extract s_131_2 s_131_0 s_131_3
        let s_131_4: Bits = (Bits::new(
            ((s_131_2) >> (s_131_0)).value(),
            u16::try_from(s_131_3).unwrap(),
        ));
        // D s_131_5: cast reint s_131_4 -> u8
        let s_131_5: bool = ((s_131_4.value()) != 0);
        // C s_131_6: const #0s : i
        let s_131_6: i128 = 0;
        // C s_131_7: const #0u : u64
        let s_131_7: u64 = 0;
        // D s_131_8: cast zx s_131_5 -> u64
        let s_131_8: u64 = (s_131_5 as u64);
        // C s_131_9: const #1u : u64
        let s_131_9: u64 = 1;
        // D s_131_10: and s_131_8 s_131_9
        let s_131_10: u64 = ((s_131_8) & (s_131_9));
        // D s_131_11: cmp-eq s_131_10 s_131_9
        let s_131_11: bool = ((s_131_10) == (s_131_9));
        // D s_131_12: lsl s_131_8 s_131_6
        let s_131_12: u64 = s_131_8 << s_131_6;
        // D s_131_13: or s_131_7 s_131_12
        let s_131_13: u64 = ((s_131_7) | (s_131_12));
        // D s_131_14: cmpl s_131_12
        let s_131_14: u64 = !s_131_12;
        // D s_131_15: and s_131_7 s_131_14
        let s_131_15: u64 = ((s_131_7) & (s_131_14));
        // D s_131_16: select s_131_11 s_131_13 s_131_15
        let s_131_16: u64 = if s_131_11 { s_131_13 } else { s_131_15 };
        // D s_131_17: cast trunc s_131_16 -> u8
        let s_131_17: bool = ((s_131_16) != 0);
        // D s_131_18: cast zx s_131_17 -> bv
        let s_131_18: Bits = Bits::new(s_131_17 as u128, 1u16);
        // C s_131_19: const #0u : u8
        let s_131_19: bool = false;
        // C s_131_20: cast zx s_131_19 -> bv
        let s_131_20: Bits = Bits::new(s_131_19 as u128, 1u16);
        // D s_131_21: cmp-ne s_131_18 s_131_20
        let s_131_21: bool = ((s_131_18) != (s_131_20));
        // D s_131_22: write-var gs#411727 <= s_131_21
        fn_state.gs_411727 = s_131_21;
        // N s_131_23: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#411727:u8
        let s_132_0: bool = fn_state.gs_411727;
        // N s_132_1: branch s_132_0 b146 b133
        if s_132_0 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #15s : i
        let s_133_0: i128 = 15;
        // D s_133_1: read-var u#33953:u32
        let s_133_1: u32 = fn_state.u_33953;
        // D s_133_2: cast zx s_133_1 -> bv
        let s_133_2: Bits = Bits::new(s_133_1 as u128, 32u16);
        // C s_133_3: const #1u : u64
        let s_133_3: u64 = 1;
        // D s_133_4: bit-extract s_133_2 s_133_0 s_133_3
        let s_133_4: Bits = (Bits::new(
            ((s_133_2) >> (s_133_0)).value(),
            u16::try_from(s_133_3).unwrap(),
        ));
        // D s_133_5: cast reint s_133_4 -> u8
        let s_133_5: bool = ((s_133_4.value()) != 0);
        // C s_133_6: const #0s : i
        let s_133_6: i128 = 0;
        // C s_133_7: const #0u : u64
        let s_133_7: u64 = 0;
        // D s_133_8: cast zx s_133_5 -> u64
        let s_133_8: u64 = (s_133_5 as u64);
        // C s_133_9: const #1u : u64
        let s_133_9: u64 = 1;
        // D s_133_10: and s_133_8 s_133_9
        let s_133_10: u64 = ((s_133_8) & (s_133_9));
        // D s_133_11: cmp-eq s_133_10 s_133_9
        let s_133_11: bool = ((s_133_10) == (s_133_9));
        // D s_133_12: lsl s_133_8 s_133_6
        let s_133_12: u64 = s_133_8 << s_133_6;
        // D s_133_13: or s_133_7 s_133_12
        let s_133_13: u64 = ((s_133_7) | (s_133_12));
        // D s_133_14: cmpl s_133_12
        let s_133_14: u64 = !s_133_12;
        // D s_133_15: and s_133_7 s_133_14
        let s_133_15: u64 = ((s_133_7) & (s_133_14));
        // D s_133_16: select s_133_11 s_133_13 s_133_15
        let s_133_16: u64 = if s_133_11 { s_133_13 } else { s_133_15 };
        // D s_133_17: cast trunc s_133_16 -> u8
        let s_133_17: bool = ((s_133_16) != 0);
        // D s_133_18: cast zx s_133_17 -> bv
        let s_133_18: Bits = Bits::new(s_133_17 as u128, 1u16);
        // C s_133_19: const #0u : u8
        let s_133_19: bool = false;
        // C s_133_20: cast zx s_133_19 -> bv
        let s_133_20: Bits = Bits::new(s_133_19 as u128, 1u16);
        // D s_133_21: cmp-ne s_133_18 s_133_20
        let s_133_21: bool = ((s_133_18) != (s_133_20));
        // D s_133_22: write-var gs#411730 <= s_133_21
        fn_state.gs_411730 = s_133_21;
        // N s_133_23: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#411730:u8
        let s_134_0: bool = fn_state.gs_411730;
        // N s_134_1: branch s_134_0 b145 b135
        if s_134_0 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #5s : i
        let s_135_0: i128 = 5;
        // D s_135_1: read-var u#33953:u32
        let s_135_1: u32 = fn_state.u_33953;
        // D s_135_2: cast zx s_135_1 -> bv
        let s_135_2: Bits = Bits::new(s_135_1 as u128, 32u16);
        // C s_135_3: const #1u : u64
        let s_135_3: u64 = 1;
        // D s_135_4: bit-extract s_135_2 s_135_0 s_135_3
        let s_135_4: Bits = (Bits::new(
            ((s_135_2) >> (s_135_0)).value(),
            u16::try_from(s_135_3).unwrap(),
        ));
        // D s_135_5: cast reint s_135_4 -> u8
        let s_135_5: bool = ((s_135_4.value()) != 0);
        // C s_135_6: const #0s : i
        let s_135_6: i128 = 0;
        // C s_135_7: const #0u : u64
        let s_135_7: u64 = 0;
        // D s_135_8: cast zx s_135_5 -> u64
        let s_135_8: u64 = (s_135_5 as u64);
        // C s_135_9: const #1u : u64
        let s_135_9: u64 = 1;
        // D s_135_10: and s_135_8 s_135_9
        let s_135_10: u64 = ((s_135_8) & (s_135_9));
        // D s_135_11: cmp-eq s_135_10 s_135_9
        let s_135_11: bool = ((s_135_10) == (s_135_9));
        // D s_135_12: lsl s_135_8 s_135_6
        let s_135_12: u64 = s_135_8 << s_135_6;
        // D s_135_13: or s_135_7 s_135_12
        let s_135_13: u64 = ((s_135_7) | (s_135_12));
        // D s_135_14: cmpl s_135_12
        let s_135_14: u64 = !s_135_12;
        // D s_135_15: and s_135_7 s_135_14
        let s_135_15: u64 = ((s_135_7) & (s_135_14));
        // D s_135_16: select s_135_11 s_135_13 s_135_15
        let s_135_16: u64 = if s_135_11 { s_135_13 } else { s_135_15 };
        // D s_135_17: cast trunc s_135_16 -> u8
        let s_135_17: bool = ((s_135_16) != 0);
        // D s_135_18: cast zx s_135_17 -> bv
        let s_135_18: Bits = Bits::new(s_135_17 as u128, 1u16);
        // C s_135_19: const #0u : u8
        let s_135_19: bool = false;
        // C s_135_20: cast zx s_135_19 -> bv
        let s_135_20: Bits = Bits::new(s_135_19 as u128, 1u16);
        // D s_135_21: cmp-ne s_135_18 s_135_20
        let s_135_21: bool = ((s_135_18) != (s_135_20));
        // D s_135_22: write-var gs#411733 <= s_135_21
        fn_state.gs_411733 = s_135_21;
        // N s_135_23: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#411733:u8
        let s_136_0: bool = fn_state.gs_411733;
        // N s_136_1: branch s_136_0 b144 b137
        if s_136_0 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #6s : i
        let s_137_0: i128 = 6;
        // D s_137_1: read-var u#33953:u32
        let s_137_1: u32 = fn_state.u_33953;
        // D s_137_2: cast zx s_137_1 -> bv
        let s_137_2: Bits = Bits::new(s_137_1 as u128, 32u16);
        // C s_137_3: const #1u : u64
        let s_137_3: u64 = 1;
        // D s_137_4: bit-extract s_137_2 s_137_0 s_137_3
        let s_137_4: Bits = (Bits::new(
            ((s_137_2) >> (s_137_0)).value(),
            u16::try_from(s_137_3).unwrap(),
        ));
        // D s_137_5: cast reint s_137_4 -> u8
        let s_137_5: bool = ((s_137_4.value()) != 0);
        // C s_137_6: const #0s : i
        let s_137_6: i128 = 0;
        // C s_137_7: const #0u : u64
        let s_137_7: u64 = 0;
        // D s_137_8: cast zx s_137_5 -> u64
        let s_137_8: u64 = (s_137_5 as u64);
        // C s_137_9: const #1u : u64
        let s_137_9: u64 = 1;
        // D s_137_10: and s_137_8 s_137_9
        let s_137_10: u64 = ((s_137_8) & (s_137_9));
        // D s_137_11: cmp-eq s_137_10 s_137_9
        let s_137_11: bool = ((s_137_10) == (s_137_9));
        // D s_137_12: lsl s_137_8 s_137_6
        let s_137_12: u64 = s_137_8 << s_137_6;
        // D s_137_13: or s_137_7 s_137_12
        let s_137_13: u64 = ((s_137_7) | (s_137_12));
        // D s_137_14: cmpl s_137_12
        let s_137_14: u64 = !s_137_12;
        // D s_137_15: and s_137_7 s_137_14
        let s_137_15: u64 = ((s_137_7) & (s_137_14));
        // D s_137_16: select s_137_11 s_137_13 s_137_15
        let s_137_16: u64 = if s_137_11 { s_137_13 } else { s_137_15 };
        // D s_137_17: cast trunc s_137_16 -> u8
        let s_137_17: bool = ((s_137_16) != 0);
        // D s_137_18: cast zx s_137_17 -> bv
        let s_137_18: Bits = Bits::new(s_137_17 as u128, 1u16);
        // C s_137_19: const #0u : u8
        let s_137_19: bool = false;
        // C s_137_20: cast zx s_137_19 -> bv
        let s_137_20: Bits = Bits::new(s_137_19 as u128, 1u16);
        // D s_137_21: cmp-ne s_137_18 s_137_20
        let s_137_21: bool = ((s_137_18) != (s_137_20));
        // D s_137_22: write-var gs#411736 <= s_137_21
        fn_state.gs_411736 = s_137_21;
        // N s_137_23: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#411736:u8
        let s_138_0: bool = fn_state.gs_411736;
        // N s_138_1: branch s_138_0 b143 b139
        if s_138_0 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #7s : i
        let s_139_0: i128 = 7;
        // D s_139_1: read-var u#33953:u32
        let s_139_1: u32 = fn_state.u_33953;
        // D s_139_2: cast zx s_139_1 -> bv
        let s_139_2: Bits = Bits::new(s_139_1 as u128, 32u16);
        // C s_139_3: const #1u : u64
        let s_139_3: u64 = 1;
        // D s_139_4: bit-extract s_139_2 s_139_0 s_139_3
        let s_139_4: Bits = (Bits::new(
            ((s_139_2) >> (s_139_0)).value(),
            u16::try_from(s_139_3).unwrap(),
        ));
        // D s_139_5: cast reint s_139_4 -> u8
        let s_139_5: bool = ((s_139_4.value()) != 0);
        // C s_139_6: const #0s : i
        let s_139_6: i128 = 0;
        // C s_139_7: const #0u : u64
        let s_139_7: u64 = 0;
        // D s_139_8: cast zx s_139_5 -> u64
        let s_139_8: u64 = (s_139_5 as u64);
        // C s_139_9: const #1u : u64
        let s_139_9: u64 = 1;
        // D s_139_10: and s_139_8 s_139_9
        let s_139_10: u64 = ((s_139_8) & (s_139_9));
        // D s_139_11: cmp-eq s_139_10 s_139_9
        let s_139_11: bool = ((s_139_10) == (s_139_9));
        // D s_139_12: lsl s_139_8 s_139_6
        let s_139_12: u64 = s_139_8 << s_139_6;
        // D s_139_13: or s_139_7 s_139_12
        let s_139_13: u64 = ((s_139_7) | (s_139_12));
        // D s_139_14: cmpl s_139_12
        let s_139_14: u64 = !s_139_12;
        // D s_139_15: and s_139_7 s_139_14
        let s_139_15: u64 = ((s_139_7) & (s_139_14));
        // D s_139_16: select s_139_11 s_139_13 s_139_15
        let s_139_16: u64 = if s_139_11 { s_139_13 } else { s_139_15 };
        // D s_139_17: cast trunc s_139_16 -> u8
        let s_139_17: bool = ((s_139_16) != 0);
        // D s_139_18: cast zx s_139_17 -> bv
        let s_139_18: Bits = Bits::new(s_139_17 as u128, 1u16);
        // C s_139_19: const #0u : u8
        let s_139_19: bool = false;
        // C s_139_20: cast zx s_139_19 -> bv
        let s_139_20: Bits = Bits::new(s_139_19 as u128, 1u16);
        // D s_139_21: cmp-ne s_139_18 s_139_20
        let s_139_21: bool = ((s_139_18) != (s_139_20));
        // D s_139_22: write-var gs#411739 <= s_139_21
        fn_state.gs_411739 = s_139_21;
        // N s_139_23: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var gs#411739:u8
        let s_140_0: bool = fn_state.gs_411739;
        // N s_140_1: branch s_140_0 b142 b141
        if s_140_0 {
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
        // D s_141_0: read-var u#33954:u8
        let s_141_0: bool = fn_state.u_33954;
        // D s_141_1: read-var u#33955:u8
        let s_141_1: bool = fn_state.u_33955;
        // D s_141_2: read-var u#33956:u8
        let s_141_2: bool = fn_state.u_33956;
        // D s_141_3: read-var mode:u8
        let s_141_3: u8 = fn_state.mode;
        // D s_141_4: call decode_aarch32_instrs_SRS_A1enc_AS_txt(s_141_0, s_141_1, s_141_2, s_141_3)
        let s_141_4: () = decode_aarch32_instrs_SRS_A1enc_AS_txt(
            state,
            tracer,
            s_141_0,
            s_141_1,
            s_141_2,
            s_141_3,
        );
        // N s_141_5: return
        return;
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_142_0: panic
        panic!("{:?}", ());
        // N s_142_1: return
        return;
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #1u : u8
        let s_143_0: bool = true;
        // D s_143_1: write-var gs#411739 <= s_143_0
        fn_state.gs_411739 = s_143_0;
        // N s_143_2: jump b140
        return block_140(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #1u : u8
        let s_144_0: bool = true;
        // D s_144_1: write-var gs#411736 <= s_144_0
        fn_state.gs_411736 = s_144_0;
        // N s_144_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #1u : u8
        let s_145_0: bool = true;
        // D s_145_1: write-var gs#411733 <= s_145_0
        fn_state.gs_411733 = s_145_0;
        // N s_145_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #1u : u8
        let s_146_0: bool = true;
        // D s_146_1: write-var gs#411730 <= s_146_0
        fn_state.gs_411730 = s_146_0;
        // N s_146_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #1u : u8
        let s_147_0: bool = true;
        // D s_147_1: write-var gs#411727 <= s_147_0
        fn_state.gs_411727 = s_147_0;
        // N s_147_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #1u : u8
        let s_148_0: bool = true;
        // D s_148_1: write-var gs#411724 <= s_148_0
        fn_state.gs_411724 = s_148_0;
        // N s_148_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #1u : u8
        let s_149_0: bool = true;
        // D s_149_1: write-var gs#411721 <= s_149_0
        fn_state.gs_411721 = s_149_0;
        // N s_149_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #1u : u8
        let s_150_0: bool = true;
        // D s_150_1: write-var gs#411718 <= s_150_0
        fn_state.gs_411718 = s_150_0;
        // N s_150_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #1u : u8
        let s_151_0: bool = true;
        // D s_151_1: write-var gs#411715 <= s_151_0
        fn_state.gs_411715 = s_151_0;
        // N s_151_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #1u : u8
        let s_152_0: bool = true;
        // D s_152_1: write-var gs#411712 <= s_152_0
        fn_state.gs_411712 = s_152_0;
        // N s_152_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var merge#var.1:struct
        let s_153_0: u32 = fn_state.merge_var._1;
        // D s_153_1: write-var u#33958 <= s_153_0
        fn_state.u_33958 = s_153_0;
        // C s_153_2: const #25s : i
        let s_153_2: i128 = 25;
        // D s_153_3: read-var u#33958:u32
        let s_153_3: u32 = fn_state.u_33958;
        // D s_153_4: cast zx s_153_3 -> bv
        let s_153_4: Bits = Bits::new(s_153_3 as u128, 32u16);
        // C s_153_5: const #1s : i64
        let s_153_5: i64 = 1;
        // C s_153_6: cast zx s_153_5 -> i
        let s_153_6: i128 = (i128::try_from(s_153_5).unwrap());
        // C s_153_7: const #2s : i
        let s_153_7: i128 = 2;
        // C s_153_8: add s_153_7 s_153_6
        let s_153_8: i128 = (s_153_7 + s_153_6);
        // D s_153_9: bit-extract s_153_4 s_153_2 s_153_8
        let s_153_9: Bits = (Bits::new(
            ((s_153_4) >> (s_153_2)).value(),
            u16::try_from(s_153_8).unwrap(),
        ));
        // D s_153_10: cast reint s_153_9 -> u8
        let s_153_10: u8 = (s_153_9.value() as u8);
        // D s_153_11: cast zx s_153_10 -> bv
        let s_153_11: Bits = Bits::new(s_153_10 as u128, 3u16);
        // C s_153_12: const #4u : u8
        let s_153_12: u8 = 4;
        // C s_153_13: cast zx s_153_12 -> bv
        let s_153_13: Bits = Bits::new(s_153_12 as u128, 3u16);
        // D s_153_14: cmp-eq s_153_11 s_153_13
        let s_153_14: bool = ((s_153_11) == (s_153_13));
        // N s_153_15: branch s_153_14 b166 b154
        if s_153_14 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #0u : u8
        let s_154_0: bool = false;
        // D s_154_1: write-var gs#411745 <= s_154_0
        fn_state.gs_411745 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var gs#411745:u8
        let s_155_0: bool = fn_state.gs_411745;
        // N s_155_1: branch s_155_0 b162 b156
        if s_155_0 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #0u : u8
        let s_156_0: bool = false;
        // D s_156_1: write-var gs#411750 <= s_156_0
        fn_state.gs_411750 = s_156_0;
        // N s_156_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var gs#411750:u8
        let s_157_0: bool = fn_state.gs_411750;
        // D s_157_1: not s_157_0
        let s_157_1: bool = !s_157_0;
        // N s_157_2: branch s_157_1 b161 b158
        if s_157_1 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #3795s : i
        let s_158_0: i128 = 3795;
        // C s_158_1: const #14696u : u32
        let s_158_1: u32 = 14696;
        // N s_158_2: write-reg s_158_1 <= s_158_0
        let s_158_2: () = {
            state.write_register::<i128>(s_158_1 as isize, s_158_0);
            tracer.write_register(s_158_1 as isize, s_158_0);
        };
        // C s_158_3: const #28s : i
        let s_158_3: i128 = 28;
        // C s_158_4: const #4s : i
        let s_158_4: i128 = 4;
        // D s_158_5: read-var u#33958:u32
        let s_158_5: u32 = fn_state.u_33958;
        // D s_158_6: cast zx s_158_5 -> bv
        let s_158_6: Bits = Bits::new(s_158_5 as u128, 32u16);
        // D s_158_7: bit-extract s_158_6 s_158_3 s_158_4
        let s_158_7: Bits = (Bits::new(
            ((s_158_6) >> (s_158_3)).value(),
            u16::try_from(s_158_4).unwrap(),
        ));
        // D s_158_8: cast reint s_158_7 -> u8
        let s_158_8: u8 = (s_158_7.value() as u8);
        // D s_158_9: write-var u#33959 <= s_158_8
        fn_state.u_33959 = s_158_8;
        // C s_158_10: const #24s : i
        let s_158_10: i128 = 24;
        // C s_158_11: const #1s : i
        let s_158_11: i128 = 1;
        // D s_158_12: read-var u#33958:u32
        let s_158_12: u32 = fn_state.u_33958;
        // D s_158_13: cast zx s_158_12 -> bv
        let s_158_13: Bits = Bits::new(s_158_12 as u128, 32u16);
        // D s_158_14: bit-extract s_158_13 s_158_10 s_158_11
        let s_158_14: Bits = (Bits::new(
            ((s_158_13) >> (s_158_10)).value(),
            u16::try_from(s_158_11).unwrap(),
        ));
        // D s_158_15: cast reint s_158_14 -> u8
        let s_158_15: bool = ((s_158_14.value()) != 0);
        // D s_158_16: write-var u#33960 <= s_158_15
        fn_state.u_33960 = s_158_15;
        // C s_158_17: const #23s : i
        let s_158_17: i128 = 23;
        // C s_158_18: const #1s : i
        let s_158_18: i128 = 1;
        // D s_158_19: read-var u#33958:u32
        let s_158_19: u32 = fn_state.u_33958;
        // D s_158_20: cast zx s_158_19 -> bv
        let s_158_20: Bits = Bits::new(s_158_19 as u128, 32u16);
        // D s_158_21: bit-extract s_158_20 s_158_17 s_158_18
        let s_158_21: Bits = (Bits::new(
            ((s_158_20) >> (s_158_17)).value(),
            u16::try_from(s_158_18).unwrap(),
        ));
        // D s_158_22: cast reint s_158_21 -> u8
        let s_158_22: bool = ((s_158_21.value()) != 0);
        // D s_158_23: write-var u#33961 <= s_158_22
        fn_state.u_33961 = s_158_22;
        // C s_158_24: const #16s : i
        let s_158_24: i128 = 16;
        // C s_158_25: const #4s : i
        let s_158_25: i128 = 4;
        // D s_158_26: read-var u#33958:u32
        let s_158_26: u32 = fn_state.u_33958;
        // D s_158_27: cast zx s_158_26 -> bv
        let s_158_27: Bits = Bits::new(s_158_26 as u128, 32u16);
        // D s_158_28: bit-extract s_158_27 s_158_24 s_158_25
        let s_158_28: Bits = (Bits::new(
            ((s_158_27) >> (s_158_24)).value(),
            u16::try_from(s_158_25).unwrap(),
        ));
        // D s_158_29: cast reint s_158_28 -> u8
        let s_158_29: u8 = (s_158_28.value() as u8);
        // D s_158_30: write-var u#33962 <= s_158_29
        fn_state.u_33962 = s_158_29;
        // C s_158_31: const #0s : i
        let s_158_31: i128 = 0;
        // C s_158_32: const #16s : i
        let s_158_32: i128 = 16;
        // D s_158_33: read-var u#33958:u32
        let s_158_33: u32 = fn_state.u_33958;
        // D s_158_34: cast zx s_158_33 -> bv
        let s_158_34: Bits = Bits::new(s_158_33 as u128, 32u16);
        // D s_158_35: bit-extract s_158_34 s_158_31 s_158_32
        let s_158_35: Bits = (Bits::new(
            ((s_158_34) >> (s_158_31)).value(),
            u16::try_from(s_158_32).unwrap(),
        ));
        // D s_158_36: cast reint s_158_35 -> u16
        let s_158_36: u16 = (s_158_35.value() as u16);
        // D s_158_37: write-var u#33963 <= s_158_36
        fn_state.u_33963 = s_158_36;
        // C s_158_38: const #21s : i
        let s_158_38: i128 = 21;
        // D s_158_39: read-var u#33958:u32
        let s_158_39: u32 = fn_state.u_33958;
        // D s_158_40: cast zx s_158_39 -> bv
        let s_158_40: Bits = Bits::new(s_158_39 as u128, 32u16);
        // C s_158_41: const #1u : u64
        let s_158_41: u64 = 1;
        // D s_158_42: bit-extract s_158_40 s_158_38 s_158_41
        let s_158_42: Bits = (Bits::new(
            ((s_158_40) >> (s_158_38)).value(),
            u16::try_from(s_158_41).unwrap(),
        ));
        // D s_158_43: cast reint s_158_42 -> u8
        let s_158_43: bool = ((s_158_42.value()) != 0);
        // C s_158_44: const #0s : i
        let s_158_44: i128 = 0;
        // C s_158_45: const #0u : u64
        let s_158_45: u64 = 0;
        // D s_158_46: cast zx s_158_43 -> u64
        let s_158_46: u64 = (s_158_43 as u64);
        // C s_158_47: const #1u : u64
        let s_158_47: u64 = 1;
        // D s_158_48: and s_158_46 s_158_47
        let s_158_48: u64 = ((s_158_46) & (s_158_47));
        // D s_158_49: cmp-eq s_158_48 s_158_47
        let s_158_49: bool = ((s_158_48) == (s_158_47));
        // D s_158_50: lsl s_158_46 s_158_44
        let s_158_50: u64 = s_158_46 << s_158_44;
        // D s_158_51: or s_158_45 s_158_50
        let s_158_51: u64 = ((s_158_45) | (s_158_50));
        // D s_158_52: cmpl s_158_50
        let s_158_52: u64 = !s_158_50;
        // D s_158_53: and s_158_45 s_158_52
        let s_158_53: u64 = ((s_158_45) & (s_158_52));
        // D s_158_54: select s_158_49 s_158_51 s_158_53
        let s_158_54: u64 = if s_158_49 { s_158_51 } else { s_158_53 };
        // D s_158_55: cast trunc s_158_54 -> u8
        let s_158_55: bool = ((s_158_54) != 0);
        // D s_158_56: cast zx s_158_55 -> bv
        let s_158_56: Bits = Bits::new(s_158_55 as u128, 1u16);
        // C s_158_57: const #0u : u8
        let s_158_57: bool = false;
        // C s_158_58: cast zx s_158_57 -> bv
        let s_158_58: Bits = Bits::new(s_158_57 as u128, 1u16);
        // D s_158_59: cmp-ne s_158_56 s_158_58
        let s_158_59: bool = ((s_158_56) != (s_158_58));
        // N s_158_60: branch s_158_59 b160 b159
        if s_158_59 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var u#33959:u8
        let s_159_0: u8 = fn_state.u_33959;
        // D s_159_1: read-var u#33960:u8
        let s_159_1: bool = fn_state.u_33960;
        // D s_159_2: read-var u#33961:u8
        let s_159_2: bool = fn_state.u_33961;
        // D s_159_3: read-var u#33962:u8
        let s_159_3: u8 = fn_state.u_33962;
        // D s_159_4: read-var u#33963:u16
        let s_159_4: u16 = fn_state.u_33963;
        // D s_159_5: call decode_aarch32_instrs_STM_u_A1enc_AS_txt(s_159_0, s_159_1, s_159_2, s_159_3, s_159_4)
        let s_159_5: () = decode_aarch32_instrs_STM_u_A1enc_AS_txt(
            state,
            tracer,
            s_159_0,
            s_159_1,
            s_159_2,
            s_159_3,
            s_159_4,
        );
        // N s_159_6: return
        return;
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_160_0: panic
        panic!("{:?}", ());
        // N s_160_1: return
        return;
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_161_0: panic
        panic!("{:?}", ());
        // N s_161_1: return
        return;
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #28s : i
        let s_162_0: i128 = 28;
        // C s_162_1: const #4s : i
        let s_162_1: i128 = 4;
        // D s_162_2: read-var u#33958:u32
        let s_162_2: u32 = fn_state.u_33958;
        // D s_162_3: cast zx s_162_2 -> bv
        let s_162_3: Bits = Bits::new(s_162_2 as u128, 32u16);
        // D s_162_4: bit-extract s_162_3 s_162_0 s_162_1
        let s_162_4: Bits = (Bits::new(
            ((s_162_3) >> (s_162_0)).value(),
            u16::try_from(s_162_1).unwrap(),
        ));
        // D s_162_5: cast reint s_162_4 -> u8
        let s_162_5: u8 = (s_162_4.value() as u8);
        // D s_162_6: cast zx s_162_5 -> bv
        let s_162_6: Bits = Bits::new(s_162_5 as u128, 4u16);
        // C s_162_7: const #15u : u8
        let s_162_7: u8 = 15;
        // C s_162_8: cast zx s_162_7 -> bv
        let s_162_8: Bits = Bits::new(s_162_7 as u128, 4u16);
        // D s_162_9: cmp-ne s_162_6 s_162_8
        let s_162_9: bool = ((s_162_6) != (s_162_8));
        // N s_162_10: branch s_162_9 b165 b163
        if s_162_9 {
            return block_165(state, tracer, fn_state);
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
        // D s_163_1: write-var gs#411749 <= s_163_0
        fn_state.gs_411749 = s_163_0;
        // N s_163_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var gs#411749:u8
        let s_164_0: bool = fn_state.gs_411749;
        // D s_164_1: write-var gs#411750 <= s_164_0
        fn_state.gs_411750 = s_164_0;
        // N s_164_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #3795s : i
        let s_165_0: i128 = 3795;
        // C s_165_1: const #14696u : u32
        let s_165_1: u32 = 14696;
        // D s_165_2: read-reg s_165_1:i
        let s_165_2: i128 = {
            let value = state.read_register::<i128>(s_165_1 as isize);
            tracer.read_register(s_165_1 as isize, value);
            value
        };
        // D s_165_3: cmp-lt s_165_2 s_165_0
        let s_165_3: bool = ((s_165_2) < (s_165_0));
        // D s_165_4: write-var gs#411749 <= s_165_3
        fn_state.gs_411749 = s_165_3;
        // N s_165_5: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #20s : i
        let s_166_0: i128 = 20;
        // D s_166_1: read-var u#33958:u32
        let s_166_1: u32 = fn_state.u_33958;
        // D s_166_2: cast zx s_166_1 -> bv
        let s_166_2: Bits = Bits::new(s_166_1 as u128, 32u16);
        // C s_166_3: const #1s : i64
        let s_166_3: i64 = 1;
        // C s_166_4: cast zx s_166_3 -> i
        let s_166_4: i128 = (i128::try_from(s_166_3).unwrap());
        // C s_166_5: const #2s : i
        let s_166_5: i128 = 2;
        // C s_166_6: add s_166_5 s_166_4
        let s_166_6: i128 = (s_166_5 + s_166_4);
        // D s_166_7: bit-extract s_166_2 s_166_0 s_166_6
        let s_166_7: Bits = (Bits::new(
            ((s_166_2) >> (s_166_0)).value(),
            u16::try_from(s_166_6).unwrap(),
        ));
        // D s_166_8: cast reint s_166_7 -> u8
        let s_166_8: u8 = (s_166_7.value() as u8);
        // D s_166_9: cast zx s_166_8 -> bv
        let s_166_9: Bits = Bits::new(s_166_8 as u128, 3u16);
        // C s_166_10: const #4u : u8
        let s_166_10: u8 = 4;
        // C s_166_11: cast zx s_166_10 -> bv
        let s_166_11: Bits = Bits::new(s_166_10 as u128, 3u16);
        // D s_166_12: cmp-eq s_166_9 s_166_11
        let s_166_12: bool = ((s_166_9) == (s_166_11));
        // D s_166_13: write-var gs#411745 <= s_166_12
        fn_state.gs_411745 = s_166_12;
        // N s_166_14: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #3792s : i
        let s_167_0: i128 = 3792;
        // C s_167_1: const #14696u : u32
        let s_167_1: u32 = 14696;
        // D s_167_2: read-reg s_167_1:i
        let s_167_2: i128 = {
            let value = state.read_register::<i128>(s_167_1 as isize);
            tracer.read_register(s_167_1 as isize, value);
            value
        };
        // D s_167_3: cmp-lt s_167_2 s_167_0
        let s_167_3: bool = ((s_167_2) < (s_167_0));
        // D s_167_4: write-var gs#411697 <= s_167_3
        fn_state.gs_411697 = s_167_3;
        // N s_167_5: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #22s : i
        let s_168_0: i128 = 22;
        // D s_168_1: read-var u#33953:u32
        let s_168_1: u32 = fn_state.u_33953;
        // D s_168_2: cast zx s_168_1 -> bv
        let s_168_2: Bits = Bits::new(s_168_1 as u128, 32u16);
        // C s_168_3: const #1s : i64
        let s_168_3: i64 = 1;
        // C s_168_4: cast zx s_168_3 -> i
        let s_168_4: i128 = (i128::try_from(s_168_3).unwrap());
        // C s_168_5: const #0s : i
        let s_168_5: i128 = 0;
        // C s_168_6: add s_168_5 s_168_4
        let s_168_6: i128 = (s_168_5 + s_168_4);
        // D s_168_7: bit-extract s_168_2 s_168_0 s_168_6
        let s_168_7: Bits = (Bits::new(
            ((s_168_2) >> (s_168_0)).value(),
            u16::try_from(s_168_6).unwrap(),
        ));
        // D s_168_8: cast reint s_168_7 -> u8
        let s_168_8: bool = ((s_168_7.value()) != 0);
        // D s_168_9: cast zx s_168_8 -> bv
        let s_168_9: Bits = Bits::new(s_168_8 as u128, 1u16);
        // C s_168_10: const #1u : u8
        let s_168_10: bool = true;
        // C s_168_11: cast zx s_168_10 -> bv
        let s_168_11: Bits = Bits::new(s_168_10 as u128, 1u16);
        // D s_168_12: cmp-eq s_168_9 s_168_11
        let s_168_12: bool = ((s_168_9) == (s_168_11));
        // N s_168_13: branch s_168_12 b171 b169
        if s_168_12 {
            return block_171(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #0u : u8
        let s_169_0: bool = false;
        // D s_169_1: write-var gs#411694 <= s_169_0
        fn_state.gs_411694 = s_169_0;
        // N s_169_2: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var gs#411694:u8
        let s_170_0: bool = fn_state.gs_411694;
        // D s_170_1: write-var gs#411695 <= s_170_0
        fn_state.gs_411695 = s_170_0;
        // N s_170_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #5s : i
        let s_171_0: i128 = 5;
        // D s_171_1: read-var u#33953:u32
        let s_171_1: u32 = fn_state.u_33953;
        // D s_171_2: cast zx s_171_1 -> bv
        let s_171_2: Bits = Bits::new(s_171_1 as u128, 32u16);
        // C s_171_3: const #1s : i64
        let s_171_3: i64 = 1;
        // C s_171_4: cast zx s_171_3 -> i
        let s_171_4: i128 = (i128::try_from(s_171_3).unwrap());
        // C s_171_5: const #15s : i
        let s_171_5: i128 = 15;
        // C s_171_6: add s_171_5 s_171_4
        let s_171_6: i128 = (s_171_5 + s_171_4);
        // D s_171_7: bit-extract s_171_2 s_171_0 s_171_6
        let s_171_7: Bits = (Bits::new(
            ((s_171_2) >> (s_171_0)).value(),
            u16::try_from(s_171_6).unwrap(),
        ));
        // D s_171_8: cast reint s_171_7 -> u16
        let s_171_8: u16 = (s_171_7.value() as u16);
        // D s_171_9: cast zx s_171_8 -> bv
        let s_171_9: Bits = Bits::new(s_171_8 as u128, 16u16);
        // C s_171_10: const #26664u : u16
        let s_171_10: u16 = 26664;
        // C s_171_11: cast zx s_171_10 -> bv
        let s_171_11: Bits = Bits::new(s_171_10 as u128, 16u16);
        // D s_171_12: cmp-eq s_171_9 s_171_11
        let s_171_12: bool = ((s_171_9) == (s_171_11));
        // D s_171_13: write-var gs#411694 <= s_171_12
        fn_state.gs_411694 = s_171_12;
        // N s_171_14: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #3787s : i
        let s_172_0: i128 = 3787;
        // C s_172_1: const #14696u : u32
        let s_172_1: u32 = 14696;
        // D s_172_2: read-reg s_172_1:i
        let s_172_2: i128 = {
            let value = state.read_register::<i128>(s_172_1 as isize);
            tracer.read_register(s_172_1 as isize, value);
            value
        };
        // D s_172_3: cmp-lt s_172_2 s_172_0
        let s_172_3: bool = ((s_172_2) < (s_172_0));
        // D s_172_4: write-var gs#411641 <= s_172_3
        fn_state.gs_411641 = s_172_3;
        // N s_172_5: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #22s : i
        let s_173_0: i128 = 22;
        // D s_173_1: read-var u#33947:u32
        let s_173_1: u32 = fn_state.u_33947;
        // D s_173_2: cast zx s_173_1 -> bv
        let s_173_2: Bits = Bits::new(s_173_1 as u128, 32u16);
        // C s_173_3: const #1s : i64
        let s_173_3: i64 = 1;
        // C s_173_4: cast zx s_173_3 -> i
        let s_173_4: i128 = (i128::try_from(s_173_3).unwrap());
        // C s_173_5: const #0s : i
        let s_173_5: i128 = 0;
        // C s_173_6: add s_173_5 s_173_4
        let s_173_6: i128 = (s_173_5 + s_173_4);
        // D s_173_7: bit-extract s_173_2 s_173_0 s_173_6
        let s_173_7: Bits = (Bits::new(
            ((s_173_2) >> (s_173_0)).value(),
            u16::try_from(s_173_6).unwrap(),
        ));
        // D s_173_8: cast reint s_173_7 -> u8
        let s_173_8: bool = ((s_173_7.value()) != 0);
        // D s_173_9: cast zx s_173_8 -> bv
        let s_173_9: Bits = Bits::new(s_173_8 as u128, 1u16);
        // C s_173_10: const #0u : u8
        let s_173_10: bool = false;
        // C s_173_11: cast zx s_173_10 -> bv
        let s_173_11: Bits = Bits::new(s_173_10 as u128, 1u16);
        // D s_173_12: cmp-eq s_173_9 s_173_11
        let s_173_12: bool = ((s_173_9) == (s_173_11));
        // N s_173_13: branch s_173_12 b176 b174
        if s_173_12 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_174(state, tracer, fn_state);
        };
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #0u : u8
        let s_174_0: bool = false;
        // D s_174_1: write-var gs#411638 <= s_174_0
        fn_state.gs_411638 = s_174_0;
        // N s_174_2: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var gs#411638:u8
        let s_175_0: bool = fn_state.gs_411638;
        // D s_175_1: write-var gs#411639 <= s_175_0
        fn_state.gs_411639 = s_175_0;
        // N s_175_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #20s : i
        let s_176_0: i128 = 20;
        // D s_176_1: read-var u#33947:u32
        let s_176_1: u32 = fn_state.u_33947;
        // D s_176_2: cast zx s_176_1 -> bv
        let s_176_2: Bits = Bits::new(s_176_1 as u128, 32u16);
        // C s_176_3: const #1s : i64
        let s_176_3: i64 = 1;
        // C s_176_4: cast zx s_176_3 -> i
        let s_176_4: i128 = (i128::try_from(s_176_3).unwrap());
        // C s_176_5: const #0s : i
        let s_176_5: i128 = 0;
        // C s_176_6: add s_176_5 s_176_4
        let s_176_6: i128 = (s_176_5 + s_176_4);
        // D s_176_7: bit-extract s_176_2 s_176_0 s_176_6
        let s_176_7: Bits = (Bits::new(
            ((s_176_2) >> (s_176_0)).value(),
            u16::try_from(s_176_6).unwrap(),
        ));
        // D s_176_8: cast reint s_176_7 -> u8
        let s_176_8: bool = ((s_176_7.value()) != 0);
        // D s_176_9: cast zx s_176_8 -> bv
        let s_176_9: Bits = Bits::new(s_176_8 as u128, 1u16);
        // C s_176_10: const #1u : u8
        let s_176_10: bool = true;
        // C s_176_11: cast zx s_176_10 -> bv
        let s_176_11: Bits = Bits::new(s_176_10 as u128, 1u16);
        // D s_176_12: cmp-eq s_176_9 s_176_11
        let s_176_12: bool = ((s_176_9) == (s_176_11));
        // N s_176_13: branch s_176_12 b179 b177
        if s_176_12 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #0u : u8
        let s_177_0: bool = false;
        // D s_177_1: write-var gs#411637 <= s_177_0
        fn_state.gs_411637 = s_177_0;
        // N s_177_2: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var gs#411637:u8
        let s_178_0: bool = fn_state.gs_411637;
        // D s_178_1: write-var gs#411638 <= s_178_0
        fn_state.gs_411638 = s_178_0;
        // N s_178_2: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #0s : i
        let s_179_0: i128 = 0;
        // D s_179_1: read-var u#33947:u32
        let s_179_1: u32 = fn_state.u_33947;
        // D s_179_2: cast zx s_179_1 -> bv
        let s_179_2: Bits = Bits::new(s_179_1 as u128, 32u16);
        // C s_179_3: const #1s : i64
        let s_179_3: i64 = 1;
        // C s_179_4: cast zx s_179_3 -> i
        let s_179_4: i128 = (i128::try_from(s_179_3).unwrap());
        // C s_179_5: const #15s : i
        let s_179_5: i128 = 15;
        // C s_179_6: add s_179_5 s_179_4
        let s_179_6: i128 = (s_179_5 + s_179_4);
        // D s_179_7: bit-extract s_179_2 s_179_0 s_179_6
        let s_179_7: Bits = (Bits::new(
            ((s_179_2) >> (s_179_0)).value(),
            u16::try_from(s_179_6).unwrap(),
        ));
        // D s_179_8: cast reint s_179_7 -> u16
        let s_179_8: u16 = (s_179_7.value() as u16);
        // D s_179_9: cast zx s_179_8 -> bv
        let s_179_9: Bits = Bits::new(s_179_8 as u128, 16u16);
        // C s_179_10: const #2560u : u16
        let s_179_10: u16 = 2560;
        // C s_179_11: cast zx s_179_10 -> bv
        let s_179_11: Bits = Bits::new(s_179_10 as u128, 16u16);
        // D s_179_12: cmp-eq s_179_9 s_179_11
        let s_179_12: bool = ((s_179_9) == (s_179_11));
        // D s_179_13: write-var gs#411637 <= s_179_12
        fn_state.gs_411637 = s_179_12;
        // N s_179_14: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #28s : i
        let s_180_0: i128 = 28;
        // C s_180_1: const #4s : i
        let s_180_1: i128 = 4;
        // D s_180_2: read-var u#33940:u32
        let s_180_2: u32 = fn_state.u_33940;
        // D s_180_3: cast zx s_180_2 -> bv
        let s_180_3: Bits = Bits::new(s_180_2 as u128, 32u16);
        // D s_180_4: bit-extract s_180_3 s_180_0 s_180_1
        let s_180_4: Bits = (Bits::new(
            ((s_180_3) >> (s_180_0)).value(),
            u16::try_from(s_180_1).unwrap(),
        ));
        // D s_180_5: cast reint s_180_4 -> u8
        let s_180_5: u8 = (s_180_4.value() as u8);
        // D s_180_6: cast zx s_180_5 -> bv
        let s_180_6: Bits = Bits::new(s_180_5 as u128, 4u16);
        // C s_180_7: const #15u : u8
        let s_180_7: u8 = 15;
        // C s_180_8: cast zx s_180_7 -> bv
        let s_180_8: Bits = Bits::new(s_180_7 as u128, 4u16);
        // D s_180_9: cmp-ne s_180_6 s_180_8
        let s_180_9: bool = ((s_180_6) != (s_180_8));
        // N s_180_10: branch s_180_9 b183 b181
        if s_180_9 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_181(state, tracer, fn_state);
        };
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #0u : u8
        let s_181_0: bool = false;
        // D s_181_1: write-var gs#411612 <= s_181_0
        fn_state.gs_411612 = s_181_0;
        // N s_181_2: jump b182
        return block_182(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var gs#411612:u8
        let s_182_0: bool = fn_state.gs_411612;
        // D s_182_1: write-var gs#411613 <= s_182_0
        fn_state.gs_411613 = s_182_0;
        // N s_182_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #3777s : i
        let s_183_0: i128 = 3777;
        // C s_183_1: const #14696u : u32
        let s_183_1: u32 = 14696;
        // D s_183_2: read-reg s_183_1:i
        let s_183_2: i128 = {
            let value = state.read_register::<i128>(s_183_1 as isize);
            tracer.read_register(s_183_1 as isize, value);
            value
        };
        // D s_183_3: cmp-lt s_183_2 s_183_0
        let s_183_3: bool = ((s_183_2) < (s_183_0));
        // D s_183_4: write-var gs#411612 <= s_183_3
        fn_state.gs_411612 = s_183_3;
        // N s_183_5: jump b182
        return block_182(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #20s : i
        let s_184_0: i128 = 20;
        // D s_184_1: read-var u#33940:u32
        let s_184_1: u32 = fn_state.u_33940;
        // D s_184_2: cast zx s_184_1 -> bv
        let s_184_2: Bits = Bits::new(s_184_1 as u128, 32u16);
        // C s_184_3: const #1s : i64
        let s_184_3: i64 = 1;
        // C s_184_4: cast zx s_184_3 -> i
        let s_184_4: i128 = (i128::try_from(s_184_3).unwrap());
        // C s_184_5: const #2s : i
        let s_184_5: i128 = 2;
        // C s_184_6: add s_184_5 s_184_4
        let s_184_6: i128 = (s_184_5 + s_184_4);
        // D s_184_7: bit-extract s_184_2 s_184_0 s_184_6
        let s_184_7: Bits = (Bits::new(
            ((s_184_2) >> (s_184_0)).value(),
            u16::try_from(s_184_6).unwrap(),
        ));
        // D s_184_8: cast reint s_184_7 -> u8
        let s_184_8: u8 = (s_184_7.value() as u8);
        // D s_184_9: cast zx s_184_8 -> bv
        let s_184_9: Bits = Bits::new(s_184_8 as u128, 3u16);
        // C s_184_10: const #5u : u8
        let s_184_10: u8 = 5;
        // C s_184_11: cast zx s_184_10 -> bv
        let s_184_11: Bits = Bits::new(s_184_10 as u128, 3u16);
        // D s_184_12: cmp-eq s_184_9 s_184_11
        let s_184_12: bool = ((s_184_9) == (s_184_11));
        // N s_184_13: branch s_184_12 b187 b185
        if s_184_12 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_185(state, tracer, fn_state);
        };
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #0u : u8
        let s_185_0: bool = false;
        // D s_185_1: write-var gs#411607 <= s_185_0
        fn_state.gs_411607 = s_185_0;
        // N s_185_2: jump b186
        return block_186(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var gs#411607:u8
        let s_186_0: bool = fn_state.gs_411607;
        // D s_186_1: write-var gs#411608 <= s_186_0
        fn_state.gs_411608 = s_186_0;
        // N s_186_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #15s : i
        let s_187_0: i128 = 15;
        // D s_187_1: read-var u#33940:u32
        let s_187_1: u32 = fn_state.u_33940;
        // D s_187_2: cast zx s_187_1 -> bv
        let s_187_2: Bits = Bits::new(s_187_1 as u128, 32u16);
        // C s_187_3: const #1s : i64
        let s_187_3: i64 = 1;
        // C s_187_4: cast zx s_187_3 -> i
        let s_187_4: i128 = (i128::try_from(s_187_3).unwrap());
        // C s_187_5: const #0s : i
        let s_187_5: i128 = 0;
        // C s_187_6: add s_187_5 s_187_4
        let s_187_6: i128 = (s_187_5 + s_187_4);
        // D s_187_7: bit-extract s_187_2 s_187_0 s_187_6
        let s_187_7: Bits = (Bits::new(
            ((s_187_2) >> (s_187_0)).value(),
            u16::try_from(s_187_6).unwrap(),
        ));
        // D s_187_8: cast reint s_187_7 -> u8
        let s_187_8: bool = ((s_187_7.value()) != 0);
        // D s_187_9: cast zx s_187_8 -> bv
        let s_187_9: Bits = Bits::new(s_187_8 as u128, 1u16);
        // C s_187_10: const #0u : u8
        let s_187_10: bool = false;
        // C s_187_11: cast zx s_187_10 -> bv
        let s_187_11: Bits = Bits::new(s_187_10 as u128, 1u16);
        // D s_187_12: cmp-eq s_187_9 s_187_11
        let s_187_12: bool = ((s_187_9) == (s_187_11));
        // D s_187_13: write-var gs#411607 <= s_187_12
        fn_state.gs_411607 = s_187_12;
        // N s_187_14: jump b186
        return block_186(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #28s : i
        let s_188_0: i128 = 28;
        // C s_188_1: const #4s : i
        let s_188_1: i128 = 4;
        // D s_188_2: read-var u#33934:u32
        let s_188_2: u32 = fn_state.u_33934;
        // D s_188_3: cast zx s_188_2 -> bv
        let s_188_3: Bits = Bits::new(s_188_2 as u128, 32u16);
        // D s_188_4: bit-extract s_188_3 s_188_0 s_188_1
        let s_188_4: Bits = (Bits::new(
            ((s_188_3) >> (s_188_0)).value(),
            u16::try_from(s_188_1).unwrap(),
        ));
        // D s_188_5: cast reint s_188_4 -> u8
        let s_188_5: u8 = (s_188_4.value() as u8);
        // D s_188_6: cast zx s_188_5 -> bv
        let s_188_6: Bits = Bits::new(s_188_5 as u128, 4u16);
        // C s_188_7: const #15u : u8
        let s_188_7: u8 = 15;
        // C s_188_8: cast zx s_188_7 -> bv
        let s_188_8: Bits = Bits::new(s_188_7 as u128, 4u16);
        // D s_188_9: cmp-ne s_188_6 s_188_8
        let s_188_9: bool = ((s_188_6) != (s_188_8));
        // N s_188_10: branch s_188_9 b191 b189
        if s_188_9 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_189(state, tracer, fn_state);
        };
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #0u : u8
        let s_189_0: bool = false;
        // D s_189_1: write-var gs#411584 <= s_189_0
        fn_state.gs_411584 = s_189_0;
        // N s_189_2: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var gs#411584:u8
        let s_190_0: bool = fn_state.gs_411584;
        // D s_190_1: write-var gs#411585 <= s_190_0
        fn_state.gs_411585 = s_190_0;
        // N s_190_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #3776s : i
        let s_191_0: i128 = 3776;
        // C s_191_1: const #14696u : u32
        let s_191_1: u32 = 14696;
        // D s_191_2: read-reg s_191_1:i
        let s_191_2: i128 = {
            let value = state.read_register::<i128>(s_191_1 as isize);
            tracer.read_register(s_191_1 as isize, value);
            value
        };
        // D s_191_3: cmp-lt s_191_2 s_191_0
        let s_191_3: bool = ((s_191_2) < (s_191_0));
        // D s_191_4: write-var gs#411584 <= s_191_3
        fn_state.gs_411584 = s_191_3;
        // N s_191_5: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #22s : i
        let s_192_0: i128 = 22;
        // D s_192_1: read-var u#33934:u32
        let s_192_1: u32 = fn_state.u_33934;
        // D s_192_2: cast zx s_192_1 -> bv
        let s_192_2: Bits = Bits::new(s_192_1 as u128, 32u16);
        // C s_192_3: const #1s : i64
        let s_192_3: i64 = 1;
        // C s_192_4: cast zx s_192_3 -> i
        let s_192_4: i128 = (i128::try_from(s_192_3).unwrap());
        // C s_192_5: const #0s : i
        let s_192_5: i128 = 0;
        // C s_192_6: add s_192_5 s_192_4
        let s_192_6: i128 = (s_192_5 + s_192_4);
        // D s_192_7: bit-extract s_192_2 s_192_0 s_192_6
        let s_192_7: Bits = (Bits::new(
            ((s_192_2) >> (s_192_0)).value(),
            u16::try_from(s_192_6).unwrap(),
        ));
        // D s_192_8: cast reint s_192_7 -> u8
        let s_192_8: bool = ((s_192_7.value()) != 0);
        // D s_192_9: cast zx s_192_8 -> bv
        let s_192_9: Bits = Bits::new(s_192_8 as u128, 1u16);
        // C s_192_10: const #1u : u8
        let s_192_10: bool = true;
        // C s_192_11: cast zx s_192_10 -> bv
        let s_192_11: Bits = Bits::new(s_192_10 as u128, 1u16);
        // D s_192_12: cmp-eq s_192_9 s_192_11
        let s_192_12: bool = ((s_192_9) == (s_192_11));
        // N s_192_13: branch s_192_12 b195 b193
        if s_192_12 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_193(state, tracer, fn_state);
        };
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #0u : u8
        let s_193_0: bool = false;
        // D s_193_1: write-var gs#411579 <= s_193_0
        fn_state.gs_411579 = s_193_0;
        // N s_193_2: jump b194
        return block_194(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var gs#411579:u8
        let s_194_0: bool = fn_state.gs_411579;
        // D s_194_1: write-var gs#411580 <= s_194_0
        fn_state.gs_411580 = s_194_0;
        // N s_194_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #20s : i
        let s_195_0: i128 = 20;
        // D s_195_1: read-var u#33934:u32
        let s_195_1: u32 = fn_state.u_33934;
        // D s_195_2: cast zx s_195_1 -> bv
        let s_195_2: Bits = Bits::new(s_195_1 as u128, 32u16);
        // C s_195_3: const #1s : i64
        let s_195_3: i64 = 1;
        // C s_195_4: cast zx s_195_3 -> i
        let s_195_4: i128 = (i128::try_from(s_195_3).unwrap());
        // C s_195_5: const #0s : i
        let s_195_5: i128 = 0;
        // C s_195_6: add s_195_5 s_195_4
        let s_195_6: i128 = (s_195_5 + s_195_4);
        // D s_195_7: bit-extract s_195_2 s_195_0 s_195_6
        let s_195_7: Bits = (Bits::new(
            ((s_195_2) >> (s_195_0)).value(),
            u16::try_from(s_195_6).unwrap(),
        ));
        // D s_195_8: cast reint s_195_7 -> u8
        let s_195_8: bool = ((s_195_7.value()) != 0);
        // D s_195_9: cast zx s_195_8 -> bv
        let s_195_9: Bits = Bits::new(s_195_8 as u128, 1u16);
        // C s_195_10: const #1u : u8
        let s_195_10: bool = true;
        // C s_195_11: cast zx s_195_10 -> bv
        let s_195_11: Bits = Bits::new(s_195_10 as u128, 1u16);
        // D s_195_12: cmp-eq s_195_9 s_195_11
        let s_195_12: bool = ((s_195_9) == (s_195_11));
        // N s_195_13: branch s_195_12 b198 b196
        if s_195_12 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_196(state, tracer, fn_state);
        };
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #0u : u8
        let s_196_0: bool = false;
        // D s_196_1: write-var gs#411578 <= s_196_0
        fn_state.gs_411578 = s_196_0;
        // N s_196_2: jump b197
        return block_197(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_197_0: read-var gs#411578:u8
        let s_197_0: bool = fn_state.gs_411578;
        // D s_197_1: write-var gs#411579 <= s_197_0
        fn_state.gs_411579 = s_197_0;
        // N s_197_2: jump b194
        return block_194(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #15s : i
        let s_198_0: i128 = 15;
        // D s_198_1: read-var u#33934:u32
        let s_198_1: u32 = fn_state.u_33934;
        // D s_198_2: cast zx s_198_1 -> bv
        let s_198_2: Bits = Bits::new(s_198_1 as u128, 32u16);
        // C s_198_3: const #1s : i64
        let s_198_3: i64 = 1;
        // C s_198_4: cast zx s_198_3 -> i
        let s_198_4: i128 = (i128::try_from(s_198_3).unwrap());
        // C s_198_5: const #0s : i
        let s_198_5: i128 = 0;
        // C s_198_6: add s_198_5 s_198_4
        let s_198_6: i128 = (s_198_5 + s_198_4);
        // D s_198_7: bit-extract s_198_2 s_198_0 s_198_6
        let s_198_7: Bits = (Bits::new(
            ((s_198_2) >> (s_198_0)).value(),
            u16::try_from(s_198_6).unwrap(),
        ));
        // D s_198_8: cast reint s_198_7 -> u8
        let s_198_8: bool = ((s_198_7.value()) != 0);
        // D s_198_9: cast zx s_198_8 -> bv
        let s_198_9: Bits = Bits::new(s_198_8 as u128, 1u16);
        // C s_198_10: const #1u : u8
        let s_198_10: bool = true;
        // C s_198_11: cast zx s_198_10 -> bv
        let s_198_11: Bits = Bits::new(s_198_10 as u128, 1u16);
        // D s_198_12: cmp-eq s_198_9 s_198_11
        let s_198_12: bool = ((s_198_9) == (s_198_11));
        // D s_198_13: write-var gs#411578 <= s_198_12
        fn_state.gs_411578 = s_198_12;
        // N s_198_14: jump b197
        return block_197(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #28s : i
        let s_199_0: i128 = 28;
        // C s_199_1: const #4s : i
        let s_199_1: i128 = 4;
        // D s_199_2: read-var u#33928:u32
        let s_199_2: u32 = fn_state.u_33928;
        // D s_199_3: cast zx s_199_2 -> bv
        let s_199_3: Bits = Bits::new(s_199_2 as u128, 32u16);
        // D s_199_4: bit-extract s_199_3 s_199_0 s_199_1
        let s_199_4: Bits = (Bits::new(
            ((s_199_3) >> (s_199_0)).value(),
            u16::try_from(s_199_1).unwrap(),
        ));
        // D s_199_5: cast reint s_199_4 -> u8
        let s_199_5: u8 = (s_199_4.value() as u8);
        // D s_199_6: cast zx s_199_5 -> bv
        let s_199_6: Bits = Bits::new(s_199_5 as u128, 4u16);
        // C s_199_7: const #15u : u8
        let s_199_7: u8 = 15;
        // C s_199_8: cast zx s_199_7 -> bv
        let s_199_8: Bits = Bits::new(s_199_7 as u128, 4u16);
        // D s_199_9: cmp-ne s_199_6 s_199_8
        let s_199_9: bool = ((s_199_6) != (s_199_8));
        // N s_199_10: branch s_199_9 b202 b200
        if s_199_9 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_200(state, tracer, fn_state);
        };
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_200_0: const #0u : u8
        let s_200_0: bool = false;
        // D s_200_1: write-var gs#411557 <= s_200_0
        fn_state.gs_411557 = s_200_0;
        // N s_200_2: jump b201
        return block_201(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_201_0: read-var gs#411557:u8
        let s_201_0: bool = fn_state.gs_411557;
        // D s_201_1: write-var gs#411558 <= s_201_0
        fn_state.gs_411558 = s_201_0;
        // N s_201_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_202_0: const #3192s : i
        let s_202_0: i128 = 3192;
        // C s_202_1: const #14696u : u32
        let s_202_1: u32 = 14696;
        // D s_202_2: read-reg s_202_1:i
        let s_202_2: i128 = {
            let value = state.read_register::<i128>(s_202_1 as isize);
            tracer.read_register(s_202_1 as isize, value);
            value
        };
        // D s_202_3: cmp-lt s_202_2 s_202_0
        let s_202_3: bool = ((s_202_2) < (s_202_0));
        // D s_202_4: write-var gs#411557 <= s_202_3
        fn_state.gs_411557 = s_202_3;
        // N s_202_5: jump b201
        return block_201(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #20s : i
        let s_203_0: i128 = 20;
        // D s_203_1: read-var u#33928:u32
        let s_203_1: u32 = fn_state.u_33928;
        // D s_203_2: cast zx s_203_1 -> bv
        let s_203_2: Bits = Bits::new(s_203_1 as u128, 32u16);
        // C s_203_3: const #1s : i64
        let s_203_3: i64 = 1;
        // C s_203_4: cast zx s_203_3 -> i
        let s_203_4: i128 = (i128::try_from(s_203_3).unwrap());
        // C s_203_5: const #0s : i
        let s_203_5: i128 = 0;
        // C s_203_6: add s_203_5 s_203_4
        let s_203_6: i128 = (s_203_5 + s_203_4);
        // D s_203_7: bit-extract s_203_2 s_203_0 s_203_6
        let s_203_7: Bits = (Bits::new(
            ((s_203_2) >> (s_203_0)).value(),
            u16::try_from(s_203_6).unwrap(),
        ));
        // D s_203_8: cast reint s_203_7 -> u8
        let s_203_8: bool = ((s_203_7.value()) != 0);
        // D s_203_9: cast zx s_203_8 -> bv
        let s_203_9: Bits = Bits::new(s_203_8 as u128, 1u16);
        // C s_203_10: const #0u : u8
        let s_203_10: bool = false;
        // C s_203_11: cast zx s_203_10 -> bv
        let s_203_11: Bits = Bits::new(s_203_10 as u128, 1u16);
        // D s_203_12: cmp-eq s_203_9 s_203_11
        let s_203_12: bool = ((s_203_9) == (s_203_11));
        // D s_203_13: write-var gs#411553 <= s_203_12
        fn_state.gs_411553 = s_203_12;
        // N s_203_14: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #28s : i
        let s_204_0: i128 = 28;
        // C s_204_1: const #4s : i
        let s_204_1: i128 = 4;
        // D s_204_2: read-var u#33922:u32
        let s_204_2: u32 = fn_state.u_33922;
        // D s_204_3: cast zx s_204_2 -> bv
        let s_204_3: Bits = Bits::new(s_204_2 as u128, 32u16);
        // D s_204_4: bit-extract s_204_3 s_204_0 s_204_1
        let s_204_4: Bits = (Bits::new(
            ((s_204_3) >> (s_204_0)).value(),
            u16::try_from(s_204_1).unwrap(),
        ));
        // D s_204_5: cast reint s_204_4 -> u8
        let s_204_5: u8 = (s_204_4.value() as u8);
        // D s_204_6: cast zx s_204_5 -> bv
        let s_204_6: Bits = Bits::new(s_204_5 as u128, 4u16);
        // C s_204_7: const #15u : u8
        let s_204_7: u8 = 15;
        // C s_204_8: cast zx s_204_7 -> bv
        let s_204_8: Bits = Bits::new(s_204_7 as u128, 4u16);
        // D s_204_9: cmp-ne s_204_6 s_204_8
        let s_204_9: bool = ((s_204_6) != (s_204_8));
        // N s_204_10: branch s_204_9 b207 b205
        if s_204_9 {
            return block_207(state, tracer, fn_state);
        } else {
            return block_205(state, tracer, fn_state);
        };
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #0u : u8
        let s_205_0: bool = false;
        // D s_205_1: write-var gs#411536 <= s_205_0
        fn_state.gs_411536 = s_205_0;
        // N s_205_2: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var gs#411536:u8
        let s_206_0: bool = fn_state.gs_411536;
        // D s_206_1: write-var gs#411537 <= s_206_0
        fn_state.gs_411537 = s_206_0;
        // N s_206_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #3190s : i
        let s_207_0: i128 = 3190;
        // C s_207_1: const #14696u : u32
        let s_207_1: u32 = 14696;
        // D s_207_2: read-reg s_207_1:i
        let s_207_2: i128 = {
            let value = state.read_register::<i128>(s_207_1 as isize);
            tracer.read_register(s_207_1 as isize, value);
            value
        };
        // D s_207_3: cmp-lt s_207_2 s_207_0
        let s_207_3: bool = ((s_207_2) < (s_207_0));
        // D s_207_4: write-var gs#411536 <= s_207_3
        fn_state.gs_411536 = s_207_3;
        // N s_207_5: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_208_0: const #20s : i
        let s_208_0: i128 = 20;
        // D s_208_1: read-var u#33922:u32
        let s_208_1: u32 = fn_state.u_33922;
        // D s_208_2: cast zx s_208_1 -> bv
        let s_208_2: Bits = Bits::new(s_208_1 as u128, 32u16);
        // C s_208_3: const #1s : i64
        let s_208_3: i64 = 1;
        // C s_208_4: cast zx s_208_3 -> i
        let s_208_4: i128 = (i128::try_from(s_208_3).unwrap());
        // C s_208_5: const #0s : i
        let s_208_5: i128 = 0;
        // C s_208_6: add s_208_5 s_208_4
        let s_208_6: i128 = (s_208_5 + s_208_4);
        // D s_208_7: bit-extract s_208_2 s_208_0 s_208_6
        let s_208_7: Bits = (Bits::new(
            ((s_208_2) >> (s_208_0)).value(),
            u16::try_from(s_208_6).unwrap(),
        ));
        // D s_208_8: cast reint s_208_7 -> u8
        let s_208_8: bool = ((s_208_7.value()) != 0);
        // D s_208_9: cast zx s_208_8 -> bv
        let s_208_9: Bits = Bits::new(s_208_8 as u128, 1u16);
        // C s_208_10: const #0u : u8
        let s_208_10: bool = false;
        // C s_208_11: cast zx s_208_10 -> bv
        let s_208_11: Bits = Bits::new(s_208_10 as u128, 1u16);
        // D s_208_12: cmp-eq s_208_9 s_208_11
        let s_208_12: bool = ((s_208_9) == (s_208_11));
        // D s_208_13: write-var gs#411532 <= s_208_12
        fn_state.gs_411532 = s_208_12;
        // N s_208_14: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #28s : i
        let s_209_0: i128 = 28;
        // C s_209_1: const #4s : i
        let s_209_1: i128 = 4;
        // D s_209_2: read-var u#33916:u32
        let s_209_2: u32 = fn_state.u_33916;
        // D s_209_3: cast zx s_209_2 -> bv
        let s_209_3: Bits = Bits::new(s_209_2 as u128, 32u16);
        // D s_209_4: bit-extract s_209_3 s_209_0 s_209_1
        let s_209_4: Bits = (Bits::new(
            ((s_209_3) >> (s_209_0)).value(),
            u16::try_from(s_209_1).unwrap(),
        ));
        // D s_209_5: cast reint s_209_4 -> u8
        let s_209_5: u8 = (s_209_4.value() as u8);
        // D s_209_6: cast zx s_209_5 -> bv
        let s_209_6: Bits = Bits::new(s_209_5 as u128, 4u16);
        // C s_209_7: const #15u : u8
        let s_209_7: u8 = 15;
        // C s_209_8: cast zx s_209_7 -> bv
        let s_209_8: Bits = Bits::new(s_209_7 as u128, 4u16);
        // D s_209_9: cmp-ne s_209_6 s_209_8
        let s_209_9: bool = ((s_209_6) != (s_209_8));
        // N s_209_10: branch s_209_9 b212 b210
        if s_209_9 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_210(state, tracer, fn_state);
        };
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_210_0: const #0u : u8
        let s_210_0: bool = false;
        // D s_210_1: write-var gs#411515 <= s_210_0
        fn_state.gs_411515 = s_210_0;
        // N s_210_2: jump b211
        return block_211(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_211_0: read-var gs#411515:u8
        let s_211_0: bool = fn_state.gs_411515;
        // D s_211_1: write-var gs#411516 <= s_211_0
        fn_state.gs_411516 = s_211_0;
        // N s_211_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_212_0: const #3189s : i
        let s_212_0: i128 = 3189;
        // C s_212_1: const #14696u : u32
        let s_212_1: u32 = 14696;
        // D s_212_2: read-reg s_212_1:i
        let s_212_2: i128 = {
            let value = state.read_register::<i128>(s_212_1 as isize);
            tracer.read_register(s_212_1 as isize, value);
            value
        };
        // D s_212_3: cmp-lt s_212_2 s_212_0
        let s_212_3: bool = ((s_212_2) < (s_212_0));
        // D s_212_4: write-var gs#411515 <= s_212_3
        fn_state.gs_411515 = s_212_3;
        // N s_212_5: jump b211
        return block_211(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #20s : i
        let s_213_0: i128 = 20;
        // D s_213_1: read-var u#33916:u32
        let s_213_1: u32 = fn_state.u_33916;
        // D s_213_2: cast zx s_213_1 -> bv
        let s_213_2: Bits = Bits::new(s_213_1 as u128, 32u16);
        // C s_213_3: const #1s : i64
        let s_213_3: i64 = 1;
        // C s_213_4: cast zx s_213_3 -> i
        let s_213_4: i128 = (i128::try_from(s_213_3).unwrap());
        // C s_213_5: const #0s : i
        let s_213_5: i128 = 0;
        // C s_213_6: add s_213_5 s_213_4
        let s_213_6: i128 = (s_213_5 + s_213_4);
        // D s_213_7: bit-extract s_213_2 s_213_0 s_213_6
        let s_213_7: Bits = (Bits::new(
            ((s_213_2) >> (s_213_0)).value(),
            u16::try_from(s_213_6).unwrap(),
        ));
        // D s_213_8: cast reint s_213_7 -> u8
        let s_213_8: bool = ((s_213_7.value()) != 0);
        // D s_213_9: cast zx s_213_8 -> bv
        let s_213_9: Bits = Bits::new(s_213_8 as u128, 1u16);
        // C s_213_10: const #0u : u8
        let s_213_10: bool = false;
        // C s_213_11: cast zx s_213_10 -> bv
        let s_213_11: Bits = Bits::new(s_213_10 as u128, 1u16);
        // D s_213_12: cmp-eq s_213_9 s_213_11
        let s_213_12: bool = ((s_213_9) == (s_213_11));
        // D s_213_13: write-var gs#411511 <= s_213_12
        fn_state.gs_411511 = s_213_12;
        // N s_213_14: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_214_0: const #28s : i
        let s_214_0: i128 = 28;
        // C s_214_1: const #4s : i
        let s_214_1: i128 = 4;
        // D s_214_2: read-var u#33910:u32
        let s_214_2: u32 = fn_state.u_33910;
        // D s_214_3: cast zx s_214_2 -> bv
        let s_214_3: Bits = Bits::new(s_214_2 as u128, 32u16);
        // D s_214_4: bit-extract s_214_3 s_214_0 s_214_1
        let s_214_4: Bits = (Bits::new(
            ((s_214_3) >> (s_214_0)).value(),
            u16::try_from(s_214_1).unwrap(),
        ));
        // D s_214_5: cast reint s_214_4 -> u8
        let s_214_5: u8 = (s_214_4.value() as u8);
        // D s_214_6: cast zx s_214_5 -> bv
        let s_214_6: Bits = Bits::new(s_214_5 as u128, 4u16);
        // C s_214_7: const #15u : u8
        let s_214_7: u8 = 15;
        // C s_214_8: cast zx s_214_7 -> bv
        let s_214_8: Bits = Bits::new(s_214_7 as u128, 4u16);
        // D s_214_9: cmp-ne s_214_6 s_214_8
        let s_214_9: bool = ((s_214_6) != (s_214_8));
        // N s_214_10: branch s_214_9 b217 b215
        if s_214_9 {
            return block_217(state, tracer, fn_state);
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
        // D s_215_1: write-var gs#411494 <= s_215_0
        fn_state.gs_411494 = s_215_0;
        // N s_215_2: jump b216
        return block_216(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_216_0: read-var gs#411494:u8
        let s_216_0: bool = fn_state.gs_411494;
        // D s_216_1: write-var gs#411495 <= s_216_0
        fn_state.gs_411495 = s_216_0;
        // N s_216_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #3186s : i
        let s_217_0: i128 = 3186;
        // C s_217_1: const #14696u : u32
        let s_217_1: u32 = 14696;
        // D s_217_2: read-reg s_217_1:i
        let s_217_2: i128 = {
            let value = state.read_register::<i128>(s_217_1 as isize);
            tracer.read_register(s_217_1 as isize, value);
            value
        };
        // D s_217_3: cmp-lt s_217_2 s_217_0
        let s_217_3: bool = ((s_217_2) < (s_217_0));
        // D s_217_4: write-var gs#411494 <= s_217_3
        fn_state.gs_411494 = s_217_3;
        // N s_217_5: jump b216
        return block_216(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_218_0: const #20s : i
        let s_218_0: i128 = 20;
        // D s_218_1: read-var u#33910:u32
        let s_218_1: u32 = fn_state.u_33910;
        // D s_218_2: cast zx s_218_1 -> bv
        let s_218_2: Bits = Bits::new(s_218_1 as u128, 32u16);
        // C s_218_3: const #1s : i64
        let s_218_3: i64 = 1;
        // C s_218_4: cast zx s_218_3 -> i
        let s_218_4: i128 = (i128::try_from(s_218_3).unwrap());
        // C s_218_5: const #0s : i
        let s_218_5: i128 = 0;
        // C s_218_6: add s_218_5 s_218_4
        let s_218_6: i128 = (s_218_5 + s_218_4);
        // D s_218_7: bit-extract s_218_2 s_218_0 s_218_6
        let s_218_7: Bits = (Bits::new(
            ((s_218_2) >> (s_218_0)).value(),
            u16::try_from(s_218_6).unwrap(),
        ));
        // D s_218_8: cast reint s_218_7 -> u8
        let s_218_8: bool = ((s_218_7.value()) != 0);
        // D s_218_9: cast zx s_218_8 -> bv
        let s_218_9: Bits = Bits::new(s_218_8 as u128, 1u16);
        // C s_218_10: const #0u : u8
        let s_218_10: bool = false;
        // C s_218_11: cast zx s_218_10 -> bv
        let s_218_11: Bits = Bits::new(s_218_10 as u128, 1u16);
        // D s_218_12: cmp-eq s_218_9 s_218_11
        let s_218_12: bool = ((s_218_9) == (s_218_11));
        // D s_218_13: write-var gs#411490 <= s_218_12
        fn_state.gs_411490 = s_218_12;
        // N s_218_14: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #28s : i
        let s_219_0: i128 = 28;
        // C s_219_1: const #4s : i
        let s_219_1: i128 = 4;
        // D s_219_2: read-var u#33904:u32
        let s_219_2: u32 = fn_state.u_33904;
        // D s_219_3: cast zx s_219_2 -> bv
        let s_219_3: Bits = Bits::new(s_219_2 as u128, 32u16);
        // D s_219_4: bit-extract s_219_3 s_219_0 s_219_1
        let s_219_4: Bits = (Bits::new(
            ((s_219_3) >> (s_219_0)).value(),
            u16::try_from(s_219_1).unwrap(),
        ));
        // D s_219_5: cast reint s_219_4 -> u8
        let s_219_5: u8 = (s_219_4.value() as u8);
        // D s_219_6: cast zx s_219_5 -> bv
        let s_219_6: Bits = Bits::new(s_219_5 as u128, 4u16);
        // C s_219_7: const #15u : u8
        let s_219_7: u8 = 15;
        // C s_219_8: cast zx s_219_7 -> bv
        let s_219_8: Bits = Bits::new(s_219_7 as u128, 4u16);
        // D s_219_9: cmp-ne s_219_6 s_219_8
        let s_219_9: bool = ((s_219_6) != (s_219_8));
        // N s_219_10: branch s_219_9 b222 b220
        if s_219_9 {
            return block_222(state, tracer, fn_state);
        } else {
            return block_220(state, tracer, fn_state);
        };
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #0u : u8
        let s_220_0: bool = false;
        // D s_220_1: write-var gs#411473 <= s_220_0
        fn_state.gs_411473 = s_220_0;
        // N s_220_2: jump b221
        return block_221(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_221_0: read-var gs#411473:u8
        let s_221_0: bool = fn_state.gs_411473;
        // D s_221_1: write-var gs#411474 <= s_221_0
        fn_state.gs_411474 = s_221_0;
        // N s_221_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_222_0: const #2922s : i
        let s_222_0: i128 = 2922;
        // C s_222_1: const #14696u : u32
        let s_222_1: u32 = 14696;
        // D s_222_2: read-reg s_222_1:i
        let s_222_2: i128 = {
            let value = state.read_register::<i128>(s_222_1 as isize);
            tracer.read_register(s_222_1 as isize, value);
            value
        };
        // D s_222_3: cmp-lt s_222_2 s_222_0
        let s_222_3: bool = ((s_222_2) < (s_222_0));
        // D s_222_4: write-var gs#411473 <= s_222_3
        fn_state.gs_411473 = s_222_3;
        // N s_222_5: jump b221
        return block_221(state, tracer, fn_state);
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_223_0: const #20s : i
        let s_223_0: i128 = 20;
        // D s_223_1: read-var u#33904:u32
        let s_223_1: u32 = fn_state.u_33904;
        // D s_223_2: cast zx s_223_1 -> bv
        let s_223_2: Bits = Bits::new(s_223_1 as u128, 32u16);
        // C s_223_3: const #1s : i64
        let s_223_3: i64 = 1;
        // C s_223_4: cast zx s_223_3 -> i
        let s_223_4: i128 = (i128::try_from(s_223_3).unwrap());
        // C s_223_5: const #0s : i
        let s_223_5: i128 = 0;
        // C s_223_6: add s_223_5 s_223_4
        let s_223_6: i128 = (s_223_5 + s_223_4);
        // D s_223_7: bit-extract s_223_2 s_223_0 s_223_6
        let s_223_7: Bits = (Bits::new(
            ((s_223_2) >> (s_223_0)).value(),
            u16::try_from(s_223_6).unwrap(),
        ));
        // D s_223_8: cast reint s_223_7 -> u8
        let s_223_8: bool = ((s_223_7.value()) != 0);
        // D s_223_9: cast zx s_223_8 -> bv
        let s_223_9: Bits = Bits::new(s_223_8 as u128, 1u16);
        // C s_223_10: const #1u : u8
        let s_223_10: bool = true;
        // C s_223_11: cast zx s_223_10 -> bv
        let s_223_11: Bits = Bits::new(s_223_10 as u128, 1u16);
        // D s_223_12: cmp-eq s_223_9 s_223_11
        let s_223_12: bool = ((s_223_9) == (s_223_11));
        // D s_223_13: write-var gs#411469 <= s_223_12
        fn_state.gs_411469 = s_223_12;
        // N s_223_14: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_224_0: const #28s : i
        let s_224_0: i128 = 28;
        // C s_224_1: const #4s : i
        let s_224_1: i128 = 4;
        // D s_224_2: read-var u#33898:u32
        let s_224_2: u32 = fn_state.u_33898;
        // D s_224_3: cast zx s_224_2 -> bv
        let s_224_3: Bits = Bits::new(s_224_2 as u128, 32u16);
        // D s_224_4: bit-extract s_224_3 s_224_0 s_224_1
        let s_224_4: Bits = (Bits::new(
            ((s_224_3) >> (s_224_0)).value(),
            u16::try_from(s_224_1).unwrap(),
        ));
        // D s_224_5: cast reint s_224_4 -> u8
        let s_224_5: u8 = (s_224_4.value() as u8);
        // D s_224_6: cast zx s_224_5 -> bv
        let s_224_6: Bits = Bits::new(s_224_5 as u128, 4u16);
        // C s_224_7: const #15u : u8
        let s_224_7: u8 = 15;
        // C s_224_8: cast zx s_224_7 -> bv
        let s_224_8: Bits = Bits::new(s_224_7 as u128, 4u16);
        // D s_224_9: cmp-ne s_224_6 s_224_8
        let s_224_9: bool = ((s_224_6) != (s_224_8));
        // N s_224_10: branch s_224_9 b227 b225
        if s_224_9 {
            return block_227(state, tracer, fn_state);
        } else {
            return block_225(state, tracer, fn_state);
        };
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_225_0: const #0u : u8
        let s_225_0: bool = false;
        // D s_225_1: write-var gs#411452 <= s_225_0
        fn_state.gs_411452 = s_225_0;
        // N s_225_2: jump b226
        return block_226(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_226_0: read-var gs#411452:u8
        let s_226_0: bool = fn_state.gs_411452;
        // D s_226_1: write-var gs#411453 <= s_226_0
        fn_state.gs_411453 = s_226_0;
        // N s_226_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_227_0: const #2920s : i
        let s_227_0: i128 = 2920;
        // C s_227_1: const #14696u : u32
        let s_227_1: u32 = 14696;
        // D s_227_2: read-reg s_227_1:i
        let s_227_2: i128 = {
            let value = state.read_register::<i128>(s_227_1 as isize);
            tracer.read_register(s_227_1 as isize, value);
            value
        };
        // D s_227_3: cmp-lt s_227_2 s_227_0
        let s_227_3: bool = ((s_227_2) < (s_227_0));
        // D s_227_4: write-var gs#411452 <= s_227_3
        fn_state.gs_411452 = s_227_3;
        // N s_227_5: jump b226
        return block_226(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_228_0: const #20s : i
        let s_228_0: i128 = 20;
        // D s_228_1: read-var u#33898:u32
        let s_228_1: u32 = fn_state.u_33898;
        // D s_228_2: cast zx s_228_1 -> bv
        let s_228_2: Bits = Bits::new(s_228_1 as u128, 32u16);
        // C s_228_3: const #1s : i64
        let s_228_3: i64 = 1;
        // C s_228_4: cast zx s_228_3 -> i
        let s_228_4: i128 = (i128::try_from(s_228_3).unwrap());
        // C s_228_5: const #0s : i
        let s_228_5: i128 = 0;
        // C s_228_6: add s_228_5 s_228_4
        let s_228_6: i128 = (s_228_5 + s_228_4);
        // D s_228_7: bit-extract s_228_2 s_228_0 s_228_6
        let s_228_7: Bits = (Bits::new(
            ((s_228_2) >> (s_228_0)).value(),
            u16::try_from(s_228_6).unwrap(),
        ));
        // D s_228_8: cast reint s_228_7 -> u8
        let s_228_8: bool = ((s_228_7.value()) != 0);
        // D s_228_9: cast zx s_228_8 -> bv
        let s_228_9: Bits = Bits::new(s_228_8 as u128, 1u16);
        // C s_228_10: const #1u : u8
        let s_228_10: bool = true;
        // C s_228_11: cast zx s_228_10 -> bv
        let s_228_11: Bits = Bits::new(s_228_10 as u128, 1u16);
        // D s_228_12: cmp-eq s_228_9 s_228_11
        let s_228_12: bool = ((s_228_9) == (s_228_11));
        // D s_228_13: write-var gs#411448 <= s_228_12
        fn_state.gs_411448 = s_228_12;
        // N s_228_14: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #28s : i
        let s_229_0: i128 = 28;
        // C s_229_1: const #4s : i
        let s_229_1: i128 = 4;
        // D s_229_2: read-var u#33892:u32
        let s_229_2: u32 = fn_state.u_33892;
        // D s_229_3: cast zx s_229_2 -> bv
        let s_229_3: Bits = Bits::new(s_229_2 as u128, 32u16);
        // D s_229_4: bit-extract s_229_3 s_229_0 s_229_1
        let s_229_4: Bits = (Bits::new(
            ((s_229_3) >> (s_229_0)).value(),
            u16::try_from(s_229_1).unwrap(),
        ));
        // D s_229_5: cast reint s_229_4 -> u8
        let s_229_5: u8 = (s_229_4.value() as u8);
        // D s_229_6: cast zx s_229_5 -> bv
        let s_229_6: Bits = Bits::new(s_229_5 as u128, 4u16);
        // C s_229_7: const #15u : u8
        let s_229_7: u8 = 15;
        // C s_229_8: cast zx s_229_7 -> bv
        let s_229_8: Bits = Bits::new(s_229_7 as u128, 4u16);
        // D s_229_9: cmp-ne s_229_6 s_229_8
        let s_229_9: bool = ((s_229_6) != (s_229_8));
        // N s_229_10: branch s_229_9 b232 b230
        if s_229_9 {
            return block_232(state, tracer, fn_state);
        } else {
            return block_230(state, tracer, fn_state);
        };
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_230_0: const #0u : u8
        let s_230_0: bool = false;
        // D s_230_1: write-var gs#411431 <= s_230_0
        fn_state.gs_411431 = s_230_0;
        // N s_230_2: jump b231
        return block_231(state, tracer, fn_state);
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_231_0: read-var gs#411431:u8
        let s_231_0: bool = fn_state.gs_411431;
        // D s_231_1: write-var gs#411432 <= s_231_0
        fn_state.gs_411432 = s_231_0;
        // N s_231_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_232_0: const #2919s : i
        let s_232_0: i128 = 2919;
        // C s_232_1: const #14696u : u32
        let s_232_1: u32 = 14696;
        // D s_232_2: read-reg s_232_1:i
        let s_232_2: i128 = {
            let value = state.read_register::<i128>(s_232_1 as isize);
            tracer.read_register(s_232_1 as isize, value);
            value
        };
        // D s_232_3: cmp-lt s_232_2 s_232_0
        let s_232_3: bool = ((s_232_2) < (s_232_0));
        // D s_232_4: write-var gs#411431 <= s_232_3
        fn_state.gs_411431 = s_232_3;
        // N s_232_5: jump b231
        return block_231(state, tracer, fn_state);
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_233_0: const #20s : i
        let s_233_0: i128 = 20;
        // D s_233_1: read-var u#33892:u32
        let s_233_1: u32 = fn_state.u_33892;
        // D s_233_2: cast zx s_233_1 -> bv
        let s_233_2: Bits = Bits::new(s_233_1 as u128, 32u16);
        // C s_233_3: const #1s : i64
        let s_233_3: i64 = 1;
        // C s_233_4: cast zx s_233_3 -> i
        let s_233_4: i128 = (i128::try_from(s_233_3).unwrap());
        // C s_233_5: const #0s : i
        let s_233_5: i128 = 0;
        // C s_233_6: add s_233_5 s_233_4
        let s_233_6: i128 = (s_233_5 + s_233_4);
        // D s_233_7: bit-extract s_233_2 s_233_0 s_233_6
        let s_233_7: Bits = (Bits::new(
            ((s_233_2) >> (s_233_0)).value(),
            u16::try_from(s_233_6).unwrap(),
        ));
        // D s_233_8: cast reint s_233_7 -> u8
        let s_233_8: bool = ((s_233_7.value()) != 0);
        // D s_233_9: cast zx s_233_8 -> bv
        let s_233_9: Bits = Bits::new(s_233_8 as u128, 1u16);
        // C s_233_10: const #1u : u8
        let s_233_10: bool = true;
        // C s_233_11: cast zx s_233_10 -> bv
        let s_233_11: Bits = Bits::new(s_233_10 as u128, 1u16);
        // D s_233_12: cmp-eq s_233_9 s_233_11
        let s_233_12: bool = ((s_233_9) == (s_233_11));
        // D s_233_13: write-var gs#411427 <= s_233_12
        fn_state.gs_411427 = s_233_12;
        // N s_233_14: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_234_0: const #28s : i
        let s_234_0: i128 = 28;
        // C s_234_1: const #4s : i
        let s_234_1: i128 = 4;
        // D s_234_2: read-var u#33889:u32
        let s_234_2: u32 = fn_state.u_33889;
        // D s_234_3: cast zx s_234_2 -> bv
        let s_234_3: Bits = Bits::new(s_234_2 as u128, 32u16);
        // D s_234_4: bit-extract s_234_3 s_234_0 s_234_1
        let s_234_4: Bits = (Bits::new(
            ((s_234_3) >> (s_234_0)).value(),
            u16::try_from(s_234_1).unwrap(),
        ));
        // D s_234_5: cast reint s_234_4 -> u8
        let s_234_5: u8 = (s_234_4.value() as u8);
        // D s_234_6: cast zx s_234_5 -> bv
        let s_234_6: Bits = Bits::new(s_234_5 as u128, 4u16);
        // C s_234_7: const #15u : u8
        let s_234_7: u8 = 15;
        // C s_234_8: cast zx s_234_7 -> bv
        let s_234_8: Bits = Bits::new(s_234_7 as u128, 4u16);
        // D s_234_9: cmp-ne s_234_6 s_234_8
        let s_234_9: bool = ((s_234_6) != (s_234_8));
        // N s_234_10: branch s_234_9 b237 b235
        if s_234_9 {
            return block_237(state, tracer, fn_state);
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
        // D s_235_1: write-var gs#411410 <= s_235_0
        fn_state.gs_411410 = s_235_0;
        // N s_235_2: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_236_0: read-var gs#411410:u8
        let s_236_0: bool = fn_state.gs_411410;
        // D s_236_1: write-var gs#411411 <= s_236_0
        fn_state.gs_411411 = s_236_0;
        // N s_236_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_237_0: const #2916s : i
        let s_237_0: i128 = 2916;
        // C s_237_1: const #14696u : u32
        let s_237_1: u32 = 14696;
        // D s_237_2: read-reg s_237_1:i
        let s_237_2: i128 = {
            let value = state.read_register::<i128>(s_237_1 as isize);
            tracer.read_register(s_237_1 as isize, value);
            value
        };
        // D s_237_3: cmp-lt s_237_2 s_237_0
        let s_237_3: bool = ((s_237_2) < (s_237_0));
        // D s_237_4: write-var gs#411410 <= s_237_3
        fn_state.gs_411410 = s_237_3;
        // N s_237_5: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_238_0: const #20s : i
        let s_238_0: i128 = 20;
        // D s_238_1: read-var u#33889:u32
        let s_238_1: u32 = fn_state.u_33889;
        // D s_238_2: cast zx s_238_1 -> bv
        let s_238_2: Bits = Bits::new(s_238_1 as u128, 32u16);
        // C s_238_3: const #1s : i64
        let s_238_3: i64 = 1;
        // C s_238_4: cast zx s_238_3 -> i
        let s_238_4: i128 = (i128::try_from(s_238_3).unwrap());
        // C s_238_5: const #0s : i
        let s_238_5: i128 = 0;
        // C s_238_6: add s_238_5 s_238_4
        let s_238_6: i128 = (s_238_5 + s_238_4);
        // D s_238_7: bit-extract s_238_2 s_238_0 s_238_6
        let s_238_7: Bits = (Bits::new(
            ((s_238_2) >> (s_238_0)).value(),
            u16::try_from(s_238_6).unwrap(),
        ));
        // D s_238_8: cast reint s_238_7 -> u8
        let s_238_8: bool = ((s_238_7.value()) != 0);
        // D s_238_9: cast zx s_238_8 -> bv
        let s_238_9: Bits = Bits::new(s_238_8 as u128, 1u16);
        // C s_238_10: const #1u : u8
        let s_238_10: bool = true;
        // C s_238_11: cast zx s_238_10 -> bv
        let s_238_11: Bits = Bits::new(s_238_10 as u128, 1u16);
        // D s_238_12: cmp-eq s_238_9 s_238_11
        let s_238_12: bool = ((s_238_9) == (s_238_11));
        // D s_238_13: write-var gs#411406 <= s_238_12
        fn_state.gs_411406 = s_238_12;
        // N s_238_14: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_239_0: const #2869s : i
        let s_239_0: i128 = 2869;
        // C s_239_1: const #14696u : u32
        let s_239_1: u32 = 14696;
        // D s_239_2: read-reg s_239_1:i
        let s_239_2: i128 = {
            let value = state.read_register::<i128>(s_239_1 as isize);
            tracer.read_register(s_239_1 as isize, value);
            value
        };
        // D s_239_3: cmp-lt s_239_2 s_239_0
        let s_239_3: bool = ((s_239_2) < (s_239_0));
        // D s_239_4: write-var gs#411394 <= s_239_3
        fn_state.gs_411394 = s_239_3;
        // N s_239_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_240_0: const #28s : i
        let s_240_0: i128 = 28;
        // C s_240_1: const #4s : i
        let s_240_1: i128 = 4;
        // D s_240_2: read-var u#33882:u32
        let s_240_2: u32 = fn_state.u_33882;
        // D s_240_3: cast zx s_240_2 -> bv
        let s_240_3: Bits = Bits::new(s_240_2 as u128, 32u16);
        // D s_240_4: bit-extract s_240_3 s_240_0 s_240_1
        let s_240_4: Bits = (Bits::new(
            ((s_240_3) >> (s_240_0)).value(),
            u16::try_from(s_240_1).unwrap(),
        ));
        // D s_240_5: cast reint s_240_4 -> u8
        let s_240_5: u8 = (s_240_4.value() as u8);
        // D s_240_6: cast zx s_240_5 -> bv
        let s_240_6: Bits = Bits::new(s_240_5 as u128, 4u16);
        // C s_240_7: const #15u : u8
        let s_240_7: u8 = 15;
        // C s_240_8: cast zx s_240_7 -> bv
        let s_240_8: Bits = Bits::new(s_240_7 as u128, 4u16);
        // D s_240_9: cmp-ne s_240_6 s_240_8
        let s_240_9: bool = ((s_240_6) != (s_240_8));
        // N s_240_10: branch s_240_9 b243 b241
        if s_240_9 {
            return block_243(state, tracer, fn_state);
        } else {
            return block_241(state, tracer, fn_state);
        };
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_241_0: const #0u : u8
        let s_241_0: bool = false;
        // D s_241_1: write-var gs#411382 <= s_241_0
        fn_state.gs_411382 = s_241_0;
        // N s_241_2: jump b242
        return block_242(state, tracer, fn_state);
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_242_0: read-var gs#411382:u8
        let s_242_0: bool = fn_state.gs_411382;
        // D s_242_1: write-var gs#411383 <= s_242_0
        fn_state.gs_411383 = s_242_0;
        // N s_242_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_243_0: const #2868s : i
        let s_243_0: i128 = 2868;
        // C s_243_1: const #14696u : u32
        let s_243_1: u32 = 14696;
        // D s_243_2: read-reg s_243_1:i
        let s_243_2: i128 = {
            let value = state.read_register::<i128>(s_243_1 as isize);
            tracer.read_register(s_243_1 as isize, value);
            value
        };
        // D s_243_3: cmp-lt s_243_2 s_243_0
        let s_243_3: bool = ((s_243_2) < (s_243_0));
        // D s_243_4: write-var gs#411382 <= s_243_3
        fn_state.gs_411382 = s_243_3;
        // N s_243_5: jump b242
        return block_242(state, tracer, fn_state);
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_244_0: const #28s : i
        let s_244_0: i128 = 28;
        // C s_244_1: const #4s : i
        let s_244_1: i128 = 4;
        // D s_244_2: read-var __opcode:u32
        let s_244_2: u32 = fn_state.u__opcode;
        // D s_244_3: cast zx s_244_2 -> bv
        let s_244_3: Bits = Bits::new(s_244_2 as u128, 32u16);
        // D s_244_4: bit-extract s_244_3 s_244_0 s_244_1
        let s_244_4: Bits = (Bits::new(
            ((s_244_3) >> (s_244_0)).value(),
            u16::try_from(s_244_1).unwrap(),
        ));
        // D s_244_5: cast reint s_244_4 -> u8
        let s_244_5: u8 = (s_244_4.value() as u8);
        // D s_244_6: cast zx s_244_5 -> bv
        let s_244_6: Bits = Bits::new(s_244_5 as u128, 4u16);
        // C s_244_7: const #15u : u8
        let s_244_7: u8 = 15;
        // C s_244_8: cast zx s_244_7 -> bv
        let s_244_8: Bits = Bits::new(s_244_7 as u128, 4u16);
        // D s_244_9: cmp-ne s_244_6 s_244_8
        let s_244_9: bool = ((s_244_6) != (s_244_8));
        // N s_244_10: branch s_244_9 b247 b245
        if s_244_9 {
            return block_247(state, tracer, fn_state);
        } else {
            return block_245(state, tracer, fn_state);
        };
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_245_0: const #0u : u8
        let s_245_0: bool = false;
        // D s_245_1: write-var gs#411368 <= s_245_0
        fn_state.gs_411368 = s_245_0;
        // N s_245_2: jump b246
        return block_246(state, tracer, fn_state);
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_246_0: read-var gs#411368:u8
        let s_246_0: bool = fn_state.gs_411368;
        // D s_246_1: write-var gs#411369 <= s_246_0
        fn_state.gs_411369 = s_246_0;
        // N s_246_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_247_0: const #2851s : i
        let s_247_0: i128 = 2851;
        // C s_247_1: const #14696u : u32
        let s_247_1: u32 = 14696;
        // D s_247_2: read-reg s_247_1:i
        let s_247_2: i128 = {
            let value = state.read_register::<i128>(s_247_1 as isize);
            tracer.read_register(s_247_1 as isize, value);
            value
        };
        // D s_247_3: cmp-lt s_247_2 s_247_0
        let s_247_3: bool = ((s_247_2) < (s_247_0));
        // D s_247_4: write-var gs#411368 <= s_247_3
        fn_state.gs_411368 = s_247_3;
        // N s_247_5: jump b246
        return block_246(state, tracer, fn_state);
    }
}
