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
use CNTHP_CTL_read::*;
use CNTP_CVAL_NS_read::*;
use u_get_CNTP_CTL_Type_IMASK::*;
use CNTP_CTL_write::*;
use u_get_CNTHP_CTL_Type_ENABLE::*;
use CNTP_CTL_read::*;
use IsTimerConditionMet::*;
use Zeros::*;
use u_get_CNTP_CTL_Type_ENABLE::*;
use u_get_CNTV_CTL_EL0_Type_IMASK::*;
use CNTHP_CVAL_read::*;
use u_update_CNTP_CTL_Type_ISTATUS::*;
use CNTP_CTL_NS_read::*;
use CNTHP_CTL_write::*;
use CNTP_CVAL_read::*;
use u_update_CNTHP_CTL_Type_ISTATUS::*;
use HaveAArch64::*;
use u_get_CNTHP_CTL_Type_IMASK::*;
use u_get_CNTV_CTL_EL0_Type_ENABLE::*;
use CNTP_CTL_NS_write::*;
use common::*;
pub fn AArch32_CheckTimerConditions<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_12420: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_9224: ProductType700c18a878c5601b,
        ga_9231: ProductType5c790c8ef59cc8b2,
        ga_9220: ProductType5c790c8ef59cc8b2,
        gs_12422: bool,
        ga_9225: bool,
        ga_9214: ProductType700c18a878c5601b,
        ga_9210: ProductType5c790c8ef59cc8b2,
        ga_9215: bool,
        status: bool,
        offset: u64,
        ga_9235: ProductType700c18a878c5601b,
        ga_9236: bool,
        gs_12420: (),
    }
    let fn_state = FunctionState {
        gs_12420,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #64s : i
        let s_0_0: i128 = 64;
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u64
        let s_0_2: u64 = (s_0_1.value() as u64);
        // D s_0_3: write-var offset <= s_0_2
        fn_state.offset = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call HaveAArch64(s_0_4)
        let s_0_5: bool = HaveAArch64(state, tracer, s_0_4);
        // S s_0_6: not s_0_5
        let s_0_6: bool = !s_0_5;
        // N s_0_7: assert s_0_6
        let s_0_7: () = assert!(s_0_6);
        // C s_0_8: const #424u : u32
        let s_0_8: u32 = 424;
        // D s_0_9: read-reg s_0_8:u8
        let s_0_9: u8 = {
            let value = state.read_register::<u8>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // C s_0_10: const #2u : u8
        let s_0_10: u8 = 2;
        // D s_0_11: cmp-lt s_0_9 s_0_10
        let s_0_11: bool = ((s_0_9) < (s_0_10));
        // N s_0_12: branch s_0_11 b23 b1
        if s_0_11 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call CNTP_CTL_read(s_1_0)
        let s_1_1: ProductType700c18a878c5601b = CNTP_CTL_read(state, tracer, s_1_0);
        // S s_1_2: call _get_CNTP_CTL_Type_ENABLE(s_1_1)
        let s_1_2: bool = u_get_CNTP_CTL_Type_ENABLE(state, tracer, s_1_1);
        // S s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 1u16);
        // C s_1_4: const #1u : u8
        let s_1_4: bool = true;
        // C s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 1u16);
        // S s_1_6: cmp-eq s_1_3 s_1_5
        let s_1_6: bool = ((s_1_3) == (s_1_5));
        // N s_1_7: branch s_1_6 b19 b2
        if s_1_6 {
            return block_19(state, tracer, fn_state);
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
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #432u : u32
        let s_4_0: u32 = 432;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // C s_4_2: const #2u : u8
        let s_4_2: u8 = 2;
        // D s_4_3: cmp-lt s_4_1 s_4_2
        let s_4_3: bool = ((s_4_1) < (s_4_2));
        // N s_4_4: branch s_4_3 b18 b5
        if s_4_3 {
            return block_18(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#12422 <= s_5_0
        fn_state.gs_12422 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#12422:u8
        let s_6_0: bool = fn_state.gs_12422;
        // N s_6_1: branch s_6_0 b14 b7
        if s_6_0 {
            return block_14(state, tracer, fn_state);
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
        // C s_8_0: const #17200u : u32
        let s_8_0: u32 = 17200;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_CNTV_CTL_EL0_Type_ENABLE(s_8_1)
        let s_8_2: bool = u_get_CNTV_CTL_EL0_Type_ENABLE(state, tracer, s_8_1);
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // C s_8_4: const #1u : u8
        let s_8_4: bool = true;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // D s_8_6: cmp-eq s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) == (s_8_5));
        // N s_8_7: branch s_8_6 b10 b9
        if s_8_6 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #23632u : u32
        let s_10_0: u32 = 23632;
        // D s_10_1: read-reg s_10_0:u64
        let s_10_1: u64 = {
            let value = state.read_register::<u64>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // C s_10_2: const #17200u : u32
        let s_10_2: u32 = 17200;
        // D s_10_3: read-reg s_10_2:struct
        let s_10_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // D s_10_4: call _get_CNTV_CTL_EL0_Type_IMASK(s_10_3)
        let s_10_4: bool = u_get_CNTV_CTL_EL0_Type_IMASK(state, tracer, s_10_3);
        // C s_10_5: const #22400u : u32
        let s_10_5: u32 = 22400;
        // D s_10_6: read-reg s_10_5:u64
        let s_10_6: u64 = {
            let value = state.read_register::<u64>(s_10_5 as isize);
            tracer.read_register(s_10_5 as isize, value);
            value
        };
        // C s_10_7: const #9u : u32
        let s_10_7: u32 = 9;
        // D s_10_8: call IsTimerConditionMet(s_10_6, s_10_1, s_10_4, s_10_7)
        let s_10_8: bool = IsTimerConditionMet(
            state,
            tracer,
            s_10_6,
            s_10_1,
            s_10_4,
            s_10_7,
        );
        // N s_10_9: branch s_10_8 b13 b11
        if s_10_8 {
            return block_13(state, tracer, fn_state);
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
        // C s_12_0: const #17200u : u32
        let s_12_0: u32 = 17200;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #17200u : u32
        let s_12_2: u32 = 17200;
        // N s_12_3: write-reg s_12_2 <= s_12_1
        let s_12_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_12_2 as isize, s_12_1);
            tracer.write_register(s_12_2 as isize, s_12_1);
        };
        // N s_12_4: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call CNTHP_CVAL_read(s_14_0)
        let s_14_1: ProductType5c790c8ef59cc8b2 = CNTHP_CVAL_read(state, tracer, s_14_0);
        // D s_14_2: write-var ga#9231 <= s_14_1
        fn_state.ga_9231 = s_14_1;
        // D s_14_3: read-var ga#9231.0:struct
        let s_14_3: u64 = fn_state.ga_9231._0;
        // C s_14_4: const #() : ()
        let s_14_4: () = ();
        // S s_14_5: call CNTHP_CTL_read(s_14_4)
        let s_14_5: ProductType700c18a878c5601b = CNTHP_CTL_read(state, tracer, s_14_4);
        // S s_14_6: call _get_CNTHP_CTL_Type_IMASK(s_14_5)
        let s_14_6: bool = u_get_CNTHP_CTL_Type_IMASK(state, tracer, s_14_5);
        // D s_14_7: read-var offset:u64
        let s_14_7: u64 = fn_state.offset;
        // C s_14_8: const #6u : u32
        let s_14_8: u32 = 6;
        // D s_14_9: call IsTimerConditionMet(s_14_7, s_14_3, s_14_6, s_14_8)
        let s_14_9: bool = IsTimerConditionMet(
            state,
            tracer,
            s_14_7,
            s_14_3,
            s_14_6,
            s_14_8,
        );
        // D s_14_10: write-var status <= s_14_9
        fn_state.status = s_14_9;
        // C s_14_11: const #() : ()
        let s_14_11: () = ();
        // S s_14_12: call CNTHP_CTL_read(s_14_11)
        let s_14_12: ProductType700c18a878c5601b = CNTHP_CTL_read(
            state,
            tracer,
            s_14_11,
        );
        // D s_14_13: write-var ga#9235 <= s_14_12
        fn_state.ga_9235 = s_14_12;
        // D s_14_14: read-var status:u8
        let s_14_14: bool = fn_state.status;
        // N s_14_15: branch s_14_14 b17 b15
        if s_14_14 {
            return block_17(state, tracer, fn_state);
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
        // D s_15_1: write-var ga#9236 <= s_15_0
        fn_state.ga_9236 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var ga#9235:struct
        let s_16_0: ProductType700c18a878c5601b = fn_state.ga_9235;
        // D s_16_1: read-var ga#9236:u8
        let s_16_1: bool = fn_state.ga_9236;
        // D s_16_2: call _update_CNTHP_CTL_Type_ISTATUS(s_16_0, s_16_1)
        let s_16_2: ProductType700c18a878c5601b = u_update_CNTHP_CTL_Type_ISTATUS(
            state,
            tracer,
            s_16_0,
            s_16_1,
        );
        // D s_16_3: call CNTHP_CTL_write(s_16_2)
        let s_16_3: () = CNTHP_CTL_write(state, tracer, s_16_2);
        // N s_16_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var ga#9236 <= s_17_0
        fn_state.ga_9236 = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call CNTHP_CTL_read(s_18_0)
        let s_18_1: ProductType700c18a878c5601b = CNTHP_CTL_read(state, tracer, s_18_0);
        // S s_18_2: call _get_CNTHP_CTL_Type_ENABLE(s_18_1)
        let s_18_2: bool = u_get_CNTHP_CTL_Type_ENABLE(state, tracer, s_18_1);
        // S s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // C s_18_4: const #1u : u8
        let s_18_4: bool = true;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // S s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // D s_18_7: write-var gs#12422 <= s_18_6
        fn_state.gs_12422 = s_18_6;
        // N s_18_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call CNTP_CVAL_read(s_19_0)
        let s_19_1: ProductType5c790c8ef59cc8b2 = CNTP_CVAL_read(state, tracer, s_19_0);
        // D s_19_2: write-var ga#9220 <= s_19_1
        fn_state.ga_9220 = s_19_1;
        // D s_19_3: read-var ga#9220.0:struct
        let s_19_3: u64 = fn_state.ga_9220._0;
        // C s_19_4: const #() : ()
        let s_19_4: () = ();
        // S s_19_5: call CNTP_CTL_read(s_19_4)
        let s_19_5: ProductType700c18a878c5601b = CNTP_CTL_read(state, tracer, s_19_4);
        // S s_19_6: call _get_CNTP_CTL_Type_IMASK(s_19_5)
        let s_19_6: bool = u_get_CNTP_CTL_Type_IMASK(state, tracer, s_19_5);
        // D s_19_7: read-var offset:u64
        let s_19_7: u64 = fn_state.offset;
        // C s_19_8: const #5u : u32
        let s_19_8: u32 = 5;
        // D s_19_9: call IsTimerConditionMet(s_19_7, s_19_3, s_19_6, s_19_8)
        let s_19_9: bool = IsTimerConditionMet(
            state,
            tracer,
            s_19_7,
            s_19_3,
            s_19_6,
            s_19_8,
        );
        // D s_19_10: write-var status <= s_19_9
        fn_state.status = s_19_9;
        // C s_19_11: const #() : ()
        let s_19_11: () = ();
        // S s_19_12: call CNTP_CTL_read(s_19_11)
        let s_19_12: ProductType700c18a878c5601b = CNTP_CTL_read(state, tracer, s_19_11);
        // D s_19_13: write-var ga#9224 <= s_19_12
        fn_state.ga_9224 = s_19_12;
        // D s_19_14: read-var status:u8
        let s_19_14: bool = fn_state.status;
        // N s_19_15: branch s_19_14 b22 b20
        if s_19_14 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var ga#9225 <= s_20_0
        fn_state.ga_9225 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var ga#9224:struct
        let s_21_0: ProductType700c18a878c5601b = fn_state.ga_9224;
        // D s_21_1: read-var ga#9225:u8
        let s_21_1: bool = fn_state.ga_9225;
        // D s_21_2: call _update_CNTP_CTL_Type_ISTATUS(s_21_0, s_21_1)
        let s_21_2: ProductType700c18a878c5601b = u_update_CNTP_CTL_Type_ISTATUS(
            state,
            tracer,
            s_21_0,
            s_21_1,
        );
        // D s_21_3: call CNTP_CTL_write(s_21_2)
        let s_21_3: () = CNTP_CTL_write(state, tracer, s_21_2);
        // N s_21_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var ga#9225 <= s_22_0
        fn_state.ga_9225 = s_22_0;
        // N s_22_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #21888u : u32
        let s_23_0: u32 = 21888;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_CNTP_CTL_Type_ENABLE(s_23_1)
        let s_23_2: bool = u_get_CNTP_CTL_Type_ENABLE(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // N s_23_7: branch s_23_6 b32 b24
        if s_23_6 {
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
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call CNTP_CTL_NS_read(s_25_0)
        let s_25_1: ProductType700c18a878c5601b = CNTP_CTL_NS_read(
            state,
            tracer,
            s_25_0,
        );
        // S s_25_2: call _get_CNTP_CTL_Type_ENABLE(s_25_1)
        let s_25_2: bool = u_get_CNTP_CTL_Type_ENABLE(state, tracer, s_25_1);
        // S s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // C s_25_4: const #1u : u8
        let s_25_4: bool = true;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 1u16);
        // S s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // N s_25_7: branch s_25_6 b28 b26
        if s_25_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call CNTP_CVAL_NS_read(s_28_0)
        let s_28_1: ProductType5c790c8ef59cc8b2 = CNTP_CVAL_NS_read(
            state,
            tracer,
            s_28_0,
        );
        // D s_28_2: write-var ga#9210 <= s_28_1
        fn_state.ga_9210 = s_28_1;
        // D s_28_3: read-var ga#9210.0:struct
        let s_28_3: u64 = fn_state.ga_9210._0;
        // C s_28_4: const #() : ()
        let s_28_4: () = ();
        // S s_28_5: call CNTP_CTL_NS_read(s_28_4)
        let s_28_5: ProductType700c18a878c5601b = CNTP_CTL_NS_read(
            state,
            tracer,
            s_28_4,
        );
        // S s_28_6: call _get_CNTP_CTL_Type_IMASK(s_28_5)
        let s_28_6: bool = u_get_CNTP_CTL_Type_IMASK(state, tracer, s_28_5);
        // D s_28_7: read-var offset:u64
        let s_28_7: u64 = fn_state.offset;
        // C s_28_8: const #5u : u32
        let s_28_8: u32 = 5;
        // D s_28_9: call IsTimerConditionMet(s_28_7, s_28_3, s_28_6, s_28_8)
        let s_28_9: bool = IsTimerConditionMet(
            state,
            tracer,
            s_28_7,
            s_28_3,
            s_28_6,
            s_28_8,
        );
        // D s_28_10: write-var status <= s_28_9
        fn_state.status = s_28_9;
        // C s_28_11: const #() : ()
        let s_28_11: () = ();
        // S s_28_12: call CNTP_CTL_NS_read(s_28_11)
        let s_28_12: ProductType700c18a878c5601b = CNTP_CTL_NS_read(
            state,
            tracer,
            s_28_11,
        );
        // D s_28_13: write-var ga#9214 <= s_28_12
        fn_state.ga_9214 = s_28_12;
        // D s_28_14: read-var status:u8
        let s_28_14: bool = fn_state.status;
        // N s_28_15: branch s_28_14 b31 b29
        if s_28_14 {
            return block_31(state, tracer, fn_state);
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
        // D s_29_1: write-var ga#9215 <= s_29_0
        fn_state.ga_9215 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var ga#9214:struct
        let s_30_0: ProductType700c18a878c5601b = fn_state.ga_9214;
        // D s_30_1: read-var ga#9215:u8
        let s_30_1: bool = fn_state.ga_9215;
        // D s_30_2: call _update_CNTP_CTL_Type_ISTATUS(s_30_0, s_30_1)
        let s_30_2: ProductType700c18a878c5601b = u_update_CNTP_CTL_Type_ISTATUS(
            state,
            tracer,
            s_30_0,
            s_30_1,
        );
        // D s_30_3: call CNTP_CTL_NS_write(s_30_2)
        let s_30_3: () = CNTP_CTL_NS_write(state, tracer, s_30_2);
        // N s_30_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var ga#9215 <= s_31_0
        fn_state.ga_9215 = s_31_0;
        // N s_31_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #16848u : u32
        let s_32_0: u32 = 16848;
        // D s_32_1: read-reg s_32_0:u64
        let s_32_1: u64 = {
            let value = state.read_register::<u64>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // C s_32_2: const #21888u : u32
        let s_32_2: u32 = 21888;
        // D s_32_3: read-reg s_32_2:struct
        let s_32_3: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_32_2 as isize);
            tracer.read_register(s_32_2 as isize, value);
            value
        };
        // D s_32_4: call _get_CNTP_CTL_Type_IMASK(s_32_3)
        let s_32_4: bool = u_get_CNTP_CTL_Type_IMASK(state, tracer, s_32_3);
        // D s_32_5: read-var offset:u64
        let s_32_5: u64 = fn_state.offset;
        // C s_32_6: const #8u : u32
        let s_32_6: u32 = 8;
        // D s_32_7: call IsTimerConditionMet(s_32_5, s_32_1, s_32_4, s_32_6)
        let s_32_7: bool = IsTimerConditionMet(
            state,
            tracer,
            s_32_5,
            s_32_1,
            s_32_4,
            s_32_6,
        );
        // D s_32_8: write-var status <= s_32_7
        fn_state.status = s_32_7;
        // D s_32_9: read-var status:u8
        let s_32_9: bool = fn_state.status;
        // N s_32_10: branch s_32_9 b35 b33
        if s_32_9 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #21888u : u32
        let s_34_0: u32 = 21888;
        // D s_34_1: read-reg s_34_0:struct
        let s_34_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // C s_34_2: const #21888u : u32
        let s_34_2: u32 = 21888;
        // N s_34_3: write-reg s_34_2 <= s_34_1
        let s_34_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_34_2 as isize, s_34_1);
            tracer.write_register(s_34_2 as isize, s_34_1);
        };
        // N s_34_4: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: jump b34
        return block_34(state, tracer, fn_state);
    }
}
