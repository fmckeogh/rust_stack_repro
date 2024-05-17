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
use u_update_PAR_Type_FST::*;
use AArch32_ReportedOuterAttrs::*;
use u_update_PAR_Type_Outer::*;
use PAR_read::*;
use AArch32_PARFaultStatusSD::*;
use u_update_PAR_Type_PA::*;
use u_update_PAR_Type_Inner::*;
use u__UNKNOWN_bit::*;
use PAR_write::*;
use u_update_PAR_Type_LPAE::*;
use AArch32_PAROuterAttrs::*;
use ReportedPARShareability::*;
use u_update_PAR_Type_SH::*;
use u_update_PAR_Type_NOS::*;
use AArch32_PARInnerAttrs::*;
use Mk_PAR_Type::*;
use u_update_PAR_Type_SS::*;
use AArch32_ReportedInnerAttrs::*;
use u__IMPDEF_bits::*;
use IsFault::*;
use u_update_PAR_Type_NS::*;
use u_update_PAR_Type_F::*;
use u__IMPDEF_bit::*;
use common::*;
pub fn AArch32_EncodePARSD<T: Tracer>(
    state: &mut State,
    tracer: &T,
    addrdesc_in: ProductTypece7c66ccb2cab13e,
    supersection: bool,
    ss: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_30414: bool,
        ga_23720: ProductTypef170cab34335b70c,
        addrdesc: ProductTypece7c66ccb2cab13e,
        ga_23723: ProductTypeda0231e9dc169f81,
        ga_23700: ProductType5c790c8ef59cc8b2,
        ga_23734: ProductType5c790c8ef59cc8b2,
        ga_23673: ProductTypef170cab34335b70c,
        ga_23671: ProductType183e6678e5239c85,
        ga_23674: ProductType183e6678e5239c85,
        ga_23710: ProductType5c790c8ef59cc8b2,
        ga_23680: ProductTypeda0231e9dc169f81,
        ga_23711: bool,
        sh: u8,
        ga_23707: ProductTypef170cab34335b70c,
        gs_30415: bool,
        ga_23715: ProductTypeda0231e9dc169f81,
        ga_23668: ProductTypef170cab34335b70c,
        nsshadow_562: bool,
        ga_23670: ProductTypef170cab34335b70c,
        addrdesc_in: ProductTypece7c66ccb2cab13e,
        supersection: bool,
        ss: u32,
    }
    let fn_state = FunctionState {
        addrdesc_in,
        supersection,
        ss,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var addrdesc_in:struct
        let s_0_0: ProductTypece7c66ccb2cab13e = fn_state.addrdesc_in;
        // D s_0_1: write-var addrdesc <= s_0_0
        fn_state.addrdesc = s_0_0;
        // D s_0_2: read-var addrdesc:struct
        let s_0_2: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // D s_0_3: call IsFault(s_0_2)
        let s_0_3: bool = IsFault(state, tracer, s_0_2);
        // D s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // N s_0_5: branch s_0_4 b2 b1
        if s_0_4 {
            return block_2(state, tracer, fn_state);
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
        // S s_1_1: call PAR_read(s_1_0)
        let s_1_1: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_1_0);
        // C s_1_2: const #1u : u8
        let s_1_2: bool = true;
        // S s_1_3: call _update_PAR_Type_F(s_1_1, s_1_2)
        let s_1_3: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_F(
            state,
            tracer,
            s_1_1,
            s_1_2,
        );
        // S s_1_4: call PAR_write(s_1_3)
        let s_1_4: () = PAR_write(state, tracer, s_1_3);
        // C s_1_5: const #() : ()
        let s_1_5: () = ();
        // S s_1_6: call PAR_read(s_1_5)
        let s_1_6: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_1_5);
        // D s_1_7: read-var addrdesc.0:struct
        let s_1_7: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_1_8: call AArch32_PARFaultStatusSD(s_1_7)
        let s_1_8: u8 = AArch32_PARFaultStatusSD(state, tracer, s_1_7);
        // D s_1_9: call _update_PAR_Type_FST(s_1_6, s_1_8)
        let s_1_9: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_FST(
            state,
            tracer,
            s_1_6,
            s_1_8,
        );
        // D s_1_10: call PAR_write(s_1_9)
        let s_1_10: () = PAR_write(state, tracer, s_1_9);
        // C s_1_11: const #() : ()
        let s_1_11: () = ();
        // S s_1_12: call PAR_read(s_1_11)
        let s_1_12: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_1_11);
        // C s_1_13: const #0u : u8
        let s_1_13: bool = false;
        // S s_1_14: call _update_PAR_Type_LPAE(s_1_12, s_1_13)
        let s_1_14: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_LPAE(
            state,
            tracer,
            s_1_12,
            s_1_13,
        );
        // S s_1_15: call PAR_write(s_1_14)
        let s_1_15: () = PAR_write(state, tracer, s_1_14);
        // C s_1_16: const #() : ()
        let s_1_16: () = ();
        // S s_1_17: call PAR_read(s_1_16)
        let s_1_17: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_1_16);
        // D s_1_18: write-var ga#23734 <= s_1_17
        fn_state.ga_23734 = s_1_17;
        // D s_1_19: read-var ga#23734.0:struct
        let s_1_19: u64 = fn_state.ga_23734._0;
        // C s_1_20: const #16s : i64
        let s_1_20: i64 = 16;
        // C s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // C s_1_22: const #"Faulting PAR" : str
        let s_1_22: &'static str = "Faulting PAR";
        // S s_1_23: call __IMPDEF_bits(s_1_21, s_1_22)
        let s_1_23: Bits = u__IMPDEF_bits(state, tracer, s_1_21, s_1_22);
        // S s_1_24: cast reint s_1_23 -> u16
        let s_1_24: u16 = (s_1_23.value() as u16);
        // C s_1_25: const #16s : i
        let s_1_25: i128 = 16;
        // D s_1_26: cast zx s_1_19 -> bv
        let s_1_26: Bits = Bits::new(s_1_19 as u128, 64u16);
        // S s_1_27: cast zx s_1_24 -> bv
        let s_1_27: Bits = Bits::new(s_1_24 as u128, 16u16);
        // C s_1_28: const #15s : i
        let s_1_28: i128 = 15;
        // C s_1_29: const #1u : u64
        let s_1_29: u64 = 1;
        // C s_1_30: cast zx s_1_29 -> bv
        let s_1_30: Bits = Bits::new(s_1_29 as u128, 64u16);
        // C s_1_31: lsl s_1_30 s_1_28
        let s_1_31: Bits = s_1_30 << s_1_28;
        // C s_1_32: sub s_1_31 s_1_30
        let s_1_32: Bits = ((s_1_31) - (s_1_30));
        // S s_1_33: and s_1_27 s_1_32
        let s_1_33: Bits = ((s_1_27) & (s_1_32));
        // S s_1_34: lsl s_1_33 s_1_25
        let s_1_34: Bits = s_1_33 << s_1_25;
        // C s_1_35: lsl s_1_32 s_1_25
        let s_1_35: Bits = s_1_32 << s_1_25;
        // C s_1_36: cmpl s_1_35
        let s_1_36: Bits = !s_1_35;
        // D s_1_37: and s_1_26 s_1_36
        let s_1_37: Bits = ((s_1_26) & (s_1_36));
        // D s_1_38: or s_1_37 s_1_34
        let s_1_38: Bits = ((s_1_37) | (s_1_34));
        // D s_1_39: cast reint s_1_38 -> u64
        let s_1_39: u64 = (s_1_38.value() as u64);
        // D s_1_40: call Mk_PAR_Type(s_1_39)
        let s_1_40: ProductType5c790c8ef59cc8b2 = Mk_PAR_Type(state, tracer, s_1_39);
        // D s_1_41: call PAR_write(s_1_40)
        let s_1_41: () = PAR_write(state, tracer, s_1_40);
        // N s_1_42: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var addrdesc.2:struct
        let s_2_0: ProductTypef170cab34335b70c = fn_state.addrdesc._2;
        // D s_2_1: write-var ga#23668 <= s_2_0
        fn_state.ga_23668 = s_2_0;
        // D s_2_2: read-var ga#23668.2:struct
        let s_2_2: u32 = fn_state.ga_23668._2;
        // C s_2_3: const #1u : u32
        let s_2_3: u32 = 1;
        // D s_2_4: cmp-eq s_2_2 s_2_3
        let s_2_4: bool = ((s_2_2) == (s_2_3));
        // N s_2_5: branch s_2_4 b25 b3
        if s_2_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var addrdesc.2:struct
        let s_3_0: ProductTypef170cab34335b70c = fn_state.addrdesc._2;
        // D s_3_1: write-var ga#23670 <= s_3_0
        fn_state.ga_23670 = s_3_0;
        // D s_3_2: read-var ga#23670.1:struct
        let s_3_2: ProductType183e6678e5239c85 = fn_state.ga_23670._1;
        // D s_3_3: write-var ga#23671 <= s_3_2
        fn_state.ga_23671 = s_3_2;
        // D s_3_4: read-var ga#23671.0:struct
        let s_3_4: u8 = fn_state.ga_23671._0;
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // C s_3_6: const #464u : u32
        let s_3_6: u32 = 464;
        // D s_3_7: read-reg s_3_6:u8
        let s_3_7: u8 = {
            let value = state.read_register::<u8>(s_3_6 as isize);
            tracer.read_register(s_3_6 as isize, value);
            value
        };
        // D s_3_8: cast zx s_3_7 -> bv
        let s_3_8: Bits = Bits::new(s_3_7 as u128, 2u16);
        // D s_3_9: cmp-eq s_3_5 s_3_8
        let s_3_9: bool = ((s_3_5) == (s_3_8));
        // N s_3_10: branch s_3_9 b24 b4
        if s_3_9 {
            return block_24(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#30414 <= s_4_0
        fn_state.gs_30414 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#30414:u8
        let s_5_0: bool = fn_state.gs_30414;
        // D s_5_1: write-var gs#30415 <= s_5_0
        fn_state.gs_30415 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#30415:u8
        let s_6_0: bool = fn_state.gs_30415;
        // N s_6_1: branch s_6_0 b23 b7
        if s_6_0 {
            return block_23(state, tracer, fn_state);
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
        // D s_8_0: read-var ss:u32
        let s_8_0: u32 = fn_state.ss;
        // C s_8_1: const #0u : u32
        let s_8_1: u32 = 0;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b22 b9
        if s_8_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var addrdesc.3:struct
        let s_9_0: ProductTypeda0231e9dc169f81 = fn_state.addrdesc._3;
        // D s_9_1: write-var ga#23680 <= s_9_0
        fn_state.ga_23680 = s_9_0;
        // D s_9_2: read-var ga#23680.1:struct
        let s_9_2: u32 = fn_state.ga_23680._1;
        // C s_9_3: const #1u : u32
        let s_9_3: u32 = 1;
        // D s_9_4: cmp-eq s_9_2 s_9_3
        let s_9_4: bool = ((s_9_2) == (s_9_3));
        // N s_9_5: branch s_9_4 b21 b10
        if s_9_4 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var addrdesc.3:struct
        let s_11_0: ProductTypeda0231e9dc169f81 = fn_state.addrdesc._3;
        // D s_11_1: write-var ga#23723 <= s_11_0
        fn_state.ga_23723 = s_11_0;
        // D s_11_2: read-var ga#23723.1:struct
        let s_11_2: u32 = fn_state.ga_23723._1;
        // C s_11_3: const #1u : u32
        let s_11_3: u32 = 1;
        // D s_11_4: cmp-eq s_11_2 s_11_3
        let s_11_4: bool = ((s_11_2) == (s_11_3));
        // N s_11_5: branch s_11_4 b20 b12
        if s_11_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var nsshadow#562 <= s_12_0
        fn_state.nsshadow_562 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var addrdesc.2:struct
        let s_13_0: ProductTypef170cab34335b70c = fn_state.addrdesc._2;
        // D s_13_1: write-var ga#23720 <= s_13_0
        fn_state.ga_23720 = s_13_0;
        // D s_13_2: read-var ga#23720.5:struct
        let s_13_2: u32 = fn_state.ga_23720._5;
        // C s_13_3: const #0u : u32
        let s_13_3: u32 = 0;
        // D s_13_4: cmp-eq s_13_2 s_13_3
        let s_13_4: bool = ((s_13_2) == (s_13_3));
        // N s_13_5: branch s_13_4 b19 b14
        if s_13_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: u8 = 0;
        // D s_14_1: write-var sh <= s_14_0
        fn_state.sh = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call PAR_read(s_15_0)
        let s_15_1: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_15_0);
        // C s_15_2: const #0u : u8
        let s_15_2: bool = false;
        // S s_15_3: call _update_PAR_Type_F(s_15_1, s_15_2)
        let s_15_3: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_F(
            state,
            tracer,
            s_15_1,
            s_15_2,
        );
        // S s_15_4: call PAR_write(s_15_3)
        let s_15_4: () = PAR_write(state, tracer, s_15_3);
        // C s_15_5: const #() : ()
        let s_15_5: () = ();
        // S s_15_6: call PAR_read(s_15_5)
        let s_15_6: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_15_5);
        // D s_15_7: read-var supersection:u8
        let s_15_7: bool = fn_state.supersection;
        // D s_15_8: call _update_PAR_Type_SS(s_15_6, s_15_7)
        let s_15_8: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_SS(
            state,
            tracer,
            s_15_6,
            s_15_7,
        );
        // D s_15_9: call PAR_write(s_15_8)
        let s_15_9: () = PAR_write(state, tracer, s_15_8);
        // C s_15_10: const #() : ()
        let s_15_10: () = ();
        // S s_15_11: call PAR_read(s_15_10)
        let s_15_11: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_15_10);
        // D s_15_12: read-var addrdesc.2:struct
        let s_15_12: ProductTypef170cab34335b70c = fn_state.addrdesc._2;
        // D s_15_13: call AArch32_PAROuterAttrs(s_15_12)
        let s_15_13: u8 = AArch32_PAROuterAttrs(state, tracer, s_15_12);
        // D s_15_14: call AArch32_ReportedOuterAttrs(s_15_13)
        let s_15_14: u8 = AArch32_ReportedOuterAttrs(state, tracer, s_15_13);
        // D s_15_15: call _update_PAR_Type_Outer(s_15_11, s_15_14)
        let s_15_15: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_Outer(
            state,
            tracer,
            s_15_11,
            s_15_14,
        );
        // D s_15_16: call PAR_write(s_15_15)
        let s_15_16: () = PAR_write(state, tracer, s_15_15);
        // C s_15_17: const #() : ()
        let s_15_17: () = ();
        // S s_15_18: call PAR_read(s_15_17)
        let s_15_18: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_15_17);
        // D s_15_19: read-var addrdesc.2:struct
        let s_15_19: ProductTypef170cab34335b70c = fn_state.addrdesc._2;
        // D s_15_20: call AArch32_PARInnerAttrs(s_15_19)
        let s_15_20: u8 = AArch32_PARInnerAttrs(state, tracer, s_15_19);
        // D s_15_21: call AArch32_ReportedInnerAttrs(s_15_20)
        let s_15_21: u8 = AArch32_ReportedInnerAttrs(state, tracer, s_15_20);
        // D s_15_22: call _update_PAR_Type_Inner(s_15_18, s_15_21)
        let s_15_22: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_Inner(
            state,
            tracer,
            s_15_18,
            s_15_21,
        );
        // D s_15_23: call PAR_write(s_15_22)
        let s_15_23: () = PAR_write(state, tracer, s_15_22);
        // C s_15_24: const #() : ()
        let s_15_24: () = ();
        // S s_15_25: call PAR_read(s_15_24)
        let s_15_25: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_15_24);
        // D s_15_26: read-var sh:u8
        let s_15_26: u8 = fn_state.sh;
        // D s_15_27: call ReportedPARShareability(s_15_26)
        let s_15_27: u8 = ReportedPARShareability(state, tracer, s_15_26);
        // D s_15_28: call _update_PAR_Type_SH(s_15_25, s_15_27)
        let s_15_28: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_SH(
            state,
            tracer,
            s_15_25,
            s_15_27,
        );
        // D s_15_29: call PAR_write(s_15_28)
        let s_15_29: () = PAR_write(state, tracer, s_15_28);
        // C s_15_30: const #() : ()
        let s_15_30: () = ();
        // S s_15_31: call PAR_read(s_15_30)
        let s_15_31: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_15_30);
        // D s_15_32: write-var ga#23700 <= s_15_31
        fn_state.ga_23700 = s_15_31;
        // D s_15_33: read-var ga#23700.0:struct
        let s_15_33: u64 = fn_state.ga_23700._0;
        // C s_15_34: const #"Non-Faulting PAR" : str
        let s_15_34: &'static str = "Non-Faulting PAR";
        // S s_15_35: call __IMPDEF_bit(s_15_34)
        let s_15_35: bool = u__IMPDEF_bit(state, tracer, s_15_34);
        // C s_15_36: const #8s : i
        let s_15_36: i128 = 8;
        // D s_15_37: cast zx s_15_33 -> bv
        let s_15_37: Bits = Bits::new(s_15_33 as u128, 64u16);
        // S s_15_38: cast zx s_15_35 -> bv
        let s_15_38: Bits = Bits::new(s_15_35 as u128, 1u16);
        // C s_15_39: const #0s : i
        let s_15_39: i128 = 0;
        // C s_15_40: const #1u : u64
        let s_15_40: u64 = 1;
        // C s_15_41: cast zx s_15_40 -> bv
        let s_15_41: Bits = Bits::new(s_15_40 as u128, 64u16);
        // C s_15_42: lsl s_15_41 s_15_39
        let s_15_42: Bits = s_15_41 << s_15_39;
        // C s_15_43: sub s_15_42 s_15_41
        let s_15_43: Bits = ((s_15_42) - (s_15_41));
        // S s_15_44: and s_15_38 s_15_43
        let s_15_44: Bits = ((s_15_38) & (s_15_43));
        // S s_15_45: lsl s_15_44 s_15_36
        let s_15_45: Bits = s_15_44 << s_15_36;
        // C s_15_46: lsl s_15_43 s_15_36
        let s_15_46: Bits = s_15_43 << s_15_36;
        // C s_15_47: cmpl s_15_46
        let s_15_47: Bits = !s_15_46;
        // D s_15_48: and s_15_37 s_15_47
        let s_15_48: Bits = ((s_15_37) & (s_15_47));
        // D s_15_49: or s_15_48 s_15_45
        let s_15_49: Bits = ((s_15_48) | (s_15_45));
        // D s_15_50: cast reint s_15_49 -> u64
        let s_15_50: u64 = (s_15_49.value() as u64);
        // D s_15_51: call Mk_PAR_Type(s_15_50)
        let s_15_51: ProductType5c790c8ef59cc8b2 = Mk_PAR_Type(state, tracer, s_15_50);
        // D s_15_52: call PAR_write(s_15_51)
        let s_15_52: () = PAR_write(state, tracer, s_15_51);
        // C s_15_53: const #() : ()
        let s_15_53: () = ();
        // S s_15_54: call PAR_read(s_15_53)
        let s_15_54: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_15_53);
        // D s_15_55: read-var nsshadow#562:u8
        let s_15_55: bool = fn_state.nsshadow_562;
        // D s_15_56: call _update_PAR_Type_NS(s_15_54, s_15_55)
        let s_15_56: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_NS(
            state,
            tracer,
            s_15_54,
            s_15_55,
        );
        // D s_15_57: call PAR_write(s_15_56)
        let s_15_57: () = PAR_write(state, tracer, s_15_56);
        // C s_15_58: const #() : ()
        let s_15_58: () = ();
        // S s_15_59: call PAR_read(s_15_58)
        let s_15_59: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_15_58);
        // D s_15_60: write-var ga#23710 <= s_15_59
        fn_state.ga_23710 = s_15_59;
        // D s_15_61: read-var addrdesc.2:struct
        let s_15_61: ProductTypef170cab34335b70c = fn_state.addrdesc._2;
        // D s_15_62: write-var ga#23707 <= s_15_61
        fn_state.ga_23707 = s_15_61;
        // D s_15_63: read-var ga#23707.5:struct
        let s_15_63: u32 = fn_state.ga_23707._5;
        // C s_15_64: const #2u : u32
        let s_15_64: u32 = 2;
        // D s_15_65: cmp-eq s_15_63 s_15_64
        let s_15_65: bool = ((s_15_63) == (s_15_64));
        // N s_15_66: branch s_15_65 b18 b16
        if s_15_65 {
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
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var ga#23711 <= s_16_0
        fn_state.ga_23711 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var ga#23710:struct
        let s_17_0: ProductType5c790c8ef59cc8b2 = fn_state.ga_23710;
        // D s_17_1: read-var ga#23711:u8
        let s_17_1: bool = fn_state.ga_23711;
        // D s_17_2: call _update_PAR_Type_NOS(s_17_0, s_17_1)
        let s_17_2: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_NOS(
            state,
            tracer,
            s_17_0,
            s_17_1,
        );
        // D s_17_3: call PAR_write(s_17_2)
        let s_17_3: () = PAR_write(state, tracer, s_17_2);
        // C s_17_4: const #() : ()
        let s_17_4: () = ();
        // S s_17_5: call PAR_read(s_17_4)
        let s_17_5: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_17_4);
        // C s_17_6: const #0u : u8
        let s_17_6: bool = false;
        // S s_17_7: call _update_PAR_Type_LPAE(s_17_5, s_17_6)
        let s_17_7: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_LPAE(
            state,
            tracer,
            s_17_5,
            s_17_6,
        );
        // S s_17_8: call PAR_write(s_17_7)
        let s_17_8: () = PAR_write(state, tracer, s_17_7);
        // C s_17_9: const #() : ()
        let s_17_9: () = ();
        // S s_17_10: call PAR_read(s_17_9)
        let s_17_10: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_17_9);
        // D s_17_11: read-var addrdesc.3:struct
        let s_17_11: ProductTypeda0231e9dc169f81 = fn_state.addrdesc._3;
        // D s_17_12: write-var ga#23715 <= s_17_11
        fn_state.ga_23715 = s_17_11;
        // D s_17_13: read-var ga#23715.0:struct
        let s_17_13: u64 = fn_state.ga_23715._0;
        // C s_17_14: const #12s : i
        let s_17_14: i128 = 12;
        // D s_17_15: cast zx s_17_13 -> bv
        let s_17_15: Bits = Bits::new(s_17_13 as u128, 56u16);
        // C s_17_16: const #1s : i64
        let s_17_16: i64 = 1;
        // C s_17_17: cast zx s_17_16 -> i
        let s_17_17: i128 = (i128::try_from(s_17_16).unwrap());
        // C s_17_18: const #27s : i
        let s_17_18: i128 = 27;
        // C s_17_19: add s_17_18 s_17_17
        let s_17_19: i128 = (s_17_18 + s_17_17);
        // D s_17_20: bit-extract s_17_15 s_17_14 s_17_19
        let s_17_20: Bits = (Bits::new(
            ((s_17_15) >> (s_17_14)).value(),
            u16::try_from(s_17_19).unwrap(),
        ));
        // D s_17_21: cast reint s_17_20 -> u28
        let s_17_21: u32 = (s_17_20.value() as u32);
        // D s_17_22: call _update_PAR_Type_PA(s_17_10, s_17_21)
        let s_17_22: ProductType5c790c8ef59cc8b2 = u_update_PAR_Type_PA(
            state,
            tracer,
            s_17_10,
            s_17_21,
        );
        // D s_17_23: call PAR_write(s_17_22)
        let s_17_23: () = PAR_write(state, tracer, s_17_22);
        // N s_17_24: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var ga#23711 <= s_18_0
        fn_state.ga_23711 = s_18_0;
        // N s_18_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: u8 = 1;
        // D s_19_1: write-var sh <= s_19_0
        fn_state.sh = s_19_0;
        // N s_19_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var nsshadow#562 <= s_20_0
        fn_state.nsshadow_562 = s_20_0;
        // N s_20_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call __UNKNOWN_bit(s_22_0)
        let s_22_1: bool = u__UNKNOWN_bit(state, tracer, s_22_0);
        // N s_22_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #2u : u32
        let s_23_0: u32 = 2;
        // D s_23_1: write-var addrdesc.2.5 <= s_23_0
        fn_state.addrdesc._2._5 = s_23_0;
        // N s_23_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var addrdesc.2:struct
        let s_24_0: ProductTypef170cab34335b70c = fn_state.addrdesc._2;
        // D s_24_1: write-var ga#23673 <= s_24_0
        fn_state.ga_23673 = s_24_0;
        // D s_24_2: read-var ga#23673.4:struct
        let s_24_2: ProductType183e6678e5239c85 = fn_state.ga_23673._4;
        // D s_24_3: write-var ga#23674 <= s_24_2
        fn_state.ga_23674 = s_24_2;
        // D s_24_4: read-var ga#23674.0:struct
        let s_24_4: u8 = fn_state.ga_23674._0;
        // D s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 2u16);
        // C s_24_6: const #464u : u32
        let s_24_6: u32 = 464;
        // D s_24_7: read-reg s_24_6:u8
        let s_24_7: u8 = {
            let value = state.read_register::<u8>(s_24_6 as isize);
            tracer.read_register(s_24_6 as isize, value);
            value
        };
        // D s_24_8: cast zx s_24_7 -> bv
        let s_24_8: Bits = Bits::new(s_24_7 as u128, 2u16);
        // D s_24_9: cmp-eq s_24_5 s_24_8
        let s_24_9: bool = ((s_24_5) == (s_24_8));
        // D s_24_10: write-var gs#30414 <= s_24_9
        fn_state.gs_30414 = s_24_9;
        // N s_24_11: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#30415 <= s_25_0
        fn_state.gs_30415 = s_25_0;
        // N s_25_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}
