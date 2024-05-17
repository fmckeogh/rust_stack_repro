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
use u__ConfigureV88::*;
use u__ConfigureV90::*;
use decode_max_sveveclen::*;
use u__ConfigureV86::*;
use u__ConfigureV81::*;
use u__ConfigureV83::*;
use u__ConfigureV84::*;
use u__ConfigureV94::*;
use u__ConfigureV89::*;
use u__ConfigureV87::*;
use u__ConfigureV92::*;
use neq_int::*;
use u__ConfigureV82::*;
use u__ConfigureV85::*;
use decode_max_smeveclen::*;
use integer_subrange::*;
use u__ConfigureV91::*;
use u__ConfigureV93::*;
use integer_access::*;
use fdiv_int::*;
use common::*;
pub fn u__SetConfig<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_428957: &'static str,
    gs_428958: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u_38598: i128,
        u_38640: i128,
        u_38752: i128,
        u_38542: i128,
        u_38580: i128,
        u_38644: i128,
        u_38744: i128,
        u_38528: i128,
        value_name: i128,
        u_38574: i128,
        u_38756: i128,
        u_38678: i128,
        u_38562: i128,
        u_38588: i128,
        u_38676: i128,
        u_38680: i128,
        u_38566: i128,
        u_38712: i128,
        u_38560: i128,
        u_38650: i128,
        u_38672: i128,
        u_38656: i128,
        u_38738: i128,
        u_38692: i128,
        u_38724: i128,
        u_38750: i128,
        u_38596: i128,
        u_38540: i128,
        u_38622: i128,
        u_38520: i128,
        u_38594: i128,
        u_38734: i128,
        u_38606: i128,
        u_38586: i128,
        u_38584: i128,
        u_38658: i128,
        u_38762: i128,
        u_38602: i128,
        u_38706: i128,
        u_38666: i128,
        u_38740: i128,
        u_38572: i128,
        u_38716: i128,
        u_38554: i128,
        u_38636: i128,
        u_38628: i128,
        u_38576: i128,
        u_38764: i128,
        u_38616: i128,
        u_38728: i128,
        u_38524: i128,
        u_38570: i128,
        merge_var: ProductTypee501372d76ef32c0,
        u_38690: i128,
        u_38742: i128,
        u_38766: i128,
        u_38608: i128,
        u_38558: i128,
        u_38578: i128,
        u_38618: i128,
        u_38626: i128,
        u_38503: i128,
        u_38688: i128,
        u_38708: i128,
        u_38720: i128,
        u_38590: i128,
        u_38746: i128,
        u_38718: i128,
        u_38514: i128,
        gs_428961: bool,
        u_38684: i128,
        u_38754: i128,
        u_38538: i128,
        u_38610: i128,
        u_38568: i128,
        u_38546: i128,
        u_38674: i128,
        u_38694: i128,
        u_38654: i128,
        u_38660: i128,
        u_38632: i128,
        u_38758: i128,
        u_38518: i128,
        u_38704: i128,
        u_38620: i128,
        u_38730: i128,
        u_38592: i128,
        u_38614: i128,
        u_38698: i128,
        u_38544: i128,
        u_38582: i128,
        gs_428973: bool,
        u_38556: i128,
        u_38668: i128,
        u_38534: i128,
        u_38530: i128,
        u_38550: i128,
        u_38612: i128,
        u_38604: i128,
        u_38642: i128,
        u_38702: i128,
        u_38646: i128,
        u_38732: i128,
        u_38526: i128,
        u_38522: i128,
        u_38634: i128,
        u_38670: i128,
        u_38710: i128,
        u_38736: i128,
        u_38722: i128,
        u_38600: i128,
        u_38552: i128,
        u_38624: i128,
        u_38726: i128,
        u_38548: i128,
        u_38682: i128,
        u_38648: i128,
        u_38509: i128,
        u_38700: i128,
        u_38536: i128,
        u_38532: i128,
        u_38664: i128,
        u_38760: i128,
        gs_428969: bool,
        u_38714: i128,
        u_38652: i128,
        u_38748: i128,
        u_38686: i128,
        u_38512: i128,
        gs_428965: bool,
        u_38564: i128,
        u_38630: i128,
        u_38516: i128,
        u_38662: i128,
        u_38638: i128,
        u_38506: i128,
        u_38696: i128,
        gs_428957: &'static str,
        gs_428958: i128,
    }
    let fn_state = FunctionState {
        gs_428957,
        gs_428958,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var gs#428957:str
        let s_0_0: &'static str = fn_state.gs_428957;
        // D s_0_1: write-var merge#var.0 <= s_0_0
        fn_state.merge_var._0 = s_0_0;
        // D s_0_2: read-var gs#428958:i
        let s_0_2: i128 = fn_state.gs_428958;
        // D s_0_3: write-var merge#var.1 <= s_0_2
        fn_state.merge_var._1 = s_0_2;
        // D s_0_4: read-var merge#var.0:struct
        let s_0_4: &'static str = fn_state.merge_var._0;
        // D s_0_5: read-var merge#var.1:struct
        let s_0_5: i128 = fn_state.merge_var._1;
        // D s_0_6: write-var value_name <= s_0_5
        fn_state.value_name = s_0_5;
        // C s_0_7: const #"ZCR_EL3.LEN" : str
        let s_0_7: &'static str = "ZCR_EL3.LEN";
        // D s_0_8: cmp-eq s_0_4 s_0_7
        let s_0_8: bool = ((s_0_4) == (s_0_7));
        // D s_0_9: not s_0_8
        let s_0_9: bool = !s_0_8;
        // N s_0_10: branch s_0_9 b7 b1
        if s_0_9 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i
        let s_1_0: i128 = 0;
        // D s_1_1: read-var value_name:i
        let s_1_1: i128 = fn_state.value_name;
        // D s_1_2: cmp-ge s_1_1 s_1_0
        let s_1_2: bool = ((s_1_1) >= (s_1_0));
        // N s_1_3: branch s_1_2 b6 b2
        if s_1_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#428961 <= s_2_0
        fn_state.gs_428961 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#428961:u8
        let s_3_0: bool = fn_state.gs_428961;
        // N s_3_1: branch s_3_0 b5 b4
        if s_3_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // N s_4_2: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var value_name:i
        let s_5_0: i128 = fn_state.value_name;
        // C s_5_1: const #14800u : u32
        let s_5_1: u32 = 14800;
        // N s_5_2: write-reg s_5_1 <= s_5_0
        let s_5_2: () = {
            state.write_register::<i128>(s_5_1 as isize, s_5_0);
            tracer.write_register(s_5_1 as isize, s_5_0);
        };
        // N s_5_3: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #15u : u8
        let s_6_0: u8 = 15;
        // C s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 4u16);
        // C s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // C s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: read-var value_name:i
        let s_6_5: i128 = fn_state.value_name;
        // D s_6_6: cmp-le s_6_5 s_6_4
        let s_6_6: bool = ((s_6_5) <= (s_6_4));
        // D s_6_7: write-var gs#428961 <= s_6_6
        fn_state.gs_428961 = s_6_6;
        // N s_6_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var merge#var.0:struct
        let s_7_0: &'static str = fn_state.merge_var._0;
        // D s_7_1: read-var merge#var.1:struct
        let s_7_1: i128 = fn_state.merge_var._1;
        // D s_7_2: write-var u#38503 <= s_7_1
        fn_state.u_38503 = s_7_1;
        // C s_7_3: const #"CPTR_EL3.EZ" : str
        let s_7_3: &'static str = "CPTR_EL3.EZ";
        // D s_7_4: cmp-eq s_7_0 s_7_3
        let s_7_4: bool = ((s_7_0) == (s_7_3));
        // D s_7_5: not s_7_4
        let s_7_5: bool = !s_7_4;
        // N s_7_6: branch s_7_5 b14 b8
        if s_7_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0s : i
        let s_8_0: i128 = 0;
        // D s_8_1: read-var u#38503:i
        let s_8_1: i128 = fn_state.u_38503;
        // D s_8_2: cmp-ge s_8_1 s_8_0
        let s_8_2: bool = ((s_8_1) >= (s_8_0));
        // N s_8_3: branch s_8_2 b13 b9
        if s_8_2 {
            return block_13(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#428965 <= s_9_0
        fn_state.gs_428965 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#428965:u8
        let s_10_0: bool = fn_state.gs_428965;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
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
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // N s_11_1: assert s_11_0
        let s_11_1: () = assert!(s_11_0);
        // N s_11_2: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var u#38503:i
        let s_12_0: i128 = fn_state.u_38503;
        // C s_12_1: const #15656u : u32
        let s_12_1: u32 = 15656;
        // N s_12_2: write-reg s_12_1 <= s_12_0
        let s_12_2: () = {
            state.write_register::<i128>(s_12_1 as isize, s_12_0);
            tracer.write_register(s_12_1 as isize, s_12_0);
        };
        // N s_12_3: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: u8 = 1;
        // C s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 4u16);
        // C s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (s_13_1.value() as i128);
        // C s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: read-var u#38503:i
        let s_13_5: i128 = fn_state.u_38503;
        // D s_13_6: cmp-le s_13_5 s_13_4
        let s_13_6: bool = ((s_13_5) <= (s_13_4));
        // D s_13_7: write-var gs#428965 <= s_13_6
        fn_state.gs_428965 = s_13_6;
        // N s_13_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var merge#var.0:struct
        let s_14_0: &'static str = fn_state.merge_var._0;
        // D s_14_1: read-var merge#var.1:struct
        let s_14_1: i128 = fn_state.merge_var._1;
        // D s_14_2: write-var u#38506 <= s_14_1
        fn_state.u_38506 = s_14_1;
        // C s_14_3: const #"CPTR_EL3.ESM" : str
        let s_14_3: &'static str = "CPTR_EL3.ESM";
        // D s_14_4: cmp-eq s_14_0 s_14_3
        let s_14_4: bool = ((s_14_0) == (s_14_3));
        // D s_14_5: not s_14_4
        let s_14_5: bool = !s_14_4;
        // N s_14_6: branch s_14_5 b21 b15
        if s_14_5 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0s : i
        let s_15_0: i128 = 0;
        // D s_15_1: read-var u#38506:i
        let s_15_1: i128 = fn_state.u_38506;
        // D s_15_2: cmp-ge s_15_1 s_15_0
        let s_15_2: bool = ((s_15_1) >= (s_15_0));
        // N s_15_3: branch s_15_2 b20 b16
        if s_15_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#428969 <= s_16_0
        fn_state.gs_428969 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#428969:u8
        let s_17_0: bool = fn_state.gs_428969;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // N s_18_1: assert s_18_0
        let s_18_1: () = assert!(s_18_0);
        // N s_18_2: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var u#38506:i
        let s_19_0: i128 = fn_state.u_38506;
        // C s_19_1: const #90352u : u32
        let s_19_1: u32 = 90352;
        // N s_19_2: write-reg s_19_1 <= s_19_0
        let s_19_2: () = {
            state.write_register::<i128>(s_19_1 as isize, s_19_0);
            tracer.write_register(s_19_1 as isize, s_19_0);
        };
        // N s_19_3: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: u8 = 1;
        // C s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 4u16);
        // C s_20_2: cast zx s_20_1 -> i
        let s_20_2: i128 = (s_20_1.value() as i128);
        // C s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // C s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // D s_20_5: read-var u#38506:i
        let s_20_5: i128 = fn_state.u_38506;
        // D s_20_6: cmp-le s_20_5 s_20_4
        let s_20_6: bool = ((s_20_5) <= (s_20_4));
        // D s_20_7: write-var gs#428969 <= s_20_6
        fn_state.gs_428969 = s_20_6;
        // N s_20_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var merge#var.0:struct
        let s_21_0: &'static str = fn_state.merge_var._0;
        // D s_21_1: read-var merge#var.1:struct
        let s_21_1: i128 = fn_state.merge_var._1;
        // D s_21_2: write-var u#38509 <= s_21_1
        fn_state.u_38509 = s_21_1;
        // C s_21_3: const #"SMCR_EL3.LEN" : str
        let s_21_3: &'static str = "SMCR_EL3.LEN";
        // D s_21_4: cmp-eq s_21_0 s_21_3
        let s_21_4: bool = ((s_21_0) == (s_21_3));
        // D s_21_5: not s_21_4
        let s_21_5: bool = !s_21_4;
        // N s_21_6: branch s_21_5 b28 b22
        if s_21_5 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0s : i
        let s_22_0: i128 = 0;
        // D s_22_1: read-var u#38509:i
        let s_22_1: i128 = fn_state.u_38509;
        // D s_22_2: cmp-ge s_22_1 s_22_0
        let s_22_2: bool = ((s_22_1) >= (s_22_0));
        // N s_22_3: branch s_22_2 b27 b23
        if s_22_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#428973 <= s_23_0
        fn_state.gs_428973 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#428973:u8
        let s_24_0: bool = fn_state.gs_428973;
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
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // N s_25_1: assert s_25_0
        let s_25_1: () = assert!(s_25_0);
        // N s_25_2: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var u#38509:i
        let s_26_0: i128 = fn_state.u_38509;
        // C s_26_1: const #90928u : u32
        let s_26_1: u32 = 90928;
        // N s_26_2: write-reg s_26_1 <= s_26_0
        let s_26_2: () = {
            state.write_register::<i128>(s_26_1 as isize, s_26_0);
            tracer.write_register(s_26_1 as isize, s_26_0);
        };
        // N s_26_3: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #15u : u8
        let s_27_0: u8 = 15;
        // C s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 4u16);
        // C s_27_2: cast zx s_27_1 -> i
        let s_27_2: i128 = (s_27_1.value() as i128);
        // C s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_5: read-var u#38509:i
        let s_27_5: i128 = fn_state.u_38509;
        // D s_27_6: cmp-le s_27_5 s_27_4
        let s_27_6: bool = ((s_27_5) <= (s_27_4));
        // D s_27_7: write-var gs#428973 <= s_27_6
        fn_state.gs_428973 = s_27_6;
        // N s_27_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var merge#var.0:struct
        let s_28_0: &'static str = fn_state.merge_var._0;
        // D s_28_1: read-var merge#var.1:struct
        let s_28_1: i128 = fn_state.merge_var._1;
        // D s_28_2: write-var u#38512 <= s_28_1
        fn_state.u_38512 = s_28_1;
        // C s_28_3: const #"cpu.has_arm_v8-1" : str
        let s_28_3: &'static str = "cpu.has_arm_v8-1";
        // D s_28_4: cmp-eq s_28_0 s_28_3
        let s_28_4: bool = ((s_28_0) == (s_28_3));
        // D s_28_5: not s_28_4
        let s_28_5: bool = !s_28_4;
        // N s_28_6: branch s_28_5 b30 b29
        if s_28_5 {
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
        // C s_29_0: const #1s : i
        let s_29_0: i128 = 1;
        // D s_29_1: read-var u#38512:i
        let s_29_1: i128 = fn_state.u_38512;
        // D s_29_2: cmp-eq s_29_1 s_29_0
        let s_29_2: bool = ((s_29_1) == (s_29_0));
        // D s_29_3: call __ConfigureV81(s_29_2)
        let s_29_3: () = u__ConfigureV81(state, tracer, s_29_2);
        // N s_29_4: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var merge#var.0:struct
        let s_30_0: &'static str = fn_state.merge_var._0;
        // D s_30_1: read-var merge#var.1:struct
        let s_30_1: i128 = fn_state.merge_var._1;
        // D s_30_2: write-var u#38514 <= s_30_1
        fn_state.u_38514 = s_30_1;
        // C s_30_3: const #"cpu.has_arm_v8-2" : str
        let s_30_3: &'static str = "cpu.has_arm_v8-2";
        // D s_30_4: cmp-eq s_30_0 s_30_3
        let s_30_4: bool = ((s_30_0) == (s_30_3));
        // D s_30_5: not s_30_4
        let s_30_5: bool = !s_30_4;
        // N s_30_6: branch s_30_5 b32 b31
        if s_30_5 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1s : i
        let s_31_0: i128 = 1;
        // D s_31_1: read-var u#38514:i
        let s_31_1: i128 = fn_state.u_38514;
        // D s_31_2: cmp-eq s_31_1 s_31_0
        let s_31_2: bool = ((s_31_1) == (s_31_0));
        // D s_31_3: call __ConfigureV82(s_31_2)
        let s_31_3: () = u__ConfigureV82(state, tracer, s_31_2);
        // N s_31_4: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var merge#var.0:struct
        let s_32_0: &'static str = fn_state.merge_var._0;
        // D s_32_1: read-var merge#var.1:struct
        let s_32_1: i128 = fn_state.merge_var._1;
        // D s_32_2: write-var u#38516 <= s_32_1
        fn_state.u_38516 = s_32_1;
        // C s_32_3: const #"cpu.has_arm_v8-3" : str
        let s_32_3: &'static str = "cpu.has_arm_v8-3";
        // D s_32_4: cmp-eq s_32_0 s_32_3
        let s_32_4: bool = ((s_32_0) == (s_32_3));
        // D s_32_5: not s_32_4
        let s_32_5: bool = !s_32_4;
        // N s_32_6: branch s_32_5 b34 b33
        if s_32_5 {
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
        // C s_33_0: const #1s : i
        let s_33_0: i128 = 1;
        // D s_33_1: read-var u#38516:i
        let s_33_1: i128 = fn_state.u_38516;
        // D s_33_2: cmp-eq s_33_1 s_33_0
        let s_33_2: bool = ((s_33_1) == (s_33_0));
        // D s_33_3: call __ConfigureV83(s_33_2)
        let s_33_3: () = u__ConfigureV83(state, tracer, s_33_2);
        // N s_33_4: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var merge#var.0:struct
        let s_34_0: &'static str = fn_state.merge_var._0;
        // D s_34_1: read-var merge#var.1:struct
        let s_34_1: i128 = fn_state.merge_var._1;
        // D s_34_2: write-var u#38518 <= s_34_1
        fn_state.u_38518 = s_34_1;
        // C s_34_3: const #"cpu.has_arm_v8-4" : str
        let s_34_3: &'static str = "cpu.has_arm_v8-4";
        // D s_34_4: cmp-eq s_34_0 s_34_3
        let s_34_4: bool = ((s_34_0) == (s_34_3));
        // D s_34_5: not s_34_4
        let s_34_5: bool = !s_34_4;
        // N s_34_6: branch s_34_5 b36 b35
        if s_34_5 {
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
        // C s_35_0: const #1s : i
        let s_35_0: i128 = 1;
        // D s_35_1: read-var u#38518:i
        let s_35_1: i128 = fn_state.u_38518;
        // D s_35_2: cmp-eq s_35_1 s_35_0
        let s_35_2: bool = ((s_35_1) == (s_35_0));
        // D s_35_3: call __ConfigureV84(s_35_2)
        let s_35_3: () = u__ConfigureV84(state, tracer, s_35_2);
        // N s_35_4: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var merge#var.0:struct
        let s_36_0: &'static str = fn_state.merge_var._0;
        // D s_36_1: read-var merge#var.1:struct
        let s_36_1: i128 = fn_state.merge_var._1;
        // D s_36_2: write-var u#38520 <= s_36_1
        fn_state.u_38520 = s_36_1;
        // C s_36_3: const #"cpu.has_arm_v8-5" : str
        let s_36_3: &'static str = "cpu.has_arm_v8-5";
        // D s_36_4: cmp-eq s_36_0 s_36_3
        let s_36_4: bool = ((s_36_0) == (s_36_3));
        // D s_36_5: not s_36_4
        let s_36_5: bool = !s_36_4;
        // N s_36_6: branch s_36_5 b38 b37
        if s_36_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1s : i
        let s_37_0: i128 = 1;
        // D s_37_1: read-var u#38520:i
        let s_37_1: i128 = fn_state.u_38520;
        // D s_37_2: cmp-eq s_37_1 s_37_0
        let s_37_2: bool = ((s_37_1) == (s_37_0));
        // D s_37_3: call __ConfigureV85(s_37_2)
        let s_37_3: () = u__ConfigureV85(state, tracer, s_37_2);
        // N s_37_4: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var merge#var.0:struct
        let s_38_0: &'static str = fn_state.merge_var._0;
        // D s_38_1: read-var merge#var.1:struct
        let s_38_1: i128 = fn_state.merge_var._1;
        // D s_38_2: write-var u#38522 <= s_38_1
        fn_state.u_38522 = s_38_1;
        // C s_38_3: const #"cpu.has_arm_v8-6" : str
        let s_38_3: &'static str = "cpu.has_arm_v8-6";
        // D s_38_4: cmp-eq s_38_0 s_38_3
        let s_38_4: bool = ((s_38_0) == (s_38_3));
        // D s_38_5: not s_38_4
        let s_38_5: bool = !s_38_4;
        // N s_38_6: branch s_38_5 b40 b39
        if s_38_5 {
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
        // C s_39_0: const #1s : i
        let s_39_0: i128 = 1;
        // D s_39_1: read-var u#38522:i
        let s_39_1: i128 = fn_state.u_38522;
        // D s_39_2: cmp-eq s_39_1 s_39_0
        let s_39_2: bool = ((s_39_1) == (s_39_0));
        // D s_39_3: call __ConfigureV86(s_39_2)
        let s_39_3: () = u__ConfigureV86(state, tracer, s_39_2);
        // N s_39_4: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var merge#var.0:struct
        let s_40_0: &'static str = fn_state.merge_var._0;
        // D s_40_1: read-var merge#var.1:struct
        let s_40_1: i128 = fn_state.merge_var._1;
        // D s_40_2: write-var u#38524 <= s_40_1
        fn_state.u_38524 = s_40_1;
        // C s_40_3: const #"cpu.has_arm_v8-7" : str
        let s_40_3: &'static str = "cpu.has_arm_v8-7";
        // D s_40_4: cmp-eq s_40_0 s_40_3
        let s_40_4: bool = ((s_40_0) == (s_40_3));
        // D s_40_5: not s_40_4
        let s_40_5: bool = !s_40_4;
        // N s_40_6: branch s_40_5 b42 b41
        if s_40_5 {
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
        // C s_41_0: const #1s : i
        let s_41_0: i128 = 1;
        // D s_41_1: read-var u#38524:i
        let s_41_1: i128 = fn_state.u_38524;
        // D s_41_2: cmp-eq s_41_1 s_41_0
        let s_41_2: bool = ((s_41_1) == (s_41_0));
        // D s_41_3: call __ConfigureV87(s_41_2)
        let s_41_3: () = u__ConfigureV87(state, tracer, s_41_2);
        // N s_41_4: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var merge#var.0:struct
        let s_42_0: &'static str = fn_state.merge_var._0;
        // D s_42_1: read-var merge#var.1:struct
        let s_42_1: i128 = fn_state.merge_var._1;
        // D s_42_2: write-var u#38526 <= s_42_1
        fn_state.u_38526 = s_42_1;
        // C s_42_3: const #"cpu.has_arm_v8-8" : str
        let s_42_3: &'static str = "cpu.has_arm_v8-8";
        // D s_42_4: cmp-eq s_42_0 s_42_3
        let s_42_4: bool = ((s_42_0) == (s_42_3));
        // D s_42_5: not s_42_4
        let s_42_5: bool = !s_42_4;
        // N s_42_6: branch s_42_5 b44 b43
        if s_42_5 {
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
        // C s_43_0: const #1s : i
        let s_43_0: i128 = 1;
        // D s_43_1: read-var u#38526:i
        let s_43_1: i128 = fn_state.u_38526;
        // D s_43_2: cmp-eq s_43_1 s_43_0
        let s_43_2: bool = ((s_43_1) == (s_43_0));
        // D s_43_3: call __ConfigureV88(s_43_2)
        let s_43_3: () = u__ConfigureV88(state, tracer, s_43_2);
        // N s_43_4: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var merge#var.0:struct
        let s_44_0: &'static str = fn_state.merge_var._0;
        // D s_44_1: read-var merge#var.1:struct
        let s_44_1: i128 = fn_state.merge_var._1;
        // D s_44_2: write-var u#38528 <= s_44_1
        fn_state.u_38528 = s_44_1;
        // C s_44_3: const #"cpu.has_arm_v8-9" : str
        let s_44_3: &'static str = "cpu.has_arm_v8-9";
        // D s_44_4: cmp-eq s_44_0 s_44_3
        let s_44_4: bool = ((s_44_0) == (s_44_3));
        // D s_44_5: not s_44_4
        let s_44_5: bool = !s_44_4;
        // N s_44_6: branch s_44_5 b46 b45
        if s_44_5 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1s : i
        let s_45_0: i128 = 1;
        // D s_45_1: read-var u#38528:i
        let s_45_1: i128 = fn_state.u_38528;
        // D s_45_2: cmp-eq s_45_1 s_45_0
        let s_45_2: bool = ((s_45_1) == (s_45_0));
        // D s_45_3: call __ConfigureV89(s_45_2)
        let s_45_3: () = u__ConfigureV89(state, tracer, s_45_2);
        // N s_45_4: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var merge#var.0:struct
        let s_46_0: &'static str = fn_state.merge_var._0;
        // D s_46_1: read-var merge#var.1:struct
        let s_46_1: i128 = fn_state.merge_var._1;
        // D s_46_2: write-var u#38530 <= s_46_1
        fn_state.u_38530 = s_46_1;
        // C s_46_3: const #"cpu.has_arm_v9-0" : str
        let s_46_3: &'static str = "cpu.has_arm_v9-0";
        // D s_46_4: cmp-eq s_46_0 s_46_3
        let s_46_4: bool = ((s_46_0) == (s_46_3));
        // D s_46_5: not s_46_4
        let s_46_5: bool = !s_46_4;
        // N s_46_6: branch s_46_5 b48 b47
        if s_46_5 {
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
        // C s_47_0: const #1s : i
        let s_47_0: i128 = 1;
        // D s_47_1: read-var u#38530:i
        let s_47_1: i128 = fn_state.u_38530;
        // D s_47_2: cmp-eq s_47_1 s_47_0
        let s_47_2: bool = ((s_47_1) == (s_47_0));
        // D s_47_3: call __ConfigureV90(s_47_2)
        let s_47_3: () = u__ConfigureV90(state, tracer, s_47_2);
        // N s_47_4: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var merge#var.0:struct
        let s_48_0: &'static str = fn_state.merge_var._0;
        // D s_48_1: read-var merge#var.1:struct
        let s_48_1: i128 = fn_state.merge_var._1;
        // D s_48_2: write-var u#38532 <= s_48_1
        fn_state.u_38532 = s_48_1;
        // C s_48_3: const #"cpu.has_arm_v9-1" : str
        let s_48_3: &'static str = "cpu.has_arm_v9-1";
        // D s_48_4: cmp-eq s_48_0 s_48_3
        let s_48_4: bool = ((s_48_0) == (s_48_3));
        // D s_48_5: not s_48_4
        let s_48_5: bool = !s_48_4;
        // N s_48_6: branch s_48_5 b50 b49
        if s_48_5 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1s : i
        let s_49_0: i128 = 1;
        // D s_49_1: read-var u#38532:i
        let s_49_1: i128 = fn_state.u_38532;
        // D s_49_2: cmp-eq s_49_1 s_49_0
        let s_49_2: bool = ((s_49_1) == (s_49_0));
        // D s_49_3: call __ConfigureV91(s_49_2)
        let s_49_3: () = u__ConfigureV91(state, tracer, s_49_2);
        // N s_49_4: return
        return;
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var merge#var.0:struct
        let s_50_0: &'static str = fn_state.merge_var._0;
        // D s_50_1: read-var merge#var.1:struct
        let s_50_1: i128 = fn_state.merge_var._1;
        // D s_50_2: write-var u#38534 <= s_50_1
        fn_state.u_38534 = s_50_1;
        // C s_50_3: const #"cpu.has_arm_v9-2" : str
        let s_50_3: &'static str = "cpu.has_arm_v9-2";
        // D s_50_4: cmp-eq s_50_0 s_50_3
        let s_50_4: bool = ((s_50_0) == (s_50_3));
        // D s_50_5: not s_50_4
        let s_50_5: bool = !s_50_4;
        // N s_50_6: branch s_50_5 b52 b51
        if s_50_5 {
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
        // C s_51_0: const #1s : i
        let s_51_0: i128 = 1;
        // D s_51_1: read-var u#38534:i
        let s_51_1: i128 = fn_state.u_38534;
        // D s_51_2: cmp-eq s_51_1 s_51_0
        let s_51_2: bool = ((s_51_1) == (s_51_0));
        // D s_51_3: call __ConfigureV92(s_51_2)
        let s_51_3: () = u__ConfigureV92(state, tracer, s_51_2);
        // N s_51_4: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var merge#var.0:struct
        let s_52_0: &'static str = fn_state.merge_var._0;
        // D s_52_1: read-var merge#var.1:struct
        let s_52_1: i128 = fn_state.merge_var._1;
        // D s_52_2: write-var u#38536 <= s_52_1
        fn_state.u_38536 = s_52_1;
        // C s_52_3: const #"cpu.has_arm_v9-3" : str
        let s_52_3: &'static str = "cpu.has_arm_v9-3";
        // D s_52_4: cmp-eq s_52_0 s_52_3
        let s_52_4: bool = ((s_52_0) == (s_52_3));
        // D s_52_5: not s_52_4
        let s_52_5: bool = !s_52_4;
        // N s_52_6: branch s_52_5 b54 b53
        if s_52_5 {
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
        // C s_53_0: const #1s : i
        let s_53_0: i128 = 1;
        // D s_53_1: read-var u#38536:i
        let s_53_1: i128 = fn_state.u_38536;
        // D s_53_2: cmp-eq s_53_1 s_53_0
        let s_53_2: bool = ((s_53_1) == (s_53_0));
        // D s_53_3: call __ConfigureV93(s_53_2)
        let s_53_3: () = u__ConfigureV93(state, tracer, s_53_2);
        // N s_53_4: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var merge#var.0:struct
        let s_54_0: &'static str = fn_state.merge_var._0;
        // D s_54_1: read-var merge#var.1:struct
        let s_54_1: i128 = fn_state.merge_var._1;
        // D s_54_2: write-var u#38538 <= s_54_1
        fn_state.u_38538 = s_54_1;
        // C s_54_3: const #"cpu.has_arm_v9-4" : str
        let s_54_3: &'static str = "cpu.has_arm_v9-4";
        // D s_54_4: cmp-eq s_54_0 s_54_3
        let s_54_4: bool = ((s_54_0) == (s_54_3));
        // D s_54_5: not s_54_4
        let s_54_5: bool = !s_54_4;
        // N s_54_6: branch s_54_5 b56 b55
        if s_54_5 {
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
        // C s_55_0: const #1s : i
        let s_55_0: i128 = 1;
        // D s_55_1: read-var u#38538:i
        let s_55_1: i128 = fn_state.u_38538;
        // D s_55_2: cmp-eq s_55_1 s_55_0
        let s_55_2: bool = ((s_55_1) == (s_55_0));
        // D s_55_3: call __ConfigureV94(s_55_2)
        let s_55_3: () = u__ConfigureV94(state, tracer, s_55_2);
        // N s_55_4: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var merge#var.0:struct
        let s_56_0: &'static str = fn_state.merge_var._0;
        // D s_56_1: read-var merge#var.1:struct
        let s_56_1: i128 = fn_state.merge_var._1;
        // D s_56_2: write-var u#38540 <= s_56_1
        fn_state.u_38540 = s_56_1;
        // C s_56_3: const #"cpu.cpu0.CONFIG64" : str
        let s_56_3: &'static str = "cpu.cpu0.CONFIG64";
        // D s_56_4: cmp-eq s_56_0 s_56_3
        let s_56_4: bool = ((s_56_0) == (s_56_3));
        // D s_56_5: not s_56_4
        let s_56_5: bool = !s_56_4;
        // N s_56_6: branch s_56_5 b58 b57
        if s_56_5 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0s : i
        let s_57_0: i128 = 0;
        // D s_57_1: read-var u#38540:i
        let s_57_1: i128 = fn_state.u_38540;
        // D s_57_2: call integer_access(s_57_1, s_57_0)
        let s_57_2: bool = integer_access(state, tracer, s_57_1, s_57_0);
        // C s_57_3: const #0s : i
        let s_57_3: i128 = 0;
        // C s_57_4: const #0u : u64
        let s_57_4: u64 = 0;
        // D s_57_5: cast zx s_57_2 -> u64
        let s_57_5: u64 = (s_57_2 as u64);
        // C s_57_6: const #1u : u64
        let s_57_6: u64 = 1;
        // D s_57_7: and s_57_5 s_57_6
        let s_57_7: u64 = ((s_57_5) & (s_57_6));
        // D s_57_8: cmp-eq s_57_7 s_57_6
        let s_57_8: bool = ((s_57_7) == (s_57_6));
        // D s_57_9: lsl s_57_5 s_57_3
        let s_57_9: u64 = s_57_5 << s_57_3;
        // D s_57_10: or s_57_4 s_57_9
        let s_57_10: u64 = ((s_57_4) | (s_57_9));
        // D s_57_11: cmpl s_57_9
        let s_57_11: u64 = !s_57_9;
        // D s_57_12: and s_57_4 s_57_11
        let s_57_12: u64 = ((s_57_4) & (s_57_11));
        // D s_57_13: select s_57_8 s_57_10 s_57_12
        let s_57_13: u64 = if s_57_8 { s_57_10 } else { s_57_12 };
        // D s_57_14: cast trunc s_57_13 -> u8
        let s_57_14: bool = ((s_57_13) != 0);
        // C s_57_15: const #16352u : u32
        let s_57_15: u32 = 16352;
        // N s_57_16: write-reg s_57_15 <= s_57_14
        let s_57_16: () = {
            state.write_register::<bool>(s_57_15 as isize, s_57_14);
            tracer.write_register(s_57_15 as isize, s_57_14);
        };
        // N s_57_17: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var merge#var.0:struct
        let s_58_0: &'static str = fn_state.merge_var._0;
        // D s_58_1: read-var merge#var.1:struct
        let s_58_1: i128 = fn_state.merge_var._1;
        // D s_58_2: write-var u#38542 <= s_58_1
        fn_state.u_38542 = s_58_1;
        // C s_58_3: const #"cpu.cpu0.RVBAR" : str
        let s_58_3: &'static str = "cpu.cpu0.RVBAR";
        // D s_58_4: cmp-eq s_58_0 s_58_3
        let s_58_4: bool = ((s_58_0) == (s_58_3));
        // D s_58_5: not s_58_4
        let s_58_5: bool = !s_58_4;
        // N s_58_6: branch s_58_5 b60 b59
        if s_58_5 {
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
        // C s_59_0: const #63s : i
        let s_59_0: i128 = 63;
        // C s_59_1: const #0s : i
        let s_59_1: i128 = 0;
        // D s_59_2: read-var u#38542:i
        let s_59_2: i128 = fn_state.u_38542;
        // D s_59_3: call integer_subrange(s_59_2, s_59_0, s_59_1)
        let s_59_3: Bits = integer_subrange(state, tracer, s_59_2, s_59_0, s_59_1);
        // D s_59_4: cast reint s_59_3 -> u64
        let s_59_4: u64 = (s_59_3.value() as u64);
        // C s_59_5: const #102440u : u32
        let s_59_5: u32 = 102440;
        // N s_59_6: write-reg s_59_5 <= s_59_4
        let s_59_6: () = {
            state.write_register::<u64>(s_59_5 as isize, s_59_4);
            tracer.write_register(s_59_5 as isize, s_59_4);
        };
        // N s_59_7: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var merge#var.0:struct
        let s_60_0: &'static str = fn_state.merge_var._0;
        // D s_60_1: read-var merge#var.1:struct
        let s_60_1: i128 = fn_state.merge_var._1;
        // D s_60_2: write-var u#38544 <= s_60_1
        fn_state.u_38544 = s_60_1;
        // C s_60_3: const #"cpu.num_loregions" : str
        let s_60_3: &'static str = "cpu.num_loregions";
        // D s_60_4: cmp-eq s_60_0 s_60_3
        let s_60_4: bool = ((s_60_0) == (s_60_3));
        // D s_60_5: not s_60_4
        let s_60_5: bool = !s_60_4;
        // N s_60_6: branch s_60_5 b62 b61
        if s_60_5 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #7s : i
        let s_61_0: i128 = 7;
        // C s_61_1: const #0s : i
        let s_61_1: i128 = 0;
        // D s_61_2: read-var u#38544:i
        let s_61_2: i128 = fn_state.u_38544;
        // D s_61_3: call integer_subrange(s_61_2, s_61_0, s_61_1)
        let s_61_3: Bits = integer_subrange(state, tracer, s_61_2, s_61_0, s_61_1);
        // C s_61_4: const #22080u : u32
        let s_61_4: u32 = 22080;
        // D s_61_5: read-reg s_61_4:struct
        let s_61_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_4 as isize);
            tracer.read_register(s_61_4 as isize, value);
            value
        };
        // C s_61_6: const #22080u : u32
        let s_61_6: u32 = 22080;
        // N s_61_7: write-reg s_61_6 <= s_61_5
        let s_61_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_61_6 as isize, s_61_5);
            tracer.write_register(s_61_6 as isize, s_61_5);
        };
        // N s_61_8: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var merge#var.0:struct
        let s_62_0: &'static str = fn_state.merge_var._0;
        // D s_62_1: read-var merge#var.1:struct
        let s_62_1: i128 = fn_state.merge_var._1;
        // D s_62_2: write-var u#38546 <= s_62_1
        fn_state.u_38546 = s_62_1;
        // C s_62_3: const #"cpu.num_loregion_descriptors" : str
        let s_62_3: &'static str = "cpu.num_loregion_descriptors";
        // D s_62_4: cmp-eq s_62_0 s_62_3
        let s_62_4: bool = ((s_62_0) == (s_62_3));
        // D s_62_5: not s_62_4
        let s_62_5: bool = !s_62_4;
        // N s_62_6: branch s_62_5 b64 b63
        if s_62_5 {
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
        // C s_63_0: const #7s : i
        let s_63_0: i128 = 7;
        // C s_63_1: const #0s : i
        let s_63_1: i128 = 0;
        // D s_63_2: read-var u#38546:i
        let s_63_2: i128 = fn_state.u_38546;
        // D s_63_3: call integer_subrange(s_63_2, s_63_0, s_63_1)
        let s_63_3: Bits = integer_subrange(state, tracer, s_63_2, s_63_0, s_63_1);
        // C s_63_4: const #22080u : u32
        let s_63_4: u32 = 22080;
        // D s_63_5: read-reg s_63_4:struct
        let s_63_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_4 as isize);
            tracer.read_register(s_63_4 as isize, value);
            value
        };
        // C s_63_6: const #22080u : u32
        let s_63_6: u32 = 22080;
        // N s_63_7: write-reg s_63_6 <= s_63_5
        let s_63_7: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_63_6 as isize, s_63_5);
            tracer.write_register(s_63_6 as isize, s_63_5);
        };
        // N s_63_8: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var merge#var.0:struct
        let s_64_0: &'static str = fn_state.merge_var._0;
        // D s_64_1: read-var merge#var.1:struct
        let s_64_1: i128 = fn_state.merge_var._1;
        // D s_64_2: write-var u#38548 <= s_64_1
        fn_state.u_38548 = s_64_1;
        // C s_64_3: const #"cpu.cpy_mops_option" : str
        let s_64_3: &'static str = "cpu.cpy_mops_option";
        // D s_64_4: cmp-eq s_64_0 s_64_3
        let s_64_4: bool = ((s_64_0) == (s_64_3));
        // D s_64_5: not s_64_4
        let s_64_5: bool = !s_64_4;
        // N s_64_6: branch s_64_5 b66 b65
        if s_64_5 {
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
        // C s_65_0: const #1s : i
        let s_65_0: i128 = 1;
        // D s_65_1: read-var u#38548:i
        let s_65_1: i128 = fn_state.u_38548;
        // D s_65_2: cmp-eq s_65_1 s_65_0
        let s_65_2: bool = ((s_65_1) == (s_65_0));
        // C s_65_3: const #23640u : u32
        let s_65_3: u32 = 23640;
        // N s_65_4: write-reg s_65_3 <= s_65_2
        let s_65_4: () = {
            state.write_register::<bool>(s_65_3 as isize, s_65_2);
            tracer.write_register(s_65_3 as isize, s_65_2);
        };
        // N s_65_5: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var merge#var.0:struct
        let s_66_0: &'static str = fn_state.merge_var._0;
        // D s_66_1: read-var merge#var.1:struct
        let s_66_1: i128 = fn_state.merge_var._1;
        // D s_66_2: write-var u#38550 <= s_66_1
        fn_state.u_38550 = s_66_1;
        // C s_66_3: const #"cpu.cpyf_mops_option" : str
        let s_66_3: &'static str = "cpu.cpyf_mops_option";
        // D s_66_4: cmp-eq s_66_0 s_66_3
        let s_66_4: bool = ((s_66_0) == (s_66_3));
        // D s_66_5: not s_66_4
        let s_66_5: bool = !s_66_4;
        // N s_66_6: branch s_66_5 b68 b67
        if s_66_5 {
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
        // C s_67_0: const #1s : i
        let s_67_0: i128 = 1;
        // D s_67_1: read-var u#38550:i
        let s_67_1: i128 = fn_state.u_38550;
        // D s_67_2: cmp-eq s_67_1 s_67_0
        let s_67_2: bool = ((s_67_1) == (s_67_0));
        // C s_67_3: const #89528u : u32
        let s_67_3: u32 = 89528;
        // N s_67_4: write-reg s_67_3 <= s_67_2
        let s_67_4: () = {
            state.write_register::<bool>(s_67_3 as isize, s_67_2);
            tracer.write_register(s_67_3 as isize, s_67_2);
        };
        // N s_67_5: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var merge#var.0:struct
        let s_68_0: &'static str = fn_state.merge_var._0;
        // D s_68_1: read-var merge#var.1:struct
        let s_68_1: i128 = fn_state.merge_var._1;
        // D s_68_2: write-var u#38552 <= s_68_1
        fn_state.u_38552 = s_68_1;
        // C s_68_3: const #"cpu.set_mops_option" : str
        let s_68_3: &'static str = "cpu.set_mops_option";
        // D s_68_4: cmp-eq s_68_0 s_68_3
        let s_68_4: bool = ((s_68_0) == (s_68_3));
        // D s_68_5: not s_68_4
        let s_68_5: bool = !s_68_4;
        // N s_68_6: branch s_68_5 b70 b69
        if s_68_5 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1s : i
        let s_69_0: i128 = 1;
        // D s_69_1: read-var u#38552:i
        let s_69_1: i128 = fn_state.u_38552;
        // D s_69_2: cmp-eq s_69_1 s_69_0
        let s_69_2: bool = ((s_69_1) == (s_69_0));
        // C s_69_3: const #14344u : u32
        let s_69_3: u32 = 14344;
        // N s_69_4: write-reg s_69_3 <= s_69_2
        let s_69_4: () = {
            state.write_register::<bool>(s_69_3 as isize, s_69_2);
            tracer.write_register(s_69_3 as isize, s_69_2);
        };
        // N s_69_5: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var merge#var.0:struct
        let s_70_0: &'static str = fn_state.merge_var._0;
        // D s_70_1: read-var merge#var.1:struct
        let s_70_1: i128 = fn_state.merge_var._1;
        // D s_70_2: write-var u#38554 <= s_70_1
        fn_state.u_38554 = s_70_1;
        // C s_70_3: const #"cpu.setg_mops_option" : str
        let s_70_3: &'static str = "cpu.setg_mops_option";
        // D s_70_4: cmp-eq s_70_0 s_70_3
        let s_70_4: bool = ((s_70_0) == (s_70_3));
        // D s_70_5: not s_70_4
        let s_70_5: bool = !s_70_4;
        // N s_70_6: branch s_70_5 b72 b71
        if s_70_5 {
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
        // C s_71_0: const #1s : i
        let s_71_0: i128 = 1;
        // D s_71_1: read-var u#38554:i
        let s_71_1: i128 = fn_state.u_38554;
        // D s_71_2: cmp-eq s_71_1 s_71_0
        let s_71_2: bool = ((s_71_1) == (s_71_0));
        // C s_71_3: const #103192u : u32
        let s_71_3: u32 = 103192;
        // N s_71_4: write-reg s_71_3 <= s_71_2
        let s_71_4: () = {
            state.write_register::<bool>(s_71_3 as isize, s_71_2);
            tracer.write_register(s_71_3 as isize, s_71_2);
        };
        // N s_71_5: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var merge#var.0:struct
        let s_72_0: &'static str = fn_state.merge_var._0;
        // D s_72_1: read-var merge#var.1:struct
        let s_72_1: i128 = fn_state.merge_var._1;
        // D s_72_2: write-var u#38556 <= s_72_1
        fn_state.u_38556 = s_72_1;
        // C s_72_3: const #"cpu.mops_cpy_default_dir" : str
        let s_72_3: &'static str = "cpu.mops_cpy_default_dir";
        // D s_72_4: cmp-eq s_72_0 s_72_3
        let s_72_4: bool = ((s_72_0) == (s_72_3));
        // D s_72_5: not s_72_4
        let s_72_5: bool = !s_72_4;
        // N s_72_6: branch s_72_5 b74 b73
        if s_72_5 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0s : i
        let s_73_0: i128 = 0;
        // D s_73_1: read-var u#38556:i
        let s_73_1: i128 = fn_state.u_38556;
        // D s_73_2: cmp-eq s_73_1 s_73_0
        let s_73_2: bool = ((s_73_1) == (s_73_0));
        // C s_73_3: const #20664u : u32
        let s_73_3: u32 = 20664;
        // N s_73_4: write-reg s_73_3 <= s_73_2
        let s_73_4: () = {
            state.write_register::<bool>(s_73_3 as isize, s_73_2);
            tracer.write_register(s_73_3 as isize, s_73_2);
        };
        // N s_73_5: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var merge#var.0:struct
        let s_74_0: &'static str = fn_state.merge_var._0;
        // D s_74_1: read-var merge#var.1:struct
        let s_74_1: i128 = fn_state.merge_var._1;
        // D s_74_2: write-var u#38558 <= s_74_1
        fn_state.u_38558 = s_74_1;
        // C s_74_3: const #"cpu.has_pstate_pan" : str
        let s_74_3: &'static str = "cpu.has_pstate_pan";
        // D s_74_4: cmp-eq s_74_0 s_74_3
        let s_74_4: bool = ((s_74_0) == (s_74_3));
        // D s_74_5: not s_74_4
        let s_74_5: bool = !s_74_4;
        // N s_74_6: branch s_74_5 b76 b75
        if s_74_5 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #0s : i
        let s_75_0: i128 = 0;
        // D s_75_1: read-var u#38558:i
        let s_75_1: i128 = fn_state.u_38558;
        // D s_75_2: call neq_int(s_75_1, s_75_0)
        let s_75_2: bool = neq_int(state, tracer, s_75_1, s_75_0);
        // C s_75_3: const #100272u : u32
        let s_75_3: u32 = 100272;
        // N s_75_4: write-reg s_75_3 <= s_75_2
        let s_75_4: () = {
            state.write_register::<bool>(s_75_3 as isize, s_75_2);
            tracer.write_register(s_75_3 as isize, s_75_2);
        };
        // N s_75_5: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var merge#var.0:struct
        let s_76_0: &'static str = fn_state.merge_var._0;
        // D s_76_1: read-var merge#var.1:struct
        let s_76_1: i128 = fn_state.merge_var._1;
        // D s_76_2: write-var u#38560 <= s_76_1
        fn_state.u_38560 = s_76_1;
        // C s_76_3: const #"cpu.has_16bit_vmids" : str
        let s_76_3: &'static str = "cpu.has_16bit_vmids";
        // D s_76_4: cmp-eq s_76_0 s_76_3
        let s_76_4: bool = ((s_76_0) == (s_76_3));
        // D s_76_5: not s_76_4
        let s_76_5: bool = !s_76_4;
        // N s_76_6: branch s_76_5 b78 b77
        if s_76_5 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0s : i
        let s_77_0: i128 = 0;
        // D s_77_1: read-var u#38560:i
        let s_77_1: i128 = fn_state.u_38560;
        // D s_77_2: call neq_int(s_77_1, s_77_0)
        let s_77_2: bool = neq_int(state, tracer, s_77_1, s_77_0);
        // C s_77_3: const #1408u : u32
        let s_77_3: u32 = 1408;
        // N s_77_4: write-reg s_77_3 <= s_77_2
        let s_77_4: () = {
            state.write_register::<bool>(s_77_3 as isize, s_77_2);
            tracer.write_register(s_77_3 as isize, s_77_2);
        };
        // N s_77_5: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var merge#var.0:struct
        let s_78_0: &'static str = fn_state.merge_var._0;
        // D s_78_1: read-var merge#var.1:struct
        let s_78_1: i128 = fn_state.merge_var._1;
        // D s_78_2: write-var u#38562 <= s_78_1
        fn_state.u_38562 = s_78_1;
        // C s_78_3: const #"cpu.cpu0.enable_crc32" : str
        let s_78_3: &'static str = "cpu.cpu0.enable_crc32";
        // D s_78_4: cmp-eq s_78_0 s_78_3
        let s_78_4: bool = ((s_78_0) == (s_78_3));
        // D s_78_5: not s_78_4
        let s_78_5: bool = !s_78_4;
        // N s_78_6: branch s_78_5 b80 b79
        if s_78_5 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #1s : i
        let s_79_0: i128 = 1;
        // D s_79_1: read-var u#38562:i
        let s_79_1: i128 = fn_state.u_38562;
        // D s_79_2: cmp-eq s_79_1 s_79_0
        let s_79_2: bool = ((s_79_1) == (s_79_0));
        // C s_79_3: const #20336u : u32
        let s_79_3: u32 = 20336;
        // N s_79_4: write-reg s_79_3 <= s_79_2
        let s_79_4: () = {
            state.write_register::<bool>(s_79_3 as isize, s_79_2);
            tracer.write_register(s_79_3 as isize, s_79_2);
        };
        // N s_79_5: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var merge#var.0:struct
        let s_80_0: &'static str = fn_state.merge_var._0;
        // D s_80_1: read-var merge#var.1:struct
        let s_80_1: i128 = fn_state.merge_var._1;
        // D s_80_2: write-var u#38564 <= s_80_1
        fn_state.u_38564 = s_80_1;
        // C s_80_3: const #"cpu.has_dot_product" : str
        let s_80_3: &'static str = "cpu.has_dot_product";
        // D s_80_4: cmp-eq s_80_0 s_80_3
        let s_80_4: bool = ((s_80_0) == (s_80_3));
        // D s_80_5: not s_80_4
        let s_80_5: bool = !s_80_4;
        // N s_80_6: branch s_80_5 b82 b81
        if s_80_5 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0s : i
        let s_81_0: i128 = 0;
        // D s_81_1: read-var u#38564:i
        let s_81_1: i128 = fn_state.u_38564;
        // D s_81_2: call neq_int(s_81_1, s_81_0)
        let s_81_2: bool = neq_int(state, tracer, s_81_1, s_81_0);
        // C s_81_3: const #15488u : u32
        let s_81_3: u32 = 15488;
        // N s_81_4: write-reg s_81_3 <= s_81_2
        let s_81_4: () = {
            state.write_register::<bool>(s_81_3 as isize, s_81_2);
            tracer.write_register(s_81_3 as isize, s_81_2);
        };
        // N s_81_5: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var merge#var.0:struct
        let s_82_0: &'static str = fn_state.merge_var._0;
        // D s_82_1: read-var merge#var.1:struct
        let s_82_1: i128 = fn_state.merge_var._1;
        // D s_82_2: write-var u#38566 <= s_82_1
        fn_state.u_38566 = s_82_1;
        // C s_82_3: const #"cpu.has_fp16" : str
        let s_82_3: &'static str = "cpu.has_fp16";
        // D s_82_4: cmp-eq s_82_0 s_82_3
        let s_82_4: bool = ((s_82_0) == (s_82_3));
        // D s_82_5: not s_82_4
        let s_82_5: bool = !s_82_4;
        // N s_82_6: branch s_82_5 b84 b83
        if s_82_5 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #0s : i
        let s_83_0: i128 = 0;
        // D s_83_1: read-var u#38566:i
        let s_83_1: i128 = fn_state.u_38566;
        // D s_83_2: call neq_int(s_83_1, s_83_0)
        let s_83_2: bool = neq_int(state, tracer, s_83_1, s_83_0);
        // C s_83_3: const #21040u : u32
        let s_83_3: u32 = 21040;
        // N s_83_4: write-reg s_83_3 <= s_83_2
        let s_83_4: () = {
            state.write_register::<bool>(s_83_3 as isize, s_83_2);
            tracer.write_register(s_83_3 as isize, s_83_2);
        };
        // N s_83_5: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var merge#var.0:struct
        let s_84_0: &'static str = fn_state.merge_var._0;
        // D s_84_1: read-var merge#var.1:struct
        let s_84_1: i128 = fn_state.merge_var._1;
        // D s_84_2: write-var u#38568 <= s_84_1
        fn_state.u_38568 = s_84_1;
        // C s_84_3: const #"cpu.has_aarch32_hpd" : str
        let s_84_3: &'static str = "cpu.has_aarch32_hpd";
        // D s_84_4: cmp-eq s_84_0 s_84_3
        let s_84_4: bool = ((s_84_0) == (s_84_3));
        // D s_84_5: not s_84_4
        let s_84_5: bool = !s_84_4;
        // N s_84_6: branch s_84_5 b86 b85
        if s_84_5 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #1s : i
        let s_85_0: i128 = 1;
        // D s_85_1: read-var u#38568:i
        let s_85_1: i128 = fn_state.u_38568;
        // D s_85_2: cmp-eq s_85_1 s_85_0
        let s_85_2: bool = ((s_85_1) == (s_85_0));
        // C s_85_3: const #16336u : u32
        let s_85_3: u32 = 16336;
        // N s_85_4: write-reg s_85_3 <= s_85_2
        let s_85_4: () = {
            state.write_register::<bool>(s_85_3 as isize, s_85_2);
            tracer.write_register(s_85_3 as isize, s_85_2);
        };
        // N s_85_5: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var merge#var.0:struct
        let s_86_0: &'static str = fn_state.merge_var._0;
        // D s_86_1: read-var merge#var.1:struct
        let s_86_1: i128 = fn_state.merge_var._1;
        // D s_86_2: write-var u#38570 <= s_86_1
        fn_state.u_38570 = s_86_1;
        // C s_86_3: const #"cpu.cpu0.crypto_aes" : str
        let s_86_3: &'static str = "cpu.cpu0.crypto_aes";
        // D s_86_4: cmp-eq s_86_0 s_86_3
        let s_86_4: bool = ((s_86_0) == (s_86_3));
        // D s_86_5: not s_86_4
        let s_86_5: bool = !s_86_4;
        // N s_86_6: branch s_86_5 b88 b87
        if s_86_5 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #0s : i
        let s_87_0: i128 = 0;
        // D s_87_1: read-var u#38570:i
        let s_87_1: i128 = fn_state.u_38570;
        // D s_87_2: call neq_int(s_87_1, s_87_0)
        let s_87_2: bool = neq_int(state, tracer, s_87_1, s_87_0);
        // C s_87_3: const #1568u : u32
        let s_87_3: u32 = 1568;
        // N s_87_4: write-reg s_87_3 <= s_87_2
        let s_87_4: () = {
            state.write_register::<bool>(s_87_3 as isize, s_87_2);
            tracer.write_register(s_87_3 as isize, s_87_2);
        };
        // N s_87_5: return
        return;
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var merge#var.0:struct
        let s_88_0: &'static str = fn_state.merge_var._0;
        // D s_88_1: read-var merge#var.1:struct
        let s_88_1: i128 = fn_state.merge_var._1;
        // D s_88_2: write-var u#38572 <= s_88_1
        fn_state.u_38572 = s_88_1;
        // C s_88_3: const #"cpu.cpu0.crypto_sha1" : str
        let s_88_3: &'static str = "cpu.cpu0.crypto_sha1";
        // D s_88_4: cmp-eq s_88_0 s_88_3
        let s_88_4: bool = ((s_88_0) == (s_88_3));
        // D s_88_5: not s_88_4
        let s_88_5: bool = !s_88_4;
        // N s_88_6: branch s_88_5 b90 b89
        if s_88_5 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #1s : i
        let s_89_0: i128 = 1;
        // D s_89_1: read-var u#38572:i
        let s_89_1: i128 = fn_state.u_38572;
        // D s_89_2: cmp-eq s_89_1 s_89_0
        let s_89_2: bool = ((s_89_1) == (s_89_0));
        // C s_89_3: const #16328u : u32
        let s_89_3: u32 = 16328;
        // N s_89_4: write-reg s_89_3 <= s_89_2
        let s_89_4: () = {
            state.write_register::<bool>(s_89_3 as isize, s_89_2);
            tracer.write_register(s_89_3 as isize, s_89_2);
        };
        // N s_89_5: return
        return;
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var merge#var.0:struct
        let s_90_0: &'static str = fn_state.merge_var._0;
        // D s_90_1: read-var merge#var.1:struct
        let s_90_1: i128 = fn_state.merge_var._1;
        // D s_90_2: write-var u#38574 <= s_90_1
        fn_state.u_38574 = s_90_1;
        // C s_90_3: const #"cpu.cpu0.crypto_sha256" : str
        let s_90_3: &'static str = "cpu.cpu0.crypto_sha256";
        // D s_90_4: cmp-eq s_90_0 s_90_3
        let s_90_4: bool = ((s_90_0) == (s_90_3));
        // D s_90_5: not s_90_4
        let s_90_5: bool = !s_90_4;
        // N s_90_6: branch s_90_5 b92 b91
        if s_90_5 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #1s : i
        let s_91_0: i128 = 1;
        // D s_91_1: read-var u#38574:i
        let s_91_1: i128 = fn_state.u_38574;
        // D s_91_2: cmp-eq s_91_1 s_91_0
        let s_91_2: bool = ((s_91_1) == (s_91_0));
        // C s_91_3: const #1632u : u32
        let s_91_3: u32 = 1632;
        // N s_91_4: write-reg s_91_3 <= s_91_2
        let s_91_4: () = {
            state.write_register::<bool>(s_91_3 as isize, s_91_2);
            tracer.write_register(s_91_3 as isize, s_91_2);
        };
        // N s_91_5: return
        return;
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var merge#var.0:struct
        let s_92_0: &'static str = fn_state.merge_var._0;
        // D s_92_1: read-var merge#var.1:struct
        let s_92_1: i128 = fn_state.merge_var._1;
        // D s_92_2: write-var u#38576 <= s_92_1
        fn_state.u_38576 = s_92_1;
        // C s_92_3: const #"cpu.cpu0.crypto_sha512" : str
        let s_92_3: &'static str = "cpu.cpu0.crypto_sha512";
        // D s_92_4: cmp-eq s_92_0 s_92_3
        let s_92_4: bool = ((s_92_0) == (s_92_3));
        // D s_92_5: not s_92_4
        let s_92_5: bool = !s_92_4;
        // N s_92_6: branch s_92_5 b94 b93
        if s_92_5 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #0s : i
        let s_93_0: i128 = 0;
        // D s_93_1: read-var u#38576:i
        let s_93_1: i128 = fn_state.u_38576;
        // D s_93_2: call neq_int(s_93_1, s_93_0)
        let s_93_2: bool = neq_int(state, tracer, s_93_1, s_93_0);
        // C s_93_3: const #14616u : u32
        let s_93_3: u32 = 14616;
        // N s_93_4: write-reg s_93_3 <= s_93_2
        let s_93_4: () = {
            state.write_register::<bool>(s_93_3 as isize, s_93_2);
            tracer.write_register(s_93_3 as isize, s_93_2);
        };
        // N s_93_5: return
        return;
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var merge#var.0:struct
        let s_94_0: &'static str = fn_state.merge_var._0;
        // D s_94_1: read-var merge#var.1:struct
        let s_94_1: i128 = fn_state.merge_var._1;
        // D s_94_2: write-var u#38578 <= s_94_1
        fn_state.u_38578 = s_94_1;
        // C s_94_3: const #"cpu.cpu0.crypto_sha3" : str
        let s_94_3: &'static str = "cpu.cpu0.crypto_sha3";
        // D s_94_4: cmp-eq s_94_0 s_94_3
        let s_94_4: bool = ((s_94_0) == (s_94_3));
        // D s_94_5: not s_94_4
        let s_94_5: bool = !s_94_4;
        // N s_94_6: branch s_94_5 b96 b95
        if s_94_5 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #0s : i
        let s_95_0: i128 = 0;
        // D s_95_1: read-var u#38578:i
        let s_95_1: i128 = fn_state.u_38578;
        // D s_95_2: call neq_int(s_95_1, s_95_0)
        let s_95_2: bool = neq_int(state, tracer, s_95_1, s_95_0);
        // C s_95_3: const #101184u : u32
        let s_95_3: u32 = 101184;
        // N s_95_4: write-reg s_95_3 <= s_95_2
        let s_95_4: () = {
            state.write_register::<bool>(s_95_3 as isize, s_95_2);
            tracer.write_register(s_95_3 as isize, s_95_2);
        };
        // N s_95_5: return
        return;
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var merge#var.0:struct
        let s_96_0: &'static str = fn_state.merge_var._0;
        // D s_96_1: read-var merge#var.1:struct
        let s_96_1: i128 = fn_state.merge_var._1;
        // D s_96_2: write-var u#38580 <= s_96_1
        fn_state.u_38580 = s_96_1;
        // C s_96_3: const #"cpu.cpu0.crypto_sm3" : str
        let s_96_3: &'static str = "cpu.cpu0.crypto_sm3";
        // D s_96_4: cmp-eq s_96_0 s_96_3
        let s_96_4: bool = ((s_96_0) == (s_96_3));
        // D s_96_5: not s_96_4
        let s_96_5: bool = !s_96_4;
        // N s_96_6: branch s_96_5 b98 b97
        if s_96_5 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0s : i
        let s_97_0: i128 = 0;
        // D s_97_1: read-var u#38580:i
        let s_97_1: i128 = fn_state.u_38580;
        // D s_97_2: call neq_int(s_97_1, s_97_0)
        let s_97_2: bool = neq_int(state, tracer, s_97_1, s_97_0);
        // C s_97_3: const #102432u : u32
        let s_97_3: u32 = 102432;
        // N s_97_4: write-reg s_97_3 <= s_97_2
        let s_97_4: () = {
            state.write_register::<bool>(s_97_3 as isize, s_97_2);
            tracer.write_register(s_97_3 as isize, s_97_2);
        };
        // N s_97_5: return
        return;
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var merge#var.0:struct
        let s_98_0: &'static str = fn_state.merge_var._0;
        // D s_98_1: read-var merge#var.1:struct
        let s_98_1: i128 = fn_state.merge_var._1;
        // D s_98_2: write-var u#38582 <= s_98_1
        fn_state.u_38582 = s_98_1;
        // C s_98_3: const #"cpu.cpu0.crypto_sm4" : str
        let s_98_3: &'static str = "cpu.cpu0.crypto_sm4";
        // D s_98_4: cmp-eq s_98_0 s_98_3
        let s_98_4: bool = ((s_98_0) == (s_98_3));
        // D s_98_5: not s_98_4
        let s_98_5: bool = !s_98_4;
        // N s_98_6: branch s_98_5 b100 b99
        if s_98_5 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #0s : i
        let s_99_0: i128 = 0;
        // D s_99_1: read-var u#38582:i
        let s_99_1: i128 = fn_state.u_38582;
        // D s_99_2: call neq_int(s_99_1, s_99_0)
        let s_99_2: bool = neq_int(state, tracer, s_99_1, s_99_0);
        // C s_99_3: const #102336u : u32
        let s_99_3: u32 = 102336;
        // N s_99_4: write-reg s_99_3 <= s_99_2
        let s_99_4: () = {
            state.write_register::<bool>(s_99_3 as isize, s_99_2);
            tracer.write_register(s_99_3 as isize, s_99_2);
        };
        // N s_99_5: return
        return;
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var merge#var.0:struct
        let s_100_0: &'static str = fn_state.merge_var._0;
        // D s_100_1: read-var merge#var.1:struct
        let s_100_1: i128 = fn_state.merge_var._1;
        // D s_100_2: write-var u#38584 <= s_100_1
        fn_state.u_38584 = s_100_1;
        // C s_100_3: const #"cpu.cpu0.BBM" : str
        let s_100_3: &'static str = "cpu.cpu0.BBM";
        // D s_100_4: cmp-eq s_100_0 s_100_3
        let s_100_4: bool = ((s_100_0) == (s_100_3));
        // D s_100_5: not s_100_4
        let s_100_5: bool = !s_100_4;
        // N s_100_6: branch s_100_5 b102 b101
        if s_100_5 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var u#38584:i
        let s_101_0: i128 = fn_state.u_38584;
        // C s_101_1: const #15624u : u32
        let s_101_1: u32 = 15624;
        // N s_101_2: write-reg s_101_1 <= s_101_0
        let s_101_2: () = {
            state.write_register::<i128>(s_101_1 as isize, s_101_0);
            tracer.write_register(s_101_1 as isize, s_101_0);
        };
        // N s_101_3: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var merge#var.0:struct
        let s_102_0: &'static str = fn_state.merge_var._0;
        // D s_102_1: read-var merge#var.1:struct
        let s_102_1: i128 = fn_state.merge_var._1;
        // D s_102_2: write-var u#38586 <= s_102_1
        fn_state.u_38586 = s_102_1;
        // C s_102_3: const #"cpu.cpu0.number-of-breakpoints" : str
        let s_102_3: &'static str = "cpu.cpu0.number-of-breakpoints";
        // D s_102_4: cmp-eq s_102_0 s_102_3
        let s_102_4: bool = ((s_102_0) == (s_102_3));
        // D s_102_5: not s_102_4
        let s_102_5: bool = !s_102_4;
        // N s_102_6: branch s_102_5 b104 b103
        if s_102_5 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var u#38586:i
        let s_103_0: i128 = fn_state.u_38586;
        // C s_103_1: const #22776u : u32
        let s_103_1: u32 = 22776;
        // N s_103_2: write-reg s_103_1 <= s_103_0
        let s_103_2: () = {
            state.write_register::<i128>(s_103_1 as isize, s_103_0);
            tracer.write_register(s_103_1 as isize, s_103_0);
        };
        // N s_103_3: return
        return;
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var merge#var.0:struct
        let s_104_0: &'static str = fn_state.merge_var._0;
        // D s_104_1: read-var merge#var.1:struct
        let s_104_1: i128 = fn_state.merge_var._1;
        // D s_104_2: write-var u#38588 <= s_104_1
        fn_state.u_38588 = s_104_1;
        // C s_104_3: const #"cpu.cpu0.number-of-watchpoints" : str
        let s_104_3: &'static str = "cpu.cpu0.number-of-watchpoints";
        // D s_104_4: cmp-eq s_104_0 s_104_3
        let s_104_4: bool = ((s_104_0) == (s_104_3));
        // D s_104_5: not s_104_4
        let s_104_5: bool = !s_104_4;
        // N s_104_6: branch s_104_5 b106 b105
        if s_104_5 {
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
        // D s_105_0: read-var u#38588:i
        let s_105_0: i128 = fn_state.u_38588;
        // C s_105_1: const #19360u : u32
        let s_105_1: u32 = 19360;
        // N s_105_2: write-reg s_105_1 <= s_105_0
        let s_105_2: () = {
            state.write_register::<i128>(s_105_1 as isize, s_105_0);
            tracer.write_register(s_105_1 as isize, s_105_0);
        };
        // N s_105_3: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var merge#var.0:struct
        let s_106_0: &'static str = fn_state.merge_var._0;
        // D s_106_1: read-var merge#var.1:struct
        let s_106_1: i128 = fn_state.merge_var._1;
        // D s_106_2: write-var u#38590 <= s_106_1
        fn_state.u_38590 = s_106_1;
        // C s_106_3: const #"cpu.cpu0.number-of-context-breakpoints" : str
        let s_106_3: &'static str = "cpu.cpu0.number-of-context-breakpoints";
        // D s_106_4: cmp-eq s_106_0 s_106_3
        let s_106_4: bool = ((s_106_0) == (s_106_3));
        // D s_106_5: not s_106_4
        let s_106_5: bool = !s_106_4;
        // N s_106_6: branch s_106_5 b108 b107
        if s_106_5 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var u#38590:i
        let s_107_0: i128 = fn_state.u_38590;
        // C s_107_1: const #102792u : u32
        let s_107_1: u32 = 102792;
        // N s_107_2: write-reg s_107_1 <= s_107_0
        let s_107_2: () = {
            state.write_register::<i128>(s_107_1 as isize, s_107_0);
            tracer.write_register(s_107_1 as isize, s_107_0);
        };
        // N s_107_3: return
        return;
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var merge#var.0:struct
        let s_108_0: &'static str = fn_state.merge_var._0;
        // D s_108_1: read-var merge#var.1:struct
        let s_108_1: i128 = fn_state.merge_var._1;
        // D s_108_2: write-var u#38592 <= s_108_1
        fn_state.u_38592 = s_108_1;
        // C s_108_3: const #"cpu.pmu-num_counters" : str
        let s_108_3: &'static str = "cpu.pmu-num_counters";
        // D s_108_4: cmp-eq s_108_0 s_108_3
        let s_108_4: bool = ((s_108_0) == (s_108_3));
        // D s_108_5: not s_108_4
        let s_108_5: bool = !s_108_4;
        // N s_108_6: branch s_108_5 b110 b109
        if s_108_5 {
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
        // D s_109_0: read-var u#38592:i
        let s_109_0: i128 = fn_state.u_38592;
        // C s_109_1: const #11104u : u32
        let s_109_1: u32 = 11104;
        // N s_109_2: write-reg s_109_1 <= s_109_0
        let s_109_2: () = {
            state.write_register::<i128>(s_109_1 as isize, s_109_0);
            tracer.write_register(s_109_1 as isize, s_109_0);
        };
        // N s_109_3: return
        return;
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var merge#var.0:struct
        let s_110_0: &'static str = fn_state.merge_var._0;
        // D s_110_1: read-var merge#var.1:struct
        let s_110_1: i128 = fn_state.merge_var._1;
        // D s_110_2: write-var u#38594 <= s_110_1
        fn_state.u_38594 = s_110_1;
        // C s_110_3: const #"cpu.PA_SIZE" : str
        let s_110_3: &'static str = "cpu.PA_SIZE";
        // D s_110_4: cmp-eq s_110_0 s_110_3
        let s_110_4: bool = ((s_110_0) == (s_110_3));
        // D s_110_5: not s_110_4
        let s_110_5: bool = !s_110_4;
        // N s_110_6: branch s_110_5 b112 b111
        if s_110_5 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var u#38594:i
        let s_111_0: i128 = fn_state.u_38594;
        // C s_111_1: const #90256u : u32
        let s_111_1: u32 = 90256;
        // N s_111_2: write-reg s_111_1 <= s_111_0
        let s_111_2: () = {
            state.write_register::<i128>(s_111_1 as isize, s_111_0);
            tracer.write_register(s_111_1 as isize, s_111_0);
        };
        // N s_111_3: return
        return;
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var merge#var.0:struct
        let s_112_0: &'static str = fn_state.merge_var._0;
        // D s_112_1: read-var merge#var.1:struct
        let s_112_1: i128 = fn_state.merge_var._1;
        // D s_112_2: write-var u#38596 <= s_112_1
        fn_state.u_38596 = s_112_1;
        // C s_112_3: const #"cpu.VA_SIZE" : str
        let s_112_3: &'static str = "cpu.VA_SIZE";
        // D s_112_4: cmp-eq s_112_0 s_112_3
        let s_112_4: bool = ((s_112_0) == (s_112_3));
        // D s_112_5: not s_112_4
        let s_112_5: bool = !s_112_4;
        // N s_112_6: branch s_112_5 b114 b113
        if s_112_5 {
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
        // D s_113_0: read-var u#38596:i
        let s_113_0: i128 = fn_state.u_38596;
        // C s_113_1: const #14216u : u32
        let s_113_1: u32 = 14216;
        // N s_113_2: write-reg s_113_1 <= s_113_0
        let s_113_2: () = {
            state.write_register::<i128>(s_113_1 as isize, s_113_0);
            tracer.write_register(s_113_1 as isize, s_113_0);
        };
        // N s_113_3: return
        return;
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var merge#var.0:struct
        let s_114_0: &'static str = fn_state.merge_var._0;
        // D s_114_1: read-var merge#var.1:struct
        let s_114_1: i128 = fn_state.merge_var._1;
        // D s_114_2: write-var u#38598 <= s_114_1
        fn_state.u_38598 = s_114_1;
        // C s_114_3: const #"ctiBase" : str
        let s_114_3: &'static str = "ctiBase";
        // D s_114_4: cmp-eq s_114_0 s_114_3
        let s_114_4: bool = ((s_114_0) == (s_114_3));
        // D s_114_5: not s_114_4
        let s_114_5: bool = !s_114_4;
        // N s_114_6: branch s_114_5 b116 b115
        if s_114_5 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #55s : i64
        let s_115_0: i64 = 55;
        // C s_115_1: const #0s : i
        let s_115_1: i128 = 0;
        // C s_115_2: cast zx s_115_0 -> i
        let s_115_2: i128 = (i128::try_from(s_115_0).unwrap());
        // D s_115_3: read-var u#38598:i
        let s_115_3: i128 = fn_state.u_38598;
        // D s_115_4: call integer_subrange(s_115_3, s_115_2, s_115_1)
        let s_115_4: Bits = integer_subrange(state, tracer, s_115_3, s_115_2, s_115_1);
        // D s_115_5: cast reint s_115_4 -> u56
        let s_115_5: u64 = (s_115_4.value() as u64);
        // C s_115_6: const #102496u : u32
        let s_115_6: u32 = 102496;
        // N s_115_7: write-reg s_115_6 <= s_115_5
        let s_115_7: () = {
            state.write_register::<u64>(s_115_6 as isize, s_115_5);
            tracer.write_register(s_115_6 as isize, s_115_5);
        };
        // N s_115_8: return
        return;
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var merge#var.0:struct
        let s_116_0: &'static str = fn_state.merge_var._0;
        // D s_116_1: read-var merge#var.1:struct
        let s_116_1: i128 = fn_state.merge_var._1;
        // D s_116_2: write-var u#38600 <= s_116_1
        fn_state.u_38600 = s_116_1;
        // C s_116_3: const #"counter_addr" : str
        let s_116_3: &'static str = "counter_addr";
        // D s_116_4: cmp-eq s_116_0 s_116_3
        let s_116_4: bool = ((s_116_0) == (s_116_3));
        // D s_116_5: not s_116_4
        let s_116_5: bool = !s_116_4;
        // N s_116_6: branch s_116_5 b118 b117
        if s_116_5 {
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
        // C s_117_0: const #55s : i64
        let s_117_0: i64 = 55;
        // C s_117_1: const #0s : i
        let s_117_1: i128 = 0;
        // C s_117_2: cast zx s_117_0 -> i
        let s_117_2: i128 = (i128::try_from(s_117_0).unwrap());
        // D s_117_3: read-var u#38600:i
        let s_117_3: i128 = fn_state.u_38600;
        // D s_117_4: call integer_subrange(s_117_3, s_117_2, s_117_1)
        let s_117_4: Bits = integer_subrange(state, tracer, s_117_3, s_117_2, s_117_1);
        // D s_117_5: cast reint s_117_4 -> u56
        let s_117_5: u64 = (s_117_4.value() as u64);
        // C s_117_6: const #104576u : u32
        let s_117_6: u32 = 104576;
        // N s_117_7: write-reg s_117_6 <= s_117_5
        let s_117_7: () = {
            state.write_register::<u64>(s_117_6 as isize, s_117_5);
            tracer.write_register(s_117_6 as isize, s_117_5);
        };
        // N s_117_8: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var merge#var.0:struct
        let s_118_0: &'static str = fn_state.merge_var._0;
        // D s_118_1: read-var merge#var.1:struct
        let s_118_1: i128 = fn_state.merge_var._1;
        // D s_118_2: write-var u#38602 <= s_118_1
        fn_state.u_38602 = s_118_1;
        // C s_118_3: const #"cntReadBase" : str
        let s_118_3: &'static str = "cntReadBase";
        // D s_118_4: cmp-eq s_118_0 s_118_3
        let s_118_4: bool = ((s_118_0) == (s_118_3));
        // D s_118_5: not s_118_4
        let s_118_5: bool = !s_118_4;
        // N s_118_6: branch s_118_5 b120 b119
        if s_118_5 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #55s : i64
        let s_119_0: i64 = 55;
        // C s_119_1: const #0s : i
        let s_119_1: i128 = 0;
        // C s_119_2: cast zx s_119_0 -> i
        let s_119_2: i128 = (i128::try_from(s_119_0).unwrap());
        // D s_119_3: read-var u#38602:i
        let s_119_3: i128 = fn_state.u_38602;
        // D s_119_4: call integer_subrange(s_119_3, s_119_2, s_119_1)
        let s_119_4: Bits = integer_subrange(state, tracer, s_119_3, s_119_2, s_119_1);
        // D s_119_5: cast reint s_119_4 -> u56
        let s_119_5: u64 = (s_119_4.value() as u64);
        // C s_119_6: const #17032u : u32
        let s_119_6: u32 = 17032;
        // N s_119_7: write-reg s_119_6 <= s_119_5
        let s_119_7: () = {
            state.write_register::<u64>(s_119_6 as isize, s_119_5);
            tracer.write_register(s_119_6 as isize, s_119_5);
        };
        // N s_119_8: return
        return;
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var merge#var.0:struct
        let s_120_0: &'static str = fn_state.merge_var._0;
        // D s_120_1: read-var merge#var.1:struct
        let s_120_1: i128 = fn_state.merge_var._1;
        // D s_120_2: write-var u#38604 <= s_120_1
        fn_state.u_38604 = s_120_1;
        // C s_120_3: const #"cntBaseN" : str
        let s_120_3: &'static str = "cntBaseN";
        // D s_120_4: cmp-eq s_120_0 s_120_3
        let s_120_4: bool = ((s_120_0) == (s_120_3));
        // D s_120_5: not s_120_4
        let s_120_5: bool = !s_120_4;
        // N s_120_6: branch s_120_5 b122 b121
        if s_120_5 {
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
        // C s_121_0: const #55s : i64
        let s_121_0: i64 = 55;
        // C s_121_1: const #0s : i
        let s_121_1: i128 = 0;
        // C s_121_2: cast zx s_121_0 -> i
        let s_121_2: i128 = (i128::try_from(s_121_0).unwrap());
        // D s_121_3: read-var u#38604:i
        let s_121_3: i128 = fn_state.u_38604;
        // D s_121_4: call integer_subrange(s_121_3, s_121_2, s_121_1)
        let s_121_4: Bits = integer_subrange(state, tracer, s_121_3, s_121_2, s_121_1);
        // D s_121_5: cast reint s_121_4 -> u56
        let s_121_5: u64 = (s_121_4.value() as u64);
        // C s_121_6: const #14544u : u32
        let s_121_6: u32 = 14544;
        // N s_121_7: write-reg s_121_6 <= s_121_5
        let s_121_7: () = {
            state.write_register::<u64>(s_121_6 as isize, s_121_5);
            tracer.write_register(s_121_6 as isize, s_121_5);
        };
        // N s_121_8: return
        return;
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var merge#var.0:struct
        let s_122_0: &'static str = fn_state.merge_var._0;
        // D s_122_1: read-var merge#var.1:struct
        let s_122_1: i128 = fn_state.merge_var._1;
        // D s_122_2: write-var u#38606 <= s_122_1
        fn_state.u_38606 = s_122_1;
        // C s_122_3: const #"cntEL0BaseN" : str
        let s_122_3: &'static str = "cntEL0BaseN";
        // D s_122_4: cmp-eq s_122_0 s_122_3
        let s_122_4: bool = ((s_122_0) == (s_122_3));
        // D s_122_5: not s_122_4
        let s_122_5: bool = !s_122_4;
        // N s_122_6: branch s_122_5 b124 b123
        if s_122_5 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #55s : i64
        let s_123_0: i64 = 55;
        // C s_123_1: const #0s : i
        let s_123_1: i128 = 0;
        // C s_123_2: cast zx s_123_0 -> i
        let s_123_2: i128 = (i128::try_from(s_123_0).unwrap());
        // D s_123_3: read-var u#38606:i
        let s_123_3: i128 = fn_state.u_38606;
        // D s_123_4: call integer_subrange(s_123_3, s_123_2, s_123_1)
        let s_123_4: Bits = integer_subrange(state, tracer, s_123_3, s_123_2, s_123_1);
        // D s_123_5: cast reint s_123_4 -> u56
        let s_123_5: u64 = (s_123_4.value() as u64);
        // C s_123_6: const #23256u : u32
        let s_123_6: u32 = 23256;
        // N s_123_7: write-reg s_123_6 <= s_123_5
        let s_123_7: () = {
            state.write_register::<u64>(s_123_6 as isize, s_123_5);
            tracer.write_register(s_123_6 as isize, s_123_5);
        };
        // N s_123_8: return
        return;
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var merge#var.0:struct
        let s_124_0: &'static str = fn_state.merge_var._0;
        // D s_124_1: read-var merge#var.1:struct
        let s_124_1: i128 = fn_state.merge_var._1;
        // D s_124_2: write-var u#38608 <= s_124_1
        fn_state.u_38608 = s_124_1;
        // C s_124_3: const #"cntCTLBase" : str
        let s_124_3: &'static str = "cntCTLBase";
        // D s_124_4: cmp-eq s_124_0 s_124_3
        let s_124_4: bool = ((s_124_0) == (s_124_3));
        // D s_124_5: not s_124_4
        let s_124_5: bool = !s_124_4;
        // N s_124_6: branch s_124_5 b126 b125
        if s_124_5 {
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
        // C s_125_0: const #55s : i64
        let s_125_0: i64 = 55;
        // C s_125_1: const #0s : i
        let s_125_1: i128 = 0;
        // C s_125_2: cast zx s_125_0 -> i
        let s_125_2: i128 = (i128::try_from(s_125_0).unwrap());
        // D s_125_3: read-var u#38608:i
        let s_125_3: i128 = fn_state.u_38608;
        // D s_125_4: call integer_subrange(s_125_3, s_125_2, s_125_1)
        let s_125_4: Bits = integer_subrange(state, tracer, s_125_3, s_125_2, s_125_1);
        // D s_125_5: cast reint s_125_4 -> u56
        let s_125_5: u64 = (s_125_4.value() as u64);
        // C s_125_6: const #15000u : u32
        let s_125_6: u32 = 15000;
        // N s_125_7: write-reg s_125_6 <= s_125_5
        let s_125_7: () = {
            state.write_register::<u64>(s_125_6 as isize, s_125_5);
            tracer.write_register(s_125_6 as isize, s_125_5);
        };
        // N s_125_8: return
        return;
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var merge#var.0:struct
        let s_126_0: &'static str = fn_state.merge_var._0;
        // D s_126_1: read-var merge#var.1:struct
        let s_126_1: i128 = fn_state.merge_var._1;
        // D s_126_2: write-var u#38610 <= s_126_1
        fn_state.u_38610 = s_126_1;
        // C s_126_3: const #"ExtDebugBase" : str
        let s_126_3: &'static str = "ExtDebugBase";
        // D s_126_4: cmp-eq s_126_0 s_126_3
        let s_126_4: bool = ((s_126_0) == (s_126_3));
        // D s_126_5: not s_126_4
        let s_126_5: bool = !s_126_4;
        // N s_126_6: branch s_126_5 b128 b127
        if s_126_5 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #55s : i64
        let s_127_0: i64 = 55;
        // C s_127_1: const #0s : i
        let s_127_1: i128 = 0;
        // C s_127_2: cast zx s_127_0 -> i
        let s_127_2: i128 = (i128::try_from(s_127_0).unwrap());
        // D s_127_3: read-var u#38610:i
        let s_127_3: i128 = fn_state.u_38610;
        // D s_127_4: call integer_subrange(s_127_3, s_127_2, s_127_1)
        let s_127_4: Bits = integer_subrange(state, tracer, s_127_3, s_127_2, s_127_1);
        // D s_127_5: cast reint s_127_4 -> u56
        let s_127_5: u64 = (s_127_4.value() as u64);
        // C s_127_6: const #104752u : u32
        let s_127_6: u32 = 104752;
        // N s_127_7: write-reg s_127_6 <= s_127_5
        let s_127_7: () = {
            state.write_register::<u64>(s_127_6 as isize, s_127_5);
            tracer.write_register(s_127_6 as isize, s_127_5);
        };
        // N s_127_8: return
        return;
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var merge#var.0:struct
        let s_128_0: &'static str = fn_state.merge_var._0;
        // D s_128_1: read-var merge#var.1:struct
        let s_128_1: i128 = fn_state.merge_var._1;
        // D s_128_2: write-var u#38612 <= s_128_1
        fn_state.u_38612 = s_128_1;
        // C s_128_3: const #"cpu.PERIPHBASE" : str
        let s_128_3: &'static str = "cpu.PERIPHBASE";
        // D s_128_4: cmp-eq s_128_0 s_128_3
        let s_128_4: bool = ((s_128_0) == (s_128_3));
        // D s_128_5: not s_128_4
        let s_128_5: bool = !s_128_4;
        // N s_128_6: branch s_128_5 b130 b129
        if s_128_5 {
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
        // C s_129_0: const #55s : i64
        let s_129_0: i64 = 55;
        // C s_129_1: const #0s : i
        let s_129_1: i128 = 0;
        // C s_129_2: cast zx s_129_0 -> i
        let s_129_2: i128 = (i128::try_from(s_129_0).unwrap());
        // D s_129_3: read-var u#38612:i
        let s_129_3: i128 = fn_state.u_38612;
        // D s_129_4: call integer_subrange(s_129_3, s_129_2, s_129_1)
        let s_129_4: Bits = integer_subrange(state, tracer, s_129_3, s_129_2, s_129_1);
        // D s_129_5: cast reint s_129_4 -> u56
        let s_129_5: u64 = (s_129_4.value() as u64);
        // C s_129_6: const #20552u : u32
        let s_129_6: u32 = 20552;
        // N s_129_7: write-reg s_129_6 <= s_129_5
        let s_129_7: () = {
            state.write_register::<u64>(s_129_6 as isize, s_129_5);
            tracer.write_register(s_129_6 as isize, s_129_5);
        };
        // N s_129_8: return
        return;
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var merge#var.0:struct
        let s_130_0: &'static str = fn_state.merge_var._0;
        // D s_130_1: read-var merge#var.1:struct
        let s_130_1: i128 = fn_state.merge_var._1;
        // D s_130_2: write-var u#38614 <= s_130_1
        fn_state.u_38614 = s_130_1;
        // C s_130_3: const #"gic_distributor.reg-base" : str
        let s_130_3: &'static str = "gic_distributor.reg-base";
        // D s_130_4: cmp-eq s_130_0 s_130_3
        let s_130_4: bool = ((s_130_0) == (s_130_3));
        // D s_130_5: not s_130_4
        let s_130_5: bool = !s_130_4;
        // N s_130_6: branch s_130_5 b132 b131
        if s_130_5 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #55s : i64
        let s_131_0: i64 = 55;
        // C s_131_1: const #0s : i
        let s_131_1: i128 = 0;
        // C s_131_2: cast zx s_131_0 -> i
        let s_131_2: i128 = (i128::try_from(s_131_0).unwrap());
        // D s_131_3: read-var u#38614:i
        let s_131_3: i128 = fn_state.u_38614;
        // D s_131_4: call integer_subrange(s_131_3, s_131_2, s_131_1)
        let s_131_4: Bits = integer_subrange(state, tracer, s_131_3, s_131_2, s_131_1);
        // D s_131_5: cast reint s_131_4 -> u56
        let s_131_5: u64 = (s_131_4.value() as u64);
        // C s_131_6: const #23272u : u32
        let s_131_6: u32 = 23272;
        // N s_131_7: write-reg s_131_6 <= s_131_5
        let s_131_7: () = {
            state.write_register::<u64>(s_131_6 as isize, s_131_5);
            tracer.write_register(s_131_6 as isize, s_131_5);
        };
        // N s_131_8: return
        return;
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var merge#var.0:struct
        let s_132_0: &'static str = fn_state.merge_var._0;
        // D s_132_1: read-var merge#var.1:struct
        let s_132_1: i128 = fn_state.merge_var._1;
        // D s_132_2: write-var u#38616 <= s_132_1
        fn_state.u_38616 = s_132_1;
        // C s_132_3: const #"rdBase" : str
        let s_132_3: &'static str = "rdBase";
        // D s_132_4: cmp-eq s_132_0 s_132_3
        let s_132_4: bool = ((s_132_0) == (s_132_3));
        // D s_132_5: not s_132_4
        let s_132_5: bool = !s_132_4;
        // N s_132_6: branch s_132_5 b134 b133
        if s_132_5 {
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
        // C s_133_0: const #55s : i64
        let s_133_0: i64 = 55;
        // C s_133_1: const #0s : i
        let s_133_1: i128 = 0;
        // C s_133_2: cast zx s_133_0 -> i
        let s_133_2: i128 = (i128::try_from(s_133_0).unwrap());
        // D s_133_3: read-var u#38616:i
        let s_133_3: i128 = fn_state.u_38616;
        // D s_133_4: call integer_subrange(s_133_3, s_133_2, s_133_1)
        let s_133_4: Bits = integer_subrange(state, tracer, s_133_3, s_133_2, s_133_1);
        // D s_133_5: cast reint s_133_4 -> u56
        let s_133_5: u64 = (s_133_4.value() as u64);
        // C s_133_6: const #11128u : u32
        let s_133_6: u32 = 11128;
        // N s_133_7: write-reg s_133_6 <= s_133_5
        let s_133_7: () = {
            state.write_register::<u64>(s_133_6 as isize, s_133_5);
            tracer.write_register(s_133_6 as isize, s_133_5);
        };
        // N s_133_8: return
        return;
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var merge#var.0:struct
        let s_134_0: &'static str = fn_state.merge_var._0;
        // D s_134_1: read-var merge#var.1:struct
        let s_134_1: i128 = fn_state.merge_var._1;
        // D s_134_2: write-var u#38618 <= s_134_1
        fn_state.u_38618 = s_134_1;
        // C s_134_3: const #"sgiBase" : str
        let s_134_3: &'static str = "sgiBase";
        // D s_134_4: cmp-eq s_134_0 s_134_3
        let s_134_4: bool = ((s_134_0) == (s_134_3));
        // D s_134_5: not s_134_4
        let s_134_5: bool = !s_134_4;
        // N s_134_6: branch s_134_5 b136 b135
        if s_134_5 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #55s : i64
        let s_135_0: i64 = 55;
        // C s_135_1: const #0s : i
        let s_135_1: i128 = 0;
        // C s_135_2: cast zx s_135_0 -> i
        let s_135_2: i128 = (i128::try_from(s_135_0).unwrap());
        // D s_135_3: read-var u#38618:i
        let s_135_3: i128 = fn_state.u_38618;
        // D s_135_4: call integer_subrange(s_135_3, s_135_2, s_135_1)
        let s_135_4: Bits = integer_subrange(state, tracer, s_135_3, s_135_2, s_135_1);
        // D s_135_5: cast reint s_135_4 -> u56
        let s_135_5: u64 = (s_135_4.value() as u64);
        // C s_135_6: const #21208u : u32
        let s_135_6: u32 = 21208;
        // N s_135_7: write-reg s_135_6 <= s_135_5
        let s_135_7: () = {
            state.write_register::<u64>(s_135_6 as isize, s_135_5);
            tracer.write_register(s_135_6 as isize, s_135_5);
        };
        // N s_135_8: return
        return;
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var merge#var.0:struct
        let s_136_0: &'static str = fn_state.merge_var._0;
        // D s_136_1: read-var merge#var.1:struct
        let s_136_1: i128 = fn_state.merge_var._1;
        // D s_136_2: write-var u#38620 <= s_136_1
        fn_state.u_38620 = s_136_1;
        // C s_136_3: const #"vlpiBase" : str
        let s_136_3: &'static str = "vlpiBase";
        // D s_136_4: cmp-eq s_136_0 s_136_3
        let s_136_4: bool = ((s_136_0) == (s_136_3));
        // D s_136_5: not s_136_4
        let s_136_5: bool = !s_136_4;
        // N s_136_6: branch s_136_5 b138 b137
        if s_136_5 {
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
        // C s_137_0: const #55s : i64
        let s_137_0: i64 = 55;
        // C s_137_1: const #0s : i
        let s_137_1: i128 = 0;
        // C s_137_2: cast zx s_137_0 -> i
        let s_137_2: i128 = (i128::try_from(s_137_0).unwrap());
        // D s_137_3: read-var u#38620:i
        let s_137_3: i128 = fn_state.u_38620;
        // D s_137_4: call integer_subrange(s_137_3, s_137_2, s_137_1)
        let s_137_4: Bits = integer_subrange(state, tracer, s_137_3, s_137_2, s_137_1);
        // D s_137_5: cast reint s_137_4 -> u56
        let s_137_5: u64 = (s_137_4.value() as u64);
        // C s_137_6: const #18264u : u32
        let s_137_6: u32 = 18264;
        // N s_137_7: write-reg s_137_6 <= s_137_5
        let s_137_7: () = {
            state.write_register::<u64>(s_137_6 as isize, s_137_5);
            tracer.write_register(s_137_6 as isize, s_137_5);
        };
        // N s_137_8: return
        return;
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var merge#var.0:struct
        let s_138_0: &'static str = fn_state.merge_var._0;
        // D s_138_1: read-var merge#var.1:struct
        let s_138_1: i128 = fn_state.merge_var._1;
        // D s_138_2: write-var u#38622 <= s_138_1
        fn_state.u_38622 = s_138_1;
        // C s_138_3: const #"gic_distributor.ITS0-base" : str
        let s_138_3: &'static str = "gic_distributor.ITS0-base";
        // D s_138_4: cmp-eq s_138_0 s_138_3
        let s_138_4: bool = ((s_138_0) == (s_138_3));
        // D s_138_5: not s_138_4
        let s_138_5: bool = !s_138_4;
        // N s_138_6: branch s_138_5 b140 b139
        if s_138_5 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #55s : i64
        let s_139_0: i64 = 55;
        // C s_139_1: const #0s : i
        let s_139_1: i128 = 0;
        // C s_139_2: cast zx s_139_0 -> i
        let s_139_2: i128 = (i128::try_from(s_139_0).unwrap());
        // D s_139_3: read-var u#38622:i
        let s_139_3: i128 = fn_state.u_38622;
        // D s_139_4: call integer_subrange(s_139_3, s_139_2, s_139_1)
        let s_139_4: Bits = integer_subrange(state, tracer, s_139_3, s_139_2, s_139_1);
        // D s_139_5: cast reint s_139_4 -> u56
        let s_139_5: u64 = (s_139_4.value() as u64);
        // C s_139_6: const #13472u : u32
        let s_139_6: u32 = 13472;
        // N s_139_7: write-reg s_139_6 <= s_139_5
        let s_139_7: () = {
            state.write_register::<u64>(s_139_6 as isize, s_139_5);
            tracer.write_register(s_139_6 as isize, s_139_5);
        };
        // N s_139_8: return
        return;
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var merge#var.0:struct
        let s_140_0: &'static str = fn_state.merge_var._0;
        // D s_140_1: read-var merge#var.1:struct
        let s_140_1: i128 = fn_state.merge_var._1;
        // D s_140_2: write-var u#38624 <= s_140_1
        fn_state.u_38624 = s_140_1;
        // C s_140_3: const #"pmuBase" : str
        let s_140_3: &'static str = "pmuBase";
        // D s_140_4: cmp-eq s_140_0 s_140_3
        let s_140_4: bool = ((s_140_0) == (s_140_3));
        // D s_140_5: not s_140_4
        let s_140_5: bool = !s_140_4;
        // N s_140_6: branch s_140_5 b142 b141
        if s_140_5 {
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
        // C s_141_0: const #55s : i64
        let s_141_0: i64 = 55;
        // C s_141_1: const #0s : i
        let s_141_1: i128 = 0;
        // C s_141_2: cast zx s_141_0 -> i
        let s_141_2: i128 = (i128::try_from(s_141_0).unwrap());
        // D s_141_3: read-var u#38624:i
        let s_141_3: i128 = fn_state.u_38624;
        // D s_141_4: call integer_subrange(s_141_3, s_141_2, s_141_1)
        let s_141_4: Bits = integer_subrange(state, tracer, s_141_3, s_141_2, s_141_1);
        // D s_141_5: cast reint s_141_4 -> u56
        let s_141_5: u64 = (s_141_4.value() as u64);
        // C s_141_6: const #23408u : u32
        let s_141_6: u32 = 23408;
        // N s_141_7: write-reg s_141_6 <= s_141_5
        let s_141_7: () = {
            state.write_register::<u64>(s_141_6 as isize, s_141_5);
            tracer.write_register(s_141_6 as isize, s_141_5);
        };
        // N s_141_8: return
        return;
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var merge#var.0:struct
        let s_142_0: &'static str = fn_state.merge_var._0;
        // D s_142_1: read-var merge#var.1:struct
        let s_142_1: i128 = fn_state.merge_var._1;
        // D s_142_2: write-var u#38626 <= s_142_1
        fn_state.u_38626 = s_142_1;
        // C s_142_3: const #"dbg_rom_addr" : str
        let s_142_3: &'static str = "dbg_rom_addr";
        // D s_142_4: cmp-eq s_142_0 s_142_3
        let s_142_4: bool = ((s_142_0) == (s_142_3));
        // D s_142_5: not s_142_4
        let s_142_5: bool = !s_142_4;
        // N s_142_6: branch s_142_5 b144 b143
        if s_142_5 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #55s : i64
        let s_143_0: i64 = 55;
        // C s_143_1: const #0s : i
        let s_143_1: i128 = 0;
        // C s_143_2: cast zx s_143_0 -> i
        let s_143_2: i128 = (i128::try_from(s_143_0).unwrap());
        // D s_143_3: read-var u#38626:i
        let s_143_3: i128 = fn_state.u_38626;
        // D s_143_4: call integer_subrange(s_143_3, s_143_2, s_143_1)
        let s_143_4: Bits = integer_subrange(state, tracer, s_143_3, s_143_2, s_143_1);
        // D s_143_5: cast reint s_143_4 -> u56
        let s_143_5: u64 = (s_143_4.value() as u64);
        // C s_143_6: const #1432u : u32
        let s_143_6: u32 = 1432;
        // N s_143_7: write-reg s_143_6 <= s_143_5
        let s_143_7: () = {
            state.write_register::<u64>(s_143_6 as isize, s_143_5);
            tracer.write_register(s_143_6 as isize, s_143_5);
        };
        // N s_143_8: return
        return;
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var merge#var.0:struct
        let s_144_0: &'static str = fn_state.merge_var._0;
        // D s_144_1: read-var merge#var.1:struct
        let s_144_1: i128 = fn_state.merge_var._1;
        // D s_144_2: write-var u#38628 <= s_144_1
        fn_state.u_38628 = s_144_1;
        // C s_144_3: const #"etmBase" : str
        let s_144_3: &'static str = "etmBase";
        // D s_144_4: cmp-eq s_144_0 s_144_3
        let s_144_4: bool = ((s_144_0) == (s_144_3));
        // D s_144_5: not s_144_4
        let s_144_5: bool = !s_144_4;
        // N s_144_6: branch s_144_5 b146 b145
        if s_144_5 {
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
        // C s_145_0: const #55s : i64
        let s_145_0: i64 = 55;
        // C s_145_1: const #0s : i
        let s_145_1: i128 = 0;
        // C s_145_2: cast zx s_145_0 -> i
        let s_145_2: i128 = (i128::try_from(s_145_0).unwrap());
        // D s_145_3: read-var u#38628:i
        let s_145_3: i128 = fn_state.u_38628;
        // D s_145_4: call integer_subrange(s_145_3, s_145_2, s_145_1)
        let s_145_4: Bits = integer_subrange(state, tracer, s_145_3, s_145_2, s_145_1);
        // D s_145_5: cast reint s_145_4 -> u56
        let s_145_5: u64 = (s_145_4.value() as u64);
        // C s_145_6: const #10032u : u32
        let s_145_6: u32 = 10032;
        // N s_145_7: write-reg s_145_6 <= s_145_5
        let s_145_7: () = {
            state.write_register::<u64>(s_145_6 as isize, s_145_5);
            tracer.write_register(s_145_6 as isize, s_145_5);
        };
        // N s_145_8: return
        return;
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var merge#var.0:struct
        let s_146_0: &'static str = fn_state.merge_var._0;
        // D s_146_1: read-var merge#var.1:struct
        let s_146_1: i128 = fn_state.merge_var._1;
        // D s_146_2: write-var u#38630 <= s_146_1
        fn_state.u_38630 = s_146_1;
        // C s_146_3: const #"trbeBase" : str
        let s_146_3: &'static str = "trbeBase";
        // D s_146_4: cmp-eq s_146_0 s_146_3
        let s_146_4: bool = ((s_146_0) == (s_146_3));
        // D s_146_5: not s_146_4
        let s_146_5: bool = !s_146_4;
        // N s_146_6: branch s_146_5 b148 b147
        if s_146_5 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #55s : i64
        let s_147_0: i64 = 55;
        // C s_147_1: const #0s : i
        let s_147_1: i128 = 0;
        // C s_147_2: cast zx s_147_0 -> i
        let s_147_2: i128 = (i128::try_from(s_147_0).unwrap());
        // D s_147_3: read-var u#38630:i
        let s_147_3: i128 = fn_state.u_38630;
        // D s_147_4: call integer_subrange(s_147_3, s_147_2, s_147_1)
        let s_147_4: Bits = integer_subrange(state, tracer, s_147_3, s_147_2, s_147_1);
        // N s_147_5: return
        return;
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var merge#var.0:struct
        let s_148_0: &'static str = fn_state.merge_var._0;
        // D s_148_1: read-var merge#var.1:struct
        let s_148_1: i128 = fn_state.merge_var._1;
        // D s_148_2: write-var u#38632 <= s_148_1
        fn_state.u_38632 = s_148_1;
        // C s_148_3: const #"globalcounter.base_frequency" : str
        let s_148_3: &'static str = "globalcounter.base_frequency";
        // D s_148_4: cmp-eq s_148_0 s_148_3
        let s_148_4: bool = ((s_148_0) == (s_148_3));
        // D s_148_5: not s_148_4
        let s_148_5: bool = !s_148_4;
        // N s_148_6: branch s_148_5 b150 b149
        if s_148_5 {
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
        // C s_149_0: const #31s : i
        let s_149_0: i128 = 31;
        // C s_149_1: const #0s : i
        let s_149_1: i128 = 0;
        // D s_149_2: read-var u#38632:i
        let s_149_2: i128 = fn_state.u_38632;
        // D s_149_3: call integer_subrange(s_149_2, s_149_0, s_149_1)
        let s_149_3: Bits = integer_subrange(state, tracer, s_149_2, s_149_0, s_149_1);
        // D s_149_4: cast reint s_149_3 -> u32
        let s_149_4: u32 = (s_149_3.value() as u32);
        // C s_149_5: const #14240u : u32
        let s_149_5: u32 = 14240;
        // N s_149_6: write-reg s_149_5 <= s_149_4
        let s_149_6: () = {
            state.write_register::<u32>(s_149_5 as isize, s_149_4);
            tracer.write_register(s_149_5 as isize, s_149_4);
        };
        // N s_149_7: return
        return;
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var merge#var.0:struct
        let s_150_0: &'static str = fn_state.merge_var._0;
        // D s_150_1: read-var merge#var.1:struct
        let s_150_1: i128 = fn_state.merge_var._1;
        // D s_150_2: write-var u#38634 <= s_150_1
        fn_state.u_38634 = s_150_1;
        // C s_150_3: const #"cpu.ext_abort_normal_cacheable_read_is_sync" : str
        let s_150_3: &'static str = "cpu.ext_abort_normal_cacheable_read_is_sync";
        // D s_150_4: cmp-eq s_150_0 s_150_3
        let s_150_4: bool = ((s_150_0) == (s_150_3));
        // D s_150_5: not s_150_4
        let s_150_5: bool = !s_150_4;
        // N s_150_6: branch s_150_5 b152 b151
        if s_150_5 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #1s : i
        let s_151_0: i128 = 1;
        // D s_151_1: read-var u#38634:i
        let s_151_1: i128 = fn_state.u_38634;
        // D s_151_2: cmp-eq s_151_1 s_151_0
        let s_151_2: bool = ((s_151_1) == (s_151_0));
        // C s_151_3: const #21768u : u32
        let s_151_3: u32 = 21768;
        // N s_151_4: write-reg s_151_3 <= s_151_2
        let s_151_4: () = {
            state.write_register::<bool>(s_151_3 as isize, s_151_2);
            tracer.write_register(s_151_3 as isize, s_151_2);
        };
        // N s_151_5: return
        return;
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var merge#var.0:struct
        let s_152_0: &'static str = fn_state.merge_var._0;
        // D s_152_1: read-var merge#var.1:struct
        let s_152_1: i128 = fn_state.merge_var._1;
        // D s_152_2: write-var u#38636 <= s_152_1
        fn_state.u_38636 = s_152_1;
        // C s_152_3: const #"cpu.ext_abort_normal_noncacheable_read_is_sync" : str
        let s_152_3: &'static str = "cpu.ext_abort_normal_noncacheable_read_is_sync";
        // D s_152_4: cmp-eq s_152_0 s_152_3
        let s_152_4: bool = ((s_152_0) == (s_152_3));
        // D s_152_5: not s_152_4
        let s_152_5: bool = !s_152_4;
        // N s_152_6: branch s_152_5 b154 b153
        if s_152_5 {
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
        // C s_153_0: const #1s : i
        let s_153_0: i128 = 1;
        // D s_153_1: read-var u#38636:i
        let s_153_1: i128 = fn_state.u_38636;
        // D s_153_2: cmp-eq s_153_1 s_153_0
        let s_153_2: bool = ((s_153_1) == (s_153_0));
        // C s_153_3: const #13248u : u32
        let s_153_3: u32 = 13248;
        // N s_153_4: write-reg s_153_3 <= s_153_2
        let s_153_4: () = {
            state.write_register::<bool>(s_153_3 as isize, s_153_2);
            tracer.write_register(s_153_3 as isize, s_153_2);
        };
        // N s_153_5: return
        return;
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var merge#var.0:struct
        let s_154_0: &'static str = fn_state.merge_var._0;
        // D s_154_1: read-var merge#var.1:struct
        let s_154_1: i128 = fn_state.merge_var._1;
        // D s_154_2: write-var u#38638 <= s_154_1
        fn_state.u_38638 = s_154_1;
        // C s_154_3: const #"cpu.ext_abort_device_read_is_sync" : str
        let s_154_3: &'static str = "cpu.ext_abort_device_read_is_sync";
        // D s_154_4: cmp-eq s_154_0 s_154_3
        let s_154_4: bool = ((s_154_0) == (s_154_3));
        // D s_154_5: not s_154_4
        let s_154_5: bool = !s_154_4;
        // N s_154_6: branch s_154_5 b156 b155
        if s_154_5 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #1s : i
        let s_155_0: i128 = 1;
        // D s_155_1: read-var u#38638:i
        let s_155_1: i128 = fn_state.u_38638;
        // D s_155_2: cmp-eq s_155_1 s_155_0
        let s_155_2: bool = ((s_155_1) == (s_155_0));
        // C s_155_3: const #90280u : u32
        let s_155_3: u32 = 90280;
        // N s_155_4: write-reg s_155_3 <= s_155_2
        let s_155_4: () = {
            state.write_register::<bool>(s_155_3 as isize, s_155_2);
            tracer.write_register(s_155_3 as isize, s_155_2);
        };
        // N s_155_5: return
        return;
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var merge#var.0:struct
        let s_156_0: &'static str = fn_state.merge_var._0;
        // D s_156_1: read-var merge#var.1:struct
        let s_156_1: i128 = fn_state.merge_var._1;
        // D s_156_2: write-var u#38640 <= s_156_1
        fn_state.u_38640 = s_156_1;
        // C s_156_3: const #"cpu.ext_abort_so_read_is_sync" : str
        let s_156_3: &'static str = "cpu.ext_abort_so_read_is_sync";
        // D s_156_4: cmp-eq s_156_0 s_156_3
        let s_156_4: bool = ((s_156_0) == (s_156_3));
        // D s_156_5: not s_156_4
        let s_156_5: bool = !s_156_4;
        // N s_156_6: branch s_156_5 b158 b157
        if s_156_5 {
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
        // C s_157_0: const #1s : i
        let s_157_0: i128 = 1;
        // D s_157_1: read-var u#38640:i
        let s_157_1: i128 = fn_state.u_38640;
        // D s_157_2: cmp-eq s_157_1 s_157_0
        let s_157_2: bool = ((s_157_1) == (s_157_0));
        // C s_157_3: const #20136u : u32
        let s_157_3: u32 = 20136;
        // N s_157_4: write-reg s_157_3 <= s_157_2
        let s_157_4: () = {
            state.write_register::<bool>(s_157_3 as isize, s_157_2);
            tracer.write_register(s_157_3 as isize, s_157_2);
        };
        // N s_157_5: return
        return;
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var merge#var.0:struct
        let s_158_0: &'static str = fn_state.merge_var._0;
        // D s_158_1: read-var merge#var.1:struct
        let s_158_1: i128 = fn_state.merge_var._1;
        // D s_158_2: write-var u#38642 <= s_158_1
        fn_state.u_38642 = s_158_1;
        // C s_158_3: const #"cpu.ext_abort_so_write_is_sync" : str
        let s_158_3: &'static str = "cpu.ext_abort_so_write_is_sync";
        // D s_158_4: cmp-eq s_158_0 s_158_3
        let s_158_4: bool = ((s_158_0) == (s_158_3));
        // D s_158_5: not s_158_4
        let s_158_5: bool = !s_158_4;
        // N s_158_6: branch s_158_5 b160 b159
        if s_158_5 {
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
        // C s_159_0: const #1s : i
        let s_159_0: i128 = 1;
        // D s_159_1: read-var u#38642:i
        let s_159_1: i128 = fn_state.u_38642;
        // D s_159_2: cmp-eq s_159_1 s_159_0
        let s_159_2: bool = ((s_159_1) == (s_159_0));
        // C s_159_3: const #23936u : u32
        let s_159_3: u32 = 23936;
        // N s_159_4: write-reg s_159_3 <= s_159_2
        let s_159_4: () = {
            state.write_register::<bool>(s_159_3 as isize, s_159_2);
            tracer.write_register(s_159_3 as isize, s_159_2);
        };
        // N s_159_5: return
        return;
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var merge#var.0:struct
        let s_160_0: &'static str = fn_state.merge_var._0;
        // D s_160_1: read-var merge#var.1:struct
        let s_160_1: i128 = fn_state.merge_var._1;
        // D s_160_2: write-var u#38644 <= s_160_1
        fn_state.u_38644 = s_160_1;
        // C s_160_3: const #"cpu.ext_abort_prefetch_is_sync" : str
        let s_160_3: &'static str = "cpu.ext_abort_prefetch_is_sync";
        // D s_160_4: cmp-eq s_160_0 s_160_3
        let s_160_4: bool = ((s_160_0) == (s_160_3));
        // D s_160_5: not s_160_4
        let s_160_5: bool = !s_160_4;
        // N s_160_6: branch s_160_5 b162 b161
        if s_160_5 {
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
        // C s_161_0: const #1s : i
        let s_161_0: i128 = 1;
        // D s_161_1: read-var u#38644:i
        let s_161_1: i128 = fn_state.u_38644;
        // D s_161_2: cmp-eq s_161_1 s_161_0
        let s_161_2: bool = ((s_161_1) == (s_161_0));
        // C s_161_3: const #15320u : u32
        let s_161_3: u32 = 15320;
        // N s_161_4: write-reg s_161_3 <= s_161_2
        let s_161_4: () = {
            state.write_register::<bool>(s_161_3 as isize, s_161_2);
            tracer.write_register(s_161_3 as isize, s_161_2);
        };
        // N s_161_5: return
        return;
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var merge#var.0:struct
        let s_162_0: &'static str = fn_state.merge_var._0;
        // D s_162_1: read-var merge#var.1:struct
        let s_162_1: i128 = fn_state.merge_var._1;
        // D s_162_2: write-var u#38646 <= s_162_1
        fn_state.u_38646 = s_162_1;
        // C s_162_3: const #"cpu.ext_abort_ttw_cacheable_read_is_sync" : str
        let s_162_3: &'static str = "cpu.ext_abort_ttw_cacheable_read_is_sync";
        // D s_162_4: cmp-eq s_162_0 s_162_3
        let s_162_4: bool = ((s_162_0) == (s_162_3));
        // D s_162_5: not s_162_4
        let s_162_5: bool = !s_162_4;
        // N s_162_6: branch s_162_5 b164 b163
        if s_162_5 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #1s : i
        let s_163_0: i128 = 1;
        // D s_163_1: read-var u#38646:i
        let s_163_1: i128 = fn_state.u_38646;
        // D s_163_2: cmp-eq s_163_1 s_163_0
        let s_163_2: bool = ((s_163_1) == (s_163_0));
        // C s_163_3: const #19952u : u32
        let s_163_3: u32 = 19952;
        // N s_163_4: write-reg s_163_3 <= s_163_2
        let s_163_4: () = {
            state.write_register::<bool>(s_163_3 as isize, s_163_2);
            tracer.write_register(s_163_3 as isize, s_163_2);
        };
        // N s_163_5: return
        return;
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var merge#var.0:struct
        let s_164_0: &'static str = fn_state.merge_var._0;
        // D s_164_1: read-var merge#var.1:struct
        let s_164_1: i128 = fn_state.merge_var._1;
        // D s_164_2: write-var u#38648 <= s_164_1
        fn_state.u_38648 = s_164_1;
        // C s_164_3: const #"cpu.ext_abort_ttw_noncacheable_read_is_sync" : str
        let s_164_3: &'static str = "cpu.ext_abort_ttw_noncacheable_read_is_sync";
        // D s_164_4: cmp-eq s_164_0 s_164_3
        let s_164_4: bool = ((s_164_0) == (s_164_3));
        // D s_164_5: not s_164_4
        let s_164_5: bool = !s_164_4;
        // N s_164_6: branch s_164_5 b166 b165
        if s_164_5 {
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
        // C s_165_0: const #1s : i
        let s_165_0: i128 = 1;
        // D s_165_1: read-var u#38648:i
        let s_165_1: i128 = fn_state.u_38648;
        // D s_165_2: cmp-eq s_165_1 s_165_0
        let s_165_2: bool = ((s_165_1) == (s_165_0));
        // C s_165_3: const #13240u : u32
        let s_165_3: u32 = 13240;
        // N s_165_4: write-reg s_165_3 <= s_165_2
        let s_165_4: () = {
            state.write_register::<bool>(s_165_3 as isize, s_165_2);
            tracer.write_register(s_165_3 as isize, s_165_2);
        };
        // N s_165_5: return
        return;
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var merge#var.0:struct
        let s_166_0: &'static str = fn_state.merge_var._0;
        // D s_166_1: read-var merge#var.1:struct
        let s_166_1: i128 = fn_state.merge_var._1;
        // D s_166_2: write-var u#38650 <= s_166_1
        fn_state.u_38650 = s_166_1;
        // C s_166_3: const #"cpu.ext_abort_normal_cacheable_write_is_sync" : str
        let s_166_3: &'static str = "cpu.ext_abort_normal_cacheable_write_is_sync";
        // D s_166_4: cmp-eq s_166_0 s_166_3
        let s_166_4: bool = ((s_166_0) == (s_166_3));
        // D s_166_5: not s_166_4
        let s_166_5: bool = !s_166_4;
        // N s_166_6: branch s_166_5 b168 b167
        if s_166_5 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #1s : i
        let s_167_0: i128 = 1;
        // D s_167_1: read-var u#38650:i
        let s_167_1: i128 = fn_state.u_38650;
        // D s_167_2: cmp-eq s_167_1 s_167_0
        let s_167_2: bool = ((s_167_1) == (s_167_0));
        // C s_167_3: const #20320u : u32
        let s_167_3: u32 = 20320;
        // N s_167_4: write-reg s_167_3 <= s_167_2
        let s_167_4: () = {
            state.write_register::<bool>(s_167_3 as isize, s_167_2);
            tracer.write_register(s_167_3 as isize, s_167_2);
        };
        // N s_167_5: return
        return;
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var merge#var.0:struct
        let s_168_0: &'static str = fn_state.merge_var._0;
        // D s_168_1: read-var merge#var.1:struct
        let s_168_1: i128 = fn_state.merge_var._1;
        // D s_168_2: write-var u#38652 <= s_168_1
        fn_state.u_38652 = s_168_1;
        // C s_168_3: const #"cpu.ext_abort_normal_noncacheable_write_is_sync" : str
        let s_168_3: &'static str = "cpu.ext_abort_normal_noncacheable_write_is_sync";
        // D s_168_4: cmp-eq s_168_0 s_168_3
        let s_168_4: bool = ((s_168_0) == (s_168_3));
        // D s_168_5: not s_168_4
        let s_168_5: bool = !s_168_4;
        // N s_168_6: branch s_168_5 b170 b169
        if s_168_5 {
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
        // C s_169_0: const #1s : i
        let s_169_0: i128 = 1;
        // D s_169_1: read-var u#38652:i
        let s_169_1: i128 = fn_state.u_38652;
        // D s_169_2: cmp-eq s_169_1 s_169_0
        let s_169_2: bool = ((s_169_1) == (s_169_0));
        // C s_169_3: const #104896u : u32
        let s_169_3: u32 = 104896;
        // N s_169_4: write-reg s_169_3 <= s_169_2
        let s_169_4: () = {
            state.write_register::<bool>(s_169_3 as isize, s_169_2);
            tracer.write_register(s_169_3 as isize, s_169_2);
        };
        // N s_169_5: return
        return;
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var merge#var.0:struct
        let s_170_0: &'static str = fn_state.merge_var._0;
        // D s_170_1: read-var merge#var.1:struct
        let s_170_1: i128 = fn_state.merge_var._1;
        // D s_170_2: write-var u#38654 <= s_170_1
        fn_state.u_38654 = s_170_1;
        // C s_170_3: const #"cpu.ext_abort_device_write_is_sync" : str
        let s_170_3: &'static str = "cpu.ext_abort_device_write_is_sync";
        // D s_170_4: cmp-eq s_170_0 s_170_3
        let s_170_4: bool = ((s_170_0) == (s_170_3));
        // D s_170_5: not s_170_4
        let s_170_5: bool = !s_170_4;
        // N s_170_6: branch s_170_5 b172 b171
        if s_170_5 {
            return block_172(state, tracer, fn_state);
        } else {
            return block_171(state, tracer, fn_state);
        };
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #1s : i
        let s_171_0: i128 = 1;
        // D s_171_1: read-var u#38654:i
        let s_171_1: i128 = fn_state.u_38654;
        // D s_171_2: cmp-eq s_171_1 s_171_0
        let s_171_2: bool = ((s_171_1) == (s_171_0));
        // C s_171_3: const #21824u : u32
        let s_171_3: u32 = 21824;
        // N s_171_4: write-reg s_171_3 <= s_171_2
        let s_171_4: () = {
            state.write_register::<bool>(s_171_3 as isize, s_171_2);
            tracer.write_register(s_171_3 as isize, s_171_2);
        };
        // N s_171_5: return
        return;
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var merge#var.0:struct
        let s_172_0: &'static str = fn_state.merge_var._0;
        // D s_172_1: read-var merge#var.1:struct
        let s_172_1: i128 = fn_state.merge_var._1;
        // D s_172_2: write-var u#38656 <= s_172_1
        fn_state.u_38656 = s_172_1;
        // C s_172_3: const #"cpu.has_mpam" : str
        let s_172_3: &'static str = "cpu.has_mpam";
        // D s_172_4: cmp-eq s_172_0 s_172_3
        let s_172_4: bool = ((s_172_0) == (s_172_3));
        // D s_172_5: not s_172_4
        let s_172_5: bool = !s_172_4;
        // N s_172_6: branch s_172_5 b174 b173
        if s_172_5 {
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
        // C s_173_0: const #1s : i
        let s_173_0: i128 = 1;
        // D s_173_1: read-var u#38656:i
        let s_173_1: i128 = fn_state.u_38656;
        // D s_173_2: cmp-eq s_173_1 s_173_0
        let s_173_2: bool = ((s_173_1) == (s_173_0));
        // C s_173_3: const #103224u : u32
        let s_173_3: u32 = 103224;
        // N s_173_4: write-reg s_173_3 <= s_173_2
        let s_173_4: () = {
            state.write_register::<bool>(s_173_3 as isize, s_173_2);
            tracer.write_register(s_173_3 as isize, s_173_2);
        };
        // N s_173_5: return
        return;
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var merge#var.0:struct
        let s_174_0: &'static str = fn_state.merge_var._0;
        // D s_174_1: read-var merge#var.1:struct
        let s_174_1: i128 = fn_state.merge_var._1;
        // D s_174_2: write-var u#38658 <= s_174_1
        fn_state.u_38658 = s_174_1;
        // C s_174_3: const #"cpu.mpam_has_hcr" : str
        let s_174_3: &'static str = "cpu.mpam_has_hcr";
        // D s_174_4: cmp-eq s_174_0 s_174_3
        let s_174_4: bool = ((s_174_0) == (s_174_3));
        // D s_174_5: not s_174_4
        let s_174_5: bool = !s_174_4;
        // N s_174_6: branch s_174_5 b176 b175
        if s_174_5 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_175(state, tracer, fn_state);
        };
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #1s : i
        let s_175_0: i128 = 1;
        // D s_175_1: read-var u#38658:i
        let s_175_1: i128 = fn_state.u_38658;
        // D s_175_2: cmp-eq s_175_1 s_175_0
        let s_175_2: bool = ((s_175_1) == (s_175_0));
        // C s_175_3: const #90392u : u32
        let s_175_3: u32 = 90392;
        // N s_175_4: write-reg s_175_3 <= s_175_2
        let s_175_4: () = {
            state.write_register::<bool>(s_175_3 as isize, s_175_2);
            tracer.write_register(s_175_3 as isize, s_175_2);
        };
        // N s_175_5: return
        return;
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var merge#var.0:struct
        let s_176_0: &'static str = fn_state.merge_var._0;
        // D s_176_1: read-var merge#var.1:struct
        let s_176_1: i128 = fn_state.merge_var._1;
        // D s_176_2: write-var u#38660 <= s_176_1
        fn_state.u_38660 = s_176_1;
        // C s_176_3: const #"cpu.mpam_max_partid" : str
        let s_176_3: &'static str = "cpu.mpam_max_partid";
        // D s_176_4: cmp-eq s_176_0 s_176_3
        let s_176_4: bool = ((s_176_0) == (s_176_3));
        // D s_176_5: not s_176_4
        let s_176_5: bool = !s_176_4;
        // N s_176_6: branch s_176_5 b178 b177
        if s_176_5 {
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
        // C s_177_0: const #15s : i
        let s_177_0: i128 = 15;
        // C s_177_1: const #0s : i
        let s_177_1: i128 = 0;
        // D s_177_2: read-var u#38660:i
        let s_177_2: i128 = fn_state.u_38660;
        // D s_177_3: call integer_subrange(s_177_2, s_177_0, s_177_1)
        let s_177_3: Bits = integer_subrange(state, tracer, s_177_2, s_177_0, s_177_1);
        // D s_177_4: cast reint s_177_3 -> u16
        let s_177_4: u16 = (s_177_3.value() as u16);
        // C s_177_5: const #21808u : u32
        let s_177_5: u32 = 21808;
        // N s_177_6: write-reg s_177_5 <= s_177_4
        let s_177_6: () = {
            state.write_register::<u16>(s_177_5 as isize, s_177_4);
            tracer.write_register(s_177_5 as isize, s_177_4);
        };
        // N s_177_7: return
        return;
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var merge#var.0:struct
        let s_178_0: &'static str = fn_state.merge_var._0;
        // D s_178_1: read-var merge#var.1:struct
        let s_178_1: i128 = fn_state.merge_var._1;
        // D s_178_2: write-var u#38662 <= s_178_1
        fn_state.u_38662 = s_178_1;
        // C s_178_3: const #"cpu.mpam_max_pmg" : str
        let s_178_3: &'static str = "cpu.mpam_max_pmg";
        // D s_178_4: cmp-eq s_178_0 s_178_3
        let s_178_4: bool = ((s_178_0) == (s_178_3));
        // D s_178_5: not s_178_4
        let s_178_5: bool = !s_178_4;
        // N s_178_6: branch s_178_5 b180 b179
        if s_178_5 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_179(state, tracer, fn_state);
        };
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #7s : i
        let s_179_0: i128 = 7;
        // C s_179_1: const #0s : i
        let s_179_1: i128 = 0;
        // D s_179_2: read-var u#38662:i
        let s_179_2: i128 = fn_state.u_38662;
        // D s_179_3: call integer_subrange(s_179_2, s_179_0, s_179_1)
        let s_179_3: Bits = integer_subrange(state, tracer, s_179_2, s_179_0, s_179_1);
        // D s_179_4: cast reint s_179_3 -> u8
        let s_179_4: u8 = (s_179_3.value() as u8);
        // C s_179_5: const #13392u : u32
        let s_179_5: u32 = 13392;
        // N s_179_6: write-reg s_179_5 <= s_179_4
        let s_179_6: () = {
            state.write_register::<u8>(s_179_5 as isize, s_179_4);
            tracer.write_register(s_179_5 as isize, s_179_4);
        };
        // N s_179_7: return
        return;
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var merge#var.0:struct
        let s_180_0: &'static str = fn_state.merge_var._0;
        // D s_180_1: read-var merge#var.1:struct
        let s_180_1: i128 = fn_state.merge_var._1;
        // D s_180_2: write-var u#38664 <= s_180_1
        fn_state.u_38664 = s_180_1;
        // C s_180_3: const #"cpu.mpam_max_vpmr" : str
        let s_180_3: &'static str = "cpu.mpam_max_vpmr";
        // D s_180_4: cmp-eq s_180_0 s_180_3
        let s_180_4: bool = ((s_180_0) == (s_180_3));
        // D s_180_5: not s_180_4
        let s_180_5: bool = !s_180_4;
        // N s_180_6: branch s_180_5 b182 b181
        if s_180_5 {
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
        // C s_181_0: const #2s : i
        let s_181_0: i128 = 2;
        // C s_181_1: const #0s : i
        let s_181_1: i128 = 0;
        // D s_181_2: read-var u#38664:i
        let s_181_2: i128 = fn_state.u_38664;
        // D s_181_3: call integer_subrange(s_181_2, s_181_0, s_181_1)
        let s_181_3: Bits = integer_subrange(state, tracer, s_181_2, s_181_0, s_181_1);
        // D s_181_4: cast reint s_181_3 -> u8
        let s_181_4: u8 = (s_181_3.value() as u8);
        // C s_181_5: const #21192u : u32
        let s_181_5: u32 = 21192;
        // N s_181_6: write-reg s_181_5 <= s_181_4
        let s_181_6: () = {
            state.write_register::<u8>(s_181_5 as isize, s_181_4);
            tracer.write_register(s_181_5 as isize, s_181_4);
        };
        // N s_181_7: return
        return;
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var merge#var.0:struct
        let s_182_0: &'static str = fn_state.merge_var._0;
        // D s_182_1: read-var merge#var.1:struct
        let s_182_1: i128 = fn_state.merge_var._1;
        // D s_182_2: write-var u#38666 <= s_182_1
        fn_state.u_38666 = s_182_1;
        // C s_182_3: const #"cpu.mpam_has_altsp" : str
        let s_182_3: &'static str = "cpu.mpam_has_altsp";
        // D s_182_4: cmp-eq s_182_0 s_182_3
        let s_182_4: bool = ((s_182_0) == (s_182_3));
        // D s_182_5: not s_182_4
        let s_182_5: bool = !s_182_4;
        // N s_182_6: branch s_182_5 b184 b183
        if s_182_5 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #1s : i
        let s_183_0: i128 = 1;
        // D s_183_1: read-var u#38666:i
        let s_183_1: i128 = fn_state.u_38666;
        // D s_183_2: cmp-eq s_183_1 s_183_0
        let s_183_2: bool = ((s_183_1) == (s_183_0));
        // C s_183_3: const #21144u : u32
        let s_183_3: u32 = 21144;
        // N s_183_4: write-reg s_183_3 <= s_183_2
        let s_183_4: () = {
            state.write_register::<bool>(s_183_3 as isize, s_183_2);
            tracer.write_register(s_183_3 as isize, s_183_2);
        };
        // N s_183_5: return
        return;
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var merge#var.0:struct
        let s_184_0: &'static str = fn_state.merge_var._0;
        // D s_184_1: read-var merge#var.1:struct
        let s_184_1: i128 = fn_state.merge_var._1;
        // D s_184_2: write-var u#38668 <= s_184_1
        fn_state.u_38668 = s_184_1;
        // C s_184_3: const #"cpu.mpamidr_has_tidr" : str
        let s_184_3: &'static str = "cpu.mpamidr_has_tidr";
        // D s_184_4: cmp-eq s_184_0 s_184_3
        let s_184_4: bool = ((s_184_0) == (s_184_3));
        // D s_184_5: not s_184_4
        let s_184_5: bool = !s_184_4;
        // N s_184_6: branch s_184_5 b186 b185
        if s_184_5 {
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
        // C s_185_0: const #0s : i
        let s_185_0: i128 = 0;
        // D s_185_1: read-var u#38668:i
        let s_185_1: i128 = fn_state.u_38668;
        // D s_185_2: cmp-gt s_185_1 s_185_0
        let s_185_2: bool = ((s_185_1) > (s_185_0));
        // C s_185_3: const #15224u : u32
        let s_185_3: u32 = 15224;
        // N s_185_4: write-reg s_185_3 <= s_185_2
        let s_185_4: () = {
            state.write_register::<bool>(s_185_3 as isize, s_185_2);
            tracer.write_register(s_185_3 as isize, s_185_2);
        };
        // N s_185_5: return
        return;
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var merge#var.0:struct
        let s_186_0: &'static str = fn_state.merge_var._0;
        // D s_186_1: read-var merge#var.1:struct
        let s_186_1: i128 = fn_state.merge_var._1;
        // D s_186_2: write-var u#38670 <= s_186_1
        fn_state.u_38670 = s_186_1;
        // C s_186_3: const #"cpu.mpamidr_has_sdeflt" : str
        let s_186_3: &'static str = "cpu.mpamidr_has_sdeflt";
        // D s_186_4: cmp-eq s_186_0 s_186_3
        let s_186_4: bool = ((s_186_0) == (s_186_3));
        // D s_186_5: not s_186_4
        let s_186_5: bool = !s_186_4;
        // N s_186_6: branch s_186_5 b188 b187
        if s_186_5 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #0s : i
        let s_187_0: i128 = 0;
        // D s_187_1: read-var u#38670:i
        let s_187_1: i128 = fn_state.u_38670;
        // D s_187_2: cmp-gt s_187_1 s_187_0
        let s_187_2: bool = ((s_187_1) > (s_187_0));
        // C s_187_3: const #21056u : u32
        let s_187_3: u32 = 21056;
        // N s_187_4: write-reg s_187_3 <= s_187_2
        let s_187_4: () = {
            state.write_register::<bool>(s_187_3 as isize, s_187_2);
            tracer.write_register(s_187_3 as isize, s_187_2);
        };
        // N s_187_5: return
        return;
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var merge#var.0:struct
        let s_188_0: &'static str = fn_state.merge_var._0;
        // D s_188_1: read-var merge#var.1:struct
        let s_188_1: i128 = fn_state.merge_var._1;
        // D s_188_2: write-var u#38672 <= s_188_1
        fn_state.u_38672 = s_188_1;
        // C s_188_3: const #"cpu.mpamidr_has_force_ns" : str
        let s_188_3: &'static str = "cpu.mpamidr_has_force_ns";
        // D s_188_4: cmp-eq s_188_0 s_188_3
        let s_188_4: bool = ((s_188_0) == (s_188_3));
        // D s_188_5: not s_188_4
        let s_188_5: bool = !s_188_4;
        // N s_188_6: branch s_188_5 b190 b189
        if s_188_5 {
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
        // C s_189_0: const #0s : i
        let s_189_0: i128 = 0;
        // D s_189_1: read-var u#38672:i
        let s_189_1: i128 = fn_state.u_38672;
        // D s_189_2: cmp-gt s_189_1 s_189_0
        let s_189_2: bool = ((s_189_1) > (s_189_0));
        // C s_189_3: const #91056u : u32
        let s_189_3: u32 = 91056;
        // N s_189_4: write-reg s_189_3 <= s_189_2
        let s_189_4: () = {
            state.write_register::<bool>(s_189_3 as isize, s_189_2);
            tracer.write_register(s_189_3 as isize, s_189_2);
        };
        // N s_189_5: return
        return;
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var merge#var.0:struct
        let s_190_0: &'static str = fn_state.merge_var._0;
        // D s_190_1: read-var merge#var.1:struct
        let s_190_1: i128 = fn_state.merge_var._1;
        // D s_190_2: write-var u#38674 <= s_190_1
        fn_state.u_38674 = s_190_1;
        // C s_190_3: const #"cpu.mpam_force_ns_rao" : str
        let s_190_3: &'static str = "cpu.mpam_force_ns_rao";
        // D s_190_4: cmp-eq s_190_0 s_190_3
        let s_190_4: bool = ((s_190_0) == (s_190_3));
        // D s_190_5: not s_190_4
        let s_190_5: bool = !s_190_4;
        // N s_190_6: branch s_190_5 b192 b191
        if s_190_5 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_191(state, tracer, fn_state);
        };
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #0s : i
        let s_191_0: i128 = 0;
        // D s_191_1: read-var u#38674:i
        let s_191_1: i128 = fn_state.u_38674;
        // D s_191_2: cmp-gt s_191_1 s_191_0
        let s_191_2: bool = ((s_191_1) > (s_191_0));
        // C s_191_3: const #90400u : u32
        let s_191_3: u32 = 90400;
        // N s_191_4: write-reg s_191_3 <= s_191_2
        let s_191_4: () = {
            state.write_register::<bool>(s_191_3 as isize, s_191_2);
            tracer.write_register(s_191_3 as isize, s_191_2);
        };
        // N s_191_5: return
        return;
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_192_0: read-var merge#var.0:struct
        let s_192_0: &'static str = fn_state.merge_var._0;
        // D s_192_1: read-var merge#var.1:struct
        let s_192_1: i128 = fn_state.merge_var._1;
        // D s_192_2: write-var u#38676 <= s_192_1
        fn_state.u_38676 = s_192_1;
        // C s_192_3: const #"cpu.mpam_frac" : str
        let s_192_3: &'static str = "cpu.mpam_frac";
        // D s_192_4: cmp-eq s_192_0 s_192_3
        let s_192_4: bool = ((s_192_0) == (s_192_3));
        // D s_192_5: not s_192_4
        let s_192_5: bool = !s_192_4;
        // N s_192_6: branch s_192_5 b194 b193
        if s_192_5 {
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
        // C s_193_0: const #3s : i
        let s_193_0: i128 = 3;
        // C s_193_1: const #0s : i
        let s_193_1: i128 = 0;
        // D s_193_2: read-var u#38676:i
        let s_193_2: i128 = fn_state.u_38676;
        // D s_193_3: call integer_subrange(s_193_2, s_193_0, s_193_1)
        let s_193_3: Bits = integer_subrange(state, tracer, s_193_2, s_193_0, s_193_1);
        // D s_193_4: cast reint s_193_3 -> u8
        let s_193_4: u8 = (s_193_3.value() as u8);
        // C s_193_5: const #14256u : u32
        let s_193_5: u32 = 14256;
        // N s_193_6: write-reg s_193_5 <= s_193_4
        let s_193_6: () = {
            state.write_register::<u8>(s_193_5 as isize, s_193_4);
            tracer.write_register(s_193_5 as isize, s_193_4);
        };
        // N s_193_7: return
        return;
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var merge#var.0:struct
        let s_194_0: &'static str = fn_state.merge_var._0;
        // D s_194_1: read-var merge#var.1:struct
        let s_194_1: i128 = fn_state.merge_var._1;
        // D s_194_2: write-var u#38678 <= s_194_1
        fn_state.u_38678 = s_194_1;
        // C s_194_3: const #"cpu.CCSIDR-L1I_override" : str
        let s_194_3: &'static str = "cpu.CCSIDR-L1I_override";
        // D s_194_4: cmp-eq s_194_0 s_194_3
        let s_194_4: bool = ((s_194_0) == (s_194_3));
        // D s_194_5: not s_194_4
        let s_194_5: bool = !s_194_4;
        // N s_194_6: branch s_194_5 b196 b195
        if s_194_5 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_195(state, tracer, fn_state);
        };
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #63s : i
        let s_195_0: i128 = 63;
        // C s_195_1: const #0s : i
        let s_195_1: i128 = 0;
        // D s_195_2: read-var u#38678:i
        let s_195_2: i128 = fn_state.u_38678;
        // D s_195_3: call integer_subrange(s_195_2, s_195_0, s_195_1)
        let s_195_3: Bits = integer_subrange(state, tracer, s_195_2, s_195_0, s_195_1);
        // D s_195_4: cast reint s_195_3 -> u64
        let s_195_4: u64 = (s_195_3.value() as u64);
        // C s_195_5: const #0s : i
        let s_195_5: i128 = 0;
        // C s_195_6: const #18328u : u32
        let s_195_6: u32 = 18328;
        // D s_195_7: read-reg s_195_6:[u64; 7]
        let s_195_7: [u64; 7usize] = {
            let value = state.read_register::<[u64; 7usize]>(s_195_6 as isize);
            tracer.read_register(s_195_6 as isize, value);
            value
        };
        // D s_195_8: mutate-element s_195_7[s_195_5] <= s_195_4
        let s_195_8: [u64; 7usize] = {
            let mut local = s_195_7.clone();
            local[(s_195_5) as usize] = s_195_4;
            local
        };
        // D s_195_9: cast cvt s_195_8 -> [u64; 0]
        let s_195_9: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_195_8);
        // D s_195_10: cast cvt s_195_9 -> [u64; 7]
        let s_195_10: [u64; 7usize] = {
            let mut buf = [Default::default(); 7usize];
            buf.copy_from_slice(&s_195_9);
            buf
        };
        // C s_195_11: const #18328u : u32
        let s_195_11: u32 = 18328;
        // N s_195_12: write-reg s_195_11 <= s_195_10
        let s_195_12: () = {
            state.write_register::<[u64; 7usize]>(s_195_11 as isize, s_195_10);
            tracer.write_register(s_195_11 as isize, s_195_10);
        };
        // N s_195_13: return
        return;
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_196_0: read-var merge#var.0:struct
        let s_196_0: &'static str = fn_state.merge_var._0;
        // D s_196_1: read-var merge#var.1:struct
        let s_196_1: i128 = fn_state.merge_var._1;
        // D s_196_2: write-var u#38680 <= s_196_1
        fn_state.u_38680 = s_196_1;
        // C s_196_3: const #"cpu.CCSIDR-L1D_override" : str
        let s_196_3: &'static str = "cpu.CCSIDR-L1D_override";
        // D s_196_4: cmp-eq s_196_0 s_196_3
        let s_196_4: bool = ((s_196_0) == (s_196_3));
        // D s_196_5: not s_196_4
        let s_196_5: bool = !s_196_4;
        // N s_196_6: branch s_196_5 b198 b197
        if s_196_5 {
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
        // C s_197_0: const #63s : i
        let s_197_0: i128 = 63;
        // C s_197_1: const #0s : i
        let s_197_1: i128 = 0;
        // D s_197_2: read-var u#38680:i
        let s_197_2: i128 = fn_state.u_38680;
        // D s_197_3: call integer_subrange(s_197_2, s_197_0, s_197_1)
        let s_197_3: Bits = integer_subrange(state, tracer, s_197_2, s_197_0, s_197_1);
        // D s_197_4: cast reint s_197_3 -> u64
        let s_197_4: u64 = (s_197_3.value() as u64);
        // C s_197_5: const #0s : i
        let s_197_5: i128 = 0;
        // C s_197_6: const #102224u : u32
        let s_197_6: u32 = 102224;
        // D s_197_7: read-reg s_197_6:[u64; 7]
        let s_197_7: [u64; 7usize] = {
            let value = state.read_register::<[u64; 7usize]>(s_197_6 as isize);
            tracer.read_register(s_197_6 as isize, value);
            value
        };
        // D s_197_8: mutate-element s_197_7[s_197_5] <= s_197_4
        let s_197_8: [u64; 7usize] = {
            let mut local = s_197_7.clone();
            local[(s_197_5) as usize] = s_197_4;
            local
        };
        // D s_197_9: cast cvt s_197_8 -> [u64; 0]
        let s_197_9: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_197_8);
        // D s_197_10: cast cvt s_197_9 -> [u64; 7]
        let s_197_10: [u64; 7usize] = {
            let mut buf = [Default::default(); 7usize];
            buf.copy_from_slice(&s_197_9);
            buf
        };
        // C s_197_11: const #102224u : u32
        let s_197_11: u32 = 102224;
        // N s_197_12: write-reg s_197_11 <= s_197_10
        let s_197_12: () = {
            state.write_register::<[u64; 7usize]>(s_197_11 as isize, s_197_10);
            tracer.write_register(s_197_11 as isize, s_197_10);
        };
        // N s_197_13: return
        return;
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_198_0: read-var merge#var.0:struct
        let s_198_0: &'static str = fn_state.merge_var._0;
        // D s_198_1: read-var merge#var.1:struct
        let s_198_1: i128 = fn_state.merge_var._1;
        // D s_198_2: write-var u#38682 <= s_198_1
        fn_state.u_38682 = s_198_1;
        // C s_198_3: const #"cpu.CCSIDR-L2_override" : str
        let s_198_3: &'static str = "cpu.CCSIDR-L2_override";
        // D s_198_4: cmp-eq s_198_0 s_198_3
        let s_198_4: bool = ((s_198_0) == (s_198_3));
        // D s_198_5: not s_198_4
        let s_198_5: bool = !s_198_4;
        // N s_198_6: branch s_198_5 b200 b199
        if s_198_5 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_199(state, tracer, fn_state);
        };
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #63s : i
        let s_199_0: i128 = 63;
        // C s_199_1: const #0s : i
        let s_199_1: i128 = 0;
        // D s_199_2: read-var u#38682:i
        let s_199_2: i128 = fn_state.u_38682;
        // D s_199_3: call integer_subrange(s_199_2, s_199_0, s_199_1)
        let s_199_3: Bits = integer_subrange(state, tracer, s_199_2, s_199_0, s_199_1);
        // D s_199_4: cast reint s_199_3 -> u64
        let s_199_4: u64 = (s_199_3.value() as u64);
        // C s_199_5: const #1s : i
        let s_199_5: i128 = 1;
        // C s_199_6: const #102224u : u32
        let s_199_6: u32 = 102224;
        // D s_199_7: read-reg s_199_6:[u64; 7]
        let s_199_7: [u64; 7usize] = {
            let value = state.read_register::<[u64; 7usize]>(s_199_6 as isize);
            tracer.read_register(s_199_6 as isize, value);
            value
        };
        // D s_199_8: mutate-element s_199_7[s_199_5] <= s_199_4
        let s_199_8: [u64; 7usize] = {
            let mut local = s_199_7.clone();
            local[(s_199_5) as usize] = s_199_4;
            local
        };
        // D s_199_9: cast cvt s_199_8 -> [u64; 0]
        let s_199_9: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_199_8);
        // D s_199_10: cast cvt s_199_9 -> [u64; 7]
        let s_199_10: [u64; 7usize] = {
            let mut buf = [Default::default(); 7usize];
            buf.copy_from_slice(&s_199_9);
            buf
        };
        // C s_199_11: const #102224u : u32
        let s_199_11: u32 = 102224;
        // N s_199_12: write-reg s_199_11 <= s_199_10
        let s_199_12: () = {
            state.write_register::<[u64; 7usize]>(s_199_11 as isize, s_199_10);
            tracer.write_register(s_199_11 as isize, s_199_10);
        };
        // N s_199_13: return
        return;
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var merge#var.0:struct
        let s_200_0: &'static str = fn_state.merge_var._0;
        // D s_200_1: read-var merge#var.1:struct
        let s_200_1: i128 = fn_state.merge_var._1;
        // D s_200_2: write-var u#38684 <= s_200_1
        fn_state.u_38684 = s_200_1;
        // C s_200_3: const #"cpu.CCSIDR-L3_override" : str
        let s_200_3: &'static str = "cpu.CCSIDR-L3_override";
        // D s_200_4: cmp-eq s_200_0 s_200_3
        let s_200_4: bool = ((s_200_0) == (s_200_3));
        // D s_200_5: not s_200_4
        let s_200_5: bool = !s_200_4;
        // N s_200_6: branch s_200_5 b202 b201
        if s_200_5 {
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
        // C s_201_0: const #63s : i
        let s_201_0: i128 = 63;
        // C s_201_1: const #0s : i
        let s_201_1: i128 = 0;
        // D s_201_2: read-var u#38684:i
        let s_201_2: i128 = fn_state.u_38684;
        // D s_201_3: call integer_subrange(s_201_2, s_201_0, s_201_1)
        let s_201_3: Bits = integer_subrange(state, tracer, s_201_2, s_201_0, s_201_1);
        // D s_201_4: cast reint s_201_3 -> u64
        let s_201_4: u64 = (s_201_3.value() as u64);
        // C s_201_5: const #2s : i
        let s_201_5: i128 = 2;
        // C s_201_6: const #102224u : u32
        let s_201_6: u32 = 102224;
        // D s_201_7: read-reg s_201_6:[u64; 7]
        let s_201_7: [u64; 7usize] = {
            let value = state.read_register::<[u64; 7usize]>(s_201_6 as isize);
            tracer.read_register(s_201_6 as isize, value);
            value
        };
        // D s_201_8: mutate-element s_201_7[s_201_5] <= s_201_4
        let s_201_8: [u64; 7usize] = {
            let mut local = s_201_7.clone();
            local[(s_201_5) as usize] = s_201_4;
            local
        };
        // D s_201_9: cast cvt s_201_8 -> [u64; 0]
        let s_201_9: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_201_8);
        // D s_201_10: cast cvt s_201_9 -> [u64; 7]
        let s_201_10: [u64; 7usize] = {
            let mut buf = [Default::default(); 7usize];
            buf.copy_from_slice(&s_201_9);
            buf
        };
        // C s_201_11: const #102224u : u32
        let s_201_11: u32 = 102224;
        // N s_201_12: write-reg s_201_11 <= s_201_10
        let s_201_12: () = {
            state.write_register::<[u64; 7usize]>(s_201_11 as isize, s_201_10);
            tracer.write_register(s_201_11 as isize, s_201_10);
        };
        // N s_201_13: return
        return;
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_202_0: read-var merge#var.0:struct
        let s_202_0: &'static str = fn_state.merge_var._0;
        // D s_202_1: read-var merge#var.1:struct
        let s_202_1: i128 = fn_state.merge_var._1;
        // D s_202_2: write-var u#38686 <= s_202_1
        fn_state.u_38686 = s_202_1;
        // C s_202_3: const #"cpu.CCSIDR-L4_override" : str
        let s_202_3: &'static str = "cpu.CCSIDR-L4_override";
        // D s_202_4: cmp-eq s_202_0 s_202_3
        let s_202_4: bool = ((s_202_0) == (s_202_3));
        // D s_202_5: not s_202_4
        let s_202_5: bool = !s_202_4;
        // N s_202_6: branch s_202_5 b204 b203
        if s_202_5 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_203(state, tracer, fn_state);
        };
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #63s : i
        let s_203_0: i128 = 63;
        // C s_203_1: const #0s : i
        let s_203_1: i128 = 0;
        // D s_203_2: read-var u#38686:i
        let s_203_2: i128 = fn_state.u_38686;
        // D s_203_3: call integer_subrange(s_203_2, s_203_0, s_203_1)
        let s_203_3: Bits = integer_subrange(state, tracer, s_203_2, s_203_0, s_203_1);
        // D s_203_4: cast reint s_203_3 -> u64
        let s_203_4: u64 = (s_203_3.value() as u64);
        // C s_203_5: const #3s : i
        let s_203_5: i128 = 3;
        // C s_203_6: const #102224u : u32
        let s_203_6: u32 = 102224;
        // D s_203_7: read-reg s_203_6:[u64; 7]
        let s_203_7: [u64; 7usize] = {
            let value = state.read_register::<[u64; 7usize]>(s_203_6 as isize);
            tracer.read_register(s_203_6 as isize, value);
            value
        };
        // D s_203_8: mutate-element s_203_7[s_203_5] <= s_203_4
        let s_203_8: [u64; 7usize] = {
            let mut local = s_203_7.clone();
            local[(s_203_5) as usize] = s_203_4;
            local
        };
        // D s_203_9: cast cvt s_203_8 -> [u64; 0]
        let s_203_9: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_203_8);
        // D s_203_10: cast cvt s_203_9 -> [u64; 7]
        let s_203_10: [u64; 7usize] = {
            let mut buf = [Default::default(); 7usize];
            buf.copy_from_slice(&s_203_9);
            buf
        };
        // C s_203_11: const #102224u : u32
        let s_203_11: u32 = 102224;
        // N s_203_12: write-reg s_203_11 <= s_203_10
        let s_203_12: () = {
            state.write_register::<[u64; 7usize]>(s_203_11 as isize, s_203_10);
            tracer.write_register(s_203_11 as isize, s_203_10);
        };
        // N s_203_13: return
        return;
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_204_0: read-var merge#var.0:struct
        let s_204_0: &'static str = fn_state.merge_var._0;
        // D s_204_1: read-var merge#var.1:struct
        let s_204_1: i128 = fn_state.merge_var._1;
        // D s_204_2: write-var u#38688 <= s_204_1
        fn_state.u_38688 = s_204_1;
        // C s_204_3: const #"cpu.CCSIDR-L5_override" : str
        let s_204_3: &'static str = "cpu.CCSIDR-L5_override";
        // D s_204_4: cmp-eq s_204_0 s_204_3
        let s_204_4: bool = ((s_204_0) == (s_204_3));
        // D s_204_5: not s_204_4
        let s_204_5: bool = !s_204_4;
        // N s_204_6: branch s_204_5 b206 b205
        if s_204_5 {
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
        // C s_205_0: const #63s : i
        let s_205_0: i128 = 63;
        // C s_205_1: const #0s : i
        let s_205_1: i128 = 0;
        // D s_205_2: read-var u#38688:i
        let s_205_2: i128 = fn_state.u_38688;
        // D s_205_3: call integer_subrange(s_205_2, s_205_0, s_205_1)
        let s_205_3: Bits = integer_subrange(state, tracer, s_205_2, s_205_0, s_205_1);
        // D s_205_4: cast reint s_205_3 -> u64
        let s_205_4: u64 = (s_205_3.value() as u64);
        // C s_205_5: const #4s : i
        let s_205_5: i128 = 4;
        // C s_205_6: const #102224u : u32
        let s_205_6: u32 = 102224;
        // D s_205_7: read-reg s_205_6:[u64; 7]
        let s_205_7: [u64; 7usize] = {
            let value = state.read_register::<[u64; 7usize]>(s_205_6 as isize);
            tracer.read_register(s_205_6 as isize, value);
            value
        };
        // D s_205_8: mutate-element s_205_7[s_205_5] <= s_205_4
        let s_205_8: [u64; 7usize] = {
            let mut local = s_205_7.clone();
            local[(s_205_5) as usize] = s_205_4;
            local
        };
        // D s_205_9: cast cvt s_205_8 -> [u64; 0]
        let s_205_9: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_205_8);
        // D s_205_10: cast cvt s_205_9 -> [u64; 7]
        let s_205_10: [u64; 7usize] = {
            let mut buf = [Default::default(); 7usize];
            buf.copy_from_slice(&s_205_9);
            buf
        };
        // C s_205_11: const #102224u : u32
        let s_205_11: u32 = 102224;
        // N s_205_12: write-reg s_205_11 <= s_205_10
        let s_205_12: () = {
            state.write_register::<[u64; 7usize]>(s_205_11 as isize, s_205_10);
            tracer.write_register(s_205_11 as isize, s_205_10);
        };
        // N s_205_13: return
        return;
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var merge#var.0:struct
        let s_206_0: &'static str = fn_state.merge_var._0;
        // D s_206_1: read-var merge#var.1:struct
        let s_206_1: i128 = fn_state.merge_var._1;
        // D s_206_2: write-var u#38690 <= s_206_1
        fn_state.u_38690 = s_206_1;
        // C s_206_3: const #"cpu.CCSIDR-L6_override" : str
        let s_206_3: &'static str = "cpu.CCSIDR-L6_override";
        // D s_206_4: cmp-eq s_206_0 s_206_3
        let s_206_4: bool = ((s_206_0) == (s_206_3));
        // D s_206_5: not s_206_4
        let s_206_5: bool = !s_206_4;
        // N s_206_6: branch s_206_5 b208 b207
        if s_206_5 {
            return block_208(state, tracer, fn_state);
        } else {
            return block_207(state, tracer, fn_state);
        };
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #63s : i
        let s_207_0: i128 = 63;
        // C s_207_1: const #0s : i
        let s_207_1: i128 = 0;
        // D s_207_2: read-var u#38690:i
        let s_207_2: i128 = fn_state.u_38690;
        // D s_207_3: call integer_subrange(s_207_2, s_207_0, s_207_1)
        let s_207_3: Bits = integer_subrange(state, tracer, s_207_2, s_207_0, s_207_1);
        // D s_207_4: cast reint s_207_3 -> u64
        let s_207_4: u64 = (s_207_3.value() as u64);
        // C s_207_5: const #5s : i
        let s_207_5: i128 = 5;
        // C s_207_6: const #102224u : u32
        let s_207_6: u32 = 102224;
        // D s_207_7: read-reg s_207_6:[u64; 7]
        let s_207_7: [u64; 7usize] = {
            let value = state.read_register::<[u64; 7usize]>(s_207_6 as isize);
            tracer.read_register(s_207_6 as isize, value);
            value
        };
        // D s_207_8: mutate-element s_207_7[s_207_5] <= s_207_4
        let s_207_8: [u64; 7usize] = {
            let mut local = s_207_7.clone();
            local[(s_207_5) as usize] = s_207_4;
            local
        };
        // D s_207_9: cast cvt s_207_8 -> [u64; 0]
        let s_207_9: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_207_8);
        // D s_207_10: cast cvt s_207_9 -> [u64; 7]
        let s_207_10: [u64; 7usize] = {
            let mut buf = [Default::default(); 7usize];
            buf.copy_from_slice(&s_207_9);
            buf
        };
        // C s_207_11: const #102224u : u32
        let s_207_11: u32 = 102224;
        // N s_207_12: write-reg s_207_11 <= s_207_10
        let s_207_12: () = {
            state.write_register::<[u64; 7usize]>(s_207_11 as isize, s_207_10);
            tracer.write_register(s_207_11 as isize, s_207_10);
        };
        // N s_207_13: return
        return;
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var merge#var.0:struct
        let s_208_0: &'static str = fn_state.merge_var._0;
        // D s_208_1: read-var merge#var.1:struct
        let s_208_1: i128 = fn_state.merge_var._1;
        // D s_208_2: write-var u#38692 <= s_208_1
        fn_state.u_38692 = s_208_1;
        // C s_208_3: const #"cpu.CCSIDR-L7_override" : str
        let s_208_3: &'static str = "cpu.CCSIDR-L7_override";
        // D s_208_4: cmp-eq s_208_0 s_208_3
        let s_208_4: bool = ((s_208_0) == (s_208_3));
        // D s_208_5: not s_208_4
        let s_208_5: bool = !s_208_4;
        // N s_208_6: branch s_208_5 b210 b209
        if s_208_5 {
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
        // C s_209_0: const #63s : i
        let s_209_0: i128 = 63;
        // C s_209_1: const #0s : i
        let s_209_1: i128 = 0;
        // D s_209_2: read-var u#38692:i
        let s_209_2: i128 = fn_state.u_38692;
        // D s_209_3: call integer_subrange(s_209_2, s_209_0, s_209_1)
        let s_209_3: Bits = integer_subrange(state, tracer, s_209_2, s_209_0, s_209_1);
        // D s_209_4: cast reint s_209_3 -> u64
        let s_209_4: u64 = (s_209_3.value() as u64);
        // C s_209_5: const #6s : i
        let s_209_5: i128 = 6;
        // C s_209_6: const #102224u : u32
        let s_209_6: u32 = 102224;
        // D s_209_7: read-reg s_209_6:[u64; 7]
        let s_209_7: [u64; 7usize] = {
            let value = state.read_register::<[u64; 7usize]>(s_209_6 as isize);
            tracer.read_register(s_209_6 as isize, value);
            value
        };
        // D s_209_8: mutate-element s_209_7[s_209_5] <= s_209_4
        let s_209_8: [u64; 7usize] = {
            let mut local = s_209_7.clone();
            local[(s_209_5) as usize] = s_209_4;
            local
        };
        // D s_209_9: cast cvt s_209_8 -> [u64; 0]
        let s_209_9: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_209_8);
        // D s_209_10: cast cvt s_209_9 -> [u64; 7]
        let s_209_10: [u64; 7usize] = {
            let mut buf = [Default::default(); 7usize];
            buf.copy_from_slice(&s_209_9);
            buf
        };
        // C s_209_11: const #102224u : u32
        let s_209_11: u32 = 102224;
        // N s_209_12: write-reg s_209_11 <= s_209_10
        let s_209_12: () = {
            state.write_register::<[u64; 7usize]>(s_209_11 as isize, s_209_10);
            tracer.write_register(s_209_11 as isize, s_209_10);
        };
        // N s_209_13: return
        return;
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_210_0: read-var merge#var.0:struct
        let s_210_0: &'static str = fn_state.merge_var._0;
        // D s_210_1: read-var merge#var.1:struct
        let s_210_1: i128 = fn_state.merge_var._1;
        // D s_210_2: write-var u#38694 <= s_210_1
        fn_state.u_38694 = s_210_1;
        // C s_210_3: const #"cpu.cpu0.DCZID-log2-block-size" : str
        let s_210_3: &'static str = "cpu.cpu0.DCZID-log2-block-size";
        // D s_210_4: cmp-eq s_210_0 s_210_3
        let s_210_4: bool = ((s_210_0) == (s_210_3));
        // D s_210_5: not s_210_4
        let s_210_5: bool = !s_210_4;
        // N s_210_6: branch s_210_5 b212 b211
        if s_210_5 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_211(state, tracer, fn_state);
        };
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_211_0: read-var u#38694:i
        let s_211_0: i128 = fn_state.u_38694;
        // C s_211_1: const #12952u : u32
        let s_211_1: u32 = 12952;
        // N s_211_2: write-reg s_211_1 <= s_211_0
        let s_211_2: () = {
            state.write_register::<i128>(s_211_1 as isize, s_211_0);
            tracer.write_register(s_211_1 as isize, s_211_0);
        };
        // N s_211_3: return
        return;
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_212_0: read-var merge#var.0:struct
        let s_212_0: &'static str = fn_state.merge_var._0;
        // D s_212_1: read-var merge#var.1:struct
        let s_212_1: i128 = fn_state.merge_var._1;
        // D s_212_2: write-var u#38696 <= s_212_1
        fn_state.u_38696 = s_212_1;
        // C s_212_3: const #"cpu.GMID-log2-block-size" : str
        let s_212_3: &'static str = "cpu.GMID-log2-block-size";
        // D s_212_4: cmp-eq s_212_0 s_212_3
        let s_212_4: bool = ((s_212_0) == (s_212_3));
        // D s_212_5: not s_212_4
        let s_212_5: bool = !s_212_4;
        // N s_212_6: branch s_212_5 b214 b213
        if s_212_5 {
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
        // D s_213_0: read-var u#38696:i
        let s_213_0: i128 = fn_state.u_38696;
        // C s_213_1: const #23376u : u32
        let s_213_1: u32 = 23376;
        // N s_213_2: write-reg s_213_1 <= s_213_0
        let s_213_2: () = {
            state.write_register::<i128>(s_213_1 as isize, s_213_0);
            tracer.write_register(s_213_1 as isize, s_213_0);
        };
        // N s_213_3: return
        return;
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_214_0: read-var merge#var.0:struct
        let s_214_0: &'static str = fn_state.merge_var._0;
        // D s_214_1: read-var merge#var.1:struct
        let s_214_1: i128 = fn_state.merge_var._1;
        // D s_214_2: write-var u#38698 <= s_214_1
        fn_state.u_38698 = s_214_1;
        // C s_214_3: const #"exclusive_monitor.log2_granule_size" : str
        let s_214_3: &'static str = "exclusive_monitor.log2_granule_size";
        // D s_214_4: cmp-eq s_214_0 s_214_3
        let s_214_4: bool = ((s_214_0) == (s_214_3));
        // D s_214_5: not s_214_4
        let s_214_5: bool = !s_214_4;
        // N s_214_6: branch s_214_5 b216 b215
        if s_214_5 {
            return block_216(state, tracer, fn_state);
        } else {
            return block_215(state, tracer, fn_state);
        };
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_215_0: const #3s : i
        let s_215_0: i128 = 3;
        // C s_215_1: const #0s : i
        let s_215_1: i128 = 0;
        // D s_215_2: read-var u#38698:i
        let s_215_2: i128 = fn_state.u_38698;
        // D s_215_3: call integer_subrange(s_215_2, s_215_0, s_215_1)
        let s_215_3: Bits = integer_subrange(state, tracer, s_215_2, s_215_0, s_215_1);
        // D s_215_4: cast reint s_215_3 -> u8
        let s_215_4: u8 = (s_215_3.value() as u8);
        // C s_215_5: const #1784u : u32
        let s_215_5: u32 = 1784;
        // N s_215_6: write-reg s_215_5 <= s_215_4
        let s_215_6: () = {
            state.write_register::<u8>(s_215_5 as isize, s_215_4);
            tracer.write_register(s_215_5 as isize, s_215_4);
        };
        // N s_215_7: return
        return;
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_216_0: read-var merge#var.0:struct
        let s_216_0: &'static str = fn_state.merge_var._0;
        // D s_216_1: read-var merge#var.1:struct
        let s_216_1: i128 = fn_state.merge_var._1;
        // D s_216_2: write-var u#38700 <= s_216_1
        fn_state.u_38700 = s_216_1;
        // C s_216_3: const #"cpu.unpred_tsize_aborts" : str
        let s_216_3: &'static str = "cpu.unpred_tsize_aborts";
        // D s_216_4: cmp-eq s_216_0 s_216_3
        let s_216_4: bool = ((s_216_0) == (s_216_3));
        // D s_216_5: not s_216_4
        let s_216_5: bool = !s_216_4;
        // N s_216_6: branch s_216_5 b218 b217
        if s_216_5 {
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
        // C s_217_0: const #1s : i
        let s_217_0: i128 = 1;
        // D s_217_1: read-var u#38700:i
        let s_217_1: i128 = fn_state.u_38700;
        // D s_217_2: cmp-eq s_217_1 s_217_0
        let s_217_2: bool = ((s_217_1) == (s_217_0));
        // C s_217_3: const #18280u : u32
        let s_217_3: u32 = 18280;
        // N s_217_4: write-reg s_217_3 <= s_217_2
        let s_217_4: () = {
            state.write_register::<bool>(s_217_3 as isize, s_217_2);
            tracer.write_register(s_217_3 as isize, s_217_2);
        };
        // N s_217_5: return
        return;
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_218_0: read-var merge#var.0:struct
        let s_218_0: &'static str = fn_state.merge_var._0;
        // D s_218_1: read-var merge#var.1:struct
        let s_218_1: i128 = fn_state.merge_var._1;
        // D s_218_2: write-var u#38702 <= s_218_1
        fn_state.u_38702 = s_218_1;
        // C s_218_3: const #"cpu.cpu0.CONFIG64" : str
        let s_218_3: &'static str = "cpu.cpu0.CONFIG64";
        // D s_218_4: cmp-eq s_218_0 s_218_3
        let s_218_4: bool = ((s_218_0) == (s_218_3));
        // D s_218_5: not s_218_4
        let s_218_5: bool = !s_218_4;
        // N s_218_6: branch s_218_5 b220 b219
        if s_218_5 {
            return block_220(state, tracer, fn_state);
        } else {
            return block_219(state, tracer, fn_state);
        };
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #0s : i
        let s_219_0: i128 = 0;
        // D s_219_1: read-var u#38702:i
        let s_219_1: i128 = fn_state.u_38702;
        // D s_219_2: call integer_access(s_219_1, s_219_0)
        let s_219_2: bool = integer_access(state, tracer, s_219_1, s_219_0);
        // C s_219_3: const #0s : i
        let s_219_3: i128 = 0;
        // C s_219_4: const #0u : u64
        let s_219_4: u64 = 0;
        // D s_219_5: cast zx s_219_2 -> u64
        let s_219_5: u64 = (s_219_2 as u64);
        // C s_219_6: const #1u : u64
        let s_219_6: u64 = 1;
        // D s_219_7: and s_219_5 s_219_6
        let s_219_7: u64 = ((s_219_5) & (s_219_6));
        // D s_219_8: cmp-eq s_219_7 s_219_6
        let s_219_8: bool = ((s_219_7) == (s_219_6));
        // D s_219_9: lsl s_219_5 s_219_3
        let s_219_9: u64 = s_219_5 << s_219_3;
        // D s_219_10: or s_219_4 s_219_9
        let s_219_10: u64 = ((s_219_4) | (s_219_9));
        // D s_219_11: cmpl s_219_9
        let s_219_11: u64 = !s_219_9;
        // D s_219_12: and s_219_4 s_219_11
        let s_219_12: u64 = ((s_219_4) & (s_219_11));
        // D s_219_13: select s_219_8 s_219_10 s_219_12
        let s_219_13: u64 = if s_219_8 { s_219_10 } else { s_219_12 };
        // D s_219_14: cast trunc s_219_13 -> u8
        let s_219_14: bool = ((s_219_13) != 0);
        // C s_219_15: const #16352u : u32
        let s_219_15: u32 = 16352;
        // N s_219_16: write-reg s_219_15 <= s_219_14
        let s_219_16: () = {
            state.write_register::<bool>(s_219_15 as isize, s_219_14);
            tracer.write_register(s_219_15 as isize, s_219_14);
        };
        // N s_219_17: return
        return;
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_220_0: read-var merge#var.0:struct
        let s_220_0: &'static str = fn_state.merge_var._0;
        // D s_220_1: read-var merge#var.1:struct
        let s_220_1: i128 = fn_state.merge_var._1;
        // D s_220_2: write-var u#38704 <= s_220_1
        fn_state.u_38704 = s_220_1;
        // C s_220_3: const #"cpu.cpu0.RVBAR" : str
        let s_220_3: &'static str = "cpu.cpu0.RVBAR";
        // D s_220_4: cmp-eq s_220_0 s_220_3
        let s_220_4: bool = ((s_220_0) == (s_220_3));
        // D s_220_5: not s_220_4
        let s_220_5: bool = !s_220_4;
        // N s_220_6: branch s_220_5 b222 b221
        if s_220_5 {
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
        // C s_221_0: const #63s : i
        let s_221_0: i128 = 63;
        // C s_221_1: const #0s : i
        let s_221_1: i128 = 0;
        // D s_221_2: read-var u#38704:i
        let s_221_2: i128 = fn_state.u_38704;
        // D s_221_3: call integer_subrange(s_221_2, s_221_0, s_221_1)
        let s_221_3: Bits = integer_subrange(state, tracer, s_221_2, s_221_0, s_221_1);
        // D s_221_4: cast reint s_221_3 -> u64
        let s_221_4: u64 = (s_221_3.value() as u64);
        // C s_221_5: const #102440u : u32
        let s_221_5: u32 = 102440;
        // N s_221_6: write-reg s_221_5 <= s_221_4
        let s_221_6: () = {
            state.write_register::<u64>(s_221_5 as isize, s_221_4);
            tracer.write_register(s_221_5 as isize, s_221_4);
        };
        // N s_221_7: return
        return;
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_222_0: read-var merge#var.0:struct
        let s_222_0: &'static str = fn_state.merge_var._0;
        // D s_222_1: read-var merge#var.1:struct
        let s_222_1: i128 = fn_state.merge_var._1;
        // D s_222_2: write-var u#38706 <= s_222_1
        fn_state.u_38706 = s_222_1;
        // C s_222_3: const #"cpu.VAL_ignore_rvbar_in_aarch32" : str
        let s_222_3: &'static str = "cpu.VAL_ignore_rvbar_in_aarch32";
        // D s_222_4: cmp-eq s_222_0 s_222_3
        let s_222_4: bool = ((s_222_0) == (s_222_3));
        // D s_222_5: not s_222_4
        let s_222_5: bool = !s_222_4;
        // N s_222_6: branch s_222_5 b224 b223
        if s_222_5 {
            return block_224(state, tracer, fn_state);
        } else {
            return block_223(state, tracer, fn_state);
        };
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_223_0: const #1s : i
        let s_223_0: i128 = 1;
        // D s_223_1: read-var u#38706:i
        let s_223_1: i128 = fn_state.u_38706;
        // D s_223_2: cmp-eq s_223_1 s_223_0
        let s_223_2: bool = ((s_223_1) == (s_223_0));
        // C s_223_3: const #21880u : u32
        let s_223_3: u32 = 21880;
        // N s_223_4: write-reg s_223_3 <= s_223_2
        let s_223_4: () = {
            state.write_register::<bool>(s_223_3 as isize, s_223_2);
            tracer.write_register(s_223_3 as isize, s_223_2);
        };
        // N s_223_5: return
        return;
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_224_0: read-var merge#var.0:struct
        let s_224_0: &'static str = fn_state.merge_var._0;
        // D s_224_1: read-var merge#var.1:struct
        let s_224_1: i128 = fn_state.merge_var._1;
        // D s_224_2: write-var u#38708 <= s_224_1
        fn_state.u_38708 = s_224_1;
        // C s_224_3: const #"cpu.has_tlb" : str
        let s_224_3: &'static str = "cpu.has_tlb";
        // D s_224_4: cmp-eq s_224_0 s_224_3
        let s_224_4: bool = ((s_224_0) == (s_224_3));
        // D s_224_5: not s_224_4
        let s_224_5: bool = !s_224_4;
        // N s_224_6: branch s_224_5 b226 b225
        if s_224_5 {
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
        // C s_225_0: const #1s : i
        let s_225_0: i128 = 1;
        // D s_225_1: read-var u#38708:i
        let s_225_1: i128 = fn_state.u_38708;
        // D s_225_2: cmp-eq s_225_1 s_225_0
        let s_225_2: bool = ((s_225_1) == (s_225_0));
        // C s_225_3: const #19088u : u32
        let s_225_3: u32 = 19088;
        // N s_225_4: write-reg s_225_3 <= s_225_2
        let s_225_4: () = {
            state.write_register::<bool>(s_225_3 as isize, s_225_2);
            tracer.write_register(s_225_3 as isize, s_225_2);
        };
        // N s_225_5: return
        return;
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_226_0: read-var merge#var.0:struct
        let s_226_0: &'static str = fn_state.merge_var._0;
        // D s_226_1: read-var merge#var.1:struct
        let s_226_1: i128 = fn_state.merge_var._1;
        // D s_226_2: write-var u#38710 <= s_226_1
        fn_state.u_38710 = s_226_1;
        // C s_226_3: const #"cpu.has_trickbox" : str
        let s_226_3: &'static str = "cpu.has_trickbox";
        // D s_226_4: cmp-eq s_226_0 s_226_3
        let s_226_4: bool = ((s_226_0) == (s_226_3));
        // D s_226_5: not s_226_4
        let s_226_5: bool = !s_226_4;
        // N s_226_6: branch s_226_5 b228 b227
        if s_226_5 {
            return block_228(state, tracer, fn_state);
        } else {
            return block_227(state, tracer, fn_state);
        };
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_227_0: const #1s : i
        let s_227_0: i128 = 1;
        // D s_227_1: read-var u#38710:i
        let s_227_1: i128 = fn_state.u_38710;
        // D s_227_2: cmp-eq s_227_1 s_227_0
        let s_227_2: bool = ((s_227_1) == (s_227_0));
        // C s_227_3: const #102392u : u32
        let s_227_3: u32 = 102392;
        // N s_227_4: write-reg s_227_3 <= s_227_2
        let s_227_4: () = {
            state.write_register::<bool>(s_227_3 as isize, s_227_2);
            tracer.write_register(s_227_3 as isize, s_227_2);
        };
        // N s_227_5: return
        return;
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_228_0: read-var merge#var.0:struct
        let s_228_0: &'static str = fn_state.merge_var._0;
        // D s_228_1: read-var merge#var.1:struct
        let s_228_1: i128 = fn_state.merge_var._1;
        // D s_228_2: write-var u#38712 <= s_228_1
        fn_state.u_38712 = s_228_1;
        // C s_228_3: const #"cpu.cpu0.MPIDR-override" : str
        let s_228_3: &'static str = "cpu.cpu0.MPIDR-override";
        // D s_228_4: cmp-eq s_228_0 s_228_3
        let s_228_4: bool = ((s_228_0) == (s_228_3));
        // D s_228_5: not s_228_4
        let s_228_5: bool = !s_228_4;
        // N s_228_6: branch s_228_5 b230 b229
        if s_228_5 {
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
        // C s_229_0: const #31s : i
        let s_229_0: i128 = 31;
        // C s_229_1: const #0s : i
        let s_229_1: i128 = 0;
        // D s_229_2: read-var u#38712:i
        let s_229_2: i128 = fn_state.u_38712;
        // D s_229_3: call integer_subrange(s_229_2, s_229_0, s_229_1)
        let s_229_3: Bits = integer_subrange(state, tracer, s_229_2, s_229_0, s_229_1);
        // D s_229_4: cast reint s_229_3 -> u32
        let s_229_4: u32 = (s_229_3.value() as u32);
        // C s_229_5: const #14048u : u32
        let s_229_5: u32 = 14048;
        // N s_229_6: write-reg s_229_5 <= s_229_4
        let s_229_6: () = {
            state.write_register::<u32>(s_229_5 as isize, s_229_4);
            tracer.write_register(s_229_5 as isize, s_229_4);
        };
        // N s_229_7: return
        return;
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_230_0: read-var merge#var.0:struct
        let s_230_0: &'static str = fn_state.merge_var._0;
        // D s_230_1: read-var merge#var.1:struct
        let s_230_1: i128 = fn_state.merge_var._1;
        // D s_230_2: write-var u#38714 <= s_230_1
        fn_state.u_38714 = s_230_1;
        // C s_230_3: const #"cpu.cpu0.semihosting-heap_base" : str
        let s_230_3: &'static str = "cpu.cpu0.semihosting-heap_base";
        // D s_230_4: cmp-eq s_230_0 s_230_3
        let s_230_4: bool = ((s_230_0) == (s_230_3));
        // D s_230_5: not s_230_4
        let s_230_5: bool = !s_230_4;
        // N s_230_6: branch s_230_5 b232 b231
        if s_230_5 {
            return block_232(state, tracer, fn_state);
        } else {
            return block_231(state, tracer, fn_state);
        };
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_231_0: const #63s : i
        let s_231_0: i128 = 63;
        // C s_231_1: const #0s : i
        let s_231_1: i128 = 0;
        // D s_231_2: read-var u#38714:i
        let s_231_2: i128 = fn_state.u_38714;
        // D s_231_3: call integer_subrange(s_231_2, s_231_0, s_231_1)
        let s_231_3: Bits = integer_subrange(state, tracer, s_231_2, s_231_0, s_231_1);
        // D s_231_4: cast reint s_231_3 -> u64
        let s_231_4: u64 = (s_231_3.value() as u64);
        // C s_231_5: const #15920u : u32
        let s_231_5: u32 = 15920;
        // N s_231_6: write-reg s_231_5 <= s_231_4
        let s_231_6: () = {
            state.write_register::<u64>(s_231_5 as isize, s_231_4);
            tracer.write_register(s_231_5 as isize, s_231_4);
        };
        // N s_231_7: return
        return;
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_232_0: read-var merge#var.0:struct
        let s_232_0: &'static str = fn_state.merge_var._0;
        // D s_232_1: read-var merge#var.1:struct
        let s_232_1: i128 = fn_state.merge_var._1;
        // D s_232_2: write-var u#38716 <= s_232_1
        fn_state.u_38716 = s_232_1;
        // C s_232_3: const #"cpu.cpu0.semihosting-heap_limit" : str
        let s_232_3: &'static str = "cpu.cpu0.semihosting-heap_limit";
        // D s_232_4: cmp-eq s_232_0 s_232_3
        let s_232_4: bool = ((s_232_0) == (s_232_3));
        // D s_232_5: not s_232_4
        let s_232_5: bool = !s_232_4;
        // N s_232_6: branch s_232_5 b234 b233
        if s_232_5 {
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
        // C s_233_0: const #63s : i
        let s_233_0: i128 = 63;
        // C s_233_1: const #0s : i
        let s_233_1: i128 = 0;
        // D s_233_2: read-var u#38716:i
        let s_233_2: i128 = fn_state.u_38716;
        // D s_233_3: call integer_subrange(s_233_2, s_233_0, s_233_1)
        let s_233_3: Bits = integer_subrange(state, tracer, s_233_2, s_233_0, s_233_1);
        // D s_233_4: cast reint s_233_3 -> u64
        let s_233_4: u64 = (s_233_3.value() as u64);
        // C s_233_5: const #14360u : u32
        let s_233_5: u32 = 14360;
        // N s_233_6: write-reg s_233_5 <= s_233_4
        let s_233_6: () = {
            state.write_register::<u64>(s_233_5 as isize, s_233_4);
            tracer.write_register(s_233_5 as isize, s_233_4);
        };
        // N s_233_7: return
        return;
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_234_0: read-var merge#var.0:struct
        let s_234_0: &'static str = fn_state.merge_var._0;
        // D s_234_1: read-var merge#var.1:struct
        let s_234_1: i128 = fn_state.merge_var._1;
        // D s_234_2: write-var u#38718 <= s_234_1
        fn_state.u_38718 = s_234_1;
        // C s_234_3: const #"cpu.cpu0.semihosting-stack_base" : str
        let s_234_3: &'static str = "cpu.cpu0.semihosting-stack_base";
        // D s_234_4: cmp-eq s_234_0 s_234_3
        let s_234_4: bool = ((s_234_0) == (s_234_3));
        // D s_234_5: not s_234_4
        let s_234_5: bool = !s_234_4;
        // N s_234_6: branch s_234_5 b236 b235
        if s_234_5 {
            return block_236(state, tracer, fn_state);
        } else {
            return block_235(state, tracer, fn_state);
        };
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_235_0: const #63s : i
        let s_235_0: i128 = 63;
        // C s_235_1: const #0s : i
        let s_235_1: i128 = 0;
        // D s_235_2: read-var u#38718:i
        let s_235_2: i128 = fn_state.u_38718;
        // D s_235_3: call integer_subrange(s_235_2, s_235_0, s_235_1)
        let s_235_3: Bits = integer_subrange(state, tracer, s_235_2, s_235_0, s_235_1);
        // D s_235_4: cast reint s_235_3 -> u64
        let s_235_4: u64 = (s_235_3.value() as u64);
        // C s_235_5: const #12696u : u32
        let s_235_5: u32 = 12696;
        // N s_235_6: write-reg s_235_5 <= s_235_4
        let s_235_6: () = {
            state.write_register::<u64>(s_235_5 as isize, s_235_4);
            tracer.write_register(s_235_5 as isize, s_235_4);
        };
        // N s_235_7: return
        return;
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_236_0: read-var merge#var.0:struct
        let s_236_0: &'static str = fn_state.merge_var._0;
        // D s_236_1: read-var merge#var.1:struct
        let s_236_1: i128 = fn_state.merge_var._1;
        // D s_236_2: write-var u#38720 <= s_236_1
        fn_state.u_38720 = s_236_1;
        // C s_236_3: const #"cpu.cpu0.semihosting-stack_limit" : str
        let s_236_3: &'static str = "cpu.cpu0.semihosting-stack_limit";
        // D s_236_4: cmp-eq s_236_0 s_236_3
        let s_236_4: bool = ((s_236_0) == (s_236_3));
        // D s_236_5: not s_236_4
        let s_236_5: bool = !s_236_4;
        // N s_236_6: branch s_236_5 b238 b237
        if s_236_5 {
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
        // C s_237_0: const #63s : i
        let s_237_0: i128 = 63;
        // C s_237_1: const #0s : i
        let s_237_1: i128 = 0;
        // D s_237_2: read-var u#38720:i
        let s_237_2: i128 = fn_state.u_38720;
        // D s_237_3: call integer_subrange(s_237_2, s_237_0, s_237_1)
        let s_237_3: Bits = integer_subrange(state, tracer, s_237_2, s_237_0, s_237_1);
        // D s_237_4: cast reint s_237_3 -> u64
        let s_237_4: u64 = (s_237_3.value() as u64);
        // C s_237_5: const #10072u : u32
        let s_237_5: u32 = 10072;
        // N s_237_6: write-reg s_237_5 <= s_237_4
        let s_237_6: () = {
            state.write_register::<u64>(s_237_5 as isize, s_237_4);
            tracer.write_register(s_237_5 as isize, s_237_4);
        };
        // N s_237_7: return
        return;
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_238_0: read-var merge#var.0:struct
        let s_238_0: &'static str = fn_state.merge_var._0;
        // D s_238_1: read-var merge#var.1:struct
        let s_238_1: i128 = fn_state.merge_var._1;
        // D s_238_2: write-var u#38722 <= s_238_1
        fn_state.u_38722 = s_238_1;
        // C s_238_3: const #"cpu.has_qarma3_pac" : str
        let s_238_3: &'static str = "cpu.has_qarma3_pac";
        // D s_238_4: cmp-eq s_238_0 s_238_3
        let s_238_4: bool = ((s_238_0) == (s_238_3));
        // D s_238_5: not s_238_4
        let s_238_5: bool = !s_238_4;
        // N s_238_6: branch s_238_5 b242 b239
        if s_238_5 {
            return block_242(state, tracer, fn_state);
        } else {
            return block_239(state, tracer, fn_state);
        };
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_239_0: const #0s : i
        let s_239_0: i128 = 0;
        // D s_239_1: read-var u#38722:i
        let s_239_1: i128 = fn_state.u_38722;
        // D s_239_2: call neq_int(s_239_1, s_239_0)
        let s_239_2: bool = neq_int(state, tracer, s_239_1, s_239_0);
        // N s_239_3: branch s_239_2 b241 b240
        if s_239_2 {
            return block_241(state, tracer, fn_state);
        } else {
            return block_240(state, tracer, fn_state);
        };
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_240_0: return
        return;
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_241_0: const #1u : u8
        let s_241_0: bool = true;
        // C s_241_1: const #101096u : u32
        let s_241_1: u32 = 101096;
        // N s_241_2: write-reg s_241_1 <= s_241_0
        let s_241_2: () = {
            state.write_register::<bool>(s_241_1 as isize, s_241_0);
            tracer.write_register(s_241_1 as isize, s_241_0);
        };
        // C s_241_3: const #0u : u8
        let s_241_3: bool = false;
        // C s_241_4: const #10120u : u32
        let s_241_4: u32 = 10120;
        // N s_241_5: write-reg s_241_4 <= s_241_3
        let s_241_5: () = {
            state.write_register::<bool>(s_241_4 as isize, s_241_3);
            tracer.write_register(s_241_4 as isize, s_241_3);
        };
        // N s_241_6: return
        return;
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_242_0: read-var merge#var.0:struct
        let s_242_0: &'static str = fn_state.merge_var._0;
        // D s_242_1: read-var merge#var.1:struct
        let s_242_1: i128 = fn_state.merge_var._1;
        // D s_242_2: write-var u#38724 <= s_242_1
        fn_state.u_38724 = s_242_1;
        // C s_242_3: const #"cpu.has_const_pac" : str
        let s_242_3: &'static str = "cpu.has_const_pac";
        // D s_242_4: cmp-eq s_242_0 s_242_3
        let s_242_4: bool = ((s_242_0) == (s_242_3));
        // D s_242_5: not s_242_4
        let s_242_5: bool = !s_242_4;
        // N s_242_6: branch s_242_5 b244 b243
        if s_242_5 {
            return block_244(state, tracer, fn_state);
        } else {
            return block_243(state, tracer, fn_state);
        };
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_243_0: const #0s : i
        let s_243_0: i128 = 0;
        // D s_243_1: read-var u#38724:i
        let s_243_1: i128 = fn_state.u_38724;
        // D s_243_2: call neq_int(s_243_1, s_243_0)
        let s_243_2: bool = neq_int(state, tracer, s_243_1, s_243_0);
        // C s_243_3: const #14912u : u32
        let s_243_3: u32 = 14912;
        // N s_243_4: write-reg s_243_3 <= s_243_2
        let s_243_4: () = {
            state.write_register::<bool>(s_243_3 as isize, s_243_2);
            tracer.write_register(s_243_3 as isize, s_243_2);
        };
        // N s_243_5: return
        return;
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_244_0: read-var merge#var.0:struct
        let s_244_0: &'static str = fn_state.merge_var._0;
        // D s_244_1: read-var merge#var.1:struct
        let s_244_1: i128 = fn_state.merge_var._1;
        // D s_244_2: write-var u#38726 <= s_244_1
        fn_state.u_38726 = s_244_1;
        // C s_244_3: const #"cpu.has_rme" : str
        let s_244_3: &'static str = "cpu.has_rme";
        // D s_244_4: cmp-eq s_244_0 s_244_3
        let s_244_4: bool = ((s_244_0) == (s_244_3));
        // D s_244_5: not s_244_4
        let s_244_5: bool = !s_244_4;
        // N s_244_6: branch s_244_5 b246 b245
        if s_244_5 {
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
        // C s_245_0: const #0s : i
        let s_245_0: i128 = 0;
        // D s_245_1: read-var u#38726:i
        let s_245_1: i128 = fn_state.u_38726;
        // D s_245_2: call neq_int(s_245_1, s_245_0)
        let s_245_2: bool = neq_int(state, tracer, s_245_1, s_245_0);
        // C s_245_3: const #17712u : u32
        let s_245_3: u32 = 17712;
        // N s_245_4: write-reg s_245_3 <= s_245_2
        let s_245_4: () = {
            state.write_register::<bool>(s_245_3 as isize, s_245_2);
            tracer.write_register(s_245_3 as isize, s_245_2);
        };
        // N s_245_5: return
        return;
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_246_0: read-var merge#var.0:struct
        let s_246_0: &'static str = fn_state.merge_var._0;
        // D s_246_1: read-var merge#var.1:struct
        let s_246_1: i128 = fn_state.merge_var._1;
        // D s_246_2: write-var u#38728 <= s_246_1
        fn_state.u_38728 = s_246_1;
        // C s_246_3: const #"cpu.rme_l0pgtsz" : str
        let s_246_3: &'static str = "cpu.rme_l0pgtsz";
        // D s_246_4: cmp-eq s_246_0 s_246_3
        let s_246_4: bool = ((s_246_0) == (s_246_3));
        // D s_246_5: not s_246_4
        let s_246_5: bool = !s_246_4;
        // N s_246_6: branch s_246_5 b248 b247
        if s_246_5 {
            return block_248(state, tracer, fn_state);
        } else {
            return block_247(state, tracer, fn_state);
        };
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_247_0: const #3s : i
        let s_247_0: i128 = 3;
        // C s_247_1: const #0s : i
        let s_247_1: i128 = 0;
        // D s_247_2: read-var u#38728:i
        let s_247_2: i128 = fn_state.u_38728;
        // D s_247_3: call integer_subrange(s_247_2, s_247_0, s_247_1)
        let s_247_3: Bits = integer_subrange(state, tracer, s_247_2, s_247_0, s_247_1);
        // D s_247_4: cast reint s_247_3 -> u8
        let s_247_4: u8 = (s_247_3.value() as u8);
        // C s_247_5: const #11584u : u32
        let s_247_5: u32 = 11584;
        // N s_247_6: write-reg s_247_5 <= s_247_4
        let s_247_6: () = {
            state.write_register::<u8>(s_247_5 as isize, s_247_4);
            tracer.write_register(s_247_5 as isize, s_247_4);
        };
        // N s_247_7: return
        return;
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_248_0: read-var merge#var.0:struct
        let s_248_0: &'static str = fn_state.merge_var._0;
        // D s_248_1: read-var merge#var.1:struct
        let s_248_1: i128 = fn_state.merge_var._1;
        // D s_248_2: write-var u#38730 <= s_248_1
        fn_state.u_38730 = s_248_1;
        // C s_248_3: const #"cpu.has_brbe" : str
        let s_248_3: &'static str = "cpu.has_brbe";
        // D s_248_4: cmp-eq s_248_0 s_248_3
        let s_248_4: bool = ((s_248_0) == (s_248_3));
        // D s_248_5: not s_248_4
        let s_248_5: bool = !s_248_4;
        // N s_248_6: branch s_248_5 b250 b249
        if s_248_5 {
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
        // C s_249_0: const #0s : i
        let s_249_0: i128 = 0;
        // D s_249_1: read-var u#38730:i
        let s_249_1: i128 = fn_state.u_38730;
        // D s_249_2: call neq_int(s_249_1, s_249_0)
        let s_249_2: bool = neq_int(state, tracer, s_249_1, s_249_0);
        // C s_249_3: const #104616u : u32
        let s_249_3: u32 = 104616;
        // N s_249_4: write-reg s_249_3 <= s_249_2
        let s_249_4: () = {
            state.write_register::<bool>(s_249_3 as isize, s_249_2);
            tracer.write_register(s_249_3 as isize, s_249_2);
        };
        // N s_249_5: return
        return;
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_250_0: read-var merge#var.0:struct
        let s_250_0: &'static str = fn_state.merge_var._0;
        // D s_250_1: read-var merge#var.1:struct
        let s_250_1: i128 = fn_state.merge_var._1;
        // D s_250_2: write-var u#38732 <= s_250_1
        fn_state.u_38732 = s_250_1;
        // C s_250_3: const #"cpu.cpu0.number-of-branch-records" : str
        let s_250_3: &'static str = "cpu.cpu0.number-of-branch-records";
        // D s_250_4: cmp-eq s_250_0 s_250_3
        let s_250_4: bool = ((s_250_0) == (s_250_3));
        // D s_250_5: not s_250_4
        let s_250_5: bool = !s_250_4;
        // N s_250_6: branch s_250_5 b252 b251
        if s_250_5 {
            return block_252(state, tracer, fn_state);
        } else {
            return block_251(state, tracer, fn_state);
        };
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_251_0: const #16s : i
        let s_251_0: i128 = 16;
        // D s_251_1: read-var u#38732:i
        let s_251_1: i128 = fn_state.u_38732;
        // D s_251_2: call fdiv_int(s_251_1, s_251_0)
        let s_251_2: i128 = fdiv_int(state, tracer, s_251_1, s_251_0);
        // C s_251_3: const #16s : i
        let s_251_3: i128 = 16;
        // D s_251_4: mul s_251_3 s_251_2
        let s_251_4: i128 = ((s_251_3) * (s_251_2));
        // C s_251_5: const #20824u : u32
        let s_251_5: u32 = 20824;
        // N s_251_6: write-reg s_251_5 <= s_251_4
        let s_251_6: () = {
            state.write_register::<i128>(s_251_5 as isize, s_251_4);
            tracer.write_register(s_251_5 as isize, s_251_4);
        };
        // N s_251_7: return
        return;
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_252_0: read-var merge#var.0:struct
        let s_252_0: &'static str = fn_state.merge_var._0;
        // D s_252_1: read-var merge#var.1:struct
        let s_252_1: i128 = fn_state.merge_var._1;
        // D s_252_2: write-var u#38734 <= s_252_1
        fn_state.u_38734 = s_252_1;
        // C s_252_3: const #"cpu.isb_is_branch" : str
        let s_252_3: &'static str = "cpu.isb_is_branch";
        // D s_252_4: cmp-eq s_252_0 s_252_3
        let s_252_4: bool = ((s_252_0) == (s_252_3));
        // D s_252_5: not s_252_4
        let s_252_5: bool = !s_252_4;
        // N s_252_6: branch s_252_5 b254 b253
        if s_252_5 {
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
        // C s_253_0: const #0s : i
        let s_253_0: i128 = 0;
        // D s_253_1: read-var u#38734:i
        let s_253_1: i128 = fn_state.u_38734;
        // D s_253_2: call neq_int(s_253_1, s_253_0)
        let s_253_2: bool = neq_int(state, tracer, s_253_1, s_253_0);
        // C s_253_3: const #22064u : u32
        let s_253_3: u32 = 22064;
        // N s_253_4: write-reg s_253_3 <= s_253_2
        let s_253_4: () = {
            state.write_register::<bool>(s_253_3 as isize, s_253_2);
            tracer.write_register(s_253_3 as isize, s_253_2);
        };
        // N s_253_5: return
        return;
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_254_0: read-var merge#var.0:struct
        let s_254_0: &'static str = fn_state.merge_var._0;
        // D s_254_1: read-var merge#var.1:struct
        let s_254_1: i128 = fn_state.merge_var._1;
        // D s_254_2: write-var u#38736 <= s_254_1
        fn_state.u_38736 = s_254_1;
        // C s_254_3: const #"cpu.has_brbe_v9_3" : str
        let s_254_3: &'static str = "cpu.has_brbe_v9_3";
        // D s_254_4: cmp-eq s_254_0 s_254_3
        let s_254_4: bool = ((s_254_0) == (s_254_3));
        // D s_254_5: not s_254_4
        let s_254_5: bool = !s_254_4;
        // N s_254_6: branch s_254_5 b256 b255
        if s_254_5 {
            return block_256(state, tracer, fn_state);
        } else {
            return block_255(state, tracer, fn_state);
        };
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_255_0: const #0s : i
        let s_255_0: i128 = 0;
        // D s_255_1: read-var u#38736:i
        let s_255_1: i128 = fn_state.u_38736;
        // D s_255_2: call neq_int(s_255_1, s_255_0)
        let s_255_2: bool = neq_int(state, tracer, s_255_1, s_255_0);
        // C s_255_3: const #12128u : u32
        let s_255_3: u32 = 12128;
        // N s_255_4: write-reg s_255_3 <= s_255_2
        let s_255_4: () = {
            state.write_register::<bool>(s_255_3 as isize, s_255_2);
            tracer.write_register(s_255_3 as isize, s_255_2);
        };
        // N s_255_5: return
        return;
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_256_0: read-var merge#var.0:struct
        let s_256_0: &'static str = fn_state.merge_var._0;
        // D s_256_1: read-var merge#var.1:struct
        let s_256_1: i128 = fn_state.merge_var._1;
        // D s_256_2: write-var u#38738 <= s_256_1
        fn_state.u_38738 = s_256_1;
        // C s_256_3: const #"SVE.ScalableVectorExtension.has_sme_f64f64" : str
        let s_256_3: &'static str = "SVE.ScalableVectorExtension.has_sme_f64f64";
        // D s_256_4: cmp-eq s_256_0 s_256_3
        let s_256_4: bool = ((s_256_0) == (s_256_3));
        // D s_256_5: not s_256_4
        let s_256_5: bool = !s_256_4;
        // N s_256_6: branch s_256_5 b258 b257
        if s_256_5 {
            return block_258(state, tracer, fn_state);
        } else {
            return block_257(state, tracer, fn_state);
        };
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_257_0: const #0s : i
        let s_257_0: i128 = 0;
        // D s_257_1: read-var u#38738:i
        let s_257_1: i128 = fn_state.u_38738;
        // D s_257_2: call neq_int(s_257_1, s_257_0)
        let s_257_2: bool = neq_int(state, tracer, s_257_1, s_257_0);
        // C s_257_3: const #23000u : u32
        let s_257_3: u32 = 23000;
        // N s_257_4: write-reg s_257_3 <= s_257_2
        let s_257_4: () = {
            state.write_register::<bool>(s_257_3 as isize, s_257_2);
            tracer.write_register(s_257_3 as isize, s_257_2);
        };
        // N s_257_5: return
        return;
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_258_0: read-var merge#var.0:struct
        let s_258_0: &'static str = fn_state.merge_var._0;
        // D s_258_1: read-var merge#var.1:struct
        let s_258_1: i128 = fn_state.merge_var._1;
        // D s_258_2: write-var u#38740 <= s_258_1
        fn_state.u_38740 = s_258_1;
        // C s_258_3: const #"SVE.ScalableVectorExtension.has_sme_priority_control" : str
        let s_258_3: &'static str = "SVE.ScalableVectorExtension.has_sme_priority_control";
        // D s_258_4: cmp-eq s_258_0 s_258_3
        let s_258_4: bool = ((s_258_0) == (s_258_3));
        // D s_258_5: not s_258_4
        let s_258_5: bool = !s_258_4;
        // N s_258_6: branch s_258_5 b260 b259
        if s_258_5 {
            return block_260(state, tracer, fn_state);
        } else {
            return block_259(state, tracer, fn_state);
        };
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_259_0: const #0s : i
        let s_259_0: i128 = 0;
        // D s_259_1: read-var u#38740:i
        let s_259_1: i128 = fn_state.u_38740;
        // D s_259_2: call neq_int(s_259_1, s_259_0)
        let s_259_2: bool = neq_int(state, tracer, s_259_1, s_259_0);
        // C s_259_3: const #14400u : u32
        let s_259_3: u32 = 14400;
        // N s_259_4: write-reg s_259_3 <= s_259_2
        let s_259_4: () = {
            state.write_register::<bool>(s_259_3 as isize, s_259_2);
            tracer.write_register(s_259_3 as isize, s_259_2);
        };
        // N s_259_5: return
        return;
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_260_0: read-var merge#var.0:struct
        let s_260_0: &'static str = fn_state.merge_var._0;
        // D s_260_1: read-var merge#var.1:struct
        let s_260_1: i128 = fn_state.merge_var._1;
        // D s_260_2: write-var u#38742 <= s_260_1
        fn_state.u_38742 = s_260_1;
        // C s_260_3: const #"SVE.ScalableVectorExtension.has_sme_i16i64" : str
        let s_260_3: &'static str = "SVE.ScalableVectorExtension.has_sme_i16i64";
        // D s_260_4: cmp-eq s_260_0 s_260_3
        let s_260_4: bool = ((s_260_0) == (s_260_3));
        // D s_260_5: not s_260_4
        let s_260_5: bool = !s_260_4;
        // N s_260_6: branch s_260_5 b262 b261
        if s_260_5 {
            return block_262(state, tracer, fn_state);
        } else {
            return block_261(state, tracer, fn_state);
        };
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_261_0: const #0s : i
        let s_261_0: i128 = 0;
        // D s_261_1: read-var u#38742:i
        let s_261_1: i128 = fn_state.u_38742;
        // D s_261_2: call neq_int(s_261_1, s_261_0)
        let s_261_2: bool = neq_int(state, tracer, s_261_1, s_261_0);
        // C s_261_3: const #17264u : u32
        let s_261_3: u32 = 17264;
        // N s_261_4: write-reg s_261_3 <= s_261_2
        let s_261_4: () = {
            state.write_register::<bool>(s_261_3 as isize, s_261_2);
            tracer.write_register(s_261_3 as isize, s_261_2);
        };
        // N s_261_5: return
        return;
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_262_0: read-var merge#var.0:struct
        let s_262_0: &'static str = fn_state.merge_var._0;
        // D s_262_1: read-var merge#var.1:struct
        let s_262_1: i128 = fn_state.merge_var._1;
        // D s_262_2: write-var u#38744 <= s_262_1
        fn_state.u_38744 = s_262_1;
        // C s_262_3: const #"SVE.ScalableVectorExtension.sme_veclens_implemented" : str
        let s_262_3: &'static str = "SVE.ScalableVectorExtension.sme_veclens_implemented";
        // D s_262_4: cmp-eq s_262_0 s_262_3
        let s_262_4: bool = ((s_262_0) == (s_262_3));
        // D s_262_5: not s_262_4
        let s_262_5: bool = !s_262_4;
        // N s_262_6: branch s_262_5 b264 b263
        if s_262_5 {
            return block_264(state, tracer, fn_state);
        } else {
            return block_263(state, tracer, fn_state);
        };
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_263_0: read-var u#38744:i
        let s_263_0: i128 = fn_state.u_38744;
        // D s_263_1: call decode_max_smeveclen(s_263_0)
        let s_263_1: i128 = decode_max_smeveclen(state, tracer, s_263_0);
        // C s_263_2: const #1576u : u32
        let s_263_2: u32 = 1576;
        // N s_263_3: write-reg s_263_2 <= s_263_1
        let s_263_3: () = {
            state.write_register::<i128>(s_263_2 as isize, s_263_1);
            tracer.write_register(s_263_2 as isize, s_263_1);
        };
        // N s_263_4: return
        return;
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_264_0: read-var merge#var.0:struct
        let s_264_0: &'static str = fn_state.merge_var._0;
        // D s_264_1: read-var merge#var.1:struct
        let s_264_1: i128 = fn_state.merge_var._1;
        // D s_264_2: write-var u#38746 <= s_264_1
        fn_state.u_38746 = s_264_1;
        // C s_264_3: const #"SVE.ScalableVectorExtension.has_sve_extended_bf16" : str
        let s_264_3: &'static str = "SVE.ScalableVectorExtension.has_sve_extended_bf16";
        // D s_264_4: cmp-eq s_264_0 s_264_3
        let s_264_4: bool = ((s_264_0) == (s_264_3));
        // D s_264_5: not s_264_4
        let s_264_5: bool = !s_264_4;
        // N s_264_6: branch s_264_5 b266 b265
        if s_264_5 {
            return block_266(state, tracer, fn_state);
        } else {
            return block_265(state, tracer, fn_state);
        };
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_265_0: const #1s : i
        let s_265_0: i128 = 1;
        // C s_265_1: const #0s : i
        let s_265_1: i128 = 0;
        // D s_265_2: read-var u#38746:i
        let s_265_2: i128 = fn_state.u_38746;
        // D s_265_3: call integer_subrange(s_265_2, s_265_0, s_265_1)
        let s_265_3: Bits = integer_subrange(state, tracer, s_265_2, s_265_0, s_265_1);
        // D s_265_4: cast reint s_265_3 -> u8
        let s_265_4: u8 = (s_265_3.value() as u8);
        // D s_265_5: cast zx s_265_4 -> bv
        let s_265_5: Bits = Bits::new(s_265_4 as u128, 2u16);
        // D s_265_6: cast zx s_265_5 -> i
        let s_265_6: i128 = (s_265_5.value() as i128);
        // C s_265_7: const #20848u : u32
        let s_265_7: u32 = 20848;
        // N s_265_8: write-reg s_265_7 <= s_265_6
        let s_265_8: () = {
            state.write_register::<i128>(s_265_7 as isize, s_265_6);
            tracer.write_register(s_265_7 as isize, s_265_6);
        };
        // N s_265_9: return
        return;
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_266_0: read-var merge#var.0:struct
        let s_266_0: &'static str = fn_state.merge_var._0;
        // D s_266_1: read-var merge#var.1:struct
        let s_266_1: i128 = fn_state.merge_var._1;
        // D s_266_2: write-var u#38748 <= s_266_1
        fn_state.u_38748 = s_266_1;
        // C s_266_3: const #"SVE.ScalableVectorExtension.has_sme" : str
        let s_266_3: &'static str = "SVE.ScalableVectorExtension.has_sme";
        // D s_266_4: cmp-eq s_266_0 s_266_3
        let s_266_4: bool = ((s_266_0) == (s_266_3));
        // D s_266_5: not s_266_4
        let s_266_5: bool = !s_266_4;
        // N s_266_6: branch s_266_5 b268 b267
        if s_266_5 {
            return block_268(state, tracer, fn_state);
        } else {
            return block_267(state, tracer, fn_state);
        };
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_267_0: const #0s : i
        let s_267_0: i128 = 0;
        // D s_267_1: read-var u#38748:i
        let s_267_1: i128 = fn_state.u_38748;
        // D s_267_2: call neq_int(s_267_1, s_267_0)
        let s_267_2: bool = neq_int(state, tracer, s_267_1, s_267_0);
        // C s_267_3: const #22000u : u32
        let s_267_3: u32 = 22000;
        // N s_267_4: write-reg s_267_3 <= s_267_2
        let s_267_4: () = {
            state.write_register::<bool>(s_267_3 as isize, s_267_2);
            tracer.write_register(s_267_3 as isize, s_267_2);
        };
        // N s_267_5: return
        return;
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_268_0: read-var merge#var.0:struct
        let s_268_0: &'static str = fn_state.merge_var._0;
        // D s_268_1: read-var merge#var.1:struct
        let s_268_1: i128 = fn_state.merge_var._1;
        // D s_268_2: write-var u#38750 <= s_268_1
        fn_state.u_38750 = s_268_1;
        // C s_268_3: const #"SVE.ScalableVectorExtension.sme_only" : str
        let s_268_3: &'static str = "SVE.ScalableVectorExtension.sme_only";
        // D s_268_4: cmp-eq s_268_0 s_268_3
        let s_268_4: bool = ((s_268_0) == (s_268_3));
        // D s_268_5: not s_268_4
        let s_268_5: bool = !s_268_4;
        // N s_268_6: branch s_268_5 b270 b269
        if s_268_5 {
            return block_270(state, tracer, fn_state);
        } else {
            return block_269(state, tracer, fn_state);
        };
    }
    fn block_269<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_269_0: const #0s : i
        let s_269_0: i128 = 0;
        // D s_269_1: read-var u#38750:i
        let s_269_1: i128 = fn_state.u_38750;
        // D s_269_2: call neq_int(s_269_1, s_269_0)
        let s_269_2: bool = neq_int(state, tracer, s_269_1, s_269_0);
        // C s_269_3: const #13584u : u32
        let s_269_3: u32 = 13584;
        // N s_269_4: write-reg s_269_3 <= s_269_2
        let s_269_4: () = {
            state.write_register::<bool>(s_269_3 as isize, s_269_2);
            tracer.write_register(s_269_3 as isize, s_269_2);
        };
        // N s_269_5: return
        return;
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_270_0: read-var merge#var.0:struct
        let s_270_0: &'static str = fn_state.merge_var._0;
        // D s_270_1: read-var merge#var.1:struct
        let s_270_1: i128 = fn_state.merge_var._1;
        // D s_270_2: write-var u#38752 <= s_270_1
        fn_state.u_38752 = s_270_1;
        // C s_270_3: const #"SVE.ScalableVectorExtension.veclen" : str
        let s_270_3: &'static str = "SVE.ScalableVectorExtension.veclen";
        // D s_270_4: cmp-eq s_270_0 s_270_3
        let s_270_4: bool = ((s_270_0) == (s_270_3));
        // D s_270_5: not s_270_4
        let s_270_5: bool = !s_270_4;
        // N s_270_6: branch s_270_5 b272 b271
        if s_270_5 {
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
        // D s_271_0: read-var u#38752:i
        let s_271_0: i128 = fn_state.u_38752;
        // D s_271_1: call decode_max_sveveclen(s_271_0)
        let s_271_1: i128 = decode_max_sveveclen(state, tracer, s_271_0);
        // C s_271_2: const #11176u : u32
        let s_271_2: u32 = 11176;
        // N s_271_3: write-reg s_271_2 <= s_271_1
        let s_271_3: () = {
            state.write_register::<i128>(s_271_2 as isize, s_271_1);
            tracer.write_register(s_271_2 as isize, s_271_1);
        };
        // N s_271_4: return
        return;
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_272_0: read-var merge#var.0:struct
        let s_272_0: &'static str = fn_state.merge_var._0;
        // D s_272_1: read-var merge#var.1:struct
        let s_272_1: i128 = fn_state.merge_var._1;
        // D s_272_2: write-var u#38754 <= s_272_1
        fn_state.u_38754 = s_272_1;
        // C s_272_3: const #"cpu.has_extended_recp_estimate" : str
        let s_272_3: &'static str = "cpu.has_extended_recp_estimate";
        // D s_272_4: cmp-eq s_272_0 s_272_3
        let s_272_4: bool = ((s_272_0) == (s_272_3));
        // D s_272_5: not s_272_4
        let s_272_5: bool = !s_272_4;
        // N s_272_6: branch s_272_5 b274 b273
        if s_272_5 {
            return block_274(state, tracer, fn_state);
        } else {
            return block_273(state, tracer, fn_state);
        };
    }
    fn block_273<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_273_0: const #0s : i
        let s_273_0: i128 = 0;
        // D s_273_1: read-var u#38754:i
        let s_273_1: i128 = fn_state.u_38754;
        // D s_273_2: call neq_int(s_273_1, s_273_0)
        let s_273_2: bool = neq_int(state, tracer, s_273_1, s_273_0);
        // C s_273_3: const #23360u : u32
        let s_273_3: u32 = 23360;
        // N s_273_4: write-reg s_273_3 <= s_273_2
        let s_273_4: () = {
            state.write_register::<bool>(s_273_3 as isize, s_273_2);
            tracer.write_register(s_273_3 as isize, s_273_2);
        };
        // N s_273_5: return
        return;
    }
    fn block_274<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_274_0: read-var merge#var.0:struct
        let s_274_0: &'static str = fn_state.merge_var._0;
        // D s_274_1: read-var merge#var.1:struct
        let s_274_1: i128 = fn_state.merge_var._1;
        // D s_274_2: write-var u#38756 <= s_274_1
        fn_state.u_38756 = s_274_1;
        // C s_274_3: const #"spe.pseudo_cycles" : str
        let s_274_3: &'static str = "spe.pseudo_cycles";
        // D s_274_4: cmp-eq s_274_0 s_274_3
        let s_274_4: bool = ((s_274_0) == (s_274_3));
        // D s_274_5: not s_274_4
        let s_274_5: bool = !s_274_4;
        // N s_274_6: branch s_274_5 b276 b275
        if s_274_5 {
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
        // C s_275_0: const #0s : i
        let s_275_0: i128 = 0;
        // D s_275_1: read-var u#38756:i
        let s_275_1: i128 = fn_state.u_38756;
        // D s_275_2: call neq_int(s_275_1, s_275_0)
        let s_275_2: bool = neq_int(state, tracer, s_275_1, s_275_0);
        // C s_275_3: const #104728u : u32
        let s_275_3: u32 = 104728;
        // N s_275_4: write-reg s_275_3 <= s_275_2
        let s_275_4: () = {
            state.write_register::<bool>(s_275_3 as isize, s_275_2);
            tracer.write_register(s_275_3 as isize, s_275_2);
        };
        // N s_275_5: return
        return;
    }
    fn block_276<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_276_0: read-var merge#var.0:struct
        let s_276_0: &'static str = fn_state.merge_var._0;
        // D s_276_1: read-var merge#var.1:struct
        let s_276_1: i128 = fn_state.merge_var._1;
        // D s_276_2: write-var u#38758 <= s_276_1
        fn_state.u_38758 = s_276_1;
        // C s_276_3: const #"cpu.has_nested_virtualization" : str
        let s_276_3: &'static str = "cpu.has_nested_virtualization";
        // D s_276_4: cmp-eq s_276_0 s_276_3
        let s_276_4: bool = ((s_276_0) == (s_276_3));
        // D s_276_5: not s_276_4
        let s_276_5: bool = !s_276_4;
        // N s_276_6: branch s_276_5 b278 b277
        if s_276_5 {
            return block_278(state, tracer, fn_state);
        } else {
            return block_277(state, tracer, fn_state);
        };
    }
    fn block_277<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_277_0: const #0s : i
        let s_277_0: i128 = 0;
        // D s_277_1: read-var u#38758:i
        let s_277_1: i128 = fn_state.u_38758;
        // D s_277_2: call neq_int(s_277_1, s_277_0)
        let s_277_2: bool = neq_int(state, tracer, s_277_1, s_277_0);
        // C s_277_3: const #100896u : u32
        let s_277_3: u32 = 100896;
        // N s_277_4: write-reg s_277_3 <= s_277_2
        let s_277_4: () = {
            state.write_register::<bool>(s_277_3 as isize, s_277_2);
            tracer.write_register(s_277_3 as isize, s_277_2);
        };
        // N s_277_5: return
        return;
    }
    fn block_278<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_278_0: read-var merge#var.0:struct
        let s_278_0: &'static str = fn_state.merge_var._0;
        // D s_278_1: read-var merge#var.1:struct
        let s_278_1: i128 = fn_state.merge_var._1;
        // D s_278_2: write-var u#38760 <= s_278_1
        fn_state.u_38760 = s_278_1;
        // C s_278_3: const #"cpu.has_rndr" : str
        let s_278_3: &'static str = "cpu.has_rndr";
        // D s_278_4: cmp-eq s_278_0 s_278_3
        let s_278_4: bool = ((s_278_0) == (s_278_3));
        // D s_278_5: not s_278_4
        let s_278_5: bool = !s_278_4;
        // N s_278_6: branch s_278_5 b280 b279
        if s_278_5 {
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
        // C s_279_0: const #0s : i
        let s_279_0: i128 = 0;
        // D s_279_1: read-var u#38760:i
        let s_279_1: i128 = fn_state.u_38760;
        // D s_279_2: call neq_int(s_279_1, s_279_0)
        let s_279_2: bool = neq_int(state, tracer, s_279_1, s_279_0);
        // C s_279_3: const #14432u : u32
        let s_279_3: u32 = 14432;
        // N s_279_4: write-reg s_279_3 <= s_279_2
        let s_279_4: () = {
            state.write_register::<bool>(s_279_3 as isize, s_279_2);
            tracer.write_register(s_279_3 as isize, s_279_2);
        };
        // N s_279_5: return
        return;
    }
    fn block_280<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_280_0: read-var merge#var.0:struct
        let s_280_0: &'static str = fn_state.merge_var._0;
        // D s_280_1: read-var merge#var.1:struct
        let s_280_1: i128 = fn_state.merge_var._1;
        // D s_280_2: write-var u#38762 <= s_280_1
        fn_state.u_38762 = s_280_1;
        // C s_280_3: const #"cpu.has_enhanced_pan" : str
        let s_280_3: &'static str = "cpu.has_enhanced_pan";
        // D s_280_4: cmp-eq s_280_0 s_280_3
        let s_280_4: bool = ((s_280_0) == (s_280_3));
        // D s_280_5: not s_280_4
        let s_280_5: bool = !s_280_4;
        // N s_280_6: branch s_280_5 b282 b281
        if s_280_5 {
            return block_282(state, tracer, fn_state);
        } else {
            return block_281(state, tracer, fn_state);
        };
    }
    fn block_281<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_281_0: const #0s : i
        let s_281_0: i128 = 0;
        // D s_281_1: read-var u#38762:i
        let s_281_1: i128 = fn_state.u_38762;
        // D s_281_2: call neq_int(s_281_1, s_281_0)
        let s_281_2: bool = neq_int(state, tracer, s_281_1, s_281_0);
        // C s_281_3: const #12800u : u32
        let s_281_3: u32 = 12800;
        // N s_281_4: write-reg s_281_3 <= s_281_2
        let s_281_4: () = {
            state.write_register::<bool>(s_281_3 as isize, s_281_2);
            tracer.write_register(s_281_3 as isize, s_281_2);
        };
        // N s_281_5: return
        return;
    }
    fn block_282<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_282_0: read-var merge#var.0:struct
        let s_282_0: &'static str = fn_state.merge_var._0;
        // D s_282_1: read-var merge#var.1:struct
        let s_282_1: i128 = fn_state.merge_var._1;
        // D s_282_2: write-var u#38764 <= s_282_1
        fn_state.u_38764 = s_282_1;
        // C s_282_3: const #"cpu.rme_mecid_width" : str
        let s_282_3: &'static str = "cpu.rme_mecid_width";
        // D s_282_4: cmp-eq s_282_0 s_282_3
        let s_282_4: bool = ((s_282_0) == (s_282_3));
        // D s_282_5: not s_282_4
        let s_282_5: bool = !s_282_4;
        // N s_282_6: branch s_282_5 b284 b283
        if s_282_5 {
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
        // C s_283_0: const #1s : i
        let s_283_0: i128 = 1;
        // D s_283_1: read-var u#38764:i
        let s_283_1: i128 = fn_state.u_38764;
        // D s_283_2: sub s_283_1 s_283_0
        let s_283_2: i128 = ((s_283_1) - (s_283_0));
        // C s_283_3: const #3s : i
        let s_283_3: i128 = 3;
        // C s_283_4: const #0s : i
        let s_283_4: i128 = 0;
        // D s_283_5: call integer_subrange(s_283_2, s_283_3, s_283_4)
        let s_283_5: Bits = integer_subrange(state, tracer, s_283_2, s_283_3, s_283_4);
        // D s_283_6: cast reint s_283_5 -> u8
        let s_283_6: u8 = (s_283_5.value() as u8);
        // C s_283_7: const #15936u : u32
        let s_283_7: u32 = 15936;
        // N s_283_8: write-reg s_283_7 <= s_283_6
        let s_283_8: () = {
            state.write_register::<u8>(s_283_7 as isize, s_283_6);
            tracer.write_register(s_283_7 as isize, s_283_6);
        };
        // N s_283_9: return
        return;
    }
    fn block_284<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_284_0: read-var merge#var.0:struct
        let s_284_0: &'static str = fn_state.merge_var._0;
        // D s_284_1: read-var merge#var.1:struct
        let s_284_1: i128 = fn_state.merge_var._1;
        // D s_284_2: write-var u#38766 <= s_284_1
        fn_state.u_38766 = s_284_1;
        // C s_284_3: const #"emulator.termination_opcode" : str
        let s_284_3: &'static str = "emulator.termination_opcode";
        // D s_284_4: cmp-eq s_284_0 s_284_3
        let s_284_4: bool = ((s_284_0) == (s_284_3));
        // D s_284_5: not s_284_4
        let s_284_5: bool = !s_284_4;
        // N s_284_6: branch s_284_5 b286 b285
        if s_284_5 {
            return block_286(state, tracer, fn_state);
        } else {
            return block_285(state, tracer, fn_state);
        };
    }
    fn block_285<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_285_0: const #31s : i
        let s_285_0: i128 = 31;
        // C s_285_1: const #0s : i
        let s_285_1: i128 = 0;
        // D s_285_2: read-var u#38766:i
        let s_285_2: i128 = fn_state.u_38766;
        // D s_285_3: call integer_subrange(s_285_2, s_285_0, s_285_1)
        let s_285_3: Bits = integer_subrange(state, tracer, s_285_2, s_285_0, s_285_1);
        // D s_285_4: cast reint s_285_3 -> u32
        let s_285_4: u32 = (s_285_3.value() as u32);
        // D s_285_5: cast zx s_285_4 -> bv
        let s_285_5: Bits = Bits::new(s_285_4 as u128, 32u16);
        // D s_285_6: create-sum enum = 1:"s_285_5"
        let s_285_6: SumType755586eec3e2b646 = SumType755586eec3e2b646::_1(s_285_5);
        // C s_285_7: const #101128u : u32
        let s_285_7: u32 = 101128;
        // N s_285_8: write-reg s_285_7 <= s_285_6
        let s_285_8: () = {
            state.write_register::<SumType755586eec3e2b646>(s_285_7 as isize, s_285_6);
            tracer.write_register(s_285_7 as isize, s_285_6);
        };
        // N s_285_9: return
        return;
    }
    fn block_286<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_286_0: return
        return;
    }
}
